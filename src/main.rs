// main.rs

// 导入包模块
// 可以合并
use clap::Parser;
use todo::core::Command;

use crate::todo::{add::add_todo, core::{create_todo, TodoItem}, find::find_todo};

mod todo;

// 声明结构体
#[derive(Parser, Debug)] // 派生宏 会应用到结构体上
#[command(version, about, long_about=None)] // 派生宏 会应用到结构体上
pub struct Program {
    #[command(subcommand)]
    pub command: Command,
}

// main 函数 是一个程序的开始
fn main() {
  let mut todo_list: Vec<TodoItem> = vec![
    create_todo("learn".to_string(), "learn rust".to_string()),
    create_todo("work".to_string(), "requirement 1 must be completed before next week".to_string()),
    create_todo("play".to_string(),"play games".to_string()),
    create_todo("read".to_string(),"read books".to_string()),
  ];


  // 使用 clap 自动解析命令行参数，返回 Program 实例
  let args = Program::parse();

  // 模式匹配 command 字段
  match args.command {
    // 如果是 Command::TODO 就将字段 find 解构出来
    Command::TODO { find } => find_todo(find, &todo_list),
    Command::ADD { title, content } => add_todo(title, content, &mut todo_list)
  }
}
