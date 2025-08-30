use eframe::egui;
use egui_extras::{Column, TableBuilder};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Radix {
    Decimal,
    Hexadecimal,
    Binary,
    Octal,
}

impl std::fmt::Display for Radix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Radix::Decimal => write!(f, "Decimal"),
            Radix::Hexadecimal => write!(f, "Hexadecimal"),
            Radix::Binary => write!(f, "Binary"),
            Radix::Octal => write!(f, "Octal"),
        }
    }
}

pub struct DataInspector {
    little_endian: bool,
    radix: Radix,
}

impl DataInspector {

    const EOF_MSG: &'static str = "No Data";

    pub fn new() -> Self {
        Self {
            little_endian: true,
            radix: Radix::Decimal,
        }
    }

    fn format_number(value: u64, radix: Radix) -> String {
        match radix {
            Radix::Decimal => format!("{}", value),
            Radix::Hexadecimal => format!("0x{:X}", value),
            Radix::Binary => format!("0b{:b}", value),
            Radix::Octal => format!("0o{:o}", value),
        }
    }

    fn format_signed_number(value: i64, radix: Radix) -> String {
        match radix {
            Radix::Decimal => format!("{}", value),
            Radix::Hexadecimal => {
                if value < 0 {
                    format!("-0x{:X}", (-value) as u64)
                } else {
                    format!("0x{:X}", value as u64)
                }
            }
            Radix::Binary => {
                if value < 0 {
                    format!("-0b{:b}", (-value) as u64)
                } else {
                    format!("0b{:b}", value as u64)
                }
            }
            Radix::Octal => {
                if value < 0 {
                    format!("-0o{:o}", (-value) as u64)
                } else {
                    format!("0o{:o}", value as u64)
                }
            }
        }
    }

    fn format_float(value: f64) -> String {
        let abs_value = value.abs();

        if value.is_nan() {
            "NaN".to_string()
        } else if value.is_infinite() {
            if value.is_sign_positive() {
                "+Inf".to_string()
            } else {
                "-Inf".to_string()
            }
        } else if abs_value == 0.0 {
            "0.0".to_string()
        } else if abs_value >= 1e-4 && abs_value < 1e6 {
            // Use fixed-point notation for reasonable range
            let formatted = format!("{:.6}", value);
            // Remove trailing zeros after decimal point
            let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
            if trimmed.contains('.') {
                trimmed.to_string()
            } else {
                format!("{}.0", trimmed)
            }
        } else {
            // Use scientific notation for very small or very large numbers
            format!("{:.10e}", value)
        }
    }

