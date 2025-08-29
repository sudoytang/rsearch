use strum_macros::EnumIter;

#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, EnumIter)]
pub enum SearchType {
    Bit8,
    Bit16,
    Bit32,
    Bit64,
    Bytes,
    String,
}

impl std::fmt::Display for SearchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchType::Bit8 => write!(f, "8-Bit"),
            SearchType::Bit16 => write!(f, "16-Bit"),
            SearchType::Bit32 => write!(f, "32-Bit"),
            SearchType::Bit64 => write!(f, "64-Bit"),
            SearchType::Bytes => write!(f, "Bytes"),
            SearchType::String => write!(f, "String"),
        }
    }
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

impl std::fmt::Display for Encoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug, Clone, Copy)]
pub struct SearchResult {
    pub index: usize,
    pub offset: usize,
}