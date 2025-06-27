struct TodoItem {
    title: String,
    content: String,
}

fn main() {
    let mut todos: Vec<TodoItem> = Vec::new();
    todos.push(TodoItem {
        title: "learn rust".to_string(),
        content: "read rust book".to_string(),
    });
    todos.push(TodoItem {
        title: "work".to_string(),
        content: "complete required".to_string(),
    });
    todos.push(TodoItem {
        title: "play".to_string(),
        content: "play game".to_string(),
    });

    let args: Vec<String> = std::env::args().collect();

    match args[1].clone().as_str() {
        "create" => {
            let mut inputs: Vec<String> = Vec::new();
            let mut ok = true;

            while ok {
                let len = inputs.len();

                if len == 0 {
                    println!("Please input todo title");

                    let mut title = String::new();

                    std::io::stdin()
                        .read_line(&mut title)
                        .expect("read line failed");

                    if title.is_empty() {
                        continue;
                    }

                    inputs.push(title.trim().to_string());
                } else if len == 1 {
                    println!("Please input todo content");

                    let mut content = String::new();

                    std::io::stdin()
                        .read_line(&mut content)
                        .expect("read line failed");

                    if content.is_empty() {
                        continue;
                    }

                    inputs.push(content.trim().to_string());
                } else {
                    println!("title:   [{}]", inputs[0].clone());
                    println!("content: [{}]", inputs[1].clone());
                    println!("Are you sure to create this todo? (y/n)");

                    let mut sure = String::new();
                    std::io::stdin()
                        .read_line(&mut sure)
                        .expect("read line failed");

                    if sure.trim().to_lowercase() != "n" {
                        ok = false;
                    } else {
                        inputs.clear();
                    }
                }
            }

            let inputs_len = inputs.len();

            let title = if inputs_len > 0 {
                inputs[0].clone()
            } else {
                String::from("default title")
            };
            let content = if inputs_len > 1 {
                inputs[1].clone()
            } else {
                String::from("default content")
            };

            println!("create todo title: {}, content: {}", title, content);
        }
        "list" => {
            for todo in todos {
                println!("todo title: {}, content: {}", todo.title, todo.content);
            }
        }
        _ => {
            println!("unknown command");
        }
    }
}
