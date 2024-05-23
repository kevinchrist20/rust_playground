use sqlite::{Connection, Error, State};

use crate::models::task::Task;

pub struct DbClient {
    connection: Connection,
}

impl DbClient {
    pub fn new() -> sqlite::Result<Self> {
        let connection = sqlite::open("./db/todo.db")?;
        Ok(DbClient { connection })
    }

    pub fn create(&self, task: &Task) -> Result<(), Error> {
        let query = "INSERT INTO tasks (title, status) VALUES (?, ?)";
        let mut stmt = self.connection.prepare(query)?;

        stmt.bind(&[task.title.as_str(), &task.status.to_string()][..])?;
        stmt.next()?;

        Ok(())
    }

    pub fn get_all(&self) -> Result<Vec<Task>, Error> {
        let mut task_list: Vec<Task> = Vec::new();

        let query = "SELECT * FROM tasks";
        let mut stmt = self.connection.prepare(query)?;

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

            task_list.push(task)
        }

        Ok(task_list)
    }

    pub fn delete_task(&self, task_id: i32) -> Result<(), Error> {
        let query = "DELETE FROM tasks WHERE id = :id";
        let mut stmt = self.connection.prepare(query)?;

        stmt.bind((":id", task_id.to_string().as_str()))?;
        stmt.next()?;

        Ok(())
    }

    pub fn get_task(&self, task_id: i32) -> Result<Task, Error> {
        let query = "SELECT * FROM tasks WHERE id = :id";

        let mut stmt = self.connection.prepare(query)?;
        stmt.bind((":id", task_id.to_string().as_str()))?;

        let mut task = Task::default();
        while let Ok(State::Row) = stmt.next() {
            let id: i64 = stmt.read::<i64, _>("id")?;
            let title: String = stmt.read::<String, _>("title")?;
            let status: i64 = stmt.read::<i64, _>("status")?;
            let created_at: String = stmt.read::<String, _>("created_at")?;

            task = Task {
                id,
                title,
                status,
                created_at,
            };
        }

        Ok(task)
    }

    pub fn update_task_status(&self, status: i32, task_id: i32) -> Result<(), Error> {
        let query = "UPDATE tasks SET status = :status WHERE id = :id;";
        let mut stmt = self.connection.prepare(query)?;

        stmt.bind(
            &[
                (":status", status.to_string().as_str()),
                (":id", task_id.to_string().as_str()),
            ][..],
        )?;
        stmt.next()?;

        Ok(())
    }  
    
    pub fn update_task_title(&self, task_id: i32, title: &str) -> Result<(), Error> {
        let query = "UPDATE tasks SET title = :title WHERE id = :id;";
        let mut stmt = self.connection.prepare(query)?;
    
        stmt.bind(
            &[
                (":title", title),
                (":id", task_id.to_string().as_str()),
            ][..],
        )?;
        stmt.next()?;

        Ok(())
    }

    pub fn clear(&self) -> Result<(), Error> {
        let query = "DELETE FROM tasks";

        self.connection.execute(query)?;

        Ok(())
    }
}
