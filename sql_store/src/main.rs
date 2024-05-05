mod models;
mod db_client;

use sqlite::{Connection, State};
use std::io;

use crate::{db_client::DbClient, models::task::Task};

fn main() {
    match run_app() {
        Ok(()) => println!("Thank you for coming. Until next time!!"),
        Err(err) => eprintln!("An error occurred: {}", err),
    }
}

fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    let db_client = DbClient::new()?;

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
            1 => add_task(&db_client)?,
            // 2 => get_all(&connection)?,
            // 3 => delete_task(&connection)?,
            // 4 => get_task(&connection)?,
            // 5 => {
            //     let mut decision = String::new();

            //     println!("What would you like to do?");
            //     println!("(A) Update task status.");
            //     println!("(B) Update task name.");

            //     io::stdin().read_line(&mut decision)?;

            //     let decision = decision.trim().parse::<char>()?;
            //     match decision.to_ascii_lowercase() {
            //         'a' => update_task_status(&connection)?,
            //         'b' => update_task_title(&connection)?,
            //         _ => println!("Unknown input. Try again"),
            //     }
            // }
            // 6 => clear_all_tasks(&connection)?,
            _ => println!("Unknown input. Try again"),
        }
    }

    Ok(())
}

fn add_task(client: &DbClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("Task title: ");

    let mut task_name = String::new();
    io::stdin().read_line(&mut task_name)?;

    let task = Task::new(task_name.trim().to_string());

    client.create_task(&task)?;

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

fn get_task(connection: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    println!("Task ID: ");
    let mut task_id = String::new();

    io::stdin().read_line(&mut task_id)?;
    let task_id: i32 = task_id.trim().parse::<i32>()?;

    let query = "SELECT * FROM tasks WHERE id = :id";

    let mut stmt = connection.prepare(query)?;
    stmt.bind((":id", task_id.to_string().as_str()))?;

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

fn clear_all_tasks(connection: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    println!("Clearing all tasks....\n");

    let query = "DELETE FROM tasks";

    connection.execute(query)?;

    println!("Database cleared successfully.\n");

    Ok(())
}

fn update_task_status(connection: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    println!("Task ID: ");
    let mut task_id = String::new();

    io::stdin().read_line(&mut task_id)?;
    let task_id = task_id.trim().parse::<i32>()?;

    let mut status = String::new();
    println!("Task status: (1) Completed. (0) Pending");

    io::stdin().read_line(&mut status)?;
    let status = match status.trim().parse::<i32>() {
        Ok(num) => {
            if num == 1 || num == 0 {
                num
            } else {
                return Err("Failed to parse status. Please enter 1 or 0.".into());
            }
        }
        _ => return Err("Failed to parse status. Please enter 1 or 0.".into()),
    };

    let query = "UPDATE tasks SET status = :status WHERE id = :id;";
    let mut stmt = connection.prepare(query)?;

    stmt.bind(
        &[
            (":status", status.to_string().as_str()),
            (":id", task_id.to_string().as_str()),
        ][..],
    )?;
    stmt.next()?;

    Ok(())
}

fn update_task_title(connection: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    println!("Task ID: ");
    let mut task_id = String::new();

    io::stdin().read_line(&mut task_id)?;
    let task_id = task_id.trim().parse::<i32>()?;

    let mut title = String::new();
    println!("Task title: ");

    io::stdin().read_line(&mut title)?;

    let query = "UPDATE tasks SET title = :title WHERE id = :id;";
    let mut stmt = connection.prepare(query)?;

    stmt.bind(
        &[
            (":title", title.to_string().as_str()),
            (":id", task_id.to_string().as_str()),
        ][..],
    )?;
    stmt.next()?;

    Ok(())
}
