use rusqlite::{params, Connection, Result};
use crate::models::Todo;

#[derive(Debug)]
pub struct TodoDatabase {
    conn: Connection,
}

impl TodoDatabase {
    pub fn new(database_path: &str) -> Result<Self, rusqlite::Error> {
        let conn = Connection::open(database_path)?;
        println!("DB creation start");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                text TEXT NOT NULL,
                completed BOOLEAN DEFAULT FALSE,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        println!("DB created");

        Ok(TodoDatabase { conn })
    }

    pub fn insert_todo(&self, text: &str) -> Result<Todo, rusqlite::Error> {
        println!("Inserting todo: {}", text);
        self.conn.execute(
            "INSERT INTO todos (text) VALUES (?1)",
            params![text],
        )?;

        let id = self.conn.last_insert_rowid();
        println!("Created todo with ID: {}", id);
        // Получаем только что созданную запись
        let mut stmt = self.conn.prepare("SELECT id, text, completed, created_at FROM todos WHERE id = ?1")?;
        let todo = stmt.query_row(params![id], |row| {
            Ok(Todo {
                id: row.get(0)?,
                text: row.get(1)?,
                completed: row.get(2)?,
                selected: false,
                created_at: row.get(3)?,
            })
        })?;

        Ok(todo)
    }

    pub fn get_all_todos(&self) -> Result<Vec<Todo>, rusqlite::Error> {
        let mut stmt = self.conn.prepare("SELECT id, text, completed, created_at FROM todos ORDER BY created_at DESC")?;
        let todo_iter = stmt.query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                text: row.get(1)?,
                completed: row.get(2)?,
                selected: false,
                created_at: row.get(3)?,
            })
        })?;

        let mut todos = Vec::new();
        for todo in todo_iter {
            todos.push(todo?);
        }
        Ok(todos)
    }

    pub fn update_todos_completed(&self, ids: &[i64], completed: bool) -> Result<(), rusqlite::Error> {
        for id in ids {
            self.conn.execute(
                "UPDATE todos SET completed = ?1 WHERE id = ?2",
                params![completed, id],
            )?;
        }
        Ok(())
    }

    pub fn delete_todos(&self, ids: &[i64]) -> Result<(), rusqlite::Error> {
        for id in ids {
            self.conn.execute(
                "DELETE FROM todos WHERE id = ?1",
                params![id],
            )?;
        }
        Ok(())
    }
}