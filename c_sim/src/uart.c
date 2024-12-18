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

void uart_puts(const char *str) {
    // 使用内联汇编实现字符串输出
    asm volatile(
        "li t0, 0x2000000\n\t"     // UART 基地址
        "1:\n\t"                    // 本地标签1
        "lb t1, 0(%0)\n\t"         // 加载字符
        "beqz t1, 2f\n\t"          // 如果是字符串结束符则跳转到标签2
        "sb t1, 0(t0)\n\t"         // 写入字符到 UART
        "addi %0, %0, 1\n\t"       // 字符串指针加1
        "j 1b\n\t"                 // 跳转回标签1继续处理下一个字符
        "2:"                       // 本地标签2
        : "+r"(str)               // 输出操作数：str 会被修改
        : 
        : "t0", "t1", "memory"    // 破坏的寄存器和内存
    );
} 