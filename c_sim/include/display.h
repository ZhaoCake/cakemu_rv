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

#ifndef _DISPLAY_H_
#define _DISPLAY_H_

#include <stdint.h>

// 显示设备基地址
#define DISPLAY_BASE 0x2000300

// 显示设备寄存器偏移
#define DISPLAY_CTRL    0x0  // 控制寄存器
#define DISPLAY_STATUS  0x4  // 状态寄存器
#define DISPLAY_X       0xC  // X坐标
#define DISPLAY_Y       0x10 // Y坐标
#define DISPLAY_COLOR   0x14 // 颜色值
#define DISPLAY_UPDATE  0x18 // 更新显示

// 控制寄存器位
#define DISPLAY_CTRL_ENABLE 0x1  // 显示使能

// 状态寄存器位
#define DISPLAY_STATUS_READY 0x1 // 显示就绪

// 显示设备尺寸
#define DISPLAY_WIDTH  300
#define DISPLAY_HEIGHT 400

// 颜色宏
#define RGB(r,g,b) ((((r) & 0xFF) << 16) | (((g) & 0xFF) << 8) | ((b) & 0xFF))

// 常用颜色
#define COLOR_BLACK   RGB(0,0,0)
#define COLOR_WHITE   RGB(255,255,255)
#define COLOR_RED     RGB(255,0,0)
#define COLOR_GREEN   RGB(0,255,0)
#define COLOR_BLUE    RGB(0,0,255)
#define COLOR_YELLOW  RGB(255,255,0)
#define COLOR_CYAN    RGB(0,255,255)
#define COLOR_MAGENTA RGB(255,0,255)

// 函数声明
void display_init(void);
void display_enable(void);
void display_disable(void);
void display_set_pixel(uint32_t x, uint32_t y, uint32_t color);
void display_update(void);
void display_clear(uint32_t color);
void display_draw_line(uint32_t x1, uint32_t y1, uint32_t x2, uint32_t y2, uint32_t color);
void display_draw_rect(uint32_t x, uint32_t y, uint32_t width, uint32_t height, uint32_t color);
void display_fill_rect(uint32_t x, uint32_t y, uint32_t width, uint32_t height, uint32_t color);

#endif // _DISPLAY_H_ 