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

#include <stdint.h>
#include "uart.h"
#include "timer.h"
#include "wave.h"
#include "display.h"

// 测试 UART 输出
void test_uart(void) {
    uart_puts("Hello from RISC-V!\n");
    uart_puts("Testing UART output...\n");
    for (char c = '0'; c <= '9'; c++) {
        uart_putc(c);
    }
    uart_puts("\n");
}

// 测试定时器
void test_timer(void) {
    uart_puts("Testing Timer...\n");
    
    // 初始化定时器，设置比较值为1000
    timer_init(1000);
    
    // 启用定时器（包含中断和自动重载功能）
    timer_enable();
    
    // 清除任何待处理的状态
    timer_clear_status();
    
    uart_puts("Timer configured and started\n");
    uart_puts("- Compare value: 1000\n");
    uart_puts("- Auto-reload: enabled\n");
    uart_puts("- Interrupt: enabled\n");
    
    // 等待并显示几次中断
    for(int i = 0; i < 3; i++) {
        while(!timer_get_status()) { }  // 等待中断
        uart_puts("Timer interrupt triggered!\n");
        timer_clear_status();
    }
}

// 测试波形生成器
void test_wave(void) {
    uart_puts("Testing Wave Generator...\n");
    
    // 设置正弦波
    wave_set_type(0);  // 假设0是正弦波类型
    wave_set_frequency(1000);  // 1kHz
    wave_set_amplitude(100);   // 振幅
    wave_set_phase(0);        // 初始相位
    wave_set_duty(50);        // 占空比（对方波有效）
    
    uart_puts("Wave generator configured:\n");
    uart_puts("- Type: Sine wave\n");
    uart_puts("- Frequency: 1kHz\n");
    uart_puts("- Amplitude: 100\n");
    uart_puts("- Phase: 0\n");
    uart_puts("- Duty: 50%\n");
}

// 测试显示器绘图功能
void test_display(void) {
    uart_puts("Testing Display...\n");
    display_clear(COLOR_BLACK);

    // 绘制边框
    display_draw_rect(0, 0, DISPLAY_WIDTH, DISPLAY_HEIGHT, COLOR_WHITE);

    // 绘制对角线
    display_draw_line(0, 0, DISPLAY_WIDTH-1, DISPLAY_HEIGHT-1, COLOR_RED);
    display_draw_line(DISPLAY_WIDTH-1, 0, 0, DISPLAY_HEIGHT-1, COLOR_BLUE);

    // 绘制彩色方块
    uint32_t colors[] = {COLOR_RED, COLOR_GREEN, COLOR_BLUE, COLOR_YELLOW, 
                        COLOR_CYAN, COLOR_MAGENTA, COLOR_WHITE};
    int block_width = 40;
    int block_height = 40;
    int spacing = 10;
    int start_x = 50;
    int start_y = 50;

    for (int i = 0; i < 7; i++) {
        display_fill_rect(start_x + i * (block_width + spacing),
                         start_y,
                         block_width,
                         block_height,
                         colors[i]);
    }

    // 绘制同心矩形
    int center_x = DISPLAY_WIDTH / 2;
    int center_y = DISPLAY_HEIGHT / 2;
    for (int i = 0; i < 5; i++) {
        int size = 40 + i * 30;
        display_draw_rect(center_x - size/2,
                         center_y - size/2,
                         size,
                         size,
                         colors[i % 7]);
    }

    // 更新显示
    display_update();
    uart_puts("Display test complete!\n");
}

int main() {
    // 初始化设备
    timer_init(1000);
    wave_init();
    display_init();
    
    uart_puts("Starting peripheral tests...\n");
    
    test_uart();
    test_timer();
    test_wave();
    test_display();
    
    uart_puts("All tests complete!\n");

    while(1) {
        // 保持程序运行以保持显示和其他功能
    }
    return 0;
}