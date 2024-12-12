use std::fs::File;
use std::io::Write;

pub struct BinaryBuilder {
    code: Vec<u8>,
}

impl BinaryBuilder {
    pub fn new() -> Self {
        Self { code: Vec::new() }
    }

    // 添加32位指令
    pub fn add_instruction(&mut self, instruction: u32) {
        let bytes = instruction.to_le_bytes();
        self.code.extend_from_slice(&bytes);
    }

    // 保存到文件
    pub fn save(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        file.write_all(&self.code)?;
        Ok(())
    }
}

// 使用示例
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_simple_program() -> std::io::Result<()> {
        let mut builder = BinaryBuilder::new();
        
        // 添加一些测试指令
        // addi x1, x0, 5    // x1 = 5
        builder.add_instruction(0x00500093);
        // addi x2, x0, 3    // x2 = 3
        builder.add_instruction(0x00300113);
        // add x3, x1, x2    // x3 = x1 + x2
        builder.add_instruction(0x002081b3);
        
        builder.save("test_program.bin")?;
        Ok(())
    }
} 