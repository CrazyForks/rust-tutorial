use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let len = args.len();
  let title = args[1].clone();

  let content = if len > 2 {
    args[2].clone()
  } else {
    String::from("default content")
  };

  println!("todo title: {}, content: {}", title, content);
}
