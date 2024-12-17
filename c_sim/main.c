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

#include <stdint.h>

// 定义内存地址
#define PERIPH_BASE 0x02000000  // 外设基地址
#define DATA_BASE   0x01000000  // 数据段基地址

// 主函数
int main() {
    // 在外设区域写入数据
    volatile uint32_t *periph_ptr = (uint32_t *)PERIPH_BASE;
    *periph_ptr = 0xDEADBEEF;

    // 在数据区域写入数据
    volatile uint32_t *data_ptr = (uint32_t *)DATA_BASE;
    *data_ptr = 0x12345678;

    // 使用 ebreak 退出程序
    asm volatile("ebreak");

    return 0;
}