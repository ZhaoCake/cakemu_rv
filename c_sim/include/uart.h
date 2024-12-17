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

#ifndef UART_H
#define UART_H

#include <stdint.h>

// UART 寄存器基地址
#define UART_BASE   0x02000000

// UART 寄存器
#define UART_DATA    (UART_BASE + 0x0)
#define UART_STATUS  (UART_BASE + 0x4)
#define UART_CONTROL (UART_BASE + 0x8)

// 函数声明
void uart_putc(char c);

// 宏定义：直接输出字符串
#define UART_PRINT_STR(str) do { \
    const char *p = str; \
    while (*p) { \
        uart_putc(*p++); \
    } \
} while(0)

#endif // UART_H 