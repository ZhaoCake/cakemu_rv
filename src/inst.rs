#[derive(Debug)]
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

#[derive(Debug)]
pub enum Operation {
    RegWrite { rd: usize, value: u32 },
    RegRegOp { rd: usize, rs1: usize, rs2: usize, op: RegOp },
    RegImmOp { rd: usize, rs1: usize, imm: i32, op: RegOp },
    Branch { rs1: usize, rs2: usize, offset: i32, op: BranchOp },
    Jump { rd: usize, offset: i32 },
    Load { rd: usize, rs1: usize, offset: i32, size: usize },
    Store { rs1: usize, rs2: usize, offset: i32, size: usize },
    SystemCall(SystemCallType),
}

#[derive(Debug)]
pub enum RegOp {
    Add, Sub, And, Or, Xor, Slt, Sltu, Sll, Srl, Sra,  // R-type
    Addi, Slti, Sltiu, Xori, Ori, Andi, Slli, Srli, Srai, // I-type
}

#[derive(Debug, Clone)]
pub enum BranchOp {
    Eq, Ne, Lt, Ge, Ltu, Geu, // B-type
}

#[derive(Debug)]
pub enum SystemCallType {
    Ecall,
    Ebreak,
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
pub struct DecodedInst {
    pub op: Operation,
    pub next_pc: NextPc,
}

#[derive(Debug)]
pub enum NextPc {
    Plus4,
    Jump(i32),
    Branch { cond: BranchOp, rs1: usize, rs2: usize, offset: i32 },
}

pub fn decode_instruction(inst: u32) -> Result<DecodedInst, &'static str> {
    let opcode = inst & 0x7f;
    let funct3 = (inst >> 12) & 0x7;
    let funct7 = (inst >> 25) & 0x7f;

    match opcode {
        0x73 => {  // SYSTEM
            match funct3 {
                0x0 => {
                    match inst {
                        0x00000073 => Ok(DecodedInst {  // ECALL
                            op: Operation::SystemCall(SystemCallType::Ecall),
                            next_pc: NextPc::Plus4,
                        }),
                        0x00100073 => Ok(DecodedInst {  // EBREAK
                            op: Operation::SystemCall(SystemCallType::Ebreak),
                            next_pc: NextPc::Plus4,
                        }),
                        _ => Err("Invalid system instruction"),
                    }
                },
                _ => Err("Invalid funct3 for system instruction"),
            }
        },
        0x37 => {  // LUI
            let ops = Operands::decode(inst, InstType::U);
            Ok(DecodedInst {
                op: Operation::RegWrite { rd: ops.rd, value: (ops.imm as u32) & 0xfffff000 },
                next_pc: NextPc::Plus4,
            })
        },
        0x17 => {  // AUIPC
            let ops = Operands::decode(inst, InstType::U);
            Ok(DecodedInst {
                op: Operation::RegWrite { rd: ops.rd, value: ops.imm as u32 },
                next_pc: NextPc::Plus4,
            })
        },
        0x6f => {  // JAL
            let ops = Operands::decode(inst, InstType::J);
            Ok(DecodedInst {
                op: Operation::Jump { rd: ops.rd, offset: ops.imm },
                next_pc: NextPc::Jump(ops.imm),
            })
        },
        0x67 => {  // JALR
            let ops = Operands::decode(inst, InstType::I);
            if funct3 != 0 {
                return Err("Invalid funct3 for JALR");
            }
            Ok(DecodedInst {
                op: Operation::Jump { rd: ops.rd, offset: ops.imm },
                next_pc: NextPc::Jump(ops.imm),
            })
        },
        0x63 => {  // Branches
            let ops = Operands::decode(inst, InstType::B);
            let op = match funct3 {
                0x0 => BranchOp::Eq,
                0x1 => BranchOp::Ne,
                0x4 => BranchOp::Lt,
                0x5 => BranchOp::Ge,
                0x6 => BranchOp::Ltu,
                0x7 => BranchOp::Geu,
                _ => return Err("Invalid funct3 for branch"),
            };
            Ok(DecodedInst {
                op: Operation::Branch { rs1: ops.rs1, rs2: ops.rs2, offset: ops.imm, op: op.clone() },
                next_pc: NextPc::Branch { cond: op, rs1: ops.rs1, rs2: ops.rs2, offset: ops.imm },
            })
        },
        0x03 => {  // Loads
            let ops = Operands::decode(inst, InstType::I);
            let size = match funct3 {
                0x0 => 1,  // LB
                0x1 => 2,  // LH
                0x2 => 4,  // LW
                0x4 => 1,  // LBU
                0x5 => 2,  // LHU
                _ => return Err("Invalid funct3 for load"),
            };
            Ok(DecodedInst {
                op: Operation::Load { rd: ops.rd, rs1: ops.rs1, offset: ops.imm, size },
                next_pc: NextPc::Plus4,
            })
        },
        0x23 => {  // Stores
            let ops = Operands::decode(inst, InstType::S);
            let size = match funct3 {
                0x0 => 1,  // SB
                0x1 => 2,  // SH
                0x2 => 4,  // SW
                _ => return Err("Invalid funct3 for store"),
            };
            Ok(DecodedInst {
                op: Operation::Store { rs1: ops.rs1, rs2: ops.rs2, offset: ops.imm, size },
                next_pc: NextPc::Plus4,
            })
        },
        0x13 => {  // I-type ALU  // checked
            let ops = Operands::decode(inst, InstType::I);
            let op = match funct3 {
                0x0 => RegOp::Addi,
                0x1 => RegOp::Slli,
                0x2 => RegOp::Slti,
                0x3 => RegOp::Sltiu,
                0x4 => RegOp::Xori,
                0x5 => if funct7 == 0x20 { RegOp::Srai } else { RegOp::Srli },
                0x6 => RegOp::Ori,
                0x7 => RegOp::Andi,
                _ => return Err("Invalid funct3 for I-type ALU"),
            };
            Ok(DecodedInst {
                op: Operation::RegImmOp { rd: ops.rd, rs1: ops.rs1, imm: ops.imm, op },
                next_pc: NextPc::Plus4,
            })
        },
        0x33 => {  // R-type
            let ops = Operands::decode(inst, InstType::R);
            let op = match (funct3, funct7) {
                (0x0, 0x00) => RegOp::Add,
                (0x0, 0x20) => RegOp::Sub,
                (0x1, 0x00) => RegOp::Sll,
                (0x2, 0x00) => RegOp::Slt,
                (0x3, 0x00) => RegOp::Sltu,
                (0x4, 0x00) => RegOp::Xor,
                (0x5, 0x00) => RegOp::Srl,
                (0x5, 0x20) => RegOp::Sra,
                (0x6, 0x00) => RegOp::Or,
                (0x7, 0x00) => RegOp::And,
                _ => return Err("Invalid funct3/funct7 for R-type"),
            };
            Ok(DecodedInst {
                op: Operation::RegRegOp { rd: ops.rd, rs1: ops.rs1, rs2: ops.rs2, op },
                next_pc: NextPc::Plus4,
            })
        },
        _ => Err("Unknown opcode"),
    }
}