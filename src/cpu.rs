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

use crate::debugger::Debugger;
use crate::inst::{decode_instruction, BranchOp, NextPc, Operation, RegOp, SystemCallType};
use crate::loader::Loader;
use crate::memory::Memory;
use crate::register::RegisterFile;

// System Call Constants
const SYS_EXIT: u32 = 93;
const SYS_WRITE: u32 = 64;

pub struct Cpu {
    registers: RegisterFile,
    pc: u32,
    memory: Memory,
    debugger: Debugger,
}

impl Cpu {
    pub fn new(memory_size: usize) -> Self {
        Self {
            registers: RegisterFile::new(),
            pc: 0x80000000, // init pc=0x80000000
            memory: Memory::new(memory_size),
            debugger: Debugger::new(),
        }
    }

    pub fn step(&mut self) -> Result<(), &'static str> {
        let raw_inst = self.fetch()?;
        let decoded = decode_instruction(raw_inst)?;

        // 执行指令前的调试信息
        if self.debugger.itrace_enabled {
            self.debugger
                .trace_instruction(self.pc, raw_inst, "TODO: add disassembly");
        }

        // 执行操作
        match decoded.op {
            Operation::RegWrite { rd, value } => {
                self.registers.write(rd, value);
            }
            Operation::RegImmOp { rd, rs1, imm, op } => {
                let rs1_val = self.registers.read(rs1);
                let result = Self::execute_alu_op(op, rs1_val, imm as u32);
                self.registers.write(rd, result);
            }
            Operation::RegRegOp { rd, rs1, rs2, op } => {
                let rs1_val = self.registers.read(rs1);
                let rs2_val = self.registers.read(rs2);
                let result = Self::execute_alu_op(op, rs1_val, rs2_val);
                self.registers.write(rd, result);
            }
            Operation::Load {
                rd,
                rs1,
                offset,
                size,
            } => {
                let addr = self.registers.read(rs1).wrapping_add(offset as u32);
                let value = self.read(addr as usize, size)?;
                self.registers.write(rd, value);
            }
            Operation::Store {
                rs1,
                rs2,
                offset,
                size,
            } => {
                let addr = self.registers.read(rs1).wrapping_add(offset as u32);
                let value = self.registers.read(rs2);
                self.write(addr as usize, value, size)?;
            }
            Operation::Jump { rd, offset: _ } => {
                self.registers.write(rd, self.pc.wrapping_add(4));
            }
            Operation::Branch { .. } => (), // 分支操作在 next_pc 中处理
            Operation::SystemCall(syscall_type) => {
                match syscall_type {
                    SystemCallType::Ebreak => {
                        println!("[SYSTEM] Breakpoint hit at PC: 0x{:08x}", self.pc);
                        // exit with code 0
                        std::process::exit(0);
                    }
                    SystemCallType::Ecall => {
                        // 获取系统调用号（在 a7 寄存器中）
                        let syscall_num = self.registers.read(17); // a7 寄存器
                                                                   // 获取参数（在 a0-a6 寄存器中）
                        let a0 = self.registers.read(10);

                        // 处理系统调用
                        match syscall_num {
                            SYS_EXIT => {
                                // exit
                                println!("[SYSTEM] Program exit with code: {}", a0);
                                return Err("Program exit");
                            }
                            SYS_WRITE => {
                                // write
                                println!("[SYSTEM] Write syscall: {}", a0);
                            }
                            _ => {
                                println!("[SYSTEM] Unimplemented syscall: {}", syscall_num);
                            }
                        }
                    }
                }
            }
        }

        // 更新 PC
        self.pc = match decoded.next_pc {
            NextPc::Plus4 => self.pc.wrapping_add(4),
            NextPc::Jump(offset) => self.pc.wrapping_add(offset as u32),
            NextPc::JumpReg { rs1, offset, .. } => {
                let rs1_val = self.registers.read(rs1);
                (rs1_val.wrapping_add(offset as u32)) & !1 // 确保最低位为0
            }
            NextPc::Branch {
                cond,
                rs1,
                rs2,
                offset,
            } => {
                let rs1_val = self.registers.read(rs1);
                let rs2_val = self.registers.read(rs2);
                let take_branch = match cond {
                    BranchOp::Eq => rs1_val == rs2_val,
                    BranchOp::Ne => rs1_val != rs2_val,
                    BranchOp::Lt => (rs1_val as i32) < (rs2_val as i32),
                    BranchOp::Ge => (rs1_val as i32) >= (rs2_val as i32),
                    BranchOp::Ltu => rs1_val < rs2_val,
                    BranchOp::Geu => rs1_val >= rs2_val,
                };
                if take_branch {
                    self.pc.wrapping_add(offset as u32)
                } else {
                    self.pc.wrapping_add(4)
                }
            }
        };

