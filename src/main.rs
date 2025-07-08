mod models;
mod services;
mod app;
mod database;

use app::todo_app::TodoApp;

use crate::database::TodoDatabase;

const TODO_INPUT_ID: &str = "todo_input";


fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "todos.db".to_string());

    let database = TodoDatabase::new(&database_url)?;
    let app = TodoApp::new(database)?;
    println!("{:?}", app);
    eframe::run_native(
        "MyTodo",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(app))),
    ).expect("TODO: panic message");

    Ok(())
}
