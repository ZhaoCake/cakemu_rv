pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }

    pub fn read(&self, addr: usize, len: usize) -> Result<u32, &'static str> {
        // 检查长度是否合法
        match len {
            1 | 2 | 4 => (),
            _ => return Err("Invalid memory access length"),
        }

        // 检查地址对齐
        if addr % len != 0 {
            return Err("Misaligned memory access");
        }

        // 检查访问是否越界
        if addr + len > self.data.len() {
            return Err("Memory access out of bounds");
        }

        // 读取数据
        let mut value: u32 = 0;
        for i in 0..len {
            value |= (self.data[addr + i] as u32) << (i * 8);
        }
        Ok(value)
    }

    pub fn write(&mut self, addr: usize, value: u32, len: usize) -> Result<(), &'static str> {
        // 检查长度是否合法
        match len {
            1 | 2 | 4 => (),
            _ => return Err("Invalid memory access length"),
        }

        // 检查地址对齐
        if addr % len != 0 {
            return Err("Misaligned memory access");
        }

        // 检查访问是否越界
        if addr + len > self.data.len() {
            return Err("Memory access out of bounds");
        }

        // 写入数据
        for i in 0..len {
            self.data[addr + i] = ((value >> (i * 8)) & 0xFF) as u8;
        }
        Ok(())
    }

    // 批量访问方法
    pub fn write_bytes(&mut self, addr: usize, data: &[u8]) -> Result<(), &'static str> {
        if addr + data.len() > self.data.len() {
            return Err("Memory write out of bounds");
        }
        
        self.data[addr..addr + data.len()].copy_from_slice(data);
        Ok(())
    }

    pub fn read_bytes(&self, addr: usize, len: usize) -> Result<&[u8], &'static str> {
        if addr + len > self.data.len() {
            return Err("Memory read out of bounds");
        }
        
        Ok(&self.data[addr..addr + len])
    }
}
