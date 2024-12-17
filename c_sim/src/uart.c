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

#include "../include/uart.h"

void uart_putc(char c) {
    // 使用内联汇编直接写入 UART 寄存器
    asm volatile(
        "li t0, 0x2000000\n\t"  // UART 基地址
        "sb %0, 0(t0)"          // 写入字符
        :                       // 无输出操作数
        : "r"(c)               // 输入操作数
        : "t0"                 // 破坏的寄存器
    );
} 