use riscv_emu::tools::binary_builder::BinaryBuilder;

fn main() -> std::io::Result<()> {
    let mut builder = BinaryBuilder::new();
    
    // 示例：构建一个简单的测试程序
    // addi x1, x0, 5    // x1 = 5
    builder.add_instruction(0x00500093);
    // addi x2, x0, 3    // x2 = 3
    builder.add_instruction(0x00300113);
    // add x3, x1, x2    // x3 = x1 + x2
    builder.add_instruction(0x002081b3);
    
    // 保存到文件
    builder.save("program.bin")?;
    println!("程序已生成：program.bin");
    
    Ok(())
} 