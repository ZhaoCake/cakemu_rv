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

use crate::devices::Devices;

pub struct Memory {
    data: Vec<u8>,
    devices: Devices,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
            devices: Devices::new(),
        }
    }

    // 将虚拟地址转换为内存索引
    #[inline]
    fn translate_address(&self, addr: usize) -> Result<usize, &'static str> {
        match addr {
            // 代码段：0x80000000-0x8FFFFFFF -> 0x00000000-0x0FFFFFFF
            addr if addr >= 0x80000000 && addr < 0x90000000 => Ok(addr - 0x80000000),
            // 数据段：0x01000000-0x01FFFFFF -> 保持原地址
            addr if addr >= 0x01000000 && addr < 0x02000000 => Ok(addr),
            // 外设段：0x02000000-0x02FFFFFF -> 转发到设备
            addr if addr >= 0x02000000 && addr < 0x03000000 => {
                Err("Device address") // 特殊错误，表示这是设备地址
            }
            _ => {
                println!("Invalid memory access at address 0x{:08x}", addr);
                Err("Invalid memory access: address out of valid ranges")
            }
        }
    }

    #[inline]
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
                    println!(
                        "Physical memory access out of bounds: addr={:08x}, len={}, data_len={}",
                        physical_addr,
                        len,
                        self.data.len()
                    );
                    return Err("Memory read out of bounds");
                }

                // 读取数据
                let slice = &self.data[physical_addr..physical_addr + len];
                let value = match len {
                    1 => slice[0] as u32,
                    2 => u16::from_le_bytes(slice.try_into().unwrap()) as u32,
                    4 => u32::from_le_bytes(slice.try_into().unwrap()),
                    _ => unreachable!(),
                };
                Ok(value)
            }
            Err("Device address") => {
                // 设备访问
                self.devices.read(addr, len)
            }
            Err(e) => Err(e),
        }
    }

    #[inline]
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
                    println!(
                        "Physical memory access out of bounds: addr={:08x}, len={}, data_len={}",
                        physical_addr,
                        len,
                        self.data.len()
                    );
                    return Err("Memory write out of bounds");
                }

                // 写入数据
                let bytes = value.to_le_bytes();
                self.data[physical_addr..physical_addr + len].copy_from_slice(&bytes[..len]);
                Ok(())
            }
            Err("Device address") => {
                // 设备访问
                self.devices.write(addr, value, len)
            }
            Err(e) => Err(e),
        }
    }

    pub fn write_bytes(&mut self, addr: usize, data: &[u8]) -> Result<(), &'static str> {
        // 首先检查数据大小是否合理
        if data.len() > 0x10000000 {
            // 代码段最大 256MB
            println!("Data too large: {} bytes", data.len());
            return Err("Data size exceeds maximum allowed");
        }

        // 尝试地址转换
        match self.translate_address(addr) {
            Ok(physical_addr) => {
                // 内存访问
                if physical_addr + data.len() > self.data.len() {
                    println!(
                        "Physical memory access out of bounds: addr={:08x}, len={}, data_len={}",
                        physical_addr,
                        data.len(),
                        self.data.len()
                    );
                    return Err("Memory write out of bounds");
                }

                println!(
                    "Writing {} bytes to physical address 0x{:08x}",
                    data.len(),
                    physical_addr
                );
                self.data[physical_addr..physical_addr + data.len()].copy_from_slice(data);
                Ok(())
            }
            Err("Device address") => {
                // 不允许对设备进行批量写入
                Err("Cannot write bytes to device address")
            }
            Err(e) => Err(e),
        }
    }

    pub fn read_bytes(&self, addr: usize, len: usize) -> Result<&[u8], &'static str> {
        // 首先检查长度是否合理
        if len > 0x10000000 {
            // 代码段最大 256MB
            println!("Read length too large: {} bytes", len);
            return Err("Read length exceeds maximum allowed");
        }

        // 尝试地址转换
        match self.translate_address(addr) {
            Ok(physical_addr) => {
                // 内存访问
                if physical_addr + len > self.data.len() {
                    println!(
                        "Physical memory access out of bounds: addr={:08x}, len={}, data_len={}",
                        physical_addr,
                        len,
                        self.data.len()
                    );
                    return Err("Memory read out of bounds");
                }

                println!(
                    "Reading {} bytes from physical address 0x{:08x}",
                    len, physical_addr
                );
                Ok(&self.data[physical_addr..physical_addr + len])
            }
            Err("Device address") => {
                // 不允许对设备进行批量读取
                Err("Cannot read bytes from device address")
            }
            Err(e) => Err(e),
        }
    }

    pub fn tick_devices(&mut self) {
        self.devices.tick();
    }
}
