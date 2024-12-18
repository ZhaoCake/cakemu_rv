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

#include "include/uart.h"
#include "include/timer.h"
#include "include/wave.h"

// UART 测试函数
void test_uart() {
    uart_puts("\n=== UART Test ===\n");
    uart_puts("Testing individual characters: ");
    uart_putc('H');
    uart_putc('e');
    uart_putc('l');
    uart_putc('l');
    uart_putc('o');
    uart_putc('\n');
    
    uart_puts("Testing string output: Hello, World!\n");
    uart_puts("UART test completed.\n");
}

// Timer 测试函数
void test_timer(uint32_t cycles) {
    uart_puts("\n=== Timer Test ===\n");
    uart_puts("Initializing timer...\n");
    
    // 初始化定时器
    timer_init(cycles);
    timer_enable();
    uart_puts("Timer started with ");
    
    // 等待定时器完成
    uart_puts("Waiting for timer...\n");
    while (!(timer_get_status() & TIMER_MATCH)) {
        asm volatile("nop");
    }
    uart_puts("Timer matched!\n");
    
    // 清除定时器状态
    timer_clear_status();
    timer_disable();
    uart_puts("Timer cleared and disabled.\n");
}

// 波形发生器测试函数
void test_wave() {
    uart_puts("\n=== Wave Generator Test ===\n");
    
    // 初始化波形发生器
    uart_puts("Initializing wave generator...\n");
    wave_init();
    
    // 测试正弦波
    uart_puts("Testing Sine Wave...\n");
    wave_set_type(WAVE_TYPE_SINE);
    wave_set_frequency(100);  // 1kHz
    wave_set_amplitude(255);   // 最大幅度
    wave_set_phase(0);        // 0度相位
    wave_enable();
    
    // 等待一段时间
    timer_init(100);
    timer_enable();
    while (!(timer_get_status() & TIMER_MATCH)) {
        asm volatile("nop");
    }
    timer_clear_status();
    
    // 测试方波
    uart_puts("Testing Square Wave...\n");
    wave_set_type(WAVE_TYPE_SQUARE);
    wave_set_duty(75);  // 75% 占空比
    
    // 等待一段时间
    timer_init(20);
    timer_enable();
    while (!(timer_get_status() & TIMER_MATCH)) {
        asm volatile("nop");
    }
    timer_clear_status();
    
    // 测试三角波
    uart_puts("Testing Triangle Wave...\n");
    wave_set_type(WAVE_TYPE_TRIANGLE);
    
    // 等待一段时间
    timer_init(100);
    timer_enable();
    while (!(timer_get_status() & TIMER_MATCH)) {
        asm volatile("nop");
    }
    timer_clear_status();
    
    // 测试锯齿波
    uart_puts("Testing Sawtooth Wave...\n");
    wave_set_type(WAVE_TYPE_SAWTOOTH);
    
    // 等待一段时间
    timer_init(100);
    timer_enable();
    while (!(timer_get_status() & TIMER_MATCH)) {
        asm volatile("nop");
    }
    timer_clear_status();
    
    // 禁用波形发生器
    wave_disable();
    uart_puts("Wave generator test completed.\n");
}

// 主函数
int main() {
    uart_puts("Comprehensive Peripheral Test Program\n");
    uart_puts("===================================\n");
    
    // UART 测试
    test_uart();
    
    // Timer 测试
    test_timer(1000);  // 短延时测试
    test_timer(5000);  // 长延时测试
    
    // 波形发生器测试
    test_wave();
    
    uart_puts("\nAll tests completed successfully!\n");
    uart_puts("You can now use tools/plot_wave.py to view the generated waveforms.\n");
    
    // 使用 ebreak 结束程序
    asm volatile("ebreak");
    return 0;
}