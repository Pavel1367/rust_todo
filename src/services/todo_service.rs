use crate::database::TodoDatabase;
use crate::models::{Todo, ErrorMessage};

pub struct TodoService;

impl TodoService {
    pub  fn add_todo(
        new_text: &str,
        database: &TodoDatabase,
        todos: &mut Vec<Todo>,
        errors: &mut Vec<ErrorMessage>
    ) -> bool {
        if !new_text.is_empty() {
            match database.insert_todo(new_text) {
                Ok(todo)=>{
                    todos.insert(0, todo)
                }
                Err(_)=>errors.push(ErrorMessage::new("Failed to create todo".to_string())),
            }
            true
        } else {
            errors.push(ErrorMessage::new("Todo text cant be empty!".to_string()));
            false
        }
    }

    pub fn delete_selected(selected_ids: Vec<i64>, database: &TodoDatabase) {
        if !selected_ids.is_empty() {
            println!("Selected ids: {:?}", selected_ids);
           database.delete_todos(&selected_ids).expect("Failed to delete")
        }
    }

    pub fn complete_selected(selected_ids: Vec<i64>, database: &TodoDatabase) {
       database.update_todos_completed(&selected_ids[..], true).unwrap()
    }

    pub fn has_selected(todos: &[Todo]) -> bool {
        todos.iter().any(|todo| todo.selected)
    }
}