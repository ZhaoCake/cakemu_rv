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
#include "include/wave.h"
#include "include/timer.h"

// Delay use nop 
#define DELAY_NOP(n) for (volatile int i = 0; i < n; i++) { asm volatile("nop"); }

// 主函数
int main() {
    // 初始化波形发生器
    wave_init();

    // 配置波形参数
    wave_set_type(WAVE_TYPE_SQUARE);    // 方波
    wave_set_frequency(2);              // 2 Hz
    wave_set_amplitude(255);            // 最大幅度
    wave_set_phase(0);                  // 0度相位
    wave_set_duty(30);                  // 30%占空比

    // 输出提示信息
    UART_PRINT_STR("Wave Generator Test\n");
    UART_PRINT_STR("Generating square wave...\n");

    // 启动波形发生器
    wave_enable();

    DELAY_NOP(100);

    // 停止波形发生器
    wave_disable();

    UART_PRINT_STR("Wave generation completed.\n");
    UART_PRINT_STR("Check wave.txt for output data.\n");

    // 使用 ebreak 结束程序
    asm volatile("ebreak");
    return 0;
}