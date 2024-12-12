use std::fs::File;
use std::io::{self, Read};

pub struct Loader;

impl Loader {
    pub fn new() -> Self {
        Self
    }

    pub fn load_program(&self, memory: &mut crate::memory::Memory, filename: &str) -> std::io::Result<()> {
        // TODO: 实现程序加载逻辑
        Ok(())
    }

    pub fn get_entry_point(&self) -> u64 {
        // TODO: 返回程序入口点
        0
    }
} 