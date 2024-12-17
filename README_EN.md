# CakeMu-RV

[简体中文](README.md) | English

## Introduction

CakeMu-RV is a RISC-V emulator written in Rust that supports the RV32I instruction set. It includes a simple peripheral system that can be used for learning and testing RISC-V assembly programs.

## Features

- Supports RV32I base instruction set
- Basic peripheral emulation:
  - UART (Tested): For character output
  - GPIO (Untested): General-purpose input/output interface
  - Timer (Untested): Timer functionality
- Provides C language development environment
- Debug output control support
- Lightweight binary program construction support

## Quick Start

1. Clone the repository:
```bash
git clone https://github.com/yourusername/cakemu_rv.git
cd cakemu_rv
```

2. Build C test program:
```bash
cd c_sim
make
```

3. Run the emulator:
```bash
cargo run --bin riscv-emu build/program.bin
```

## Command Line Options

- `--no-mtrace`: Disable memory access tracing
- `--no-regtrace`: Disable register tracing
- `--no-itrace`: Disable instruction tracing
- `--step`: Enable single-step execution mode

## C Development

The project provides a C language development environment with the following peripheral support:

### UART
- Basic function: Character output
- Interface:
  - `uart_putc(char c)`: Output single character
  - `UART_PRINT_STR(str)`: Output string (macro)

### GPIO (Untested)
- Basic function: General-purpose input/output
- Register mapping: 0x02100000

### Timer (Untested)
- Basic function: Timer
- Register mapping: 0x02200000

## Lightweight Debugging

The project provides several debugging options:

1. Instruction tracing:
```bash
cargo run --bin riscv-emu program.bin  # All traces enabled by default
```

2. Step execution:
```bash
cargo run --bin riscv-emu program.bin --step  # Pause after each instruction
```

3. Selective tracing:
```bash
# Enable instruction trace only
cargo run --bin riscv-emu program.bin --no-mtrace --no-regtrace
```

## Binary Program Construction

In addition to the C development environment, the project provides a lightweight binary program construction tool:

1. Using the build tool:
```bash
cargo run --bin build_binary  # Generate example program
```

2. Custom program construction:
```rust
use riscv_emu::tools::binary_builder::BinaryBuilder;

let mut builder = BinaryBuilder::new();
// addi x1, x0, 5
builder.add_instruction(0x00500093);
builder.save("custom_program.bin")?;
```

This method is suitable for:
- Quick testing of single or few instructions
- Verifying instruction execution effects
- Debugging emulator functionality

## License

This project is licensed under the GPL-3.0 License. See the [LICENSE](LICENSE) file for details. 