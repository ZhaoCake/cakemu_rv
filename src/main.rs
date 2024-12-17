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
    eprintln!("  --no-regtrace  Disable register trace");
    eprintln!("  --step         Enable single-step execution");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage(&args[0]);
        std::process::exit(1);
    }

    let program_file = &args[1];

    // 创建 CPU 实例，分配足够的内存
    // 代码段：0x00000000-0x0FFFFFFF (256MB)
    // 数据段：0x01000000-0x01FFFFFF (16MB)
    // 外设段：0x02000000-0x02FFFFFF (16MB)
    let memory_size = 0x03000000;  // 48MB total
    let mut cpu = cpu::Cpu::new(memory_size);

    // 默认启用所有跟踪
    let mut enable_itrace = true;
    let mut enable_mtrace = true;
    let mut enable_regtrace = true;
    let mut enable_step = false;

    // 处理命令行选项
    for arg in &args[2..] {
        match arg.as_str() {
            "--no-itrace" => enable_itrace = false,
            "--no-mtrace" => enable_mtrace = false,
            "--no-regtrace" => enable_regtrace = false,
            "--step" => enable_step = true,
            _ => {
                eprintln!("Unknown option: {}", arg);
                print_usage(&args[0]);
                std::process::exit(1);
            }
        }
    }

    // 设置调试选项
    cpu.set_itrace(enable_itrace);
    cpu.set_mtrace(enable_mtrace);
    cpu.set_regtrace(enable_regtrace);
    cpu.set_single_step(enable_step);

    println!("RISC-V Emulator Starting...");
    println!("Loading program: {}", program_file);

    // 加载程序
    cpu.load_program(program_file)?;

    // 显示初始寄存器状态（如果启用）
    if enable_regtrace {
        cpu.show_registers();
    }

    // 执行程序
    loop {
        match cpu.step() {
            Ok(()) => {
                if enable_regtrace {
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
