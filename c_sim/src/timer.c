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

#include "../include/timer.h"

void timer_init(uint32_t compare_value) {
    asm volatile(
        "li t0, 0x02000200\n\t"  // 加载 Timer 基地址
        "sw %0, 8(t0)\n\t"       // 设置比较值
        "sw zero, 4(t0)\n\t"     // 初始禁用
        "sw zero, 12(t0)"        // 清除状态
        : 
        : "r"(compare_value)
        : "t0", "memory"
    );
}

void timer_enable(void) {
    asm volatile(
        "li t0, 0x02000200\n\t"  // 加载 Timer 基地址
        "li t1, 7\n\t"           // TIMER_ENABLE | TIMER_INTERRUPT | TIMER_RELOAD
        "sw t1, 4(t0)"           // 写入控制寄存器
        : 
        : 
        : "t0", "t1", "memory"
    );
}

void timer_disable(void) {
    asm volatile(
        "li t0, 0x02000200\n\t"  // 加载 Timer 基地址
        "sw zero, 4(t0)"         // 写入控制寄存器
        : 
        : 
        : "t0", "memory"
    );
}

uint32_t timer_get_status(void) {
    volatile uint32_t *status = (volatile uint32_t *)0x0200020C;
    return *status;
}

void timer_clear_status(void) {
    volatile uint32_t *status = (volatile uint32_t *)0x0200020C;
    *status = 1;  // 写1清零
} 