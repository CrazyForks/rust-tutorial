// main.rs

// 导入包模块
// 可以合并
// use clap::{Parser, Subcommand};
use clap::Parser;
use clap::Subcommand;

// 声明结构体
#[derive(Parser, Debug)] // 派生宏 会应用到结构体上
#[command(version, about, long_about=None)] // 派生宏 会应用到结构体上
pub struct Program {
    #[command(subcommand)]
    pub command: Command,
}

// 声明枚举
#[derive(Debug, Clone, Subcommand)] // 派生宏
pub enum Command {
    TODO {
        #[arg(short, long, default_value = None)]
        find: String,
    },
}

// main 函数 是一个程序的开始
fn main() {
    let args = Program::parse();
    println!("args: -- {:?}", args);
}
