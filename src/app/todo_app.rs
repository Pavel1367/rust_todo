use crate::TODO_INPUT_ID;
use crate::app::ui::{AddTodoComponent, ErrorDisplayComponent, GroupAction, GroupActionsComponent, TodoListComponent};
use crate::database::TodoDatabase;
use crate::models::{ErrorMessage, Todo};
use crate::services::TodoService;
use egui::Id;
#[derive(Debug)]
pub struct TodoApp {
    pub database: TodoDatabase,
    pub todos: Vec<Todo>,
    pub errors: Vec<ErrorMessage>,
    pub new_todo_text: String,
    pub add_todo_input_focused: bool,
    pub ctrl_pressed: bool,
    pub is_adding: bool,
}
impl TodoApp {
    pub fn new(database: TodoDatabase) -> Result<Self, rusqlite::Error> {
        let todos = database.get_all_todos().expect("Failed fetching todos");
        Ok(TodoApp {
            todos,
            database,
            is_adding: false,
            errors: Vec::new(),
            ctrl_pressed: false,
            add_todo_input_focused: false,
            new_todo_text: String::from("")
        })
    }
    pub fn get_selected_ids(&self) -> Vec<i64>{
        self.todos.iter().filter(|todo| todo.selected).map(|todo| todo.id).collect()
    }
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My Todo List");
            let add_todo = AddTodoComponent::render(
                &mut self.new_todo_text,
                &mut self.add_todo_input_focused,
                &mut self.is_adding,
                ui,
            );
            if add_todo{
                if TodoService::add_todo(&self.new_todo_text, &self.database, &mut self.todos, &mut self.errors ){
                    self.new_todo_text.clear();
                    ctx.memory_mut(|mem| mem.request_focus(Id::from(TODO_INPUT_ID)));
                }
            };
            let action = GroupActionsComponent::render(&mut self.todos, ui);
            match action {
                GroupAction::Complete => {
                    TodoService::complete_selected(self.get_selected_ids(), &self.database)
                }
                GroupAction::Delete => {
                    TodoService::delete_selected(self.get_selected_ids(), &self.database)
                }
                GroupAction::None => {
                }
            }
            TodoListComponent::render(&mut self.todos, self.ctrl_pressed, ui);
        });
        ErrorDisplayComponent::render(&mut self.errors, ctx);
        if ctx.input(|i| i.key_pressed(egui::Key::Enter)) && self.add_todo_input_focused {
            if TodoService::add_todo(&self.new_todo_text, &self.database, &mut self.todos, &mut self.errors) {
                self.new_todo_text.clear();
                ctx.memory_mut(|mem| mem.request_focus(Id::from(TODO_INPUT_ID)));
            }
        }

        if ctx.input(|i| i.key_pressed(egui::Key::Delete)) && TodoService::has_selected(&self.todos)
        {
            TodoService::delete_selected(self.get_selected_ids(), &self.database);
        }

        self.ctrl_pressed = ctx.input(|i| i.modifiers.command);
        ctx.request_repaint();
    }
}
