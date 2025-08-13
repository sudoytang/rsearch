use std::{
    sync::mpsc, thread::{
        self, 
        JoinHandle
    }
};

use memchr::memmem;
use color_eyre::{eyre::eyre, Result as EyreReult};

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
    Bytes(&'n[u8]),
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

struct NeedleOwned {
    needle: Box<[u8]>
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

    pub fn create<'s, H, S>(haystack: H, s: S) -> Self 
    where
        H: AsRef<[u8]> + Send + 'static,
        S: Into<Needle<'s>>,
    {
        let s_owned: NeedleOwned = s.into().into();
        let (tx, rx) = mpsc::channel();
        let join_handle = thread::spawn(move || {
            let hs = haystack.as_ref();
            let it = memmem::find_iter(hs, &s_owned.needle);
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

    pub fn try_get(&self) -> Result<usize, SearchState> {
        self.receiver.try_recv().map_err(|try_recv_err| {
            match try_recv_err {
                mpsc::TryRecvError::Empty => SearchState::Pending,
                mpsc::TryRecvError::Disconnected => SearchState::Finished,
            }
        })
    }

    pub fn drain<F>(&self, mut callback: F) -> SearchState
    where
        F: FnMut(usize) -> (),
    {
        loop {
            match self.try_get() {
                Ok(v) => callback(v),
                Err(e) => return e
            }
        }
    }

    pub fn cancel(self) -> EyreReult<()> {
        drop(self.receiver);
        self.join_handle.join().map_err(|_| {
            eyre!("Sub-thread panicked")
        })
    }
}
