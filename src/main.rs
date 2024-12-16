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
use riscv_emu::cpu;

fn print_usage(program: &str) {
    eprintln!("Usage: {} <program-file> [options]", program);
    eprintln!("Options:");
    eprintln!("  --no-itrace    Disable instruction trace");
    eprintln!("  --no-mtrace    Disable memory trace");
    eprintln!("  --step         Enable single-step execution");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage(&args[0]);
        std::process::exit(1);
    }

    let program_file = &args[1];

    // 创建 CPU 实例
    let mut cpu = cpu::Cpu::new( 0x30000000);

    // 处理命令行选项
    for arg in &args[2..] {
        match arg.as_str() {
            "--no-itrace" => cpu.set_itrace(false),
            "--no-mtrace" => cpu.set_mtrace(false),
            "--step" => cpu.set_single_step(true),
            _ => {
                eprintln!("Unknown option: {}", arg);
                print_usage(&args[0]);
                std::process::exit(1);
            }
        }
    }

    println!("RISC-V Emulator Starting...");
    println!("Loading program: {}", program_file);

    // 加载程序
    cpu.load_program(program_file)?;

    // 显示初始寄存器状态
    cpu.show_registers();

    // 执行程序
    loop {
        match cpu.step() {
            Ok(()) => {
                cpu.show_registers();
            }
            Err(e) => {
                println!("Execution error: {}", e);
                break;
            }
        }
    }

    Ok(())
}
