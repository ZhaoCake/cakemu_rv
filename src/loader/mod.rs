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