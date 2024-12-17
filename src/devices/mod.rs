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

// 设备模块的基本结构
pub mod simple_device;
pub mod uart;
pub mod gpio;
pub mod timer;

use uart::Uart;
use gpio::Gpio;
use timer::Timer;

pub struct Devices {
    uart: Uart,
    gpio: Gpio,
    timer: Timer,
}

impl Devices {
    pub fn new() -> Self {
        Self {
            uart: Uart::new(),
            gpio: Gpio::new(),
            timer: Timer::new(),
        }
    }

    pub fn read(&mut self, addr: usize, size: usize) -> Result<u32, &'static str> {
        match addr {
            0x02000000..=0x0200000F => self.uart.read(addr & 0xF, size),
            0x02000100..=0x0200010F => self.gpio.read(addr & 0xF, size),
            0x02000200..=0x0200020F => self.timer.read(addr & 0xF, size),
            _ => Err("Invalid device address"),
        }
    }

    pub fn write(&mut self, addr: usize, value: u32, size: usize) -> Result<(), &'static str> {
        match addr {
            0x02000000..=0x0200000F => self.uart.write(addr & 0xF, value, size),
            0x02000100..=0x0200010F => self.gpio.write(addr & 0xF, value, size),
            0x02000200..=0x0200020F => self.timer.write(addr & 0xF, value, size),
            _ => Err("Invalid device address"),
        }
    }

    // 更新所有设备状态
    pub fn tick(&mut self) {
        self.timer.tick();
    }

    // 检查是否有待处理的中断
    pub fn check_interrupts(&self) -> u32 {
        let mut interrupts = 0;
        if self.timer.interrupt_pending() {
            interrupts |= 1 << 0;  // Timer 中断位
        }
        interrupts
    }
}