use std::io;
use colored::Colorize;
use crate::config;
use crate::resources;

pub fn render() -> io::Result<()> {
    if let Some(cpu_info) = resources::cpu::cpu_info() {
        if let Some(cpu) = cpu_info.first() {
            println!("{:<10}{}", "CPU".color(config::color()), cpu.model_name);
        }
    }
    if let Ok(Some(gpu_info)) = resources::gpu::gpu_info() {
        println!("{:<10}{}", "GPU".color(config::color()), gpu_info.name);
    }
    if let Some(ram_info) = resources::ram::mem_info() {
        println!("{:<10}{}/{}", "RAM".color(config::color()), ram_info.used, ram_info.total);
    }
    if let Ok(Some(env_info)) = resources::xorgenv::env_info() {
        println!("{:<10}{}", "WM:".color(config::color()), env_info.XORG_ENV);
    }
    Ok(())
}
