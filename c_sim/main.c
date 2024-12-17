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

// c_sim/main.c

#include "include/uart.h"

// 主函数
int main() {
    // 单个字符测试
    uart_putc('H');
    uart_putc('i');
    uart_putc('\n');

    // 使用宏输出字符串
    UART_PRINT_STR("Test String 1\n");
    uart_putc('-');
    UART_PRINT_STR("Test String 2\n");

    // 使用 ebreak 结束程序
    asm volatile("ebreak");
    return 0;
}