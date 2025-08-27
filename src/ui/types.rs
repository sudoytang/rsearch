#[derive(Debug, Clone, Copy, PartialEq)]
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
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub index: usize,
    pub offset: usize,
}