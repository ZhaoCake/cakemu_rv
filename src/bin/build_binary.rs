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

use riscv_emu::tools::binary_builder::BinaryBuilder;

fn main() -> std::io::Result<()> {
    let mut builder = BinaryBuilder::new();
    
    // 示例：构建一个简单的测试程序
    builder.add_instruction(0x00278793);  // addi a5, a5, 2

    builder.add_instruction(0x00278793);

    builder.add_instruction(0x00278793);
    
    // 保存到文件
    builder.save("program.bin")?;
    println!("程序已生成：program.bin");
    
    Ok(())
} 