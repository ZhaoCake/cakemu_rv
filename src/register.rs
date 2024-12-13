pub struct RegisterFile {
    regs: [u32; 32],
}

impl RegisterFile {
    pub fn new() -> Self {
        Self { regs: [0; 32] }
    }

    pub fn read(&self, index: usize) -> u32 {
        if index == 0 {
            0  // x0 永远是 0
        } else {
            self.regs[index]
        }
    }

    pub fn write(&mut self, index: usize, value: u32) {
        if index != 0 {  // 不能写入 x0
            self.regs[index] = value;
        }
    }

    pub fn dump(&self) {
        let reg_names = [
            "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2",
            "s0/fp", "s1", "a0", "a1", "a2", "a3", "a4", "a5",
            "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7",
            "s8", "s9", "s10", "s11", "t3", "t4", "t5", "t6"
        ];

        for (i, (name, value)) in reg_names.iter().zip(self.regs.iter()).enumerate() {
            println!("x{:<2} ({:<5}): 0x{:08x}", i, name, value);
        }
    }
} 