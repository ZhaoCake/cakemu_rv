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

#include "../include/wave.h"

void wave_init(void) {
    // 初始化波形发生器
    asm volatile(
        "li t0, 0x02000300\n\t"  // 加载波形发生器基地址
        "sw zero, 0(t0)\n\t"     // 控制寄存器清零
        "li t1, 1000\n\t"        // 默认频率 1kHz
        "sw t1, 4(t0)\n\t"       // 设置频率
        "li t1, 255\n\t"         // 默认幅度最大
        "sw t1, 8(t0)\n\t"       // 设置幅度
        "sw zero, 12(t0)\n\t"    // 相位清零
        "li t1, 50\n\t"          // 默认占空比 50%
        "sw t1, 16(t0)"          // 设置占空比
        :
        :
        : "t0", "t1", "memory"
    );
}

void wave_enable(void) {
    asm volatile(
        "li t0, 0x02000300\n\t"  // 加载波形发生器基地址
        "lw t1, 0(t0)\n\t"       // 读取当前控制值
        "ori t1, t1, 1\n\t"      // 设置使能位
        "sw t1, 0(t0)"           // 写回控制寄存器
        :
        :
        : "t0", "t1", "memory"
    );
}

void wave_disable(void) {
    asm volatile(
        "li t0, 0x02000300\n\t"  // 加载波形发生器基地址
        "lw t1, 0(t0)\n\t"       // 读取当前控制值
        "andi t1, t1, -2\n\t"    // 清除使能位
        "sw t1, 0(t0)"           // 写回控制寄存器
        :
        :
        : "t0", "t1", "memory"
    );
}

void wave_set_type(uint32_t type) {
    asm volatile(
        "li t0, 0x02000300\n\t"  // 加载波形发生器基地址
        "lw t1, 0(t0)\n\t"       // 读取当前控制值
        "li t2, -15\n\t"         // ~(0x7 << 1)
        "and t1, t1, t2\n\t"     // 清除类型位
        "slli t2, %0, 1\n\t"     // 将类型左移1位
        "or t1, t1, t2\n\t"      // 设置新的类型
        "sw t1, 0(t0)"           // 写回控制寄存器
        :
        : "r"(type)
        : "t0", "t1", "t2", "memory"
    );
}

void wave_set_frequency(uint32_t freq) {
    asm volatile(
        "li t0, 0x02000300\n\t"  // 加载波形发生器基地址
        "sw %0, 4(t0)"           // 设置频率
        :
        : "r"(freq)
        : "t0", "memory"
    );
}

void wave_set_amplitude(uint32_t amp) {
    asm volatile(
        "li t0, 0x02000300\n\t"  // 加载波形发生器基地址
        "sw %0, 8(t0)"           // 设置幅度
        :
        : "r"(amp)
        : "t0", "memory"
    );
}

void wave_set_phase(uint32_t phase) {
    asm volatile(
        "li t0, 0x02000300\n\t"  // 加载波形发生器基地址
        "sw %0, 12(t0)"          // 设置相位
        :
        : "r"(phase)
        : "t0", "memory"
    );
}

void wave_set_duty(uint32_t duty) {
    asm volatile(
        "li t0, 0x02000300\n\t"  // 加载波形发生器基地址
        "sw %0, 16(t0)"          // 设置占空比
        :
        : "r"(duty)
        : "t0", "memory"
    );
} 