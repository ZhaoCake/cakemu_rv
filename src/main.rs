/*
 * Copyright (C) 2024 ZhaoCake
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::env;
use riscv_emu::{cpu, memory, devices, debugger, config};

fn print_usage(program: &str) {
    eprintln!("Usage: {} <config-file>", program);
    eprintln!("Example: {} config.toml", program);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        print_usage(&args[0]);
        std::process::exit(1);
    }

    // 读取并解析配置文件
    let config = config::Config::from_file(&args[1])?;

    println!("RISC-V Emulator Starting...");
    println!("Loading configuration from: {}", args[1]);

    // 创建 CPU 配置
    let cpu_config = cpu::CpuConfig {
        memory: memory::MemoryConfig {
            size: memory::MEMORY_SIZE,
        },
        devices: devices::DeviceConfig {
            uart_enabled: config.uart.enabled,
            uart_base: config.uart.base_addr,
            timer_enabled: config.timer.enabled,
            timer_base: config.timer.base_addr,
            timer_auto_reload: config.timer.auto_reload,
            timer_interrupt: config.timer.interrupt_enabled,
            wave_enabled: config.wave.enabled,
            wave_base: config.wave.base_addr,
            wave_output: config.wave.output_file,
            wave_sample_rate: config.wave.sample_rate,
            display_enabled: config.display.enabled,
            display_base: config.display.base_addr,
            display_title: config.display.title,
        },
        debug: debugger::DebugConfig {
            instruction_trace: config.debug.instruction_trace,
            memory_trace: config.debug.memory_trace,
            register_trace: config.debug.register_trace,
            single_step: config.debug.single_step,
            trace_limit: config.debug.trace_limit,
        },
    };

    // 创建 CPU 实例
    let mut cpu = cpu::Cpu::new(cpu_config);

    // 加载程序
    println!("Loading program: {}", config.program.binary);
    cpu.load_program(&config.program.binary)?;

    // 显示初始寄存器状态
    if config.debug.register_trace {
        cpu.show_registers();
    }

    // 执行程序
    loop {
        match cpu.step() {
            Ok(()) => {
                if config.debug.register_trace {
                    cpu.show_registers();
                }
            }
            Err(e) => {
                println!("Execution error: {}", e);
                break;
            }
        }
    }

    Ok(())
}
