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

use crate::devices::{Devices, DeviceConfig};

// 固定的内存布局
pub const MEMORY_SIZE: usize = 0x10000000;      // 256MB 总大小
pub const CODE_BASE: usize = 0x80000000;        // 代码段基地址
pub const DATA_BASE: usize = 0x81000000;        // 数据段基地址
pub const DEVICE_BASE: usize = 0x82000000;      // 设备基地址

#[derive(Debug)]
pub struct MemoryConfig {
    pub size: usize,  // 仅保留大小配置
}

pub struct Memory {
    data: Vec<u8>,
    devices: Devices,
}

impl Memory {
    pub fn new(dev_config: DeviceConfig) -> Self {
        Self {
            data: vec![0; MEMORY_SIZE],
            devices: Devices::new(dev_config),
        }
    }

    fn translate_address(&self, addr: usize) -> Result<usize, &'static str> {
        match addr {
            // 代码段 (0x80000000 - 0x80FFFFFF)
            addr if addr >= CODE_BASE && addr < DATA_BASE => {
                Ok(addr - CODE_BASE)  // 转换到物理地址 0 开始
            }
            // 数据段 (0x81000000 - 0x81FFFFFF)
            addr if addr >= DATA_BASE && addr < DEVICE_BASE => {
                Ok((addr - DATA_BASE) + 0x1000000)  // 数据段映射到 16MB 开始
            }
            // 外设段 (0x82000000+)
            addr if addr >= DEVICE_BASE => {
                Err("Device address")
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

        // 尝试地址转换
        match self.translate_address(addr) {
            Ok(physical_addr) => {
                // 内存访问
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
            },
            Err("Device address") => {
                // 设备访问
                self.devices.read(addr, len)
            },
            Err(e) => Err(e),
        }
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

        // 尝试地址转换
        match self.translate_address(addr) {
            Ok(physical_addr) => {
                // 内存访问
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
            },
            Err("Device address") => {
                // 设备访问
                self.devices.write(addr, value, len)
            },
            Err(e) => Err(e),
        }
    }

    pub fn write_bytes(&mut self, addr: usize, data: &[u8]) -> Result<(), &'static str> {
        // 首先检查数据大小是否合理
        if data.len() > 0x10000000 {  // 代码段最大 256MB
            println!("Data too large: {} bytes", data.len());
            return Err("Data size exceeds maximum allowed");
        }

        // 尝试地址转换
        match self.translate_address(addr) {
            Ok(physical_addr) => {
                // 内存访问
                if physical_addr + data.len() > self.data.len() {
                    println!("Physical memory access out of bounds: addr={:08x}, len={}, data_len={}", 
                        physical_addr, data.len(), self.data.len());
                    return Err("Memory write out of bounds");
                }
                
                println!("Writing {} bytes to physical address 0x{:08x}", data.len(), physical_addr);
                self.data[physical_addr..physical_addr + data.len()].copy_from_slice(data);
                Ok(())
            },
            Err(e) => {
                println!("Address translation failed: {}", e);
                Err(e)
            }
        }
    }

    pub fn read_bytes(&self, addr: usize, len: usize) -> Result<&[u8], &'static str> {
        // 首先检查长度是否合理
        if len > 0x10000000 {  // 代码段最大 256MB
            println!("Read length too large: {} bytes", len);
            return Err("Read length exceeds maximum allowed");
        }

        // 尝试地址转换
        match self.translate_address(addr) {
            Ok(physical_addr) => {
                // 内存访问
                if physical_addr + len > self.data.len() {
                    println!("Physical memory access out of bounds: addr={:08x}, len={}, data_len={}", 
                        physical_addr, len, self.data.len());
                    return Err("Memory read out of bounds");
                }
                
                println!("Reading {} bytes from physical address 0x{:08x}", len, physical_addr);
                Ok(&self.data[physical_addr..physical_addr + len])
            },
            Err("Device address") => {
                // 不允许对设备进行批量读取
                Err("Cannot read bytes from device address")
            },
            Err(e) => Err(e),
        }
    }

    pub fn tick_devices(&mut self) {
        self.devices.tick();
    }
}
