use strum_macros::EnumIter;
use crate::ui::int_parse::IntParserError;
use std::{error::Error, fmt::Display};

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
        matches!(
            self,
            SearchType::Bit16 | SearchType::Bit32 | SearchType::Bit64
        )
    }

    pub fn is_signedness_enabled(&self) -> bool {
        matches!(
            self,
            SearchType::Bit8 | SearchType::Bit16 | SearchType::Bit32 | SearchType::Bit64
        )
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Selection {
    start: usize,
    end: usize,
    // Both end inclusive, end may be SMALLER than start.
    // (this implies that this type cannot express a null set)
}

impl Selection {
    pub fn new(offset: usize) -> Self {
        Self {
            start: offset,
            end: offset,
        }
    }

    pub fn range(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn lower(&self) -> usize {
        return usize::min(self.start, self.end);
    }

    pub fn upper(&self) -> usize {
        return usize::max(self.start, self.end);
    }

    pub fn contains(&self, offset: usize) -> bool {
        offset >= self.lower() && offset <= self.upper()
    }

    pub fn update_end(&mut self, end: usize) {
        self.end = end;
    }

    pub fn update_start(&mut self, start: usize) {
        self.start = start;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SearchResult {
    pub index: usize,
    pub offset: usize,
}

#[derive(Debug, Clone)]
pub enum InputParseError {
    IntParser(IntParserError),
    Other(String),
}

impl Display for InputParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputParseError::IntParser(err) => write!(f, "{}", err),
            InputParseError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for InputParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            InputParseError::IntParser(err) => Some(err),
            InputParseError::Other(_) => None,
        }
    }
}

impl From<IntParserError> for InputParseError {
    fn from(err: IntParserError) -> Self {
        InputParseError::IntParser(err)
    }
}

impl From<String> for InputParseError {
    fn from(msg: String) -> Self {
        InputParseError::Other(msg)
    }
}

impl From<&str> for InputParseError {
    fn from(msg: &str) -> Self {
        InputParseError::Other(msg.to_string())
    }
}
