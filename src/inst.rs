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

#[derive(Debug, Copy, Clone)]
pub enum InstType {
    R,
    I,
    S,
    B,
    U,
    J,
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
    RegWrite {
        rd: usize,
        value: u32,
    },
    RegRegOp {
        rd: usize,
        rs1: usize,
        rs2: usize,
        op: RegOp,
    },
    RegImmOp {
        rd: usize,
        rs1: usize,
        imm: i32,
        op: RegOp,
    },
    Branch {
        rs1: usize,
        rs2: usize,
        offset: i32,
        op: BranchOp,
    },
    Jump {
        rd: usize,
        offset: i32,
    },
    Load {
        rd: usize,
        rs1: usize,
        offset: i32,
        size: usize,
    },
    Store {
        rs1: usize,
        rs2: usize,
        offset: i32,
        size: usize,
    },
    SystemCall(SystemCallType),
}

#[derive(Debug, Copy, Clone)]
pub enum RegOp {
    Add,
    Sub,
    And,
    Or,
    Xor,
    Slt,
    Sltu,
    Sll,
    Srl,
    Sra, // R-type
    Addi,
    Slti,
    Sltiu,
    Xori,
    Ori,
    Andi,
    Slli,
    Srli,
    Srai, // I-type
}

#[derive(Debug, Copy, Clone)]
pub enum BranchOp {
    Eq,
    Ne,
    Lt,
    Ge,
    Ltu,
    Geu, // B-type
}

#[derive(Debug, Copy, Clone)]
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
            }
            InstType::U => (inst & 0xfffff000) as i32,
            InstType::J => {
                let imm20 = (inst & 0x80000000) as i32;
                let imm19_12 = ((inst >> 12) & 0xff) << 12;
                let imm11 = ((inst >> 20) & 0x1) << 11;
                let imm10_1 = ((inst >> 21) & 0x3ff) << 1;
                (imm20 >> 11) | (imm19_12 as i32) | (imm11 as i32) | (imm10_1 as i32)
            }
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
    JumpReg {
        rd: usize,
        rs1: usize,
        offset: i32,
    },
    Branch {
        cond: BranchOp,
        rs1: usize,
        rs2: usize,
        offset: i32,
    },
}

// -----------------------------------------------------------------------------
// Pattern Matching Helper
// -----------------------------------------------------------------------------

pub struct BitPat {
    mask: u32,
    pattern: u32,
}

impl BitPat {
    pub const fn new(mask: u32, pattern: u32) -> Self {
        Self { mask, pattern }
    }

    #[inline]
    pub fn matches(&self, inst: u32) -> bool {
        (inst & self.mask) == self.pattern
    }
}

// -----------------------------------------------------------------------------
// Decoding Logic
// -----------------------------------------------------------------------------

pub fn decode_instruction(inst: u32) -> Result<DecodedInst, &'static str> {
    let opcode = inst & 0x7f;

    match opcode {
        0x33 => decode_r_type(inst),
        0x13 => decode_i_type_alu(inst),
        0x03 => decode_load(inst),
        0x23 => decode_store(inst),
        0x63 => decode_branch(inst),
        0x67 => decode_jalr(inst),
        0x6f => decode_jal(inst),
        0x37 => decode_lui(inst),
        0x17 => decode_auipc(inst),
        0x73 => decode_system(inst),
        _ => Err("Unknown opcode"),
    }
}

fn decode_r_type(inst: u32) -> Result<DecodedInst, &'static str> {
    let ops = Operands::decode(inst, InstType::R);
    let funct3 = (inst >> 12) & 0x7;
    let funct7 = (inst >> 25) & 0x7f;

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
        op: Operation::RegRegOp {
            rd: ops.rd,
            rs1: ops.rs1,
            rs2: ops.rs2,
            op,
        },
        next_pc: NextPc::Plus4,
    })
}

fn decode_i_type_alu(inst: u32) -> Result<DecodedInst, &'static str> {
    let ops = Operands::decode(inst, InstType::I);
    let funct3 = (inst >> 12) & 0x7;
    let funct7 = (inst >> 25) & 0x7f;

    let op = match funct3 {
        0x0 => RegOp::Addi,
        0x1 => RegOp::Slli,
        0x2 => RegOp::Slti,
        0x3 => RegOp::Sltiu,
        0x4 => RegOp::Xori,
        0x5 => {
            if funct7 == 0x20 {
                RegOp::Srai
            } else {
                RegOp::Srli
            }
        }
        0x6 => RegOp::Ori,
        0x7 => RegOp::Andi,
        _ => return Err("Invalid funct3 for I-type ALU"),
    };

    Ok(DecodedInst {
        op: Operation::RegImmOp {
            rd: ops.rd,
            rs1: ops.rs1,
            imm: ops.imm,
            op,
        },
        next_pc: NextPc::Plus4,
    })
}

