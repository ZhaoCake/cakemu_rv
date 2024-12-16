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