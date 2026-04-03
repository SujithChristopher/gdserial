# GdSerial - Godot 4 串口通信库

[![GitHub Downloads](https://img.shields.io/github/downloads/SujithChristopher/gdserial/total?style=flat-square&label=Downloads&color=blue)](https://github.com/SujithChristopher/gdserial/releases)
[![Latest Release](https://img.shields.io/github/v/release/SujithChristopher/gdserial?style=flat-square&label=Latest)](https://github.com/SujithChristopher/gdserial/releases/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow?style=flat-square)](https://github.com/SujithChristopher/gdserial/blob/main/LICENSE)
[![Godot 4](https://img.shields.io/badge/Godot-4.x-478CBF?style=flat-square&logo=godot-engine&logoColor=white)](https://godotengine.org)
[![Rust 2021](https://img.shields.io/badge/Rust-2021_edition-orange?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org)

**语言 / Language**: [English](README.md) | 简体中文 | [Português do Brasil](README.pt-BR.md)

<img src="addons/gdserial/icon.png" alt="GdSerial 图标" width="64" height="64" align="left" style="margin-right: 20px;">

基于 Rust 的高性能 Godot 4 串口通信库，通过 gdext 提供类似 PySerial 的功能。让你的 Godot 游戏和应用直接与 Arduino、ESP32、传感器、调制解调器和物联网设备通信。

<br clear="left">

## 功能特性

- **Arduino 与 ESP32 集成**：直接与微控制器和开发板通信
- **硬件传感器支持**：接口温度、运动、GPS 及环境传感器
- **跨平台串口通信**：支持 Windows、Linux 和 macOS
- **类 PySerial API**：为从 Python 迁移到 Godot 的开发者提供熟悉的接口
- **游戏引擎集成**：原生 Godot 4 类型和错误处理，无缝游戏开发体验
- **物联网设备通信**：连接 WiFi 模块、蓝牙适配器和蜂窝调制解调器
- **实时数据流**：低延迟二进制和文本操作，响应迅速
- **端口自动发现**：自动枚举和识别可用串行设备

## 平台支持

| 平台 | 架构 | 状态 | 端口示例 |
| :--- | :--- | :--- | :--- |
| **Windows** | x64 | ✅ 已支持 | `COM1`, `COM3`, `COM8` |
| **Windows** | ARM64 | ✅ 已支持 | `COM1`, `COM3`, `COM8` |
| **Linux** | x64 | ✅ 已支持 | `/dev/ttyUSB0`, `/dev/ttyACM0` |
| **Linux** | ARM64 | ✅ 已支持 | `/dev/ttyUSB0`, `/dev/ttyACM0` |
| **macOS** | x64 (Intel) | ✅ 已支持 | `/dev/tty.usbserial-*`, `/dev/tty.usbmodem*` |
| **macOS** | ARM64 (Apple Silicon) | ✅ 已支持 | `/dev/tty.usbserial-*`, `/dev/tty.usbmodem*` |

> **macOS 用户**：若遇到"恶意软件"警告或库加载错误，请参阅 [MACOS_SECURITY.md](MACOS_SECURITY.md)。

## 安装

### 方式一：GitHub Releases（推荐）

1. 从 [GitHub Releases](https://github.com/SujithChristopher/gdserial/releases) 下载适合你平台的最新版本
2. 将 `addons/gdserial` 文件夹复制到项目的 `addons/` 目录
3. 进入 项目 > 项目设置 > 插件
4. 启用 "GdSerial - Serial Communication Library" 插件

### 方式二：从源码构建

#### 前置要求

- Rust（最新稳定版）
- Godot 4.2+
- Git

#### 构建步骤

1. 克隆仓库：

```bash
git clone https://github.com/SujithChristopher/gdserial.git
cd gdserial
```

2. 构建库：

```bash
# Linux/Mac
./build_release.sh

# Windows
build_release.bat
```

3. 插件将在 `addons/gdserial` 文件夹中准备好，包含已编译的库。

## 编辑器内文档

GdSerial 内置了类参考文档，可在 Godot 内置帮助（F1）中直接查看。Godot 会根据你的系统语言自动选择语言。

| 语言 | 文件夹 |
| :--- | :--- |
| English | [doc/en/](addons/gdserial/doc/en/) |
| 简体中文 | [doc/zh_CN/](addons/gdserial/doc/zh_CN/) |
| Português do Brasil | [doc/pt_BR/](addons/gdserial/doc/pt_BR/) |

## API 参考

### 核心类

#### `GdSerial`（基础 API）

简单的类 PySerial API，用于直接同步控制单个端口。

##### 端口管理

- `list_ports() -> Dictionary` — 获取所有可用串口（索引 -> 信息字典）
- `set_port(port_name: String)` — 设置要使用的端口（如 "COM3"、"/dev/ttyUSB0"）
- `set_baud_rate(rate: int)` — 设置波特率（默认：9600）
- `set_data_bits(bits: int)` — 设置数据位（6、7、8）
- `set_parity(type: int)` — 设置校验位（0：无，1：奇，2：偶）
- `set_stop_bits(bits: int)` — 设置停止位（1、2）
- `set_flow_control(type: int)` — 设置流控制（0：无，1：软件，2：硬件）
- `set_timeout(timeout_ms: int)` — 设置读取超时（毫秒，默认：1000）
- `open() -> bool` — 打开串口
- `close()` — 关闭串口
- `is_open() -> bool` — 检查端口是否打开（执行主动连接测试）

##### 数据操作

- `write(data: PackedByteArray) -> bool` — 写入原始字节
- `write_string(data: String) -> bool` — 写入字符串数据
- `writeline(data: String) -> bool` — 写入带换行符的字符串
- `read(size: int) -> PackedByteArray` — 读取原始字节
- `read_string(size: int) -> String` — 读取并转换为字符串
- `readline() -> String` — 读取直到换行符

##### 工具方法

- `bytes_available() -> int` — 获取等待读取的字节数
- `clear_buffer() -> bool` — 清空输入/输出缓冲区

#### `GdSerialManager`（高级 API）

使用后台线程和信号的多端口异步管理器。适用于复杂应用程序。

##### 方法

- `list_ports() -> Dictionary` — 同 `GdSerial.list_ports()`
- `open(name: String, baud: int, timeout: int) -> bool` — 以 RAW 模式打开端口
- `open_buffered(name: String, baud: int, timeout: int, mode: int) -> bool` — 以指定缓冲模式打开端口（0：RAW，1：行缓冲，2：自定义分隔符）
- `close(name: String)` — 关闭并停止读取线程，发送 `port_disconnected` 信号
- `is_open(name: String) -> bool` — 检查指定端口是否打开
- `write(name: String, data: PackedByteArray) -> bool` — 向指定端口写入原始字节
- `reconfigure_port(...) -> bool` — 更新已打开端口的设置
- `set_delimiter(name: String, delimiter: int) -> bool` — 为模式 2 设置自定义分隔符字节
- `poll_events() -> Array` — **关键**：在 `_process` 中调用以触发信号并获取事件

##### 信号

- `data_received(port: String, data: PackedByteArray)` — 新数据到达时触发
- `port_disconnected(port: String)` — 端口断线时触发（手动关闭或硬件断线）

### 方式 A：异步管理器（推荐用于非阻塞 UI）

```gdscript
extends Node

var manager: GdSerialManager

func _ready():
    manager = GdSerialManager.new()
    manager.data_received.connect(_on_data)
    manager.port_disconnected.connect(_on_disconnect)

    # 模式 0：RAW（立即发送所有数据块）
    # 模式 1：行缓冲（等待 \n）
    # 模式 2：自定义分隔符
    if manager.open("COM3", 9600, 1000):
        print("已连接到 COM3")

func _process(_delta):
    # 此调用触发上述信号
    manager.poll_events()

func _on_data(port: String, data: PackedByteArray):
    print("来自 ", port, " 的数据：", data.get_string_from_utf8())

func _on_disconnect(port: String):
    print("与 ", port, " 的连接已断开")
```

### 方式 B：简单阻塞 API

```gdscript
extends Node

var serial: GdSerial

func _ready():
    serial = GdSerial.new()
    
    # 列出可用端口
    var ports: Dictionary = serial.list_ports()
    for i in ports:
        var info = ports[i]
        print("- ", info["port_name"], " (", info["device_name"], ")")
    
    serial.set_port("COM3")
    serial.set_baud_rate(115200)
    
    if serial.open():
        serial.writeline("Hello!")
        await get_tree().create_timer(0.1).timeout
        if serial.bytes_available() > 0:
            print("响应：", serial.readline())
        serial.close()
```

> **注意**：启用插件后，GdSerial 类会自动可用，无需导入！

## 常见用例

### Arduino 通信

```gdscript
# 同步（简单，阻塞）
serial.writeline("GET_SENSOR")
var reading = serial.readline()
print("传感器值：", reading)
```

异步多端口通信，使用模式 1（行缓冲）的 `GdSerialManager`：
```gdscript
manager.open_buffered("COM3", 9600, 1000, 1)  # 模式 1：等待换行符
```

### AT 命令（调制解调器、WiFi 模块）

```gdscript
serial.writeline("AT+VERSION?")
var version = serial.readline()
print("模块版本：", version)
```

### 二进制数据传输

```gdscript
# 同步方式
var data = PackedByteArray([0x01, 0x02, 0x03, 0x04])
serial.write(data)
var response = serial.read(10)
```

异步二进制协议，使用模式 0（RAW）或模式 2（自定义分隔符）：
```gdscript
manager.open_buffered("COM3", 9600, 1000, 0)  # 模式 0：立即发送所有数据块
# 或
manager.open_buffered("COM3", 9600, 1000, 2)  # 模式 2：等待分隔符字节
manager.set_delimiter("COM3", 0xFF)            # 设置自定义结束标记
```

## 平台特定说明

### Windows

- 端口名通常为 `COM1`、`COM2` 等
- 某些设备可能需要管理员权限

### Linux

- 端口名通常为 `/dev/ttyUSB0`、`/dev/ttyACM0` 等
- 用户必须在 `dialout` 组中才能访问串口：

  ```bash
  sudo usermod -a -G dialout $USER
  ```

### macOS

- 端口名通常为 `/dev/tty.usbserial-*` 或 `/dev/tty.usbmodem*`

## 错误处理

本库通过 Godot 内置日志系统提供详细错误日志。常见错误包括：

- 端口未找到或无法访问
- 权限被拒绝（检查用户权限）
- 操作期间设备断开连接
- 读取操作超时

操作失败时，请查看 Godot 控制台中的详细错误信息。

## 故障排除

### 端口未找到

- 确认设备已连接并被操作系统识别
- 检查设备管理器（Windows）或 `dmesg`（Linux）
- 尝试不同的 USB 端口或数据线

### 权限被拒绝（Linux）

```bash
sudo usermod -a -G dialout $USER
# 注销并重新登录使更改生效
```

### 构建问题

- 确保 Rust 为最新版本：`rustup update`
- 清除 cargo 缓存：`cargo clean`
- 检查所有依赖项是否可用

## 插件结构

```text
addons/gdserial/
├── plugin.cfg              # 插件配置
├── plugin.gd               # 插件激活脚本
├── gdserial.gdextension    # 扩展加载器
├── bin/                    # 已编译库
│   ├── windows-x86_64/
│   ├── linux-x86_64/
│   ├── linux-arm64/
│   ├── macos-x86_64/
│   └── macos-arm64/
├── doc/                    # 编辑器内文档
│   ├── en/
│   ├── zh_CN/
│   └── pt_BR/
└── README.md
```

## 贡献

1. Fork 本仓库
2. 创建功能分支
3. 进行修改
4. 使用构建脚本测试
5. 提交 Pull Request

## 许可证

本项目基于 MIT 许可证 — 详情请见 LICENSE 文件。

## 依赖项

- [gdext](https://github.com/godot-rust/gdext) — Godot 4 Rust 绑定
- [serialport](https://crates.io/crates/serialport) — 跨平台串口库
