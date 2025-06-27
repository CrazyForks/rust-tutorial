use super::core::TodoItem;
use std::fs;

pub fn read_todo_list(save_file: &str) -> Vec<TodoItem> {
    let mut result: Vec<TodoItem> = Vec::new();

    if let Ok(content) = fs::read_to_string(save_file) {
        if let Ok(mut list) = serde_json::from_str(content.as_str()) {
            result.append(&mut list)
        }
    };

    if result.len() == 0 {
        result.push(TodoItem::new("learn rust", "read rust book"));
        result.push(TodoItem::new("work", "complete required"));
        result.push(TodoItem::new("play", "play game"));
    }

    return result;
}

pub fn save_todo_list(save_file: &str, todos: &Vec<TodoItem>) {
    fs::write(save_file, serde_json::to_string(todos).unwrap()).unwrap();
}
