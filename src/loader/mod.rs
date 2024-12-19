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
        
        println!("Loading program, size: {} bytes", buffer.len());
        println!("First few bytes: {:02x} {:02x} {:02x} {:02x}", 
            buffer.get(0).unwrap_or(&0), 
            buffer.get(1).unwrap_or(&0),
            buffer.get(2).unwrap_or(&0),
            buffer.get(3).unwrap_or(&0));
        
        // 从代码段基地址开始加载程序
        println!("Attempting to load program at 0x80000000");     
        match memory.write_bytes(0x80000000, &buffer) {
            Ok(_) => println!("Program loaded successfully"),
            Err(e) => {
                println!("Failed to load program: {}", e);
                return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
            }
        }
        
        Ok(())
    }

    pub fn get_entry_point(&self) -> u32 {
        0x80000000  // 程序入口点从 0x80000000 开始
    }
} 