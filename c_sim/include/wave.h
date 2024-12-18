#ifndef _WAVE_H
#define _WAVE_H

#include <stdint.h>

// 波形发生器寄存器基地址
#define WAVE_BASE   0x02000300

// 波形发生器寄存器
#define WAVE_CONTROL    (WAVE_BASE + 0x0)  // 控制寄存器
#define WAVE_FREQUENCY  (WAVE_BASE + 0x4)  // 频率寄存器 (Hz)
#define WAVE_AMPLITUDE  (WAVE_BASE + 0x8)  // 幅度寄存器 (0-255)
#define WAVE_PHASE      (WAVE_BASE + 0xC)  // 相位寄存器 (0-359度)
#define WAVE_DUTY       (WAVE_BASE + 0x10) // 占空比寄存器 (0-100%)

// 波形控制寄存器位
#define WAVE_ENABLE     (1 << 0)  // 使能位
#define WAVE_TYPE_MASK  (0x7 << 1)  // 波形类型掩码
#define WAVE_TYPE_SHIFT 1

// 波形类型
#define WAVE_TYPE_SINE     0  // 正弦波
#define WAVE_TYPE_SQUARE   1  // 方波
#define WAVE_TYPE_TRIANGLE 2  // 三角波
#define WAVE_TYPE_SAWTOOTH 3  // 锯齿波

// 函数声明
void wave_init(void);
void wave_enable(void);
void wave_disable(void);
void wave_set_type(uint32_t type);
void wave_set_frequency(uint32_t freq);
void wave_set_amplitude(uint32_t amp);
void wave_set_phase(uint32_t phase);
void wave_set_duty(uint32_t duty);

#endif // _WAVE_H 