        // 更新设备状态
        self.memory.tick_devices();

        // 单步执行等待
        if self.debugger.single_step {
            self.debugger.wait_for_next();
        }
        Ok(())
    }

    fn execute_alu_op(op: RegOp, val1: u32, val2: u32) -> u32 {
        match op {
            RegOp::Add | RegOp::Addi => val1.wrapping_add(val2),
            RegOp::Sub => val1.wrapping_sub(val2),
            RegOp::Sll | RegOp::Slli => val1 << (val2 & 0x1f),
            RegOp::Slt | RegOp::Slti => ((val1 as i32) < (val2 as i32)) as u32,
            RegOp::Sltu | RegOp::Sltiu => (val1 < val2) as u32,
            RegOp::Xor | RegOp::Xori => val1 ^ val2,
            RegOp::Srl | RegOp::Srli => val1 >> (val2 & 0x1f),
            RegOp::Sra | RegOp::Srai => ((val1 as i32) >> (val2 & 0x1f)) as u32,
            RegOp::Or | RegOp::Ori => val1 | val2,
            RegOp::And | RegOp::Andi => val1 & val2,
        }
    }

    fn fetch(&mut self) -> Result<u32, &'static str> {
        self.read(self.pc as usize, 4)
    }

    // memory read/write
    fn read(&mut self, addr: usize, len: usize) -> Result<u32, &'static str> {
        let value = self.memory.vread(addr, len)?;
        if self.debugger.mtrace_enabled {
            self.debugger.trace_memory_read(addr, len, value);
        }
        Ok(value)
    }

    fn write(&mut self, addr: usize, value: u32, len: usize) -> Result<(), &'static str> {
        if self.debugger.mtrace_enabled {
            self.debugger.trace_memory_write(addr, len, value);
        }
        self.memory.vwrite(addr, value, len)
    }

    fn set_pc(&mut self, new_pc: u32) -> Result<(), &'static str> {
        if new_pc % 4 != 0 {
            return Err("PC must be aligned to 4 bytes");
        }
        self.pc = new_pc;
        Ok(())
    }

    // 添加新方法
    pub fn load_program(&mut self, filename: &str) -> std::io::Result<()> {
        let loader = Loader::new();
        loader.load_program(&mut self.memory, filename)?;

        // 设置 PC 为程序入口点
        self.set_pc(loader.get_entry_point() as u32)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        Ok(())
    }

    // 添加内存转储方法（用于调试）
    pub fn dump_memory(&mut self, start: u32, length: usize) -> Vec<u8> {
        if let Ok(data) = self.memory.read_bytes(start as usize, length) {
            data.to_vec()
        } else {
            // fallback if read_bytes fails or is not implemented for the region
            let mut result = Vec::with_capacity(length);
            for addr in (start as usize)..(start as usize + length) {
                if let Ok(value) = self.read(addr, 1) {
                    result.push(value as u8);
                } else {
                    result.push(0); // or handle error
                }
            }
            result
        }
    }

    // 添加调试器控制方法
    pub fn set_itrace(&mut self, enabled: bool) {
        self.debugger.itrace_enabled = enabled;
    }

    pub fn set_mtrace(&mut self, enabled: bool) {
        self.debugger.mtrace_enabled = enabled;
    }

    pub fn set_regtrace(&mut self, enabled: bool) {
        self.debugger.regtrace_enabled = enabled;
    }

    pub fn set_single_step(&mut self, enabled: bool) {
        self.debugger.single_step = enabled;
    }

    pub fn show_registers(&self) {
        if self.debugger.regtrace_enabled {
            println!("=== Register State ===");
            println!("PC: 0x{:08x}", self.pc);
            self.registers.dump();
        }
    }
}
