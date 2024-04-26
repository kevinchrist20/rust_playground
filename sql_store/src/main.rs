mod models;

use sqlite::{Connection, State};
use std::io;

use crate::models::task::Task;

fn main() {
    let mut connection = sqlite::open("./db/todo_db").unwrap();

    println!("******** Welcome to the ToDo Store *********\n");

    println!("Choose any of the options below");

    println!("1. Add new todo");
    println!("2. List all todos");
    println!("3. Delete todo");
    println!("4. Read entry");
    println!("5. Update entry \n");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    let choice: u32 = choice
        .trim()
        .parse::<u32>()
        .expect("Input must be between 1-5");

    match choice {
        1 => {
            println!("Add new todo: ");

            let mut task_name = String::new();

            io::stdin()
                .read_line(&mut task_name)
                .expect("Failed to read input");

            add_task(Task::new(task_name.trim().to_string()), &mut connection)
        }
        2 => get_all(&mut connection),
        3 => println!("Delete todo"),
        4 => println!("Read entry"),
        5 => println!("Update entry"),
        _ => println!("Unknown input. Try again"),
    }
}

fn add_task(task: Task, connection: &mut Connection) {
    let query = "INSERT INTO tasks (title, status) VALUES (?, ?)";

    let mut stmt = connection.prepare(query).unwrap();

    stmt.bind(&[task.title.as_str(), &task.status.to_string()][..])
        .unwrap();
    stmt.next().unwrap();

    println!("Inserted successfully");
}

fn get_all(connection: &mut Connection) {
    let query = "SELECT * FROM tasks";

    let mut stmt = connection.prepare(query).unwrap();

    while let Ok(State::Row) = stmt.next() {
        let id: i64 = stmt.read::<i64, _>("id").unwrap();
        let title: String = stmt.read::<String, _>("title").unwrap();
        let status: i64 = stmt.read::<i64, _>("status").unwrap();
        let created_at: String = stmt.read::<String, _>("created_at").unwrap();

        let task = Task {
            id,
            title,
            status,
            created_at,
        };

        println!("{}", task);
    }
}
