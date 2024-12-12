# RISC-V 模拟器 cakemu_rv

这是一个用 Rust 实现的 RISC-V 指令集模拟器，支持 RV32I 基础指令集。主要用于学习

## 功能特点

- 支持完整的 RV32I 基础指令集
- 模拟 32 个通用寄存器
- 实现虚拟内存访问
- 支持程序加载和执行
- 提供二进制程序构建工具

## 项目结构

```
src/
├── bin/
│   └── build_binary.rs    # 二进制程序构建工具
├── cpu/                   # CPU 相关实现
│   ├── mod.rs
│   └── register.rs
├── isa/                   # 指令集实现
│   ├── mod.rs
│   └── inst.rs
├── memory/               # 内存模拟
│   └── mod.rs
├── loader/              # 程序加载器
│   └── mod.rs
└── tools/               # 工具
    └── binary_builder.rs
```

## 使用方法

### 编译项目

```bash
cargo build --release
```

### 创建测试程序

使用提供的二进制构建工具创建测试程序：

```bash
cargo run --bin build_binary
```

这将生成一个示例程序 `program.bin`。

### 运行模拟器

```bash
cargo run --bin riscv-emu program.bin --step  # step by step 
```

### 创建自定义程序

可以使用 `BinaryBuilder` 创建自定义的 RISC-V 程序：

```rust
use riscv_emu::tools::binary_builder::BinaryBuilder;

let mut builder = BinaryBuilder::new();
// addi x1, x0, 5
builder.add_instruction(0x00500093);
builder.save("custom_program.bin")?;
```

## 支持的指令

- 算术指令：ADD, ADDI, SUB 等
- 逻辑指令：AND, OR, XOR 等
- 分支指令：BEQ, BNE, BLT 等
- 加载存储：LW, SW, LB, SB 等
- 跳转指令：JAL, JALR

## 开发说明

以上功能几乎暂未实现，期待你的到来

## 待实现功能

- [ ] RV32I 指令支持
- [ ] 外设寻址空间挂载到总线
