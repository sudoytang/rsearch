use crate::search::{AsyncSearch, Needle, NeedleOwned, SearchState};
use crate::ui;
use crate::ui::components::{
    DataInspector, FilePanel, HexViewer, SearchControlPanel, SearchResultsPanel,
};
use crate::ui::util::{Encoding, SearchType, Selection, InputParseError};
use crate::ui::int_parse::IntParser;
use eframe::egui;
use egui_extras::{Size, StripBuilder};

enum CurrentSearch {
    Empty,
    Searching(usize, AsyncSearch),
    Finished(usize),
    // usize is the byte length of this search
}

impl CurrentSearch {
    pub fn take(&mut self) -> CurrentSearch {
        let mut res = CurrentSearch::Empty;
        std::mem::swap(&mut res, self);
        res
    }
    pub fn finish(&mut self) {
        match self {
            CurrentSearch::Empty => {}
            CurrentSearch::Searching(i, _) => *self = CurrentSearch::Finished(*i),
            CurrentSearch::Finished(_) => {}
        }
    }
}

pub struct BinarySearchApp {
    // UI components
    file_panel: FilePanel,
    search_control_panel: SearchControlPanel,
    search_results_panel: SearchResultsPanel,
    hex_viewer: HexViewer,
    data_inspector: DataInspector,
    selection: Option<Selection>,
    last_selection: Option<Selection>,
    // Search state
    // current_search: byte length + search handle
    current_search: CurrentSearch,
}

impl Default for BinarySearchApp {
    fn default() -> Self {
        Self {
            selection: None,
            last_selection: None,
            file_panel: FilePanel::new(),
            search_control_panel: SearchControlPanel::new(),
            search_results_panel: SearchResultsPanel::new(),
            hex_viewer: HexViewer::new(),
            data_inspector: DataInspector::new(),
            current_search: CurrentSearch::Empty,
        }
    }
}

impl BinarySearchApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn perform_search(&mut self) {
        // Clear previous results
        self.search_results_panel.clear_results();

        // Cancel any ongoing search
        if let CurrentSearch::Searching(_, search) = self.current_search.take() {
            let _ = search.cancel();
        }

        // Get file data
        let file_data = match self.file_panel.get_file_data_arc() {
            Some(data) => data,
            None => {
                eprintln!("No file loaded for search");
                return;
            }
        };

        // Get search input
        let search_input = self.search_control_panel.get_search_input();
        if search_input.is_empty() {
            return;
        }

        // Parse search input and create needle
        let needle = match self.parse_search_input() {
            Ok(needle) => needle,
            Err(e) => {
                eprintln!("Failed to parse search input: {}", e);
                return;
            }
        };

        // Create and start async search
        let len = needle.byte_length();
        let search = AsyncSearch::create_from_owned(file_data, needle);
        self.current_search = CurrentSearch::Searching(len, search);
    }

    fn parse_search_input(&self) -> Result<NeedleOwned, InputParseError> {
        let input = self.search_control_panel.get_search_input();
        let search_type = self.search_control_panel.get_search_type();
        let endianness = self.search_control_panel.get_endianness();
        let is_signed = self.search_control_panel.get_is_signed();
        let encoding = self.search_control_panel.get_encoding();

        let needle = match search_type {
            SearchType::Bit8 => {
                if is_signed {
                    let value = IntParser::parse_i8(input)?;
                    Needle::I8(value)
                } else {
                    let value = IntParser::parse_u8(input)?;
                    Needle::U8(value)
                }
            }
            SearchType::Bit16 => {
                if is_signed {
                    let value = IntParser::parse_i16(input)?;
                    Needle::I16(endianness, value)
                } else {
                    let value = IntParser::parse_u16(input)?;
                    Needle::U16(endianness, value)
                }
            }
            SearchType::Bit32 => {
                if is_signed {
                    let value = IntParser::parse_i32(input)?;
                    Needle::I32(endianness, value)
                } else {
                    let value = IntParser::parse_u32(input)?;
                    Needle::U32(endianness, value)
                }
            }
            SearchType::Bit64 => {
                if is_signed {
                    let value = IntParser::parse_i64(input)?;
                    Needle::I64(endianness, value)
                } else {
                    let value = IntParser::parse_u64(input)?;
                    Needle::U64(endianness, value)
                }
            }
            SearchType::String => match encoding {
                Encoding::UTF8 => Needle::Str(input),
            },
            SearchType::Bytes => {
                // Parse hex string like "41 42 43" or "414243"
                let cleaned = input.replace(" ", "").replace("0x", "");
                if cleaned.len() % 2 != 0 {
                    return Err("Hex string must have even number of characters".into());
                }

                let mut bytes = Vec::new();
                for i in (0..cleaned.len()).step_by(2) {
                    let hex_byte = &cleaned[i..i + 2];
                    let byte = u8::from_str_radix(hex_byte, 16).map_err(|_| "Invalid hex byte")?;
                    bytes.push(byte);
                }

                return Ok(NeedleOwned::from_data(bytes));
            }
        };

        Ok(needle.into())
    }

    fn update_search_results(&mut self) {
        if let CurrentSearch::Searching(_, search) = &self.current_search {
            let mut results = Vec::new();
            let mut result_count = 0;

            // Collect up to a reasonable number of results per frame to avoid blocking UI
            const MAX_RESULTS_PER_FRAME: usize = 100000;

            loop {
                match search.try_get() {
                    Ok(offset) => {
                        results.push(ui::SearchResult {
                            index: result_count,
                            offset,
                        });
                        result_count += 1;

                        if result_count >= MAX_RESULTS_PER_FRAME {
                            break;
                        }
                    }
                    Err(SearchState::Pending) => {
                        // No more results available right now
                        break;
                    }
                    Err(SearchState::Finished) => {
                        // Search is complete, remove it
                        self.current_search.finish();
                        break;
                    }
                }
            }

            // Add new results to the panel
            if !results.is_empty() {
                self.search_results_panel.add_search_results(results);
            }
        }
    }
}

