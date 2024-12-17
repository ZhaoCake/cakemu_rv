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

/* linker.ld */
OUTPUT_FORMAT("elf32-littleriscv")
ENTRY(_start)

SECTIONS {
    . = 0x80000000;
    
    .text : {
        *(.text.init)    /* 启动代码放在最前面 */
        *(.text)         /* 其他代码 */
        *(.text.*)       /* 其他代码段 */
        *(.rodata)       /* 只读数据 */
        *(.rodata.*)     /* 其他只读数据段 */
        . = ALIGN(4);
    }

    .data : {
        . = ALIGN(4);
        *(.data)
        *(.data.*)
        *(.sdata)
        *(.sdata.*)
        . = ALIGN(4);
    }

    .bss : {
        . = ALIGN(4);
        *(.bss)
        *(.bss.*)
        *(COMMON)
        . = ALIGN(4);
    }

    /* 删除所有其他不需要的段 */
    /DISCARD/ : {
        *(.eh_frame)
        *(.note.gnu.build-id)
        *(.comment)
    }
}