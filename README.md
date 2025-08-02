# RSearch - 高性能二进制搜索库

RSearch 是一个高性能的 Rust 二进制搜索库，支持同步和异步搜索操作，特别适合 GUI 应用程序和实时搜索需求。

## 特性

- ✅ **多数据类型支持**: u8, i8, u16, i16, u32, i32, u64, i64
- ✅ **字节序支持**: 大端和小端字节序
- ✅ **字节模式搜索**: 支持任意字节模式匹配
- ✅ **异步搜索**: 实时进度更新和结果流
- ✅ **GUI 友好**: 支持回调机制，适合 GUI 集成
- ✅ **高性能**: 优化的搜索算法
- ✅ **上下文支持**: 可选的匹配上下文信息

## 快速开始

### 基本同步搜索

```rust
use rsearch::search::*;

// 搜索 u8 值
let data = vec![1, 2, 3, 4, 5, 3, 6, 3];
let result = search_u8(&data, 3);
println!("找到 {} 个匹配: {:?}", result.count, result.all_positions());

// 搜索 u16 值（大端）
let data = vec![0x00, 0x01, 0x00, 0x02, 0x00, 0x01];
let result = search_u16(&data, 1, Endianness::Big);
println!("找到 {} 个匹配", result.count);

// 搜索字节模式
let data = b"hello world hello";
let result = search_bytes(data, b"hello");
println!("找到 {} 个匹配", result.count);
```

### 异步搜索（推荐用于 GUI）

```rust
use rsearch::search::*;
use std::time::Duration;

// 创建搜索引擎
let engine = SearchEngine::new();

// 启动异步搜索
let streaming_result = engine.search_u8_async(&large_data, 0x42);

// 实时处理结果
loop {
    match streaming_result.receive_timeout(Duration::from_millis(100)) {
        Some(SearchUpdate::MatchFound(match_info)) => {
            println!("找到匹配: 位置 {}", match_info.position);
            // 立即更新 GUI
        }
        
        Some(SearchUpdate::Progress(progress)) => {
            println!("进度: {:.1}%, 已找到 {} 个匹配", 
                progress.progress * 100.0, progress.matches_found);
            // 更新进度条
        }
        
        Some(SearchUpdate::Completed(result)) => {
            println!("搜索完成! 总共 {} 个匹配", result.count);
            break;
        }
        
        None => {
            // 超时，可以处理其他 GUI 事件
        }
    }
}
```

### GUI 集成示例

```rust
use rsearch::search::*;
use std::sync::{Arc, Mutex};

// 定义 GUI 回调 trait
trait SearchCallback {
    fn on_match_found(&self, position: usize, value: String);
    fn on_progress(&self, progress: f64, matches_found: usize);
    fn on_completed(&self, total_matches: usize);
}

// 实现 GUI 回调
struct MyGUI {
    matches: Arc<Mutex<Vec<(usize, String)>>>,
    progress: Arc<Mutex<f64>>,
}

impl SearchCallback for MyGUI {
    fn on_match_found(&self, position: usize, value: String) {
        // 更新 GUI 列表
        self.matches.lock().unwrap().push((position, value));
        // 触发 GUI 重绘
    }
    
    fn on_progress(&self, progress: f64, _matches_found: usize) {
        // 更新进度条
        *self.progress.lock().unwrap() = progress;
    }
    
    fn on_completed(&self, total_matches: usize) {
        // 显示完成状态
        println!("搜索完成: {} 个匹配", total_matches);
    }
}

// 使用搜索管理器
let search_manager = AsyncSearchManager::new();
let gui = MyGUI::new();

// 启动搜索
search_manager.start_search(large_data, 0x42, gui);

// GUI 主循环继续运行，搜索在后台进行
```

## 配置选项

```rust
let config = SearchConfig {
    streaming: true,                    // 启用流式更新
    progress_frequency: 10000,          // 每10KB更新一次进度
    include_context: true,              // 包含匹配上下文
    context_size: 16,                   // 上下文大小（字节）
    chunk_size: 1024 * 1024,           // 处理块大小
};

let engine = SearchEngine::with_config(config);
```

## 运行示例

```bash
# 运行异步搜索演示
cargo run --example async_search_demo

# 运行 GUI 集成演示
cargo run --example gui_integration

# 运行测试
cargo test
```

## 性能特点

### 同步搜索
- **时间复杂度**: O(n) 对于单字节搜索，O(n*m) 对于模式搜索
- **空间复杂度**: O(1) 对于数值搜索，O(1) 对于简单模式搜索
- **内存使用**: 最小化，适合大文件处理

### 异步搜索
- **实时反馈**: 立即报告匹配结果
- **非阻塞**: 不阻塞主线程
- **可配置**: 可调整进度更新频率
- **GUI 友好**: 支持回调机制

## 使用场景

### 1. 文件分析工具
```rust
// 在大型二进制文件中搜索特定模式
let file_data = std::fs::read("large_file.bin")?;
let result = search_bytes(&file_data, b"PNG");
```

### 2. 网络协议分析
```rust
// 搜索网络数据包中的特定标记
let packet_data = receive_packet();
let result = search_u32(&packet_data, 0xDEADBEEF, Endianness::Big);
```

### 3. 内存转储分析
```rust
// 在内存转储中搜索特定值
let memory_dump = load_memory_dump();
let result = search_u64(&memory_dump, target_address, Endianness::Little);
```

### 4. GUI 应用程序
```rust
// 实时搜索，立即显示结果
let engine = SearchEngine::new();
let streaming_result = engine.search_bytes_async(&data, pattern);

// 在 GUI 线程中处理更新
while let Some(update) = streaming_result.try_receive() {
    match update {
        SearchUpdate::MatchFound(match_info) => {
            update_gui_list(match_info.position);
        }
        SearchUpdate::Progress(progress) => {
            update_progress_bar(progress.progress);
        }
        _ => {}
    }
}
```

## API 参考

### 核心结构

- `SearchEngine`: 搜索引擎，支持同步和异步操作
- `SearchResult`: 搜索结果，包含所有匹配位置
- `StreamingSearchResult`: 流式搜索结果，支持实时更新
- `SearchConfig`: 搜索配置选项
- `SearchUpdate`: 搜索更新类型（匹配、进度、完成、错误）

### 主要函数

#### 同步搜索
- `search_u8(data, target)` - 搜索 u8 值
- `search_u16(data, target, endianness)` - 搜索 u16 值
- `search_u32(data, target, endianness)` - 搜索 u32 值
- `search_u64(data, target, endianness)` - 搜索 u64 值
- `search_bytes(data, pattern)` - 搜索字节模式

#### 异步搜索
- `engine.search_u8_async(data, target)` - 异步搜索 u8 值
- `engine.search_u16_async(data, target, endianness)` - 异步搜索 u16 值
- `engine.search_bytes_async(data, pattern)` - 异步搜索字节模式

## 许可证

MIT License 