# Copyright (C) 2024 ZhaoCake
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

# Makefile for RISC-V bare metal project

# 编译器和工具链
CC = riscv32-unknown-elf-gcc
OBJCOPY = riscv32-unknown-elf-objcopy
OBJDUMP = riscv32-unknown-elf-objdump

# 目录设置
SRC_DIR = src
INC_DIR = include
BUILD_DIR = build

# 源文件
SRCS = main.c start.S $(wildcard $(SRC_DIR)/*.c)

# 头文件
INC = -I$(INC_DIR)

# 输出文件
OUTPUT_ELF = $(BUILD_DIR)/program.elf
OUTPUT_BIN = $(BUILD_DIR)/program.bin
OUTPUT_TXT = $(BUILD_DIR)/program.txt

# 编译选项
CFLAGS = -march=rv32i -mabi=ilp32 -nostdlib -T linker.ld \
         -ffreestanding -O2 -flto -ffunction-sections -fdata-sections \
         -Wall -Wextra \
         $(INC)

# 链接选项
LDFLAGS = -Wl,--gc-sections

# 默认目标
all: $(BUILD_DIR) $(OUTPUT_ELF) $(OUTPUT_BIN) $(OUTPUT_TXT)

# 创建构建目录
$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

# 生成 ELF 文件
$(OUTPUT_ELF): $(SRCS)
	$(CC) $(CFLAGS) $(LDFLAGS) -o $@ $^

# 生成 BIN 文件
$(OUTPUT_BIN): $(OUTPUT_ELF)
	$(OBJCOPY) -j .text -j .data -O binary $< $@

# 生成反汇编文本文件
$(OUTPUT_TXT): $(OUTPUT_ELF)
	$(OBJDUMP) -d $< > $@

# 清理生成的文件
clean:
	rm -rf $(BUILD_DIR)

# 显示文件大小
size: $(OUTPUT_ELF)
	size $<
	ls -l $(OUTPUT_BIN)

.PHONY: all clean size