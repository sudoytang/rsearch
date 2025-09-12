# RSearch

A high-performance binary file search tool built with Rust and modern GUI framework.

## Overview

RSearch is a powerful desktop application designed for searching and analyzing binary files. It provides an intuitive graphical interface for performing complex searches across large binary datasets, making it ideal for reverse engineering, data analysis, and file forensics.

## Features

### 🔍 Advanced Search Capabilities
- **Multi-type Search**: Support for 8/16/32/64-bit integers, strings, and raw bytes
- **Endianness Support**: Handle both big-endian and little-endian data formats
- **Signed/Unsigned Integers**: Full support for both signed and unsigned integer types
- **Flexible Input Parsing**: Accept decimal, hexadecimal, octal, and binary number formats

### ⚡ High Performance
- **Async Search Engine**: Multi-threaded asynchronous searching for real-time results
- **Memory-Mapped Files**: Efficient handling of large files using memory mapping
- **Optimized Algorithms**: Built on top of the `memchr` crate for fast pattern matching

### 🖥️ Modern User Interface
- **Hex Viewer**: Interactive hexadecimal file viewer with selection support
- **Data Inspector**: Real-time data interpretation at cursor position
- **Search Results Panel**: Organized display of search matches with navigation
- **File Panel**: Easy file loading with drag-and-drop support
- **Responsive Layout**: Adaptive UI that works across different screen sizes

### 🛠️ Developer-Friendly
- **Type Safety**: Leverages Rust's type system for memory safety and performance
- **Modular Design**: Clean separation of concerns with well-defined components
- **Extensible Architecture**: Easy to add new search types and data formats

## Installation

### Prerequisites
- Rust 2024 edition or later
- Operating System: Windows, macOS, or Linux

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/rsearch.git
cd rsearch

# Build the project
cargo build --release

# Run the application
cargo run --release
```

## Usage

### Basic Search
1. **Load a File**: Use the file panel to select and load a binary file
2. **Choose Search Type**: Select the data type you want to search for (8-bit, 16-bit, 32-bit, 64-bit, bytes, or string)
3. **Configure Options**: Set endianness, signedness, and encoding as needed
4. **Enter Search Value**: Input the value to search for
5. **Start Search**: Click search to begin the asynchronous search process
6. **View Results**: Browse through search results and click to navigate to locations

### Search Types

#### Integer Search
- **8-bit**: Search for single bytes (0-255 or -128 to 127)
- **16-bit**: Search for 2-byte values with endianness control
- **32-bit**: Search for 4-byte values with endianness control  
- **64-bit**: Search for 8-byte values with endianness control

#### String Search
- **UTF-8**: Search for text strings with proper encoding handling

#### Byte Search
- **Raw Bytes**: Search for hexadecimal byte sequences (e.g., "41 42 43" or "414243")

### Input Formats
The application supports multiple input formats for numeric values:
- **Decimal**: `123`, `-456`
- **Hexadecimal**: `0x1A2B`, `0xFF`
- **Octal**: `0o777`, `0o123`
- **Binary**: `0b1010`, `0b11110000`

## Architecture

### Core Components

- **Search Engine** (`src/search.rs`): Asynchronous binary search implementation
- **UI Framework** (`src/ui/`): Modern GUI built with egui
  - **App Controller** (`app.rs`): Main application logic and state management
  - **Hex Viewer** (`components/hex_viewer.rs`): Interactive hexadecimal display
  - **Data Inspector** (`components/data_inspector.rs`): Real-time data interpretation
  - **Search Controls** (`components/search_control_panel.rs`): Search configuration UI
  - **File Panel** (`components/file_panel.rs`): File loading interface
- **Utilities** (`src/ui/util.rs`): Common data structures and helpers
- **Input Parsing** (`src/ui/int_parse.rs`): Flexible numeric input parser

### Key Technologies

- **[egui](https://github.com/emilk/egui)**: Immediate mode GUI framework
- **[memchr](https://github.com/BurntSushi/memchr)**: Fast string searching algorithms
- **[memmap2](https://github.com/RazrFalcon/memmap2-rs)**: Memory-mapped file I/O
- **[color-eyre](https://github.com/yaahc/color-eyre)**: Enhanced error handling
- **[rfd](https://github.com/PolyMeilex/rfd)**: Native file dialogs

## Development

### Project Structure
```
src/
├── lib.rs              # Library root
├── main.rs             # Application entry point
├── search.rs           # Core search engine
└── ui/                 # User interface components
    ├── mod.rs
    ├── app.rs          # Main application logic
    ├── util.rs         # Common utilities
    ├── int_parse.rs    # Input parsing logic
    └── components/     # UI components
        ├── mod.rs
        ├── hex_viewer.rs
        ├── data_inspector.rs
        ├── file_panel.rs
        ├── search_control_panel.rs
        └── search_results_panel.rs
```

### Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Running Tests

```bash
cargo test
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with the excellent [egui](https://github.com/emilk/egui) immediate mode GUI framework
- Search algorithms powered by the high-performance [memchr](https://github.com/BurntSushi/memchr) crate
- Thanks to the Rust community for the amazing ecosystem of crates

## Status

🚧 **Work in Progress** - This project is under active development. Features and APIs may change.

