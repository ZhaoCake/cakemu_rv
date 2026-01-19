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

pub struct RegisterFile {
    regs: [u32; 32],
}

impl RegisterFile {
    pub fn new() -> Self {
        Self { regs: [0; 32] }
    }

    #[inline]
    pub fn read(&self, index: usize) -> u32 {
        // x0 is always 0, and we maintain this invariant in write
        self.regs[index]
    }

    #[inline]
    pub fn write(&mut self, index: usize, value: u32) {
        if index != 0 {
            // Cannot write to x0
            self.regs[index] = value;
        }
    }

    pub fn dump(&self) {
        let reg_names = [
            "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0/fp", "s1", "a0", "a1", "a2",
            "a3", "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10",
            "s11", "t3", "t4", "t5", "t6",
        ];

        for (i, (name, value)) in reg_names.iter().zip(self.regs.iter()).enumerate() {
            println!("x{:<2} ({:<5}): 0x{:08x}", i, name, value);
        }
    }
}
