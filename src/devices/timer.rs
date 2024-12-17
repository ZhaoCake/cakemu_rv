// Timer 寄存器偏移
const TIMER_COUNT: usize = 0x0;    // 计数器值
const TIMER_CONTROL: usize = 0x4;  // 控制寄存器
const TIMER_COMPARE: usize = 0x8;  // 比较寄存器
const TIMER_STATUS: usize = 0xC;   // 状态寄存器

// 控制寄存器位
const CONTROL_ENABLE: u32 = 1 << 0;    // 定时器使能
const CONTROL_INTERRUPT: u32 = 1 << 1;  // 中断使能
const CONTROL_RELOAD: u32 = 1 << 2;     // 自动重载

// 状态寄存器位
const STATUS_MATCH: u32 = 1 << 0;  // 比较匹配标志

#[allow(dead_code)]
pub struct Timer {
    count: u32,      // 当前计数值
    control: u32,    // 控制寄存器
    compare: u32,    // 比较值
    status: u32,     // 状态寄存器
}

impl Timer {
    pub fn new() -> Self {
        Self {
            count: 0,
            control: 0,
            compare: 0xFFFFFFFF,
            status: 0,
        }
    }

    pub fn read(&self, offset: usize, size: usize) -> Result<u32, &'static str> {
        if size != 4 {
            return Err("Timer only supports word access");
        }

        match offset {
            TIMER_COUNT => Ok(self.count),
            TIMER_CONTROL => Ok(self.control),
            TIMER_COMPARE => Ok(self.compare),
            TIMER_STATUS => Ok(self.status),
            _ => Err("Invalid Timer register offset"),
        }
    }

    pub fn write(&mut self, offset: usize, value: u32, size: usize) -> Result<(), &'static str> {
        if size != 4 {
            return Err("Timer only supports word access");
        }

        match offset {
            TIMER_COUNT => {
                self.count = value;
                println!("[Timer] Count set to {}", value);
                Ok(())
            },
            TIMER_CONTROL => {
                self.control = value;
                println!("[Timer] Control set to 0x{:08x}", value);
                if value & CONTROL_ENABLE != 0 {
                    println!("[Timer] Timer enabled");
                }
                Ok(())
            },
            TIMER_COMPARE => {
                self.compare = value;
                println!("[Timer] Compare value set to {}", value);
                Ok(())
            },
            TIMER_STATUS => {
                // 写1清零对应位
                self.status &= !value;
                println!("[Timer] Status cleared to 0x{:08x}", self.status);
                Ok(())
            },
            _ => Err("Invalid Timer register offset"),
        }
    }

    // 更新定时器状态（每个时钟周期调用）
    pub fn tick(&mut self) {
        if self.control & CONTROL_ENABLE != 0 {
            // 每个指令周期增加1
            self.count = self.count.wrapping_add(1);
            
            // 检查是否匹配
            if self.count == self.compare {
                self.status |= STATUS_MATCH;
                println!("[Timer] Match! Count = {}", self.count);
                
                // 如果启用了自动重载
                if self.control & CONTROL_RELOAD != 0 {
                    self.count = 0;
                }
            }
        }
    }

    // 检查是否需要触发中断
    pub fn interrupt_pending(&self) -> bool {
        (self.control & CONTROL_INTERRUPT != 0) && (self.status & STATUS_MATCH != 0)
    }
} 