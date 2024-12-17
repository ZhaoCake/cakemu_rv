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