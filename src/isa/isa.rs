use crate::cpu::register::RegisterFile;
use crate::isa::inst;

pub struct Isa {
    pub regs: RegisterFile,
}

impl Isa {
    pub fn new() -> Self {
        Self {
            regs: RegisterFile::new(),
        }
    }

    pub fn execute(&mut self, inst: u32, pc: u32) -> Result<u32, &'static str> {
        let decoded = inst::Instruction::decode(inst)?;
        decoded.execute(self, pc)
    }
}
