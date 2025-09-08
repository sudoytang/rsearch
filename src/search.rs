use std::{
    sync::{Arc, mpsc},
    thread::{self, JoinHandle},
};

use color_eyre::{Result as EyreReult, eyre::eyre};
use memchr::memmem;
use memmap2::Mmap;

pub trait Haystack: Send + 'static {
    fn as_bytes(&self) -> &[u8];
}

impl Haystack for Vec<u8> {
    fn as_bytes(&self) -> &[u8] {
        self.as_ref()
    }
}

impl Haystack for &'static str {
    fn as_bytes(&self) -> &[u8] {
        self.as_ref()
    }
}

impl Haystack for &'static [u8] {
    fn as_bytes(&self) -> &[u8] {
        self
    }
}

impl Haystack for String {
    fn as_bytes(&self) -> &[u8] {
        self.as_ref()
    }
}

impl<const N: usize> Haystack for [u8; N] {
    fn as_bytes(&self) -> &[u8] {
        self
    }
}

impl<const N: usize> Haystack for Box<[u8; N]> {
    fn as_bytes(&self) -> &[u8] {
        self.as_slice()
    }
}

impl Haystack for Box<[u8]> {
    fn as_bytes(&self) -> &[u8] {
        self
    }
}

impl<const N: usize> Haystack for Arc<[u8; N]> {
    fn as_bytes(&self) -> &[u8] {
        self.as_slice()
    }
}

impl Haystack for Arc<[u8]> {
    fn as_bytes(&self) -> &[u8] {
        self
    }
}

impl Haystack for Arc<Vec<u8>> {
    fn as_bytes(&self) -> &[u8] {
        self
    }
}

impl Haystack for Mmap {
    fn as_bytes(&self) -> &[u8] {
        self
    }
}

impl Haystack for Arc<Mmap> {
    fn as_bytes(&self) -> &[u8] {
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endianness {
    BigEndian,
    LittleEndian,
}

#[derive(Debug, Clone, Copy)]
pub enum Needle<'n> {
    U8(u8),
    I8(i8),
    U16(Endianness, u16),
    I16(Endianness, i16),
    U32(Endianness, u32),
    I32(Endianness, i32),
    U64(Endianness, u64),
    I64(Endianness, i64),
    Bytes(&'n [u8]),
    Str(&'n str),
}

impl<'n> From<&'n str> for Needle<'n> {
    fn from(value: &'n str) -> Self {
        Self::Str(value)
    }
}

impl<'n> From<&'n [u8]> for Needle<'n> {
    fn from(value: &'n [u8]) -> Self {
        Self::Bytes(value)
    }
}

pub struct NeedleOwned {
    needle: Box<[u8]>,
}

impl NeedleOwned {
    pub fn from_data<T: Into<Box<[u8]>>>(data: T) -> Self {
        Self {
            needle: data.into(),
        }
    }

    pub fn byte_length(&self) -> usize {
        self.needle.len()
    }
}

impl<'n> From<Needle<'n>> for NeedleOwned {
    fn from(value: Needle<'n>) -> Self {
        use Endianness::BigEndian as BE;
        use Endianness::LittleEndian as LE;
        use Needle::*;
        let needle: Box<[u8]> = match value {
            U8(v) => Box::new([v]),
            I8(v) => Box::new([v as u8]),
            Bytes(v) => v.into(),
            Str(v) => v.as_bytes().into(),
            U16(BE, v) => Box::new(v.to_be_bytes()),
            U16(LE, v) => Box::new(v.to_le_bytes()),
            I16(BE, v) => Box::new(v.to_be_bytes()),
            I16(LE, v) => Box::new(v.to_le_bytes()),
            U32(BE, v) => Box::new(v.to_be_bytes()),
            U32(LE, v) => Box::new(v.to_le_bytes()),
            I32(BE, v) => Box::new(v.to_be_bytes()),
            I32(LE, v) => Box::new(v.to_le_bytes()),
            U64(BE, v) => Box::new(v.to_be_bytes()),
            U64(LE, v) => Box::new(v.to_le_bytes()),
            I64(BE, v) => Box::new(v.to_be_bytes()),
            I64(LE, v) => Box::new(v.to_le_bytes()),
        };
        Self { needle }
    }
}

pub struct AsyncSearch {
    join_handle: JoinHandle<()>,
    receiver: mpsc::Receiver<usize>,
}

pub enum SearchState {
    Pending,
    Finished,
}

impl AsyncSearch {
    pub fn create_from_owned<H>(haystack: H, needle: NeedleOwned) -> Self
    where
        H: Haystack,
    {
        let (tx, rx) = mpsc::channel();
        let join_handle = thread::spawn(move || {
            let hs = haystack.as_bytes();
            let it = memmem::find_iter(hs, &needle.needle);
            for n in it {
                if tx.send(n).is_err() {
                    break;
                }
            }
        });
        Self {
            join_handle,
            receiver: rx,
        }
    }

