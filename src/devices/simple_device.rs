// 简单的外设，用于课程设计实验, 仅仅表现统一编址的编址方式

pub struct SimpleDevice {
    pub base_address: u32,
    pub size: u32,
}

impl SimpleDevice {
    pub fn new(base_address: u32, size: u32) -> Self {
        Self { base_address, size }
    }

    pub fn read(&self, addr: u32) -> u32 {
        // 读取外设数据
        addr    
    }

    pub fn write(&self, addr: u32, value: u32) {
        // 写入外设数据
        addr
    }
}