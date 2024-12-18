# CakeMu-RV

[简体中文](README.md) | English

## Introduction

CakeMu-RV is a simple RISC-V emulator written in Rust that supports the basic RV32I instruction set. This is a personal project for learning computer organization principles. By implementing basic CPU execution processes and a simple peripheral system, it helps understand fundamental computer operations. Currently, it includes basic features such as instruction execution, memory access, and basic I/O operations. This emulator can serve as a reference and practical tool for studying computer organization principles.

## Features

- Supports RV32I base instruction set
- Complete peripheral emulation system:
  - UART: Character and string output support
  - Timer: Programmable timer with interrupt support
  - Wave Generator: Multiple waveform output support
    - Sine wave
    - Square wave (adjustable duty cycle)
    - Triangle wave
    - Sawtooth wave
- C language development environment
- Debug output control
- Waveform data visualization tools
- Lightweight binary program construction

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

4. View waveform data (if wave generator was used):
```bash
python3 tools/plot_wave.py
```

## Command Line Options

- `--no-mtrace`: Disable memory access tracing
- `--no-regtrace`: Disable register tracing
- `--no-itrace`: Disable instruction tracing
- `--step`: Enable single-step execution mode

## C Development

The project provides a complete C language development environment with the following peripheral support:

### UART
- Basic function: Character and string output
- Interface:
  - `uart_putc(char c)`: Output single character
  - `uart_puts(const char *str)`: Output string

### Timer
- Basic function: Programmable timer
- Register mapping: 0x02000200
- Main features:
  - Programmable count value
  - Interrupt support
  - Auto-reload capability
- Interface:
  - `timer_init(uint32_t compare_value)`: Initialize timer
  - `timer_enable()`: Start timer
  - `timer_disable()`: Stop timer
  - `timer_get_status()`: Get timer status
  - `timer_clear_status()`: Clear timer status

### Wave Generator
- Basic function: Waveform generator
- Register mapping: 0x02000300
- Supported waveforms:
  - Sine wave (WAVE_TYPE_SINE)
  - Square wave (WAVE_TYPE_SQUARE)
  - Triangle wave (WAVE_TYPE_TRIANGLE)
  - Sawtooth wave (WAVE_TYPE_SAWTOOTH)
- Configurable parameters:
  - Frequency (1Hz-100kHz)
  - Amplitude (0-255)
  - Phase (0-359 degrees)
  - Duty cycle (0-100%, square wave only)
- Interface:
  - `wave_init()`: Initialize wave generator
  - `wave_enable()`: Start output
  - `wave_disable()`: Stop output
  - `wave_set_type(uint32_t type)`: Set waveform type
  - `wave_set_frequency(uint32_t freq)`: Set frequency
  - `wave_set_amplitude(uint32_t amp)`: Set amplitude
  - `wave_set_phase(uint32_t phase)`: Set phase
  - `wave_set_duty(uint32_t duty)`: Set duty cycle

## Waveform Data Visualization

The project provides waveform data visualization tools:

1. Waveform data is automatically saved to `wave.txt`
2. Visualize using Python script:
```bash
python3 tools/plot_wave.py
```

## Example Program

The project includes a comprehensive test program (`c_sim/main.c`) that demonstrates the usage of all peripherals:
- UART character and string output testing
- Timer testing (short and long delays)
- Wave Generator testing for all waveform types

## License

This project is licensed under the GPL-3.0 License. See the [LICENSE](LICENSE) file for details. 