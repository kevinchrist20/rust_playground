# Rust Playground

Welcome to the Rust Playground! This is a space where I experiment with the Rust language features, test ideas, and explore different crates available on [crates.io](https://crates.io/). Below, are some resources used in this playground.

## Dependencies

- [SQLite Database](https://crates.io/crates/sqlite): A crate for working with SQLite databases in Rust.
- [Tokio](https://crates.io/crates/tokio): An asynchronous runtime for Rust.
- [Reqwest](https://crates.io/crates/reqwest): An HTTP client for Rust.
- [Scraper](https://crates.io/crates/scraper): A crate for scraping HTML documents.
- [html2text](https://crates.io/crates/html2text): A crate for converting HTML to plain text.


## Resources

- [SQLite Docs](https://docs.rs/sqlite/0.36.0/sqlite/): Official documentation for the SQLite crate.
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html): Rust tutorials with examples.
- [The Rust Book](https://doc.rust-lang.org/book/title-page.html): Comprehensive guide to learning Rust.
- [Rust Lang Nursery](https://rust-lang-nursery.github.io/rust-cookbook/intro.html): Rust Cookbook with examples.
- [Azlyrics](https://www.azlyrics.com/): A website for song lyrics.
- [Tokio](https://tokio.rs/): An asynchronous runtime for the Rust programming language.

## Projects

### SQL store

[SQL Store](https://github.com/kevinchrist20/rust_playground/tree/master/sql_store) is a simple task management application that uses SQLite for storage. It allows you to add, list, delete, read, and update tasks.

#### Features

- Add new tasks with titles and statuses.
- List all tasks stored in the database.
- Delete tasks from the database.
- Read details of a specific task.
- Update the status of a task.

### How to run

To successfully run the code, make sure you've SQLite installed. Next, run the following commands:

```bash
$ cd sql_store

# Navigate to where the table.sql file is located
$ sqlite3 todo.db < table.sql

```

The command above creates a new SQLite database file named `todo.db` and executes the SQL commands in `table.sql` to set up the database schema.

Next, run the following command to build and run the application:

```bash
# Navigate to the root of the project and run the following command
$ cargo run

```

### Lyrical

[Lyrical](https://github.com/kevinchrist20/rust_playground/tree/master/lyrical) is a simple web scraper that fetches song lyrics from the [Azlyrics](https://www.azlyrics.com/) website. It allows you to search for a song by its title and artist name.

#### Features

- Fetch song lyrics by its title and artist name from the Azlyrics website.
- The lyrics are written to a text file in the current directory with the song title and artist name as the filename.

### How to run

To run the code, navigate to the `lyrical` directory and run the following command:

```bash
# Navigate to the lyrical directory
$ cd lyrical

# Run the following command to build and run the application
$ cargo run -- "artist name" "song title"

# Example
$ cargo run -- "eminem" "lose yourself"
```

Replace `song title` and `artist name` with the title and artist name of the song you want to fetch lyrics for. The lyrics will be written to a text file in the current directory with the song title and artist name as the filename.

Feel free to explore the code, modify it, or contribute!
