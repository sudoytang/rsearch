use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntParserErrorKind {
    Empty,
    Invalid,
    Overflow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntParserError {
    kind: IntParserErrorKind,
    input: String,
}

impl Display for IntParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            IntParserErrorKind::Empty => write!(f, "Cannot parse empty string into an integer."),
            IntParserErrorKind::Invalid => {
                write!(f, "Cannot parse {} into an integer.", self.input)
            }
            IntParserErrorKind::Overflow => {
                write!(
                    f,
                    "{} is too large/small to be intepreted as given integer type.",
                    self.input
                )
            }
        }
    }
}

impl Error for IntParserError {}

impl IntParserError {
    fn new(kind: IntParserErrorKind, input: &str) -> Self {
        Self {
            kind,
            input: input.to_string(),
        }
    }
}

pub struct IntParser;

impl IntParser {
    /// Helper function to detect the base and extract the numeric part
    fn parse_base_and_number(input: &str) -> (u32, &str) {
        if input.len() >= 2 {
            let prefix = &input[..2];
            match prefix.to_lowercase().as_str() {
                "0x" => (16, &input[2..]),
                "0b" => (2, &input[2..]),
                "0o" => (8, &input[2..]),
                _ => (10, input),
            }
        } else {
            (10, input)
        }
    }

    /// Parse a string into u8
    pub fn parse_u8(input: &str) -> Result<u8, IntParserError> {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return Err(IntParserError::new(IntParserErrorKind::Empty, input));
        }

        let (base, number_part) = Self::parse_base_and_number(trimmed);

        if number_part.is_empty() {
            return Err(IntParserError::new(IntParserErrorKind::Invalid, input));
        }

        let result = match base {
            10 => u8::from_str_radix(number_part, 10),
            16 => u8::from_str_radix(number_part, 16),
            2 => u8::from_str_radix(number_part, 2),
            8 => u8::from_str_radix(number_part, 8),
            _ => unreachable!(),
        };

