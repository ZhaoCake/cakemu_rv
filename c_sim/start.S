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

// c_sim/start.S

.section .text.init
.globl _start

_start:
    # 设置栈指针到数据段末尾
    li sp, 0x01ffffff

    # 调用主函数
    call main

    # 程序结束，使用 ebreak
    ebreak

.section .data
    .align 4
    .global _test_data
_test_data:
    .word 0x12345678