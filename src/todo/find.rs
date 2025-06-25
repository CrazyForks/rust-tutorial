use super::core::TodoItem;

pub fn find_todo(find: Option<String>, todo_list: &Vec<TodoItem>){
  match find {
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
  }
}
