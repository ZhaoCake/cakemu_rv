pub enum InstType {
    R, I, S, B, U, J
}

#[derive(Debug)]
pub struct Operands {
    pub rd: usize,
    pub rs1: usize,
    pub rs2: usize,
    pub imm: i32,
}

impl Operands {
    pub fn decode(inst: u32, inst_type: InstType) -> Self {
        let rd = ((inst >> 7) & 0x1f) as usize;
        let rs1 = ((inst >> 15) & 0x1f) as usize;
        let rs2 = ((inst >> 20) & 0x1f) as usize;
        
        let imm = match inst_type {
            InstType::I => (inst as i32) >> 20,
            InstType::S => ((inst & 0xfe000000) as i32) >> 20 | ((inst >> 7) & 0x1f) as i32,
            InstType::B => {
                let imm12 = (inst & 0x80000000) as i32;
                let imm11 = ((inst >> 7) & 0x1) << 11;
                let imm10_5 = ((inst >> 25) & 0x3f) << 5;
                let imm4_1 = ((inst >> 8) & 0xf) << 1;
                (imm12 >> 19) | (imm11 as i32) | (imm10_5 as i32) | (imm4_1 as i32)
            },
            InstType::U => (inst & 0xfffff000) as i32,
            InstType::J => {
                let imm20 = (inst & 0x80000000) as i32;
                let imm19_12 = ((inst >> 12) & 0xff) << 12;
                let imm11 = ((inst >> 20) & 0x1) << 11;
                let imm10_1 = ((inst >> 21) & 0x3ff) << 1;
                (imm20 >> 11) | (imm19_12 as i32) | (imm11 as i32) | (imm10_1 as i32)
            },
            InstType::R => 0,
        };

        Self { rd, rs1, rs2, imm }
    }
}

// 定义指令执行函数类型
pub type InstructionFn = fn(&Operands, &mut crate::cpu::Cpu) -> Result<u32, &'static str>;

// 定义指令结构体
#[derive(Debug)]
pub struct RiscvInstruction {
    pub operands: Operands,
    pub execute: InstructionFn,
}

// 解码函数
pub fn decode_instruction(inst: u32) -> RiscvInstruction {
    let opcode = inst & 0x7f;
    let inst_type = InstType::I; // 假设为 I 类型，实际情况应根据 opcode 确定

    let operands = Operands::decode(inst, inst_type);
    match opcode {
        0x13 => RiscvInstruction {
            operands,
            execute, // BUG: make it without cpu structure
        },
        // 其他指令...
        _ => panic!("Unknown opcode"),
    }
}