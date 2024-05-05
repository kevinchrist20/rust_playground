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

    pub fn create_task(&self, task: &Task) -> Result<(), Error> {
        let query = "INSERT INTO tasks (title, status) VALUES (?, ?)";
        let mut stmt = self.connection.prepare(query)?;

        stmt.bind(&[task.title.as_str(), &task.status.to_string()][..])?;
        stmt.next()?;

        Ok(())
    }

    // pub fn delete_task() -> Result<(), ()> {}
}
