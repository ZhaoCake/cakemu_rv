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

use serde_derive::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub program: ProgramConfig,
    pub memory: MemoryConfig,
    pub debug: DebugConfig,
    pub uart: UartConfig,
    pub timer: TimerConfig,
    pub wave: WaveConfig,
    pub display: DisplayConfig,
}

#[derive(Deserialize)]
pub struct ProgramConfig {
    pub binary: String,
}

#[derive(Deserialize)]
pub struct MemoryConfig {
    pub size: usize,
}

#[derive(Deserialize)]
pub struct DebugConfig {
    pub instruction_trace: bool,
    pub memory_trace: bool,
    pub register_trace: bool,
    pub single_step: bool,
    pub trace_limit: usize,
}

#[derive(Deserialize)]
pub struct UartConfig {
    pub enabled: bool,
    pub base_addr: usize,
}

#[derive(Deserialize)]
pub struct TimerConfig {
    pub enabled: bool,
    pub base_addr: usize,
    pub auto_reload: bool,
    pub interrupt_enabled: bool,
}

#[derive(Deserialize)]
pub struct WaveConfig {
    pub enabled: bool,
    pub base_addr: usize,
    pub output_file: String,
    pub sample_rate: u32,
}

#[derive(Deserialize)]
pub struct DisplayConfig {
    pub enabled: bool,
    pub base_addr: usize,
    pub title: String,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
} 