    fn intepret_ascii(b: &[u8]) -> (String, String) {
        // 1) ASCII control characters and their names
        const ASCII_CTRL_NAMES: [&str; 33] = [
            "NUL", "SOH", "STX", "ETX", "EOT", "ENQ", "ACK", "BEL", "BS", "TAB", "LF", "VT", "FF",
            "CR", "SO", "SI", "DLE", "DC1", "DC2", "DC3", "DC4", "NAK", "SYN", "ETB", "CAN", "EM",
            "SUB", "ESC", "FS", "GS", "RS", "US", "DEL", // 0x7F
        ];

        // 2) Windows-1252 special mappings for 0x80–0x9F (None for unmapped)
        // Unmapped: 0x81, 0x8D, 0x8F, 0x90, 0x9D
        const WIN1252_80_9F: [Option<char>; 32] = [
            Some('€'),
            None,
            Some('‚'),
            Some('ƒ'),
            Some('„'),
            Some('…'),
            Some('†'),
            Some('‡'),
            Some('ˆ'),
            Some('‰'),
            Some('Š'),
            Some('‹'),
            Some('Œ'),
            None,
            Some('Ž'),
            None,
            None,
            Some('‘'),
            Some('’'),
            Some('“'),
            Some('”'),
            Some('•'),
            Some('–'),
            Some('—'),
            Some('˜'),
            Some('™'),
            Some('š'),
            Some('›'),
            Some('œ'),
            None,
            Some('ž'),
            Some('Ÿ'),
        ];

        // Helper function: wrap displayable character in single quotes
        fn quoted(c: char) -> String {
            format!("'{}'", c)
        }
        if b.is_empty() {
            return ("ASCII".into(), Self::EOF_MSG.into());
        }
        let b = b[0];
        match b {
            // ASCII control: 0x00–0x1F, and 0x7F (DEL)
            0x00..=0x1F | 0x7F => {
                let name = if b == 0x7F {
                    ASCII_CTRL_NAMES[32] // DEL
                } else {
                    ASCII_CTRL_NAMES[b as usize]
                };
                ("ASCII".to_string(), name.to_string())
            }

            // ASCII printable: 0x20–0x7E
            0x20..=0x7E => ("ASCII".to_string(), quoted(b as char)),

            // Key difference area: 0x80–0x9F
            0x80..=0x9F => {
                let mapped = WIN1252_80_9F[(b - 0x80) as usize];
                match mapped {
                    Some(ch) => ("Win1252".to_string(), quoted(ch)),
                    None => ("Win1252".to_string(), "\u{fffd}".to_string()),
                }
            }

            // 0xA0–0xFF: displayable characters in ISO-8859-1 (same as Win-1252)
            0xA0..=0xFF => {
                // In Rust, char::from(b) for 0xA0–0xFF gives U+00A0–U+00FF (Latin-1 Supplement)
                ("Win1252".to_string(), quoted(char::from(b)))
            }
        }
    }
    fn intepret_u8(b: &[u8], radix: Radix) -> (String, String) {
        (
            "u8".into(),
            if b.is_empty() {
                Self::EOF_MSG.into()
            } else {
                DataInspector::format_number(b[0] as u64, radix)
            },
        )
    }
    fn intepret_i8(b: &[u8], radix: Radix) -> (String, String) {
        (
            "i8".into(),
            if b.is_empty() {
                Self::EOF_MSG.into()
            } else {
                DataInspector::format_signed_number(b[0] as i8 as i64, radix)
            },
        )
    }
    fn intepret_u16(b: &[u8], radix: Radix, is_little_endian: bool) -> (String, String) {
        if b.len() < 2 {
            return ("u16".into(), Self::EOF_MSG.into());
        }
        let value = if is_little_endian {
            u16::from_le_bytes([b[0], b[1]])
        } else {
            u16::from_be_bytes([b[0], b[1]])
        };
        (
            "u16".into(),
            DataInspector::format_number(value as u64, radix),
        )
    }
    fn intepret_i16(b: &[u8], radix: Radix, is_little_endian: bool) -> (String, String) {
        if b.len() < 2 {
            return ("i16".into(), Self::EOF_MSG.into());
        }
        let value = if is_little_endian {
            i16::from_le_bytes([b[0], b[1]])
        } else {
            i16::from_be_bytes([b[0], b[1]])
        };
        (
            "i16".into(),
            DataInspector::format_signed_number(value as i64, radix),
        )
    }
    fn intepret_u24(b: &[u8], radix: Radix, is_little_endian: bool) -> (String, String) {
        if b.len() < 3 {
            return ("u24".into(), Self::EOF_MSG.into());
        }
        let value = if is_little_endian {
            (b[0] as u32) | ((b[1] as u32) << 8) | ((b[2] as u32) << 16)
        } else {
            ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | (b[2] as u32)
        };
        (
            "u24".into(),
            DataInspector::format_number(value as u64, radix),
        )
    }
    fn intepret_i24(b: &[u8], radix: Radix, is_little_endian: bool) -> (String, String) {
        if b.len() < 3 {
            return ("i24".into(), Self::EOF_MSG.into());
        }
        let value = if is_little_endian {
            (b[0] as u32) | ((b[1] as u32) << 8) | ((b[2] as u32) << 16)
        } else {
            ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | (b[2] as u32)
        };
        // Sign extend from 24-bit to 32-bit
        let signed_value = if value & 0x800000 != 0 {
            (value | 0xFF000000) as i32
        } else {
            value as i32
        };
        (
            "i24".into(),
            DataInspector::format_signed_number(signed_value as i64, radix),
        )
    }
    fn intepret_u32(b: &[u8], radix: Radix, is_little_endian: bool) -> (String, String) {
        if b.len() < 4 {
            return ("u32".into(), Self::EOF_MSG.into());
        }
        let value = if is_little_endian {
            u32::from_le_bytes([b[0], b[1], b[2], b[3]])
        } else {
            u32::from_be_bytes([b[0], b[1], b[2], b[3]])
        };
        (
            "u32".into(),
            DataInspector::format_number(value as u64, radix),
        )
    }
    fn intepret_i32(b: &[u8], radix: Radix, is_little_endian: bool) -> (String, String) {
        if b.len() < 4 {
            return ("i32".into(), Self::EOF_MSG.into());
        }
        let value = if is_little_endian {
            i32::from_le_bytes([b[0], b[1], b[2], b[3]])
        } else {
            i32::from_be_bytes([b[0], b[1], b[2], b[3]])
        };
        (
            "i32".into(),
            DataInspector::format_signed_number(value as i64, radix),
        )
    }
    fn intepret_u64(b: &[u8], radix: Radix, is_little_endian: bool) -> (String, String) {
        if b.len() < 8 {
            return ("u64".into(), Self::EOF_MSG.into());
        }
        let value = if is_little_endian {
            u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
        } else {
            u64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
        };
        ("u64".into(), DataInspector::format_number(value, radix))
    }
    fn intepret_i64(b: &[u8], radix: Radix, is_little_endian: bool) -> (String, String) {
        if b.len() < 8 {
            return ("i64".into(), Self::EOF_MSG.into());
        }
        let value = if is_little_endian {
            i64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
        } else {
            i64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
        };
        (
            "i64".into(),
            DataInspector::format_signed_number(value, radix),
        )
    }