    pub fn create<'s, H, S>(haystack: H, s: S) -> Self
    where
        H: Haystack,
        S: Into<Needle<'s>>,
    {
        let s_owned: NeedleOwned = s.into().into();
        Self::create_from_owned(haystack, s_owned)
    }

    pub fn try_get(&self) -> Result<usize, SearchState> {
        self.receiver
            .try_recv()
            .map_err(|try_recv_err| match try_recv_err {
                mpsc::TryRecvError::Empty => SearchState::Pending,
                mpsc::TryRecvError::Disconnected => SearchState::Finished,
            })
    }

    pub fn drain<F>(&self, mut callback: F) -> SearchState
    where
        F: FnMut(usize) -> (),
    {
        loop {
            match self.try_get() {
                Ok(v) => callback(v),
                Err(e) => return e,
            }
        }
    }

    pub fn cancel(self) -> EyreReult<()> {
        drop(self.receiver);
        self.join_handle
            .join()
            .map_err(|_| eyre!("Sub-thread panicked"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_needle_owned_creation() {
        // Test basic types
        let needle_u8: NeedleOwned = Needle::U8(42).into();
        assert_eq!(needle_u8.needle.as_ref(), &[42]);

        let needle_i8: NeedleOwned = Needle::I8(-1).into();
        assert_eq!(needle_i8.needle.as_ref(), &[255]); // -1 as u8 is 255

        // Test endianness
        let needle_u16_le: NeedleOwned = Needle::U16(Endianness::LittleEndian, 0x1234).into();
        assert_eq!(needle_u16_le.needle.as_ref(), &[0x34, 0x12]);

        let needle_u16_be: NeedleOwned = Needle::U16(Endianness::BigEndian, 0x1234).into();
        assert_eq!(needle_u16_be.needle.as_ref(), &[0x12, 0x34]);

        // Test string
        let needle_str: NeedleOwned = Needle::Str("hello").into();
        assert_eq!(needle_str.needle.as_ref(), b"hello");

        // Test bytes
        let needle_bytes: NeedleOwned = Needle::Bytes(&[1, 2, 3, 4]).into();
        assert_eq!(needle_bytes.needle.as_ref(), &[1, 2, 3, 4]);
    }

    #[test]
    fn test_async_search_with_needle_owned() {
        let haystack = b"hello world hello universe";
        let needle = Needle::Str("hello");

        let search = AsyncSearch::create(haystack.as_slice(), needle);

        // Give it a moment to find results
        std::thread::sleep(std::time::Duration::from_millis(10));

        let mut results = Vec::new();
        loop {
            match search.try_get() {
                Ok(offset) => results.push(offset),
                Err(SearchState::Pending) => break,
                Err(SearchState::Finished) => break,
            }
        }

        // Should find "hello" at positions 0 and 12
        assert_eq!(results.len(), 2);
        assert!(results.contains(&0));
        assert!(results.contains(&12));
    }
}
