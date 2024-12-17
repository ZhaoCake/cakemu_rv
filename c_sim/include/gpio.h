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

#ifndef _GPIO_H
#define _GPIO_H

#include <stdint.h>

// GPIO 寄存器基地址
#define GPIO_BASE   0x02000100

// GPIO 寄存器
#define GPIO_DIRECTION (GPIO_BASE + 0x0)
#define GPIO_OUTPUT    (GPIO_BASE + 0x4)
#define GPIO_INPUT     (GPIO_BASE + 0x8)

// 函数声明
void gpio_set_direction(uint32_t mask);
void gpio_write(uint32_t value);
uint32_t gpio_read(void);

#endif // _GPIO_H 