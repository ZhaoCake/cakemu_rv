use crate::memory::Memory;

pub struct Bus {
    memory: Memory,
}

impl Bus {
    pub fn new(memory_size: usize) -> Self {
        Self {
            memory: Memory::new(memory_size),
        }
    }

    pub fn read(&self, addr: usize, len: usize) -> Result<u32, &'static str> {
        self.memory.read(addr, len)
    }

    pub fn write(&mut self, addr: usize, value: u32, len: usize) -> Result<(), &'static str> {
        self.memory.write(addr, value, len)
    }
} 