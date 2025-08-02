use memchr::memmem;

pub enum Endianness {
    BigEndian,
    LittleEndian,
}

#[derive(Debug, Clone, Copy)]
pub enum SearchNeedle<'n> {
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    Bytes(&'n[u8]),
    Str(&'n str),
}

pub struct Search<'n> {
    endianness: Endianness,
    needle: SearchNeedle<'n>
}

pub enum SearchIter<'h> {
    Chr(memchr::Memchr<'h>),
    Mem(memmem::FindIter<'h, 'static>),
}

impl<'h> From<memchr::Memchr<'h>> for SearchIter<'h> {
    fn from(value: memchr::Memchr<'h>) -> Self {
        SearchIter::Chr(value)
    }
}

impl<'h, 'n> From<memmem::FindIter<'h, 'static>> for SearchIter<'h> {
    fn from(value: memmem::FindIter<'h, 'static>) -> Self {
        SearchIter::Mem(value)
    }
}

impl<'h> Iterator for SearchIter<'h> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SearchIter::Chr(c) => c.next(),
            SearchIter::Mem(m) => m.next()
        }
    }
}



pub fn search<'h, 'n>(haystack: &'h[u8], s: Search<'n>) -> SearchIter<'h> 
where
    'h: 'n,
{
    use Endianness::BigEndian as BE;
    use Endianness::LittleEndian as LE;
    use SearchNeedle::*;
    match (s.endianness, s.needle) {
        (_, U8(needle)) => memchr::memchr_iter(needle, haystack).into(),
        (_, I8(needle)) => memchr::memchr_iter(needle as u8, haystack).into(),
        (BE, U16(v)) => search_bytes(haystack, &v.to_be_bytes()),
        (LE, U16(v)) => search_bytes(haystack, &v.to_le_bytes()),
        (BE, I16(v)) => search_bytes(haystack, &v.to_be_bytes()),
        (LE, I16(v)) => search_bytes(haystack, &v.to_le_bytes()),
        (BE, U32(v)) => search_bytes(haystack, &v.to_be_bytes()),
        (LE, U32(v)) => search_bytes(haystack, &v.to_le_bytes()),
        (BE, I32(v)) => search_bytes(haystack, &v.to_be_bytes()),
        (LE, I32(v)) => search_bytes(haystack, &v.to_le_bytes()),
        (BE, U64(v)) => search_bytes(haystack, &v.to_be_bytes()),
        (LE, U64(v)) => search_bytes(haystack, &v.to_le_bytes()),
        (BE, I64(v)) => search_bytes(haystack, &v.to_be_bytes()),
        (LE, I64(v)) => search_bytes(haystack, &v.to_le_bytes()),
        (_, Bytes(needle)) => search_bytes(haystack, needle),
        (_, Str(needle)) => search_bytes(haystack, needle.as_bytes()),
    }
}


fn search_bytes<'h>(haystack: &'h [u8], needle: &[u8]) -> SearchIter<'h> {
    SearchIter::Mem(memmem::find_iter(haystack, needle).into_owned())
}
