mod err;
use err::{ParseErr, ReadErr};
use json::JsonValue; // Import JsonValue for parsing

pub use json::{parse, stringify};
pub use std::error::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let json_str = std::fs::read_to_string(path)
            .map_err(|e| Box::new(ReadErr { child_err: Box::new(e) }) as Box<dyn Error>)?;

        let json_value: JsonValue = json::parse(&json_str)
            .map_err(|e| Box::new(ParseErr::Malformed(Box::new(e))) as Box<dyn Error>)?;

        let tasks_array = json_value["tasks"].clone(); // Get tasks array from JSON
        
        if tasks_array.is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }

        let mut tasks = Vec::new();
        for task_value in tasks_array.members() {
            let id = task_value["id"].as_u32().unwrap_or_default();
            let description = task_value["description"].as_str().unwrap_or_default().to_string();
            let level = task_value["level"].as_u32().unwrap_or_default();

            tasks.push(Task { id, description, level });
        }

        let title = json_value["title"].as_str().unwrap_or_default().to_string();

        Ok(TodoList { title, tasks })
    }
}
