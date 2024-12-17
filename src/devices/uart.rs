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

// UART 寄存器偏移
const UART_DATA: usize = 0x0;     // 数据寄存器
const UART_STATUS: usize = 0x4;   // 状态寄存器
const UART_CONTROL: usize = 0x8;  // 控制寄存器

// 状态寄存器位
#[allow(dead_code)]
const STATUS_TX_READY: u8 = 1 << 0;  // 发送就绪
#[allow(dead_code)]
const STATUS_RX_READY: u8 = 1 << 1;  // 接收就绪

pub struct Uart {
    data: u8,        // 数据寄存器
    status: u8,      // 状态寄存器
    control: u8,     // 控制寄存器
}

impl Uart {
    pub fn new() -> Self {
        Self {
            data: 0,
            status: STATUS_TX_READY,  // 初始状态：发送就绪
            control: 0,
        }
    }

    pub fn read(&self, offset: usize, size: usize) -> Result<u32, &'static str> {
        if size != 1 {
            return Err("UART only supports byte access");
        }

        match offset {
            UART_DATA => Ok(self.data as u32),
            UART_STATUS => Ok(self.status as u32),
            UART_CONTROL => Ok(self.control as u32),
            _ => Err("Invalid UART register offset"),
        }
    }

    pub fn write(&mut self, offset: usize, value: u32, size: usize) -> Result<(), &'static str> {
        if size != 1 {
            return Err("UART only supports byte access");
        }

        let value = value as u8;
        match offset {
            UART_DATA => {
                self.data = value;
                // 输出字符到控制台
                print!("{}", value as char);
                Ok(())
            },
            UART_STATUS => {
                self.status = value;
                Ok(())
            },
            UART_CONTROL => {
                self.control = value;
                Ok(())
            },
            _ => Err("Invalid UART register offset"),
        }
    }
} 