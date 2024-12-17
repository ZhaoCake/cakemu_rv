#include "../include/gpio.h"

void gpio_set_direction(uint32_t mask) {
    asm volatile(
        "li t0, 0x02000100\n\t"  // 加载 GPIO 基地址
        "sw %0, 0(t0)"           // 写入方向寄存器
        : 
        : "r"(mask)
        : "t0", "memory"
    );
}

void gpio_write(uint32_t value) {
    asm volatile(
        "li t0, 0x02000100\n\t"  // 加载 GPIO 基地址
        "sw %0, 4(t0)"           // 写入输出寄存器
        : 
        : "r"(value)
        : "t0", "memory"
    );
}

uint32_t gpio_read(void) {
    uint32_t value;
    asm volatile(
        "li t0, 0x02000100\n\t"  // 加载 GPIO 基地址
        "lw %0, 8(t0)"           // 读取输入寄存器
        : "=r"(value)
        : 
        : "t0", "memory"
    );
    return value;
} 