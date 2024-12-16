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