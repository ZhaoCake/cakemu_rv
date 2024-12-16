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

// 简单的外设，用于课程设计实验, 仅仅表现统一编址的编址方式

pub struct SimpleDevice {
    pub base_address: u32,
    pub size: u32,
}

impl SimpleDevice {
    pub fn new(base_address: u32, size: u32) -> Self {
        Self { base_address, size }
    }

    pub fn read(&self, addr: u32) -> u32 {
        // 读取外设数据
        addr    
    }

    pub fn write(&self, addr: u32, value: u32) {
        // 写入外设数据
        addr
    }
}