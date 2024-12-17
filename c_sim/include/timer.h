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

#ifndef _TIMER_H
#define _TIMER_H

#include <stdint.h>

// Timer 寄存器基地址
#define TIMER_BASE  0x02000200

// Timer 寄存器
#define TIMER_COUNT    (TIMER_BASE + 0x0)
#define TIMER_CONTROL  (TIMER_BASE + 0x4)
#define TIMER_COMPARE  (TIMER_BASE + 0x8)
#define TIMER_STATUS   (TIMER_BASE + 0xC)

// Timer 控制位
#define TIMER_ENABLE    (1 << 0)
#define TIMER_INTERRUPT (1 << 1)
#define TIMER_RELOAD    (1 << 2)

// Timer 状态位
#define TIMER_MATCH     (1 << 0)

// 函数声明
void timer_init(uint32_t compare_value);
void timer_enable(void);
void timer_disable(void);
uint32_t timer_get_status(void);
void timer_clear_status(void);

#endif // _TIMER_H 