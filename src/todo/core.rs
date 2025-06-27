use super::storage::read_todo_list;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize, Serialize)]
pub struct TodoItem {
    pub title: String,
    pub content: String,
}

pub fn create_todo_item(title: &str, content: &str) -> TodoItem {
    TodoItem {
        title: title.to_string(),
        content: content.to_string(),
    }
}

impl TodoItem {
    pub fn new(title: &str, content: &str) -> Self {
        create_todo_item(title, content)
    }

    pub fn serializer(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn deserializer(s: &str) -> Self {
        serde_json::from_str(s).unwrap()
    }
}
