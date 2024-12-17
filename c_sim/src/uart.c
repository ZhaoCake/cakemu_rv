#include "../include/uart.h"

void uart_putc(char c) {
    // 使用内联汇编直接写入 UART 寄存器
    asm volatile(
        "li t0, 0x2000000\n\t"  // UART 基地址
        "sb %0, 0(t0)"          // 写入字符
        :                       // 无输出操作数
        : "r"(c)               // 输入操作数
        : "t0"                 // 破坏的寄存器
    );
} 