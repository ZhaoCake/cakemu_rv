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

use opencv::{
    prelude::*,
    highgui,
    core::{Mat, Scalar},
};

// 显示设备寄存器偏移
const DISPLAY_CTRL: usize = 0x0;     // 控制寄存器
const DISPLAY_STATUS: usize = 0x4;   // 状态寄存器
const DISPLAY_X: usize = 0xC;        // X坐标
const DISPLAY_Y: usize = 0x10;       // Y坐标
const DISPLAY_COLOR: usize = 0x14;   // 颜色值
const DISPLAY_UPDATE: usize = 0x18;  // 更新显示

// 控制寄存器位
const CTRL_ENABLE: u32 = 1 << 0;     // 显示使能

// 状态寄存器位
const STATUS_READY: u32 = 1 << 0;    // 显示就绪

const DISPLAY_WIDTH: usize = 300;
const DISPLAY_HEIGHT: usize = 400;

pub struct Display {
    window: Option<String>,
    image: Option<Mat>,
    control: u32,
    status: u32,
    x: u32,
    y: u32,
    color: u32,
}

impl Display {
    pub fn new() -> Self {
        Self {
            window: None,
            image: None,
            control: 0,
            status: STATUS_READY,
            x: 0,
            y: 0,
            color: 0,
        }
    }

    pub fn new_with_config(title: &str) -> Self {
        // 创建黑色图像
        let image = Mat::new_rows_cols_with_default(
            DISPLAY_HEIGHT as i32,
            DISPLAY_WIDTH as i32,
            opencv::core::CV_8UC3,
            Scalar::all(0.0)
        ).unwrap();

        // 创建窗口
        highgui::named_window(title, highgui::WINDOW_AUTOSIZE).unwrap();
        highgui::imshow(title, &image).unwrap();
        highgui::wait_key(1).unwrap();

        Self {
            window: Some(title.to_string()),
            image: Some(image),
            control: CTRL_ENABLE,
            status: STATUS_READY,
            x: 0,
            y: 0,
            color: 0,
        }
    }

    pub fn new_disabled() -> Self {
        Self::new()
    }

    pub fn read(&self, offset: usize, size: usize) -> Result<u32, &'static str> {
        if size != 4 {
            return Err("Display only supports word access");
        }

        match offset {
            DISPLAY_CTRL => Ok(self.control),
            DISPLAY_STATUS => Ok(self.status),
            DISPLAY_X => Ok(self.x),
            DISPLAY_Y => Ok(self.y),
            DISPLAY_COLOR => Ok(self.color),
            _ => Err("Invalid Display register offset"),
        }
    }

    pub fn write(&mut self, offset: usize, value: u32, size: usize) -> Result<(), &'static str> {
        if size != 4 {
            return Err("Display only supports word access");
        }

        match offset {
            DISPLAY_CTRL => {
                self.control = value;
                if value & CTRL_ENABLE != 0 {
                    println!("[Display] Display enabled");
                }
                Ok(())
            },
            DISPLAY_X => {
                if value >= DISPLAY_WIDTH as u32 {
                    return Err("X coordinate out of range");
                }
                self.x = value;
                Ok(())
            },
            DISPLAY_Y => {
                if value >= DISPLAY_HEIGHT as u32 {
                    return Err("Y coordinate out of range");
                }
                self.y = value;
                Ok(())
            },
            DISPLAY_COLOR => {
                self.color = value;
                if self.control & CTRL_ENABLE != 0 {
                    if let Some(image) = &mut self.image {
                        // 转换 RGB 颜色
                        let b = (value & 0xFF) as u8;
                        let g = ((value >> 8) & 0xFF) as u8;
                        let r = ((value >> 16) & 0xFF) as u8;
                        
                        let row = self.y as i32;
                        let ptr = image.ptr_mut(row).unwrap();
                        let offset = (self.x as usize * 3) as isize;
                        unsafe {
                            *ptr.offset(offset + 0) = b;
                            *ptr.offset(offset + 1) = g;
                            *ptr.offset(offset + 2) = r;
                        }
                    }
                }
                Ok(())
            },
            DISPLAY_UPDATE => {
                if self.control & CTRL_ENABLE != 0 {
                    if let (Some(title), Some(image)) = (&self.window, &self.image) {
                        highgui::imshow(title, image).unwrap();
                        highgui::wait_key(1).unwrap();
                    }
                }
                Ok(())
            },
            _ => Err("Invalid Display register offset"),
        }
    }

    pub fn tick(&mut self) {
        if self.control & CTRL_ENABLE != 0 {
            if let Some(_) = &self.window {
                let key = highgui::wait_key(1).unwrap();
                if key == 27 {  // ESC 键
                    self.control &= !CTRL_ENABLE;
                    println!("[Display] Window closed");
                }
            }
        }
    }
} 