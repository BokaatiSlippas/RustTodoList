mod todo;

use clap::{Parser, Subcommand};
use todo::{TodoList, TodoError, Priority};


#[derive(Parser)]
#[command(name = "todo-cli")]
#[command(about = "TODO list CLI app", version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        description: String,
        #[arg(value_enum)]
        priority: Priority,
    },
    List,
    Complete {
        id: usize,
    },
    Delete {
        id: usize,
    },
    Clear,
}


fn main() -> Result<(), TodoError> {
    let cli = Cli::parse();
    let mut todo_list = TodoList::load()?;
    match cli.command {
        Commands::Add { description, priority } => {
            todo_list.add(description, priority);
            todo_list.save()?;
            println!("Task added succesfully!");
        }
        Commands::List => {
            todo_list.list();
        }
        Commands::Complete { id } => {
            todo_list.complete(id)?;
            todo_list.save()?;
            println!("Task {} marked as compelted!", id);
        }
        Commands::Delete { id } => {
            todo_list.delete(id)?;
            todo_list.save()?;
            println!("Task {} deleted", id);
        }
        Commands::Clear => {
            todo_list.clear();
            todo_list.save()?;
            println!("All tasks cleared");
        }
    }
    Ok(())
}
