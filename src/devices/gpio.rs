// GPIO 寄存器偏移
const GPIO_DIRECTION: usize = 0x0;  // 方向寄存器
const GPIO_OUTPUT: usize = 0x4;     // 输出寄存器
const GPIO_INPUT: usize = 0x8;      // 输入寄存器

pub struct Gpio {
    direction: u32,  // 0: 输入, 1: 输出
    output: u32,     // 输出值
    input: u32,      // 输入值
}

impl Gpio {
    pub fn new() -> Self {
        Self {
            direction: 0,  // 默认全部为输入
            output: 0,
            input: 0,
        }
    }

    pub fn read(&self, offset: usize, size: usize) -> Result<u32, &'static str> {
        if size != 4 {
            return Err("GPIO only supports word access");
        }

        match offset {
            GPIO_DIRECTION => Ok(self.direction),
            GPIO_OUTPUT => Ok(self.output),
            GPIO_INPUT => Ok(self.input),
            _ => Err("Invalid GPIO register offset"),
        }
    }

    pub fn write(&mut self, offset: usize, value: u32, size: usize) -> Result<(), &'static str> {
        if size != 4 {
            return Err("GPIO only supports word access");
        }

        match offset {
            GPIO_DIRECTION => {
                self.direction = value;
                println!("[GPIO] Direction set to 0x{:08x}", value);
                Ok(())
            },
            GPIO_OUTPUT => {
                self.output = value;
                println!("[GPIO] Output set to 0x{:08x}", value);
                Ok(())
            },
            GPIO_INPUT => {
                // 输入寄存器是只读的
                Err("GPIO input register is read-only")
            },
            _ => Err("Invalid GPIO register offset"),
        }
    }

    // 模拟输入变化（例如按钮按下）
    pub fn set_input(&mut self, value: u32) {
        self.input = value;
        println!("[GPIO] Input changed to 0x{:08x}", value);
    }
} 