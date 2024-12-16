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

pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }

    // 将物理地址转换为内存索引
    fn translate_address(&self, addr: usize) -> Result<usize, &'static str> {
        if addr < 0x80000000 {
            // return Err("Invalid memory access: address below 0x80000000");
            // return addr directly
            return Ok(addr + 0x20000000); 
            // 实际上是把小于0x80000000的这些内存访问映射到了0x20000000以尽量
            // 和程序中0x80000000实际地址0x00000000开头的代码存储空间错开
            // 这要求最好有0x30000000个存储单元
        }
        Ok(addr - 0x80000000)
    }

    pub fn vread(&self, addr: usize, len: usize) -> Result<u32, &'static str> {

        let addr = self.translate_address(addr)?;
        
        // 检查长度是否合法
        match len {
            1 | 2 | 4 => (),
            _ => return Err("Invalid memory access length"),
        }

        // 检查地址对齐
        if addr % len != 0 {
            return Err("Misaligned memory access");
        }

        // 检查访问是否越界
        if addr + len > self.data.len() {
            return Err("Memory access out of bounds");
        }

        // 读取数据
        let mut value: u32 = 0;
        for i in 0..len {
            value |= (self.data[addr + i] as u32) << (i * 8);
        }
        Ok(value)
    }

    pub fn vwrite(&mut self, addr: usize, value: u32, len: usize) -> Result<(), &'static str> {

        let addr = self.translate_address(addr)?;
        
        // 检查长度是否合法
        match len {
            1 | 2 | 4 => (),
            _ => return Err("Invalid memory access length"),
        }

        // 检查地址对齐
        if addr % len != 0 {
            return Err("Misaligned memory access");
        }

        // 检查访问是否越界
        if addr + len > self.data.len() {
            return Err("Memory access out of bounds");
        }

        // 写入数据
        for i in 0..len {
            self.data[addr + i] = ((value >> (i * 8)) & 0xFF) as u8;
        }
        Ok(())
    }

    pub fn write_bytes(&mut self, addr: usize, data: &[u8]) -> Result<(), &'static str> {
        let addr = self.translate_address(addr)?;
        if addr + data.len() > self.data.len() {
            return Err("Memory write out of bounds");
        }
        
        self.data[addr..addr + data.len()].copy_from_slice(data);
        Ok(())
    }

    pub fn read_bytes(&self, addr: usize, len: usize) -> Result<&[u8], &'static str> {
        let addr = self.translate_address(addr)?;
        if addr + len > self.data.len() {
            return Err("Memory read out of bounds");
        }
        
        Ok(&self.data[addr..addr + len])
    }
}
