use std::fs::File;
use std::io::Read;

pub struct Loader;

impl Loader {
    pub fn new() -> Self {
        Self
    }

    pub fn load_program(&self, memory: &mut crate::memory::Memory, filename: &str) -> std::io::Result<()> {
        let mut file = File::open(filename)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        
        // 从 0x80000000 开始加载程序
        memory.write_bytes(0x80000000, &buffer)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        Ok(())
    }

    pub fn get_entry_point(&self) -> u32 {
        // 程序入口点固定为 0x80000000
        0x80000000
    }
} 