    fn interpret_f16(b: &[u8], is_little_endian: bool) -> (String, String) {
        if b.len() < 2 {
            return ("f16".into(), Self::EOF_MSG.into());
        }
        let value = if is_little_endian {
            half::f16::from_le_bytes([b[0], b[1]])
        } else {
            half::f16::from_be_bytes([b[0], b[1]])
        }
        .to_f64();

        ("f16".into(), DataInspector::format_float(value))
    }
    fn interpret_bf16(b: &[u8], is_little_endian: bool) -> (String, String) {
        if b.len() < 2 {
            return ("bf16".into(), Self::EOF_MSG.into());
        }
        let value = if is_little_endian {
            half::bf16::from_le_bytes([b[0], b[1]])
        } else {
            half::bf16::from_be_bytes([b[0], b[1]])
        }
        .to_f64();
        ("bf16".into(), DataInspector::format_float(value))
    }
    fn interpret_f32(b: &[u8], is_little_endian: bool) -> (String, String) {
        if b.len() < 4 {
            return ("f32".into(), Self::EOF_MSG.into());
        }
        let value = if is_little_endian {
            f32::from_le_bytes([b[0], b[1], b[2], b[3]])
        } else {
            f32::from_be_bytes([b[0], b[1], b[2], b[3]])
        };
        ("f32".into(), DataInspector::format_float(value as f64))
    }
    fn interpret_f64(b: &[u8], is_little_endian: bool) -> (String, String) {
        if b.len() < 8 {
            return ("f64".into(), Self::EOF_MSG.into());
        }
        let value = if is_little_endian {
            f64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
        } else {
            f64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
        };
        ("f64".into(), DataInspector::format_float(value))
    }

    fn interpret_utf8(b: &[u8]) -> (String, String) {
        if b.is_empty() {
            return ("UTF-8".into(), Self::EOF_MSG.into());
        }

        // Determine how many bytes we need for the first UTF-8 character
        let first_byte = b[0];
        let expected_len = if first_byte & 0x80 == 0 {
            1 // ASCII
        } else if first_byte & 0xE0 == 0xC0 {
            2 // 2-byte sequence
        } else if first_byte & 0xF0 == 0xE0 {
            3 // 3-byte sequence
        } else if first_byte & 0xF8 == 0xF0 {
            4 // 4-byte sequence
        } else {
            // Invalid UTF-8 start byte
            return ("UTF-8".into(), "\u{FFFD}".into()); // Unicode replacement character
        };

        if b.len() < expected_len {
            return ("UTF-8".into(), Self::EOF_MSG.into());
        }

        match std::str::from_utf8(&b[..expected_len]) {
            Ok(s) => {
                if let Some(ch) = s.chars().next() {
                    (format!("UTF-8({})", expected_len), format!("'{}'", ch))
                } else {
                    ("UTF-8".into(), "\u{FFFD}".into())
                }
            }
            Err(_) => ("UTF-8".into(), "\u{FFFD}".into()),
        }
    }

    fn interpret_utf16(b: &[u8], is_little_endian: bool) -> (String, String) {
        if b.len() < 2 {
            return ("UTF-16".into(), Self::EOF_MSG.into());
        }

        let first_unit = if is_little_endian {
            u16::from_le_bytes([b[0], b[1]])
        } else {
            u16::from_be_bytes([b[0], b[1]])
        };

        // Check if it's a high surrogate (needs another 16-bit unit)
        let (units_needed, utf16_data) = if (0xD800..=0xDBFF).contains(&first_unit) {
            // High surrogate, need low surrogate
            if b.len() < 4 {
                return ("UTF-16".into(), Self::EOF_MSG.into());
            }
            let second_unit = if is_little_endian {
                u16::from_le_bytes([b[2], b[3]])
            } else {
                u16::from_be_bytes([b[2], b[3]])
            };
            (2, vec![first_unit, second_unit])
        } else {
            (1, vec![first_unit])
        };

        match String::from_utf16(&utf16_data) {
            Ok(s) => {
                if let Some(ch) = s.chars().next() {
                    (format!("UTF-16({})", units_needed), format!("'{}'", ch))
                } else {
                    ("UTF-16".into(), "\u{FFFD}".into())
                }
            }
            Err(_) => ("UTF-16".into(), "\u{FFFD}".into()),
        }
    }