impl BinarySearchApp {
    // Layout Spec

    const CELL0_MIN_WIDTH: f32 = 360.;
    const CELL1_MIN_WIDTH: f32 = HexViewer::WIDGET_MIN_WIDTH;
    const CELL2_MIN_WIDTH: f32 = 260.;
    pub const APP_MIN_WIDTH: f32 =
        Self::CELL0_MIN_WIDTH + Self::CELL1_MIN_WIDTH + Self::CELL2_MIN_WIDTH;
    const CELL0_RATIO: f32 = Self::CELL0_MIN_WIDTH / Self::APP_MIN_WIDTH;
    const CELL1_RATIO: f32 = Self::CELL1_MIN_WIDTH / Self::APP_MIN_WIDTH;
    #[allow(unused)]
    const CELL2_RATIO: f32 = Self::CELL2_MIN_WIDTH / Self::APP_MIN_WIDTH;

    const APP_MIN_HEIGHT: f32 = 350.;
}

impl eframe::App for BinarySearchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.send_viewport_cmd(egui::ViewportCommand::MinInnerSize(egui::vec2(
            Self::APP_MIN_WIDTH,
            Self::APP_MIN_HEIGHT,
        )));

        // Debug: Print mouse position when hovering over the window
        // ctx.input(|i| {
        //     if let Some(pos) = i.pointer.hover_pos() {
        //         println!("Mouse position: x={:.2}, y={:.2}", pos.x, pos.y);
        //     } else if let Some(pos) = i.pointer.interact_pos() {
        //         println!("Mouse interact position: x={:.2}, y={:.2}", pos.x, pos.y);
        //     }
        // });
        // Left-right split layout

        // Check for new search results
        self.update_search_results();
        self.last_selection = self.selection;
        egui::CentralPanel::default().show(ctx, |ui| {
            let sb: StripBuilder<'_> = StripBuilder::new(ui)
                .size(Size::relative(Self::CELL0_RATIO))
                .size(Size::relative(Self::CELL1_RATIO))
                .size(Size::remainder());
            sb.horizontal(|mut strip| {
                strip.cell(|ui| {
                    // Left panel - File controls, Search controls, Search results
                    // File panel
                    if self.file_panel.render(ui) {
                        // File was opened, clear search results and cancel ongoing search
                        self.selection = None;
                        self.search_results_panel.clear_results();
                        if let CurrentSearch::Searching(_, search) = self.current_search.take() {
                            let _ = search.cancel();
                        }
                    }

                    ui.separator();

                    // Search controls panel
                    if self.search_control_panel.render(ui) {
                        self.perform_search();
                    }

                    ui.separator();

                    // Search results panel
                    if let Some(sel) = self.search_results_panel.render(ui) {
                        match self.current_search {
                            CurrentSearch::Empty => todo!(),
                            CurrentSearch::Searching(len, _) => {
                                self.selection = Some(Selection::range(sel, sel + len - 1));
                            }
                            CurrentSearch::Finished(len) => {
                                self.selection = Some(Selection::range(sel, sel + len - 1));
                            }
                        }
                    }
                });
                strip.cell(|ui| {
                    let diff = self.selection != self.last_selection;

                    self.hex_viewer.render(
                        ui,
                        self.file_panel.get_file_data(),
                        &mut self.selection,
                        diff,
                    );
                });
                strip.cell(|ui| {
                    self.data_inspector.render(
                        ui,
                        self.selection.map(|s| s.lower()),
                        self.file_panel.get_file_data(),
                    );
                })
            });
        });
    }
}
