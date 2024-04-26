mod models;

use sqlite::{Connection, State};
use std::io;

use crate::models::task::Task;

fn main() {
    match run_app() {
        Ok(()) => println!("Thank you for coming. Until next time!!"),
        Err(err) => eprintln!("An error occurred: {}", err),
    }
}

fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    let connection = sqlite::open("./db/todo_db")?;

    println!("******** Welcome to the ToDo Store *********\n");

    loop {
        println!("Choose any of the options below");
        println!("1. Add new task");
        println!("2. List all tasks");
        println!("3. Delete task");
        println!("4. Read task");
        println!("5. Update task");
        println!("6. Clear all tasks");
        println!("0. Exit\n");

        let choice: u32 = loop {
            let mut choice = String::new();

            io::stdin().read_line(&mut choice)?;

            match choice.trim().parse::<u32>() {
                Ok(num) if num <= 6 => break num,
                _ => println!("Input must be between 0-6"),
            }
        };

        match choice {
            0 => break,
            1 => add_task(&connection)?,
            2 => get_all(&connection)?,
            3 => delete_task(&connection)?,
            4 => println!("Read entry"),
            5 => println!("Update entry"),
            6 => println!("Clear all"),
            _ => println!("Unknown input. Try again"),
        }
    }

    Ok(())
}

fn add_task(connection: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    println!("Task title: ");

    let mut task_name = String::new();
    io::stdin().read_line(&mut task_name)?;

    let task = Task::new(task_name.trim().to_string());

    let query = "INSERT INTO tasks (title, status) VALUES (?, ?)";
    let mut stmt = connection.prepare(query)?;

    stmt.bind(&[task.title.as_str(), &task.status.to_string()][..])?;
    stmt.next()?;

    println!("Task inserted successfully! \n");

    Ok(())
}

fn get_all(connection: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let query = "SELECT * FROM tasks";

    let mut stmt = connection.prepare(query)?;

    println!("\nTasks list");

    while let Ok(State::Row) = stmt.next() {
        let id: i64 = stmt.read::<i64, _>("id")?;
        let title: String = stmt.read::<String, _>("title")?;
        let status: i64 = stmt.read::<i64, _>("status")?;
        let created_at: String = stmt.read::<String, _>("created_at")?;

        let task = Task {
            id,
            title,
            status,
            created_at,
        };

        println!("{}", task);
    }

    println!();
    Ok(())
}

fn delete_task(connection: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    println!("Task to delete ID: ");
    let mut task_id = String::new();

    io::stdin().read_line(&mut task_id)?;
    let task_id = task_id.trim().parse::<i32>()?;

    let query = "DELETE FROM tasks WHERE id = :id";
    let mut stmt = connection.prepare(query)?;

    stmt.bind((":id", task_id.to_string().as_str()))?;
    stmt.next()?;

    println!("Task deleted successfully! \n");

    Ok(())
}
