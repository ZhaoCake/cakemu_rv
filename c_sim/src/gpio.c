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

#include "../include/gpio.h"

void gpio_set_direction(uint32_t mask) {
    asm volatile(
        "li t0, 0x02000100\n\t"  // 加载 GPIO 基地址
        "sw %0, 0(t0)"           // 写入方向寄存器
        : 
        : "r"(mask)
        : "t0", "memory"
    );
}

void gpio_write(uint32_t value) {
    asm volatile(
        "li t0, 0x02000100\n\t"  // 加载 GPIO 基地址
        "sw %0, 4(t0)"           // 写入输出寄存器
        : 
        : "r"(value)
        : "t0", "memory"
    );
}

uint32_t gpio_read(void) {
    uint32_t value;
    asm volatile(
        "li t0, 0x02000100\n\t"  // 加载 GPIO 基地址
        "lw %0, 8(t0)"           // 读取输入寄存器
        : "=r"(value)
        : 
        : "t0", "memory"
    );
    return value;
} 