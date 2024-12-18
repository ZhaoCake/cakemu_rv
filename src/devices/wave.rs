use std::fs::File;
use std::io::Write;

// 波形发生器寄存器偏移
const WAVE_CONTROL: usize = 0x0;    // 控制寄存器
const WAVE_FREQUENCY: usize = 0x4;  // 频率寄存器
const WAVE_AMPLITUDE: usize = 0x8;  // 幅度寄存器
const WAVE_PHASE: usize = 0xC;      // 相位寄存器
const WAVE_DUTY: usize = 0x10;      // 占空比寄存器

// 波形类型
#[derive(Debug, Clone, Copy)]
enum WaveType {
    Sine,
    Square,
    Triangle,
    Sawtooth,
}

pub struct Wave {
    control: u32,     // 控制寄存器
    frequency: u32,   // 频率 (Hz)
    amplitude: u32,   // 幅度 (0-255)
    phase: u32,       // 相位 (0-359度)
    duty: u32,        // 占空比 (0-100%)
    sample_count: u32,// 采样计数
    output_file: Option<File>,
}

impl Wave {
    pub fn new() -> Self {
        Self {
            control: 0,
            frequency: 1000,  // 默认 1kHz
            amplitude: 255,   // 默认最大幅度
            phase: 0,        // 默认相位 0
            duty: 50,        // 默认占空比 50%
            sample_count: 0,
            output_file: None,
        }
    }

    fn get_wave_type(&self) -> WaveType {
        match (self.control >> 1) & 0x7 {
            0 => WaveType::Sine,
            1 => WaveType::Square,
            2 => WaveType::Triangle,
            3 => WaveType::Sawtooth,
            _ => WaveType::Sine,
        }
    }

    fn is_enabled(&self) -> bool {
        self.control & 1 != 0
    }

    fn calculate_value(&self) -> f64 {
        if !self.is_enabled() {
            return 0.0;
        }

        let t = self.sample_count as f64 / 1000.0;  // 时间（秒）
        let f = self.frequency as f64;
        let a = self.amplitude as f64 / 255.0;  // 归一化幅度
        let p = self.phase as f64 * std::f64::consts::PI / 180.0;  // 相位（弧度）

        match self.get_wave_type() {
            WaveType::Sine => {
                a * (2.0 * std::f64::consts::PI * f * t + p).sin()
            },
            WaveType::Square => {
                let d = self.duty as f64 / 100.0;  // 占空比仅用于方波
                let phase = (2.0 * std::f64::consts::PI * f * t + p) % (2.0 * std::f64::consts::PI);
                if phase / (2.0 * std::f64::consts::PI) < d { a } else { -a }
            },
            WaveType::Triangle => {
                let phase = (2.0 * std::f64::consts::PI * f * t + p) % (2.0 * std::f64::consts::PI);
                let normalized = phase / (2.0 * std::f64::consts::PI);
                if normalized < 0.5 {
                    a * (4.0 * normalized - 1.0)
                } else {
                    a * (3.0 - 4.0 * normalized)
                }
            },
            WaveType::Sawtooth => {
                let phase = (2.0 * std::f64::consts::PI * f * t + p) % (2.0 * std::f64::consts::PI);
                a * (phase / std::f64::consts::PI - 1.0)
            },
        }
    }

    pub fn read(&self, offset: usize, size: usize) -> Result<u32, &'static str> {
        if size != 4 {
            return Err("Wave generator only supports word access");
        }

        match offset {
            WAVE_CONTROL => Ok(self.control),
            WAVE_FREQUENCY => Ok(self.frequency),
            WAVE_AMPLITUDE => Ok(self.amplitude),
            WAVE_PHASE => Ok(self.phase),
            WAVE_DUTY => Ok(self.duty),
            _ => Err("Invalid wave generator register offset"),
        }
    }

    pub fn write(&mut self, offset: usize, value: u32, size: usize) -> Result<(), &'static str> {
        if size != 4 {
            return Err("Wave generator only supports word access");
        }

        match offset {
            WAVE_CONTROL => {
                self.control = value;
                if self.is_enabled() && self.output_file.is_none() {
                    self.output_file = Some(File::create("wave.txt").map_err(|_| "Failed to create wave.txt")?);
                }
                Ok(())
            },
            WAVE_FREQUENCY => {
                self.frequency = value;
                Ok(())
            },
            WAVE_AMPLITUDE => {
                self.amplitude = value.min(255);
                Ok(())
            },
            WAVE_PHASE => {
                self.phase = value % 360;
                Ok(())
            },
            WAVE_DUTY => {
                self.duty = value.min(100);
                Ok(())
            },
            _ => Err("Invalid wave generator register offset"),
        }
    }

    pub fn tick(&mut self) {
        if self.is_enabled() {
            let value = self.calculate_value();  // 先计算值
            if let Some(file) = &mut self.output_file {
                writeln!(file, "{}", value).ok();
            }
            self.sample_count = self.sample_count.wrapping_add(1);
        }
    }
} 