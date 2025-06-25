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

/// TodoItem 结构体
pub struct TodoItem {
  /// 标题
  pub title: String,
  /// 内容
  pub content: String,
  /// 状态
  pub done: bool,
}

fn create_todo(title: String, content: String) -> TodoItem{
  TodoItem {
    title,
    content,
    done: false
  }
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
        Command::TODO { find } => match find {
            // 因为是 Option<String> 类型
            // 需要额外的模式匹配
            Some(val) => {
              println!("find {}", val);
              for item in todo_list {
                if item.title.contains(&val) {
                  let status: String;
                  if item.done{
                    status = String::from("X");
                  } else {
                    status = String::from(" ");
                  }
                  println!("[{:}] {:?} {:?}", status, item.title, item.content);
                }
              }
            },
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

                  todo_list.push(create_todo(title, todo));

                }
                _ => println!("No title or content provided"),
            }
        }
    }
}
