use colored::*;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Priority {
    Low,
    Medium,
    High
}

impl clap::ValueEnum for Priority {
    fn value_variants<'a>() -> &'a [Self] {
        &[Priority::Low, Priority::Medium, Priority::High]
    }    

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Priority::Low => Some(clap::builder::PossibleValue::new("low")),
            Priority::Medium => Some(clap::builder::PossibleValue::new("medium")),
            Priority::High => Some(clap::builder::PossibleValue::new("high")),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: usize,
    pub description: String,
    pub completed: bool,
    pub priority: Priority,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    pub tasks: Vec<Task>,
    next_id: usize,
}

#[derive(Debug, Error)]
pub enum TodoError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Task with ID {0} not found")]
    TaskNotFound(usize),
}

impl TodoList {
    const FILE_PATH: &'static str = "todo.json";

    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    pub fn load() -> Result<Self, TodoError> {
        let path = Path::new(Self::FILE_PATH);

        if !path.exists() {
            return Ok(Self::new());
        }

        let data: String = fs::read_to_string(path)?;
        let todo_list: TodoList = serde_json::from_str(&data)?;
        Ok(todo_list)
    }

    pub fn save(&self) -> Result<(), TodoError> {
        let data: String = serde_json::to_string_pretty(self)?;
        fs::write(Self::FILE_PATH, data)?;
        Ok(())
    }

    pub fn add(&mut self, description: String, priority: Priority) {
        let task = Task {
            id: self.next_id,
            description,
            completed: false,
            priority,
            created_at: chrono::Local::now().to_rfc3339(),
        };
        self.tasks.push(task);
        self.next_id += 1;
    }

    pub fn list(&self) {
        if self.tasks.is_empty() {
            println!("{}", "No tasks found!".yellow());
            return;
        }

        println!("{}", "TODO List:".green().bold());
        println!("{}", "-".repeat(40).blue());
        
        for task in &self.tasks {
            let status = if task.completed { 
                "✓".green().bold() 
            } else { 
                "x".red().bold() 
            };
            
            let id_display = format!("#{}", task.id).cyan();
            
            if task.completed {
                println!("{} {}: {}", status, id_display, task.description.dimmed());
            } else {
                println!("{} {}: {}", status, id_display, task.description);
            }
            
            // Show creation date for completed tasks
            if task.completed {
                if let Ok(parsed) = chrono::DateTime::parse_from_rfc3339(&task.created_at) {
                    let local_time = parsed.with_timezone(&chrono::Local);
                    println!("   {} {}", "Completed on:".dimmed(), local_time.format("%Y-%m-%d %H:%M"));
                }
            }
        }
        
        let completed_count = self.tasks.iter().filter(|t| t.completed).count();
        let pending_count = self.tasks.len() - completed_count;
        
        println!("\n{}", "-".repeat(40).blue());
        println!("{}: {} | {}: {} | {}: {}", 
                "Total".bold(), 
                self.tasks.len(),
                "Completed".green().bold(),
                completed_count,
                "Pending".red().bold(),
                pending_count);
    }

    pub fn complete(&mut self, id: usize) -> Result<(), TodoError> {
        let task = self.tasks.iter_mut()
        .find(|t| t.id == id)
        .ok_or(TodoError::TaskNotFound(id))?;
        task.completed = true;
        Ok(())
    }

    pub fn delete(&mut self, id: usize) -> Result<(), TodoError> {
        let index = self.tasks.iter()
        .position(|t| t.id ==id)
        .ok_or(TodoError::TaskNotFound(id))?;
        self.tasks.remove(index);
        Ok(())
    }

    pub fn clear(&mut self) {
        self.tasks.clear();
        self.next_id = 1;
    }
}