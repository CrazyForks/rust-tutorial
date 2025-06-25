use clap::Subcommand;

/// TodoItem 结构体
pub struct TodoItem {
  /// 标题
  pub title: String,
  /// 内容
  pub content: String,
  /// 状态
  pub done: bool,
}

pub fn create_todo(title: String, content: String) -> TodoItem{
  TodoItem {
    title,
    content,
    done: false
  }
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
