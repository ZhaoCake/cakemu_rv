pub mod inst;

use crate::cpu::register::RegisterFile;
use crate::memory::Memory;

pub struct Isa {
    pub regs: RegisterFile,
    pub mem: Memory,
}

impl Isa {
    pub fn new(memory_size: usize) -> Self {
        Self {
            regs: RegisterFile::new(),
            mem: Memory::new(memory_size),
        }
    }

    pub fn execute(&mut self, inst: u32, pc: u64) -> Result<u64, &'static str> {
        let decoded = inst::Instruction::decode(inst)?;
        decoded.execute(self, pc)
    }
}
