// RISC-V 指令模块
#[derive(Debug, Clone, Copy)]
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

#[derive(Debug)]
pub enum Instruction {
    // RV32I Base Instructions
    Lui(Operands),
    Auipc(Operands),
    Jal(Operands),
    Jalr(Operands),
    Beq(Operands),
    Bne(Operands),
    Blt(Operands),
    Bge(Operands),
    Bltu(Operands),
    Bgeu(Operands),
    Lb(Operands),
    Lh(Operands),
    Lw(Operands),
    Lbu(Operands),
    Lhu(Operands),
    Sb(Operands),
    Sh(Operands),
    Sw(Operands),
    Addi(Operands),
    Slti(Operands),
    Sltiu(Operands),
    Xori(Operands),
    Ori(Operands),
    Andi(Operands),
    Slli(Operands),
    Srli(Operands),
    Srai(Operands),
    Add(Operands),
    Sub(Operands),
    Sll(Operands),
    Slt(Operands),
    Sltu(Operands),
    Xor(Operands),
    Srl(Operands),
    Sra(Operands),
    Or(Operands),
    And(Operands),
}

impl Instruction {
    pub fn decode(inst: u32) -> Result<Self, &'static str> {
        let opcode = inst & 0x7f;
        let funct3 = (inst >> 12) & 0x7;
        let funct7 = (inst >> 25) & 0x7f;

        match opcode {
            0x37 => Ok(Self::Lui(Operands::decode(inst, InstType::U))),
            0x17 => Ok(Self::Auipc(Operands::decode(inst, InstType::U))),
            0x6f => Ok(Self::Jal(Operands::decode(inst, InstType::J))),
            0x67 => {
                match funct3 {
                    0x0 => Ok(Self::Jalr(Operands::decode(inst, InstType::I))),
                    _ => Err("Invalid funct3 for JALR"),
                }
            },
            0x63 => {
                let ops = Operands::decode(inst, InstType::B);
                match funct3 {
                    0x0 => Ok(Self::Beq(ops)),
                    0x1 => Ok(Self::Bne(ops)),
                    0x4 => Ok(Self::Blt(ops)),
                    0x5 => Ok(Self::Bge(ops)),
                    0x6 => Ok(Self::Bltu(ops)),
                    0x7 => Ok(Self::Bgeu(ops)),
                    _ => Err("Invalid funct3 for branch"),
                }
            },
            0x03 => {
                let ops = Operands::decode(inst, InstType::I);
                match funct3 {
                    0x0 => Ok(Self::Lb(ops)),
                    0x1 => Ok(Self::Lh(ops)),
                    0x2 => Ok(Self::Lw(ops)),
                    0x4 => Ok(Self::Lbu(ops)),
                    0x5 => Ok(Self::Lhu(ops)),
                    _ => Err("Invalid funct3 for load"),
                }
            },
            0x23 => {
                let ops = Operands::decode(inst, InstType::S);
                match funct3 {
                    0x0 => Ok(Self::Sb(ops)),
                    0x1 => Ok(Self::Sh(ops)),
                    0x2 => Ok(Self::Sw(ops)),
                    _ => Err("Invalid funct3 for store"),
                }
            },
            0x13 => {
                let ops = Operands::decode(inst, InstType::I);
                match funct3 {
                    0x0 => Ok(Self::Addi(ops)),
                    0x2 => Ok(Self::Slti(ops)),
                    0x3 => Ok(Self::Sltiu(ops)),
                    0x4 => Ok(Self::Xori(ops)),
                    0x6 => Ok(Self::Ori(ops)),
                    0x7 => Ok(Self::Andi(ops)),
                    0x1 => Ok(Self::Slli(ops)),
                    0x5 => match funct7 {
                        0x00 => Ok(Self::Srli(ops)),
                        0x20 => Ok(Self::Srai(ops)),
                        _ => Err("Invalid funct7 for shift right"),
                    },
                    _ => Err("Invalid funct3 for immediate arithmetic"),
                }
            },
            0x33 => {
                let ops = Operands::decode(inst, InstType::R);
                match (funct3, funct7) {
                    (0x0, 0x00) => Ok(Self::Add(ops)),
                    (0x0, 0x20) => Ok(Self::Sub(ops)),
                    (0x1, 0x00) => Ok(Self::Sll(ops)),
                    (0x2, 0x00) => Ok(Self::Slt(ops)),
                    (0x3, 0x00) => Ok(Self::Sltu(ops)),
                    (0x4, 0x00) => Ok(Self::Xor(ops)),
                    (0x5, 0x00) => Ok(Self::Srl(ops)),
                    (0x5, 0x20) => Ok(Self::Sra(ops)),
                    (0x6, 0x00) => Ok(Self::Or(ops)),
                    (0x7, 0x00) => Ok(Self::And(ops)),
                    _ => Err("Invalid funct3/funct7 for register arithmetic"),
                }
            },
            _ => Err("Unknown opcode"),
        }
    }

    pub fn execute(&self, isa: &mut super::Isa, pc: u64) -> Result<u64, &'static str> {
        match self {
            Self::Lui(op) => {
                isa.regs.write(op.rd, op.imm as u32);
                Ok(pc + 4)
            },
            _ => Err("指令尚未实现"),
        }
    }
}