        result.map_err(|err| match err.kind() {
            std::num::IntErrorKind::PosOverflow | std::num::IntErrorKind::NegOverflow => {
                IntParserError::new(IntParserErrorKind::Overflow, input)
            }
            _ => IntParserError::new(IntParserErrorKind::Invalid, input),
        })
    }

    /// Parse a string into i8
    pub fn parse_i8(input: &str) -> Result<i8, IntParserError> {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return Err(IntParserError::new(IntParserErrorKind::Empty, input));
        }

        let (base, number_part) = Self::parse_base_and_number(trimmed);

        if number_part.is_empty() {
            return Err(IntParserError::new(IntParserErrorKind::Invalid, input));
        }

        let result = match base {
            10 => i8::from_str_radix(number_part, 10),
            16 => i8::from_str_radix(number_part, 16),
            2 => i8::from_str_radix(number_part, 2),
            8 => i8::from_str_radix(number_part, 8),
            _ => unreachable!(),
        };

        result.map_err(|err| match err.kind() {
            std::num::IntErrorKind::PosOverflow | std::num::IntErrorKind::NegOverflow => {
                IntParserError::new(IntParserErrorKind::Overflow, input)
            }
            _ => IntParserError::new(IntParserErrorKind::Invalid, input),
        })
    }

    /// Parse a string into u16
    pub fn parse_u16(input: &str) -> Result<u16, IntParserError> {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return Err(IntParserError::new(IntParserErrorKind::Empty, input));
        }

        let (base, number_part) = Self::parse_base_and_number(trimmed);

        if number_part.is_empty() {
            return Err(IntParserError::new(IntParserErrorKind::Invalid, input));
        }

        let result = match base {
            10 => u16::from_str_radix(number_part, 10),
            16 => u16::from_str_radix(number_part, 16),
            2 => u16::from_str_radix(number_part, 2),
            8 => u16::from_str_radix(number_part, 8),
            _ => unreachable!(),
        };

        result.map_err(|err| match err.kind() {
            std::num::IntErrorKind::PosOverflow | std::num::IntErrorKind::NegOverflow => {
                IntParserError::new(IntParserErrorKind::Overflow, input)
            }
            _ => IntParserError::new(IntParserErrorKind::Invalid, input),
        })
    }

    /// Parse a string into i16
    pub fn parse_i16(input: &str) -> Result<i16, IntParserError> {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return Err(IntParserError::new(IntParserErrorKind::Empty, input));
        }

        let (base, number_part) = Self::parse_base_and_number(trimmed);

        if number_part.is_empty() {
            return Err(IntParserError::new(IntParserErrorKind::Invalid, input));
        }

        let result = match base {
            10 => i16::from_str_radix(number_part, 10),
            16 => i16::from_str_radix(number_part, 16),
            2 => i16::from_str_radix(number_part, 2),
            8 => i16::from_str_radix(number_part, 8),
            _ => unreachable!(),
        };

        result.map_err(|err| match err.kind() {
            std::num::IntErrorKind::PosOverflow | std::num::IntErrorKind::NegOverflow => {
                IntParserError::new(IntParserErrorKind::Overflow, input)
            }
            _ => IntParserError::new(IntParserErrorKind::Invalid, input),
        })
    }

    /// Parse a string into u32
    pub fn parse_u32(input: &str) -> Result<u32, IntParserError> {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return Err(IntParserError::new(IntParserErrorKind::Empty, input));
        }

        let (base, number_part) = Self::parse_base_and_number(trimmed);

        if number_part.is_empty() {
            return Err(IntParserError::new(IntParserErrorKind::Invalid, input));
        }

        let result = match base {
            10 => u32::from_str_radix(number_part, 10),
            16 => u32::from_str_radix(number_part, 16),
            2 => u32::from_str_radix(number_part, 2),
            8 => u32::from_str_radix(number_part, 8),
            _ => unreachable!(),
        };

        result.map_err(|err| match err.kind() {
            std::num::IntErrorKind::PosOverflow | std::num::IntErrorKind::NegOverflow => {
                IntParserError::new(IntParserErrorKind::Overflow, input)
            }
            _ => IntParserError::new(IntParserErrorKind::Invalid, input),
        })
    }

    /// Parse a string into i32
    pub fn parse_i32(input: &str) -> Result<i32, IntParserError> {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return Err(IntParserError::new(IntParserErrorKind::Empty, input));
        }

        let (base, number_part) = Self::parse_base_and_number(trimmed);

        if number_part.is_empty() {
            return Err(IntParserError::new(IntParserErrorKind::Invalid, input));
        }

        let result = match base {
            10 => i32::from_str_radix(number_part, 10),
            16 => i32::from_str_radix(number_part, 16),
            2 => i32::from_str_radix(number_part, 2),
            8 => i32::from_str_radix(number_part, 8),
            _ => unreachable!(),
        };

        result.map_err(|err| match err.kind() {
            std::num::IntErrorKind::PosOverflow | std::num::IntErrorKind::NegOverflow => {
                IntParserError::new(IntParserErrorKind::Overflow, input)
            }
            _ => IntParserError::new(IntParserErrorKind::Invalid, input),
        })
    }

    /// Parse a string into u64
    pub fn parse_u64(input: &str) -> Result<u64, IntParserError> {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return Err(IntParserError::new(IntParserErrorKind::Empty, input));
        }

        let (base, number_part) = Self::parse_base_and_number(trimmed);

        if number_part.is_empty() {
            return Err(IntParserError::new(IntParserErrorKind::Invalid, input));
        }

        let result = match base {
            10 => u64::from_str_radix(number_part, 10),
            16 => u64::from_str_radix(number_part, 16),
            2 => u64::from_str_radix(number_part, 2),
            8 => u64::from_str_radix(number_part, 8),
            _ => unreachable!(),
        };

        result.map_err(|err| match err.kind() {
            std::num::IntErrorKind::PosOverflow | std::num::IntErrorKind::NegOverflow => {
                IntParserError::new(IntParserErrorKind::Overflow, input)
            }
            _ => IntParserError::new(IntParserErrorKind::Invalid, input),
        })
    }

    /// Parse a string into i64
    pub fn parse_i64(input: &str) -> Result<i64, IntParserError> {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return Err(IntParserError::new(IntParserErrorKind::Empty, input));
        }

        let (base, number_part) = Self::parse_base_and_number(trimmed);

        if number_part.is_empty() {
            return Err(IntParserError::new(IntParserErrorKind::Invalid, input));
        }

        let result = match base {
            10 => i64::from_str_radix(number_part, 10),
            16 => i64::from_str_radix(number_part, 16),
            2 => i64::from_str_radix(number_part, 2),
            8 => i64::from_str_radix(number_part, 8),
            _ => unreachable!(),
        };

        result.map_err(|err| match err.kind() {
            std::num::IntErrorKind::PosOverflow | std::num::IntErrorKind::NegOverflow => {
                IntParserError::new(IntParserErrorKind::Overflow, input)
            }
            _ => IntParserError::new(IntParserErrorKind::Invalid, input),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_u8_decimal() {
        assert_eq!(IntParser::parse_u8("123").unwrap(), 123u8);
        assert_eq!(IntParser::parse_u8("0").unwrap(), 0u8);
        assert_eq!(IntParser::parse_u8("255").unwrap(), 255u8);
        assert_eq!(IntParser::parse_u8("  42  ").unwrap(), 42u8);
    }

    #[test]
    fn test_parse_u8_hex() {
        assert_eq!(IntParser::parse_u8("0xFF").unwrap(), 255u8);
        assert_eq!(IntParser::parse_u8("0x0").unwrap(), 0u8);
        assert_eq!(IntParser::parse_u8("0xa").unwrap(), 10u8);
        assert_eq!(IntParser::parse_u8("0XFF").unwrap(), 255u8);
    }

    #[test]
    fn test_parse_u8_binary() {
        assert_eq!(IntParser::parse_u8("0b11111111").unwrap(), 255u8);
        assert_eq!(IntParser::parse_u8("0b0").unwrap(), 0u8);
        assert_eq!(IntParser::parse_u8("0B1010").unwrap(), 10u8);
    }

    #[test]
    fn test_parse_u8_octal() {
        assert_eq!(IntParser::parse_u8("0o377").unwrap(), 255u8);
        assert_eq!(IntParser::parse_u8("0o0").unwrap(), 0u8);
        assert_eq!(IntParser::parse_u8("0O12").unwrap(), 10u8);
    }

    #[test]
    fn test_parse_u8_errors() {
        assert!(matches!(
            IntParser::parse_u8("").unwrap_err().kind,
            IntParserErrorKind::Empty
        ));
        assert!(matches!(
            IntParser::parse_u8("   ").unwrap_err().kind,
            IntParserErrorKind::Empty
        ));
        assert!(matches!(
            IntParser::parse_u8("256").unwrap_err().kind,
            IntParserErrorKind::Overflow
        ));
        assert!(matches!(
            IntParser::parse_u8("abc").unwrap_err().kind,
            IntParserErrorKind::Invalid
        ));
        assert!(matches!(
            IntParser::parse_u8("0x").unwrap_err().kind,
            IntParserErrorKind::Invalid
        ));
        assert!(matches!(
            IntParser::parse_u8("0b").unwrap_err().kind,
            IntParserErrorKind::Invalid
        ));
        assert!(matches!(
            IntParser::parse_u8("0o").unwrap_err().kind,
            IntParserErrorKind::Invalid
        ));
    }

    #[test]
    fn test_parse_i8_negative() {
        assert_eq!(IntParser::parse_i8("-128").unwrap(), -128i8);
        assert_eq!(IntParser::parse_i8("127").unwrap(), 127i8);
        assert_eq!(IntParser::parse_i8("-1").unwrap(), -1i8);
    }

    #[test]
    fn test_all_types_basic() {
        // Test basic functionality for all types
        assert_eq!(IntParser::parse_u16("65535").unwrap(), 65535u16);
        assert_eq!(IntParser::parse_i16("-32768").unwrap(), -32768i16);
        assert_eq!(IntParser::parse_u32("4294967295").unwrap(), 4294967295u32);
        assert_eq!(IntParser::parse_i32("-2147483648").unwrap(), -2147483648i32);
        assert_eq!(
            IntParser::parse_u64("18446744073709551615").unwrap(),
            18446744073709551615u64
        );
        assert_eq!(
            IntParser::parse_i64("-9223372036854775808").unwrap(),
            -9223372036854775808i64
        );

        // Test hex
        assert_eq!(IntParser::parse_u16("0xFFFF").unwrap(), 65535u16);
        assert_eq!(IntParser::parse_u32("0xFFFFFFFF").unwrap(), 4294967295u32);

        // Test binary
        assert_eq!(IntParser::parse_u16("0b1111111111111111").unwrap(), 65535u16);

        // Test octal
        assert_eq!(IntParser::parse_u16("0o177777").unwrap(), 65535u16);
    }

    #[test]
    fn test_whitespace_handling() {
        // Test that leading/trailing whitespaces are ignored
        assert_eq!(IntParser::parse_u32("  123  ").unwrap(), 123u32);
        assert_eq!(IntParser::parse_u32("\t456\n").unwrap(), 456u32);
        assert_eq!(IntParser::parse_i32("  -789  ").unwrap(), -789i32);
        assert_eq!(IntParser::parse_u32("  0xFF  ").unwrap(), 255u32);
        assert_eq!(IntParser::parse_u32("  0b1010  ").unwrap(), 10u32);
        assert_eq!(IntParser::parse_u32("  0o17  ").unwrap(), 15u32);
    }

    #[test]
    fn test_u8_overflow() {
        // u8 max is 255
        assert!(matches!(IntParser::parse_u8("256").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_u8("300").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_u8("0x100").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_u8("0b100000000").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_u8("0o400").unwrap_err().kind, IntParserErrorKind::Overflow));
        // Negative values should be invalid for unsigned types
        assert!(matches!(IntParser::parse_u8("-1").unwrap_err().kind, IntParserErrorKind::Invalid));
    }

    #[test]
    fn test_i8_overflow_underflow() {
        // i8 range is -128 to 127
        assert!(matches!(IntParser::parse_i8("128").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_i8("200").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_i8("-129").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_i8("-200").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_i8("0x80").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_i8("0b10000000").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_i8("0o200").unwrap_err().kind, IntParserErrorKind::Overflow));
    }

    #[test]
    fn test_u16_overflow() {
        // u16 max is 65535
        assert!(matches!(IntParser::parse_u16("65536").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_u16("100000").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_u16("0x10000").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_u16("0b10000000000000000").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_u16("0o200000").unwrap_err().kind, IntParserErrorKind::Overflow));
        // Negative values should be invalid for unsigned types
        assert!(matches!(IntParser::parse_u16("-1").unwrap_err().kind, IntParserErrorKind::Invalid));
    }

    #[test]
    fn test_i16_overflow_underflow() {
        // i16 range is -32768 to 32767
        assert!(matches!(IntParser::parse_i16("32768").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_i16("50000").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_i16("-32769").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_i16("-50000").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_i16("0x8000").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_i16("0b1000000000000000").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_i16("0o100000").unwrap_err().kind, IntParserErrorKind::Overflow));
    }

    #[test]
    fn test_u32_overflow() {
        // u32 max is 4294967295
        assert!(matches!(IntParser::parse_u32("4294967296").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_u32("5000000000").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_u32("0x100000000").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_u32("0b100000000000000000000000000000000").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_u32("0o40000000000").unwrap_err().kind, IntParserErrorKind::Overflow));
        // Negative values should be invalid for unsigned types
        assert!(matches!(IntParser::parse_u32("-1").unwrap_err().kind, IntParserErrorKind::Invalid));
    }

    #[test]
    fn test_i32_overflow_underflow() {
        // i32 range is -2147483648 to 2147483647
        assert!(matches!(IntParser::parse_i32("2147483648").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_i32("3000000000").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_i32("-2147483649").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_i32("-3000000000").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_i32("0x80000000").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_i32("0b10000000000000000000000000000000").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_i32("0o20000000000").unwrap_err().kind, IntParserErrorKind::Overflow));
    }

    #[test]
    fn test_u64_overflow() {
        // u64 max is 18446744073709551615
        assert!(matches!(IntParser::parse_u64("18446744073709551616").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_u64("20000000000000000000").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_u64("0x10000000000000000").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_u64("0b10000000000000000000000000000000000000000000000000000000000000000").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_u64("0o2000000000000000000000").unwrap_err().kind, IntParserErrorKind::Overflow));
        // Negative values should be invalid for unsigned types
        assert!(matches!(IntParser::parse_u64("-1").unwrap_err().kind, IntParserErrorKind::Invalid));
    }

    #[test]
    fn test_i64_overflow_underflow() {
        // i64 range is -9223372036854775808 to 9223372036854775807
        assert!(matches!(IntParser::parse_i64("9223372036854775808").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_i64("10000000000000000000").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_i64("-9223372036854775809").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_i64("-10000000000000000000").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_i64("0x8000000000000000").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_i64("0b1000000000000000000000000000000000000000000000000000000000000000").unwrap_err().kind, IntParserErrorKind::Overflow));
        assert!(matches!(IntParser::parse_i64("0o1000000000000000000000").unwrap_err().kind, IntParserErrorKind::Overflow));
    }

    #[test]
    fn test_boundary_values() {
        // Test exact boundary values that should work
        assert_eq!(IntParser::parse_u8("255").unwrap(), 255u8);
        assert_eq!(IntParser::parse_i8("127").unwrap(), 127i8);
        assert_eq!(IntParser::parse_i8("-128").unwrap(), -128i8);
        
        assert_eq!(IntParser::parse_u16("65535").unwrap(), 65535u16);
        assert_eq!(IntParser::parse_i16("32767").unwrap(), 32767i16);
        assert_eq!(IntParser::parse_i16("-32768").unwrap(), -32768i16);
        
        assert_eq!(IntParser::parse_u32("4294967295").unwrap(), 4294967295u32);
        assert_eq!(IntParser::parse_i32("2147483647").unwrap(), 2147483647i32);
        assert_eq!(IntParser::parse_i32("-2147483648").unwrap(), -2147483648i32);
        
        assert_eq!(IntParser::parse_u64("18446744073709551615").unwrap(), 18446744073709551615u64);
        assert_eq!(IntParser::parse_i64("9223372036854775807").unwrap(), 9223372036854775807i64);
        assert_eq!(IntParser::parse_i64("-9223372036854775808").unwrap(), -9223372036854775808i64);
    }

    #[test]
    fn test_hex_overflow() {
        // Test hex overflow for different types
        assert_eq!(IntParser::parse_u8("0xFF").unwrap(), 255u8);
        assert!(matches!(IntParser::parse_u8("0x100").unwrap_err().kind, IntParserErrorKind::Overflow));
        
        assert_eq!(IntParser::parse_u16("0xFFFF").unwrap(), 65535u16);
        assert!(matches!(IntParser::parse_u16("0x10000").unwrap_err().kind, IntParserErrorKind::Overflow));
        
        assert_eq!(IntParser::parse_u32("0xFFFFFFFF").unwrap(), 4294967295u32);
        assert!(matches!(IntParser::parse_u32("0x100000000").unwrap_err().kind, IntParserErrorKind::Overflow));
        
        assert_eq!(IntParser::parse_u64("0xFFFFFFFFFFFFFFFF").unwrap(), 18446744073709551615u64);
        assert!(matches!(IntParser::parse_u64("0x10000000000000000").unwrap_err().kind, IntParserErrorKind::Overflow));
    }

    #[test]
    fn test_binary_overflow() {
        // Test binary overflow for different types
        assert_eq!(IntParser::parse_u8("0b11111111").unwrap(), 255u8);
        assert!(matches!(IntParser::parse_u8("0b100000000").unwrap_err().kind, IntParserErrorKind::Overflow));
        
        assert_eq!(IntParser::parse_u16("0b1111111111111111").unwrap(), 65535u16);
        assert!(matches!(IntParser::parse_u16("0b10000000000000000").unwrap_err().kind, IntParserErrorKind::Overflow));
    }

    #[test]
    fn test_octal_overflow() {
        // Test octal overflow for different types
        assert_eq!(IntParser::parse_u8("0o377").unwrap(), 255u8);
        assert!(matches!(IntParser::parse_u8("0o400").unwrap_err().kind, IntParserErrorKind::Overflow));
        
        assert_eq!(IntParser::parse_u16("0o177777").unwrap(), 65535u16);
        assert!(matches!(IntParser::parse_u16("0o200000").unwrap_err().kind, IntParserErrorKind::Overflow));
    }
}