fn decode_load(inst: u32) -> Result<DecodedInst, &'static str> {
    let ops = Operands::decode(inst, InstType::I);
    let funct3 = (inst >> 12) & 0x7;

    let size = match funct3 {
        0x0 => 1, // LB
        0x1 => 2, // LH
        0x2 => 4, // LW
        0x4 => 1, // LBU
        0x5 => 2, // LHU
        _ => return Err("Invalid funct3 for load"),
    };

    Ok(DecodedInst {
        op: Operation::Load {
            rd: ops.rd,
            rs1: ops.rs1,
            offset: ops.imm,
            size,
        },
        next_pc: NextPc::Plus4,
    })
}

fn decode_store(inst: u32) -> Result<DecodedInst, &'static str> {
    let ops = Operands::decode(inst, InstType::S);
    let funct3 = (inst >> 12) & 0x7;

    let size = match funct3 {
        0x0 => 1, // SB
        0x1 => 2, // SH
        0x2 => 4, // SW
        _ => return Err("Invalid funct3 for store"),
    };

    Ok(DecodedInst {
        op: Operation::Store {
            rs1: ops.rs1,
            rs2: ops.rs2,
            offset: ops.imm,
            size,
        },
        next_pc: NextPc::Plus4,
    })
}

fn decode_branch(inst: u32) -> Result<DecodedInst, &'static str> {
    let ops = Operands::decode(inst, InstType::B);
    let funct3 = (inst >> 12) & 0x7;

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
        op: Operation::Branch {
            rs1: ops.rs1,
            rs2: ops.rs2,
            offset: ops.imm,
            op,
        },
        next_pc: NextPc::Branch {
            cond: op,
            rs1: ops.rs1,
            rs2: ops.rs2,
            offset: ops.imm,
        },
    })
}

fn decode_jalr(inst: u32) -> Result<DecodedInst, &'static str> {
    let ops = Operands::decode(inst, InstType::I);
    let funct3 = (inst >> 12) & 0x7;

    if funct3 != 0 {
        return Err("Invalid funct3 for JALR");
    }

    Ok(DecodedInst {
        op: Operation::Jump {
            rd: ops.rd,
            offset: ops.imm,
        },
        next_pc: NextPc::JumpReg {
            rd: ops.rd,
            rs1: ops.rs1,
            offset: ops.imm,
        },
    })
}

fn decode_jal(inst: u32) -> Result<DecodedInst, &'static str> {
    let ops = Operands::decode(inst, InstType::J);
    Ok(DecodedInst {
        op: Operation::Jump {
            rd: ops.rd,
            offset: ops.imm,
        },
        next_pc: NextPc::Jump(ops.imm),
    })
}

fn decode_lui(inst: u32) -> Result<DecodedInst, &'static str> {
    let ops = Operands::decode(inst, InstType::U);
    Ok(DecodedInst {
        op: Operation::RegWrite {
            rd: ops.rd,
            value: (ops.imm as u32) & 0xfffff000,
        },
        next_pc: NextPc::Plus4,
    })
}

fn decode_auipc(inst: u32) -> Result<DecodedInst, &'static str> {
    let ops = Operands::decode(inst, InstType::U);
    Ok(DecodedInst {
        op: Operation::RegWrite {
            rd: ops.rd,
            value: ops.imm as u32,
        },
        next_pc: NextPc::Plus4,
    })
}

fn decode_system(inst: u32) -> Result<DecodedInst, &'static str> {
    // Ecall: 000000000000_00000_000_00000_1110011 -> 0x00000073
    const ECALL_PAT: BitPat = BitPat::new(0xFFFFFFFF, 0x00000073);
    // Ebreak: 000000000001_00000_000_00000_1110011 -> 0x00100073
    const EBREAK_PAT: BitPat = BitPat::new(0xFFFFFFFF, 0x00100073);

    if ECALL_PAT.matches(inst) {
        Ok(DecodedInst {
            op: Operation::SystemCall(SystemCallType::Ecall),
            next_pc: NextPc::Plus4,
        })
    } else if EBREAK_PAT.matches(inst) {
        Ok(DecodedInst {
            op: Operation::SystemCall(SystemCallType::Ebreak),
            next_pc: NextPc::Plus4,
        })
    } else {
        Err("Invalid system instruction")
    }
}
