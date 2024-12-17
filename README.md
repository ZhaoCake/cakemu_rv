# CakeMu-RV

[English](README_EN.md) | 简体中文

## 简介

CakeMu-RV 是一个用 Rust 编写的 RISC-V 模拟器，支持 RV32I 指令集。它包含了一个简单的外设系统，可以用于学习和测试 RISC-V 汇编程序。

## 功能特性

- 支持 RV32I 基本指令集
- 包含基本外设模拟：
  - UART（已测试）：用于字符输出
  - GPIO（未测试）：通用输入输出接口
  - Timer（未测试）：定时器功能
- 提供 C 语言开发环境
- 支持调试输出控制
- 支持轻量级二进制程序构建

## 快速开始

1. 克隆仓库：
```bash
git clone https://github.com/yourusername/cakemu_rv.git
cd cakemu_rv
```

2. 编译 C 测试程序：
```bash
cd c_sim
make
```

3. 运行模拟器：
```bash
cargo run --bin riscv-emu build/program.bin
```

## 命令行选项

- `--no-mtrace`：禁用内存访问跟踪
- `--no-regtrace`：禁用寄存器跟踪
- `--no-itrace`：禁用指令跟踪
- `--step`：启用单步执行模式

## C 语言开发

项目提供了 C 语言开发环境，包含以下外设支持：

### UART
- 基本功能：字符输出
- 接口：
  - `uart_putc(char c)`：输出单个字符
  - `UART_PRINT_STR(str)`：输出字符串（宏）

### GPIO（未测试）
- 基本功能：通用输入输出
- 寄存器映射：0x02100000

### Timer（未测试）
- 基本功能：定时器
- 寄存器映射：0x02200000

## 轻量级调试

项目提供了多种调试选项：

1. 指令跟踪：
```bash
cargo run --bin riscv-emu program.bin  # 默认启用所有跟踪
```

2. 单步执行：
```bash
cargo run --bin riscv-emu program.bin --step  # 每条指令暂停
```

3. 选择性跟踪：
```bash
# 仅启用指令跟踪
cargo run --bin riscv-emu program.bin --no-mtrace --no-regtrace
```

## 二进制程序构建

除了 C 语言开发环境，项目还提供了轻量级的二进制程序构建工具：

1. 使用构建工具：
```bash
cargo run --bin build_binary  # 生成示例程序
```

2. 自定义程序构建：
```rust
use riscv_emu::tools::binary_builder::BinaryBuilder;

let mut builder = BinaryBuilder::new();
// addi x1, x0, 5
builder.add_instruction(0x00500093);
builder.save("custom_program.bin")?;
```

这种方式适合：
- 快速测试单条或少量指令
- 验证指令执行效果
- 调试模拟器功能

## 许可证

本项目采用 GPL-3.0 许可证。详见 [LICENSE](LICENSE) 文件。
