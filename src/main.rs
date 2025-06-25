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
        find: Option<String>,
    },
    ADD {
        #[arg(short, long, default_value = None)]
        title: Option<String>,
        #[arg(short, long, default_value = None)]
        content: Option<String>,
    },
}

// main 函数 是一个程序的开始
fn main() {
    // 使用 clap 自动解析命令行参数，返回 Program 实例
    let args = Program::parse();

    // 模式匹配 command 字段
    match args.command {
        // 如果是 Command::TODO 就将字段 find 解构出来
        Command::TODO { find } => match find {
            // 因为是 Option<String> 类型
            // 需要额外的模式匹配
            Some(val) => println!("find {}", val),
            None => println!("No --find argument provided"),
        },
        Command::ADD { title, content } => {
            match title {
                Some(title) => {
                  // 这里声明了一个变量 todo, 类型为 String, 尚未赋值
                  let todo: String;

                  // 这里使用 if let 表达式
                  // 它的作用是将 content 绑定到一个新的变量 content
                  // 并在 content 不是 None 时执行代码块
                  // 这样就不需要再额外的模式匹配了
                  if let Some(content) = content {
                    todo = content;
                  } else{
                    // 如果 content 是 None, 则使用默认值
                    todo = String::from("default todo content");
                  }

                  println!("title: {}, content: {}", title, todo);
                }
                _ => println!("No title or content provided"),
            }
        }
    }
}
