use std::collections::VecDeque;

pub struct Debugger {
    pub itrace_enabled: bool,
    pub mtrace_enabled: bool,
    pub single_step: bool,
    instruction_trace: VecDeque<String>,
    memory_trace: VecDeque<String>,
    trace_limit: usize,
}

impl Debugger {
    pub fn new() -> Self {
        Self {
            itrace_enabled: true, // 默认开启
            mtrace_enabled: true, // 默认开启
            single_step: false,   // 默认关闭
            instruction_trace: VecDeque::with_capacity(16),
            memory_trace: VecDeque::with_capacity(16),
            trace_limit: 16,
        }
    }

    pub fn trace_instruction(&mut self, pc: u32, instruction: u32, disasm: &str) {
        if !self.itrace_enabled {
            return;
        }
        let trace = format!("0x{:08x}: 0x{:08x} {}", pc, instruction, disasm);
        if self.instruction_trace.len() >= self.trace_limit {
            self.instruction_trace.pop_front();
        }
        self.instruction_trace.push_back(trace);
        println!("[ITRACE] {}", self.instruction_trace.back().unwrap());
    }

    pub fn trace_memory_read(&mut self, addr: usize, size: usize, value: u32) {
        if !self.mtrace_enabled {
            return;
        }
        let trace = format!("read  0x{:08x}: {} bytes = 0x{:x}", addr, size, value);
        if self.memory_trace.len() >= self.trace_limit {
            self.memory_trace.pop_front();
        }
        self.memory_trace.push_back(trace);
        println!("[MTRACE] {}", self.memory_trace.back().unwrap());
    }

    pub fn trace_memory_write(&mut self, addr: usize, size: usize, value: u32) {
        if !self.mtrace_enabled {
            return;
        }
        let trace = format!("write 0x{:08x}: {} bytes = 0x{:x}", addr, size, value);
        if self.memory_trace.len() >= self.trace_limit {
            self.memory_trace.pop_front();
        }
        self.memory_trace.push_back(trace);
        println!("[MTRACE] {}", self.memory_trace.back().unwrap());
    }

    pub fn show_instruction_trace(&self) {
        println!("=== Instruction Trace ===");
        for trace in &self.instruction_trace {
            println!("{}", trace);
        }
    }

    pub fn show_memory_trace(&self) {
        println!("=== Memory Trace ===");
        for trace in &self.memory_trace {
            println!("{}", trace);
        }
    }

    pub fn wait_for_next(&self) {
        if self.single_step {
            println!("Press Enter to continue...");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
        }
    }
}
