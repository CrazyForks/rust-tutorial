use clap::Subcommand;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Clone, Subcommand)]
pub enum TodoCommand {
    /// Create a new todo item
    Create {
        #[arg(short, long)]
        title: Option<String>,
        #[arg(short, long)]
        content: Option<String>,
    },
    /// List all todo items
    List {
        #[arg(short, long)]
        title: Option<String>,
        #[arg(short, long)]
        content: Option<String>,
    },
}

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

    // pub fn serializer(&self) -> String {
    //     serde_json::to_string(self).unwrap()
    // }

    // pub fn deserializer(s: &str) -> Self {
    //     serde_json::from_str(s).unwrap()
    // }
}

// pub trait Serializer
// where
//     Self: Sized + Serialize + for<'de> Deserialize<'de>,
// {
//     fn serialize(&self) -> String {
//         serde_json::to_string(self).unwrap()
//     }

//     fn deserialize<S: Into<String>>(s: S) -> Self {
//         let raw: String = s.into();
//         serde_json::from_str(&raw).unwrap()
//     }
// }

// impl Serializer for TodoItem {}

pub trait Serializer
where
    Self: Sized + Serialize + for<'a> Deserialize<'a>,
{
    fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn deserialize<S: Into<String>>(s: S) -> Self {
        serde_json::from_str(&s.into()).unwrap()
    }
}

impl Serializer for TodoItem {}
