use super::core::{TodoItem,create_todo};

pub fn add_todo(title: Option<String>, content: Option<String>, todo_list: &mut Vec<TodoItem>) {
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
