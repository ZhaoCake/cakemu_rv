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

    // 将虚拟地址转换为内存索引
    fn translate_address(&self, addr: usize) -> Result<usize, &'static str> {
        match addr {
            // 代码段：0x80000000-0x8FFFFFFF -> 0x00000000-0x0FFFFFFF
            addr if addr >= 0x80000000 && addr < 0x90000000 => {
                Ok(addr - 0x80000000)
            }
            // 数据段：0x01000000-0x01FFFFFF -> 保持原地址
            addr if addr >= 0x01000000 && addr < 0x02000000 => {
                Ok(addr)
            }
            // 外设段：0x02000000-0x02FFFFFF -> 保持原地址
            addr if addr >= 0x02000000 && addr < 0x03000000 => {
                Ok(addr)
            }
            _ => {
                println!("Invalid memory access at address 0x{:08x}", addr);
                Err("Invalid memory access: address out of valid ranges")
            }
        }
    }

    pub fn vread(&mut self, addr: usize, len: usize) -> Result<u32, &'static str> {
        // 首先检查长度是否合法
        match len {
            1 | 2 | 4 => (),
            _ => return Err("Invalid memory access length"),
        }

        // 检查地址对齐
        if addr % len != 0 {
            return Err("Misaligned memory access");
        }

        // 然后进行地址转换
        let physical_addr = self.translate_address(addr)?;
        
        // 最后检查转换后的地址是否在物理内存范围内
        if physical_addr + len > self.data.len() {
            println!("Physical memory access out of bounds: addr={:08x}, len={}, data_len={}", 
                physical_addr, len, self.data.len());
            return Err("Memory read out of bounds");
        }

        // 读取数据
        let mut value: u32 = 0;
        for i in 0..len {
            value |= (self.data[physical_addr + i] as u32) << (i * 8);
        }
        Ok(value)
    }

    pub fn vwrite(&mut self, addr: usize, value: u32, len: usize) -> Result<(), &'static str> {
        // 首先检查长度是否合法
        match len {
            1 | 2 | 4 => (),
            _ => return Err("Invalid memory access length"),
        }

        // 检查地址对齐
        if addr % len != 0 {
            return Err("Misaligned memory access");
        }

        // 然后进行地址转换
        let physical_addr = self.translate_address(addr)?;
        
        // 最后检查转换后的地址是否在物理内存范围内
        if physical_addr + len > self.data.len() {
            println!("Physical memory access out of bounds: addr={:08x}, len={}, data_len={}", 
                physical_addr, len, self.data.len());
            return Err("Memory write out of bounds");
        }

        // 写入数据
        for i in 0..len {
            self.data[physical_addr + i] = ((value >> (i * 8)) & 0xFF) as u8;
        }
        Ok(())
    }

    pub fn write_bytes(&mut self, addr: usize, data: &[u8]) -> Result<(), &'static str> {
        // 首先检查数据大小是否合理
        if data.len() > 0x10000000 {  // 代码段最大 256MB
            println!("Data too large: {} bytes", data.len());
            return Err("Data size exceeds maximum allowed");
        }

        // 然后进行地址转换
        let physical_addr = self.translate_address(addr)?;
        
        // 最后检查转换后的地址是否在物理内存范围内
        if physical_addr + data.len() > self.data.len() {
            println!("Physical memory access out of bounds: addr={:08x}, len={}, data_len={}", 
                physical_addr, data.len(), self.data.len());
            return Err("Memory write out of bounds");
        }
        
        println!("Writing {} bytes to physical address 0x{:08x}", data.len(), physical_addr);
        self.data[physical_addr..physical_addr + data.len()].copy_from_slice(data);
        Ok(())
    }

    pub fn read_bytes(&self, addr: usize, len: usize) -> Result<&[u8], &'static str> {
        // 首先检查长度是否合理
        if len > 0x10000000 {  // 代码段最大 256MB
            println!("Read length too large: {} bytes", len);
            return Err("Read length exceeds maximum allowed");
        }

        // 然后进行地址转换
        let physical_addr = self.translate_address(addr)?;
        
        // 最后检查转换后的地址是否在物理内存范围内
        if physical_addr + len > self.data.len() {
            println!("Physical memory access out of bounds: addr={:08x}, len={}, data_len={}", 
                physical_addr, len, self.data.len());
            return Err("Memory read out of bounds");
        }
        
        println!("Reading {} bytes from physical address 0x{:08x}", len, physical_addr);
        Ok(&self.data[physical_addr..physical_addr + len])
    }
}
