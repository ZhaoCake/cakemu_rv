use crate::debugger::Debugger;
use crate::loader::Loader;
use crate::memory::Memory;
use crate::register::RegisterFile;
use crate::isa::inst::decode_instruction;


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
            pc: 0,
            memory: Memory::new(memory_size),
            debugger: Debugger::new(),
        }
    }

    pub fn step(&mut self) -> Result<(), &'static str> {
        let raw_inst = self.fetch()?;
        let instruction = decode_instruction(raw_inst);
        self.pc = (instruction.execute)(&instruction.operands, self)?;
        Ok(())
    }

    fn fetch(&self) -> Result<u32, &'static str> {
        self.memory.vread(self.pc as usize, 4)
    }

    // 寄存器访问方法
    pub fn read_reg(&self, index: usize) -> u32 {
        self.registers.read(index)
    }

    pub fn write_reg(&mut self, index: usize, value: u32) {
        self.registers.write(index, value)
    }

    // PC 相关操作
    pub fn get_pc(&self) -> u32 {
        self.pc
    }

    pub fn set_pc(&mut self, new_pc: u32) -> Result<(), &'static str> {
        if new_pc % 4 != 0 {
            return Err("PC must be aligned to 4 bytes");
        }
        self.pc = new_pc;
        Ok(())
    }

    // 分支跳转
    pub fn branch(&mut self, offset: i32) -> Result<(), &'static str> {
        let new_pc = self.pc.wrapping_add(offset as u32);
        self.set_pc(new_pc)
    }

    // 内存访问包装方法
    pub fn read_memory(&mut self, addr: u32, len: usize) -> Result<u32, &'static str> {
        let value = self.memory.vread(addr as usize, len)?;
        self.debugger.trace_memory_read(addr as usize, len, value);
        Ok(value)
    }

    pub fn write_memory(&mut self, addr: u32, value: u32, len: usize) -> Result<(), &'static str> {
        self.debugger.trace_memory_write(addr as usize, len, value);
        self.memory.vwrite(addr as usize, value, len)
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
    pub fn dump_memory(&self, start: u32, length: usize) -> Vec<u8> {
        let mut result = Vec::with_capacity(length);
        if let Ok(data) = self.memory.read_bytes(start as usize, length) {
            result.extend_from_slice(data);
        }
        result
    }

    // 添加调试器控制方法
    pub fn set_itrace(&mut self, enabled: bool) {
        self.debugger.itrace_enabled = enabled;
    }

    pub fn set_mtrace(&mut self, enabled: bool) {
        self.debugger.mtrace_enabled = enabled;
    }

    pub fn set_single_step(&mut self, enabled: bool) {
        self.debugger.single_step = enabled;
    }

    pub fn show_registers(&self) {
        println!("=== Register State ===");
        println!("PC: 0x{:08x}", self.pc);
        self.registers.dump();
    }
}
