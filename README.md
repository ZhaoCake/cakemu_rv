# RISC-V ISA Simulator cakemu_rv

This is a RISC-V instruction set simulator implemented in Rust, supporting the RV32I basic instruction set. Mainly used for learning.

---

# RISC-V 指令集模拟器 cakemu_rv

这是一个用 Rust 实现的 RISC-V 指令集模拟器，支持 RV32I 基础指令集。主要用于学习。

## 功能特点  Function Features

- 支持 RV32I 基础指令集
- 模拟 32 个通用寄存器
- 支持程序加载和执行
- 提供简单的二进制程序构建工具

## 项目结构  Project Structure

```
❯ tree -L 2
.
├── Cargo.lock
├── Cargo.toml
├── program.bin
├── README.md
├── reference
├── src
│   ├── bin
│   ├── cpu.rs
│   ├── debugger
│   ├── devices
│   ├── inst.rs
│   ├── lib.rs
│   ├── loader
│   ├── main.rs
│   ├── memory.rs
│   ├── register.rs
│   └── tools

```

## 使用方法  Usage

### 编译项目  Build Project

```bash
cargo build --release
```

### 创建测试程序  Build Test Program

使用提供的二进制构建工具创建测试程序：

Use the provided binary builder to create a test program:

```bash
cargo run --bin build_binary
```

这将生成一个示例程序 `program.bin`。

This will generate an example program `program.bin`.

### 运行模拟器  Run Simulator

```bash
cargo run --bin riscv-emu program.bin --step  # step by step 
```

### 创建自定义程序  Build Custom Program

可以使用 `BinaryBuilder` 创建自定义的 RISC-V 程序：

You can use `BinaryBuilder` to create a custom RISC-V program:

```rust
use riscv_emu::tools::binary_builder::BinaryBuilder;

let mut builder = BinaryBuilder::new();
// addi x1, x0, 5
builder.add_instruction(0x00500093);
builder.save("custom_program.bin")?;
```

## 支持的指令  Supported Instructions

- 算术指令：ADD, ADDI, SUB 等
- 逻辑指令：AND, OR, XOR 等
- 分支指令：BEQ, BNE, BLT 等
- 加载存储：LW, SW, LB, SB 等
- 跳转指令：JAL, JALR

## 开发说明  Development Notes

已经实现了RV32I的指令，准备进行指令测试以及添加外设用于课程设计实验

The instructions for RV32I have been implemented and are ready for instruction testing and adding peripherals for course design experiments

## 待实现功能  To-Do List

- [x] RV32I 指令支持  RV32I instruction support
- [ ] 外设寻址空间挂载到总线  Peripheral addressing space mounted to the bus
- [x] 修正失败的Instruction模块  Fix the failed Instruction module
- [ ] 编写进行指令测试的批处理脚本  Write batch scripts for instruction testing
