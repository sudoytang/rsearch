use std::{error::Error, fmt::Display};



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BytesParseKind {
    Hex,        // 1A2B3C4D
    Base64,     // aGVsbG8gd29ybGQ= for hello world
    Escaped     // \x01\x02\x03\r\n\b\t
}

impl Display for BytesParseKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BytesParseKind::Hex => write!(f, "Hex"),
            BytesParseKind::Base64 => write!(f, "Base64"),
            BytesParseKind::Escaped => write!(f, "Escaped"),
        }
    }
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BytesParserError {
    reason: String,
    kind: BytesParseKind,
    input: String,
}

impl Display for BytesParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot parse '{}' into bytes of {} type:", self.input, self.kind)?;
        write!(f, "{}", self.reason)
    }
}

impl Error for BytesParserError {}

impl BytesParserError {
    fn new(kind: BytesParseKind, input: &str, reason: String) -> Self {
        Self {
            reason,
            kind,
            input: input.to_string()
        }
    }
}

pub trait BytesParser {
    fn parse(&self, input: &str) -> Result<Vec<u8>, BytesParserError>;
}

pub struct HexParser;

impl BytesParser for HexParser {
    fn parse(&self, input: &str) -> Result<Vec<u8>, BytesParserError> {
        // Parse hex string like "41 42 43" or "414243"
        let cleaned = input.replace(" ", "").replace("0x", "");
        eprintln!("{cleaned}");
        if !cleaned.len().is_multiple_of(2) {
            return Err(BytesParserError::new(BytesParseKind::Hex, input, format!("Hex string must have even number of characters, got {}", cleaned.len())));
        }

        let mut bytes = Vec::new();
        for i in (0..cleaned.len()).step_by(2) {
            let hex_byte = &cleaned[i..i + 2];
            let byte = u8::from_str_radix(hex_byte, 16).map_err(|_| {
                BytesParserError::new(BytesParseKind::Hex, input, format!("Invalid hex byte #{}: {}", i / 2, hex_byte))
            })?;
            bytes.push(byte);
        }
        Ok(bytes)
    }
}

pub struct Base64Parser;

impl BytesParser for Base64Parser {
    fn parse(&self, input: &str) -> Result<Vec<u8>, BytesParserError> {
        use base64::{Engine as _, engine::{general_purpose}};
        
        let trimmed_input = input.trim();
        
        // Try standard base64 first
        if let Ok(bytes) = general_purpose::STANDARD.decode(trimmed_input) {
            return Ok(bytes);
        }
        
        // Try URL-safe base64 if standard fails
        if let Ok(bytes) = general_purpose::URL_SAFE.decode(trimmed_input) {
            return Ok(bytes);
        }
        
        // Try standard base64 without padding
        if let Ok(bytes) = general_purpose::STANDARD_NO_PAD.decode(trimmed_input) {
            return Ok(bytes);
        }
        
        // Try URL-safe base64 without padding
        if let Ok(bytes) = general_purpose::URL_SAFE_NO_PAD.decode(trimmed_input) {
            return Ok(bytes);
        }
        
        // If all attempts fail, return error
        Err(BytesParserError::new(
            BytesParseKind::Base64, 
            input, 
            "Invalid base64 string. Tried standard and URL-safe variants with and without padding.".to_string()
        ))
    }
}

pub struct EscapedParser;

impl BytesParser for EscapedParser {
    fn parse(&self, input: &str) -> Result<Vec<u8>, BytesParserError> {
        use escape_bytes::unescape;
        
        let trimmed_input = input.trim();
        
        // Use escape-bytes crate to parse escaped string
        match unescape(trimmed_input.as_bytes()) {
            Ok(bytes) => Ok(bytes),
            Err(err) => Err(BytesParserError::new(
                BytesParseKind::Escaped,
                input,
                format!("Failed to parse escaped string: {:?}", err)
            ))
        }
    }
}

