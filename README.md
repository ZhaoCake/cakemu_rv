# CakeMu-RV

[English](README_EN.md) | 简体中文

## 简介

CakeMu-RV 是一个用 Rust 编写的简单 RISC-V 模拟器，支持 RV32I 基本指令集。这是一个用于学习计算机组成原理的个人项目，通过实现基本的 CPU 执行过程和简单的外设系统，帮助理解计算机的基本工作原理。目前实现了指令执行、内存访问和基础的 I/O 操作等基本功能。该模拟器可以作为学习计算机组成原理的一个参考和实践工具。

## 功能特性

- 支持 RV32I 基本指令集
- 完整的外设模拟系统：
  - UART：支持字符和字符串输出
  - Timer：可编程定时器，支持中断
  - Wave Generator：波形发生器，支持多种波形输出
    - 正弦波
    - 方波（可调占空比）
    - 三角波
    - 锯齿波
- 提供 C 语言开发环境
- 支持调试输出控制
- 波形数据可视化工具
- 轻量级二进制程序构建工具

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

4. 查看波形数据（如果使用了波形发生器）：
```bash
python3 tools/plot_wave.py
```

## 命令行选项

- `--no-mtrace`：禁用内存访问跟踪
- `--no-regtrace`：禁用寄存器跟踪
- `--no-itrace`：禁用指令跟踪
- `--step`：启用单步执行模式

## C 语言开发

项目提供了完整的 C 语言开发环境，包含以下外设支持：

### UART
- 基本功能：字符和字符串输出
- 接口：
  - `uart_putc(char c)`：输出单个字符
  - `uart_puts(const char *str)`：输出字符串

### Timer
- 基本功能：可编程定时器
- 寄存器映射：0x02000200
- 主要功能：
  - 可编程计数值
  - 中断支持
  - 自动重载功能
- 接口：
  - `timer_init(uint32_t compare_value)`：初始化定时器
  - `timer_enable()`：启动定时器
  - `timer_disable()`：停止定时器
  - `timer_get_status()`：获取定时器状态
  - `timer_clear_status()`：清除定时器状态

### Wave Generator
- 基本功能：波形发生器
- 寄存器映射：0x02000300
- 支持波形：
  - 正弦波 (WAVE_TYPE_SINE)
  - 方波 (WAVE_TYPE_SQUARE)
  - 三角波 (WAVE_TYPE_TRIANGLE)
  - 锯齿波 (WAVE_TYPE_SAWTOOTH)
- 可配置参数：
  - 频率 (1Hz-100kHz)
  - 幅度 (0-255)
  - 相位 (0-359度)
  - 占空比 (0-100%, 仅方波)
- 接口：
  - `wave_init()`：初始化波形发生器
  - `wave_enable()`：启动输出
  - `wave_disable()`：停止输出
  - `wave_set_type(uint32_t type)`：设置波形类型
  - `wave_set_frequency(uint32_t freq)`：设置频率
  - `wave_set_amplitude(uint32_t amp)`：设置幅度
  - `wave_set_phase(uint32_t phase)`：设置相位
  - `wave_set_duty(uint32_t duty)`：设置占空比

## 波形数据可视化

项目提供了波形数据可视化工具：

1. 波形数据自动保存到 `wave.txt`
2. 使用 Python 脚本可视化：
```bash
python3 tools/plot_wave.py
```

## 示例程序

项目包含了一个综合测试程序 (`c_sim/main.c`)，演示了所有外设的使用方法：
- UART 字符和字符串输出测试
- Timer 定时器测试（短延时和长延时）
- Wave Generator 所有波形类型测试

## 许可证

本项目采用 GPL-3.0 许可证。详见 [LICENSE](LICENSE) 文件。
