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
pub mod uart;
pub mod gpio;
pub mod timer;
pub mod wave;
pub mod display;

use uart::Uart;
// use gpio::Gpio;
use timer::Timer;
use wave::Wave;
use display::Display;

#[derive(Debug)]
pub struct DeviceConfig {
    pub uart_enabled: bool,
    pub uart_base: usize,
    pub timer_enabled: bool,
    pub timer_base: usize,
    pub timer_auto_reload: bool,
    pub timer_interrupt: bool,
    pub wave_enabled: bool,
    pub wave_base: usize,
    pub wave_output: String,
    pub wave_sample_rate: u32,
    pub display_enabled: bool,
    pub display_base: usize,
    pub display_title: String,
}

pub struct Devices {
    uart: Uart,
    timer: Timer,
    wave: Wave,
    display: Display,
    config: DeviceConfig,
}

impl Devices {
    pub fn new(config: DeviceConfig) -> Self {
        Self {
            uart: if config.uart_enabled { Uart::new() } else { Uart::new_disabled() },
            timer: if config.timer_enabled { 
                Timer::new_with_config(config.timer_auto_reload, config.timer_interrupt) 
            } else { 
                Timer::new_disabled() 
            },
            wave: if config.wave_enabled { 
                Wave::new_with_config(&config.wave_output, config.wave_sample_rate) 
            } else { 
                Wave::new_disabled() 
            },
            display: if config.display_enabled {
                Display::new_with_config(&config.display_title)
            } else {
                Display::new_disabled()
            },
            config,
        }
    }

    pub fn read(&mut self, addr: usize, size: usize) -> Result<u32, &'static str> {
        match addr {
            addr if addr >= self.config.uart_base && addr < self.config.uart_base + 0x10 => {
                if !self.config.uart_enabled {
                    return Err("UART is disabled");
                }
                self.uart.read(addr - self.config.uart_base, size)
            },
            addr if addr >= self.config.timer_base && addr < self.config.timer_base + 0x10 => {
                if !self.config.timer_enabled {
                    return Err("Timer is disabled");
                }
                self.timer.read(addr - self.config.timer_base, size)
            },
            addr if addr >= self.config.wave_base && addr < self.config.wave_base + 0x20 => {
                if !self.config.wave_enabled {
                    return Err("Wave generator is disabled");
                }
                self.wave.read(addr - self.config.wave_base, size)
            },
            addr if addr >= self.config.display_base && addr < self.config.display_base + 0x20 => {
                if !self.config.display_enabled {
                    return Err("Display is disabled");
                }
                self.display.read(addr - self.config.display_base, size)
            },
            _ => Err("Invalid device address"),
        }
    }

    pub fn write(&mut self, addr: usize, value: u32, size: usize) -> Result<(), &'static str> {
        match addr {
            addr if addr >= self.config.uart_base && addr < self.config.uart_base + 0x10 => {
                if !self.config.uart_enabled {
                    return Err("UART is disabled");
                }
                self.uart.write(addr - self.config.uart_base, value, size)
            },
            addr if addr >= self.config.timer_base && addr < self.config.timer_base + 0x10 => {
                if !self.config.timer_enabled {
                    return Err("Timer is disabled");
                }
                self.timer.write(addr - self.config.timer_base, value, size)
            },
            addr if addr >= self.config.wave_base && addr < self.config.wave_base + 0x20 => {
                if !self.config.wave_enabled {
                    return Err("Wave generator is disabled");
                }
                self.wave.write(addr - self.config.wave_base, value, size)
            },
            addr if addr >= self.config.display_base && addr < self.config.display_base + 0x20 => {
                if !self.config.display_enabled {
                    return Err("Display is disabled");
                }
                self.display.write(addr - self.config.display_base, value, size)
            },
            _ => Err("Invalid device address"),
        }
    }

    pub fn tick(&mut self) {
        if self.config.timer_enabled {
            self.timer.tick();
        }
        if self.config.wave_enabled {
            self.wave.tick();
        }
        if self.config.display_enabled {
            self.display.tick();
        }
    }

    pub fn check_interrupts(&self) -> u32 {
        let mut interrupts = 0;
        if self.config.timer_enabled && self.timer.interrupt_pending() {
            interrupts |= 1 << 0;  // Timer 中断位
        }
        interrupts
    }
}