use std::fmt;

#[derive(Default)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub status: i64,
    pub created_at: String,
}

impl Task {
    pub fn new(title: String) -> Self {
        Task {
            id: 0,
            title,
            status: 0,
            created_at: String::from(""),
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: {}, Title: {}, Status: {}, Created At: {}",
            self.id, self.title, self.status, self.created_at
        )
    }
}
