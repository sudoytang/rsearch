use strum_macros::EnumIter;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
pub enum SearchType {
    Bit8,
    Bit16,
    Bit32,
    Bit64,
    Bytes,
    String,
}

impl SearchType {
    pub fn is_endianness_enabled(&self) -> bool {
        matches!(self, SearchType::Bit16 | SearchType::Bit32 | SearchType::Bit64)
    }

    pub fn is_signedness_enabled(&self) -> bool {
        matches!(self, SearchType::Bit8 | SearchType::Bit16 | SearchType::Bit32 | SearchType::Bit64)
    }

    pub fn is_encoding_enabled(&self) -> bool {
        matches!(self, SearchType::String)
    }
}
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
pub enum Encoding {
    UTF8,
    /* ... */
}


#[derive(Debug, Clone, Copy)]
pub struct SearchResult {
    pub index: usize,
    pub offset: usize,
}