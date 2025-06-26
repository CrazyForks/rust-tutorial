use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let len = args.len();
  let title = args[1].clone();
  let mut content = String::from("default content");

  if len > 2{
    content = args[2].clone();
  }

  println!("todo title: {}, content: {}", title, content);
}