    fn interpret_utf32(b: &[u8], is_little_endian: bool) -> (String, String) {
        if b.len() < 4 {
            return ("UTF-32".into(), Self::EOF_MSG.into());
        }

        let code_point = if is_little_endian {
            u32::from_le_bytes([b[0], b[1], b[2], b[3]])
        } else {
            u32::from_be_bytes([b[0], b[1], b[2], b[3]])
        };

        match char::from_u32(code_point) {
            Some(ch) => ("UTF-32".into(), format!("'{}'", ch)),
            None => ("UTF-32".into(), "\u{FFFD}".into()),
        }
    }

    fn get_data_interpretations(&self, data: &[u8], offset: Option<usize>) -> [(String, String); 18] {
        let data_slice = offset.map_or_else( || &[] as &[u8], |off| &data[off..]);
        [
            // Integer interpretations
            Self::intepret_u8(data_slice, self.radix),
            Self::intepret_i8(data_slice, self.radix),
            Self::intepret_u16(data_slice, self.radix, self.little_endian),
            Self::intepret_i16(data_slice, self.radix, self.little_endian),
            Self::intepret_u24(data_slice, self.radix, self.little_endian),
            Self::intepret_i24(data_slice, self.radix, self.little_endian),
            Self::intepret_u32(data_slice, self.radix, self.little_endian),
            Self::intepret_i32(data_slice, self.radix, self.little_endian),
            Self::intepret_u64(data_slice, self.radix, self.little_endian),
            Self::intepret_i64(data_slice, self.radix, self.little_endian),
            // Float interpretations
            Self::interpret_f16(data_slice, self.little_endian),
            Self::interpret_bf16(data_slice, self.little_endian),
            Self::interpret_f32(data_slice, self.little_endian),
            Self::interpret_f64(data_slice, self.little_endian),
            // ASCII/Character interpretations
            Self::intepret_ascii(data_slice),
            Self::interpret_utf8(data_slice),
            Self::interpret_utf16(data_slice, self.little_endian),
            Self::interpret_utf32(data_slice, self.little_endian),
        ]
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        selected_offset: Option<usize>,
        file_data: Option<&[u8]>,
    ) {
        // println!("Data Inspector Available width: {}", ui.available_width());
        
        let _resp = egui::Frame::group(ui.style())
        // .corner_radius(20.)
        // .outer_margin(1.)
        .show(ui, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Data Inspector");
                });

                ui.horizontal(|ui| {
                    // Endianness radio buttons
                    ui.radio_value(&mut self.little_endian, true, "LE");
                    ui.radio_value(&mut self.little_endian, false, "BE");

                    ui.separator();

                    // Radix combo box
                    egui::ComboBox::from_id_salt("radix_selector")
                        .selected_text(format!("{}", self.radix))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.radix, Radix::Decimal, "Decimal");
                            ui.selectable_value(&mut self.radix, Radix::Hexadecimal, "Hexadecimal");
                            ui.selectable_value(&mut self.radix, Radix::Binary, "Binary");
                            ui.selectable_value(&mut self.radix, Radix::Octal, "Octal");
                        });
                });

                ui.separator();

                let data = file_data.unwrap_or(&[]);

                if selected_offset.is_some_and(|v| v >= data.len()) {
                    panic!("Impossible!");
                }
                ui.horizontal(|ui| {
                    ui.label("Offset:");
                    ui.label(selected_offset.map_or("N/A".into(), |off| {
                        format!(
                            "0x{:08X} ({})",
                            off, off
                        )
                    }));
                });


                ui.separator(); 

                let interpretations = self.get_data_interpretations(data, selected_offset);
                let table = TableBuilder::new(ui)
                    .striped(true)
                    .column(Column::exact(80.0)) // Type
                    .column(Column::remainder()); // Value
                table
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.strong("Type");
                        });
                        header.col(|ui| {
                            ui.strong("Value");
                        });
                    })
                    .body(|mut body| {
                        for (data_type, value) in interpretations {
                            body.row(18.0, |mut row| {
                                row.col(|ui| {
                                    ui.label(&data_type);
                                });
                                row.col(|ui| {
                                    ui.label(&value);
                                });
                            });
                        }
                    });
            });
        });
        // println!("Data Inspector used width: {}", _resp.response.rect.width());
    }
}