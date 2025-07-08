use egui::Id;
use crate::TODO_INPUT_ID;

pub struct AddTodoComponent;

impl AddTodoComponent {
    pub fn render(
        new_todo_text: &mut String,
        add_todo_input_focused: &mut bool,
        is_adding: &mut bool,
        ui: &mut egui::Ui,
    ) -> bool {
        let mut should_add = false;

        ui.horizontal(|ui| {
            let input_ui = ui.add(egui::TextEdit::singleline(new_todo_text).id(Id::from(TODO_INPUT_ID)));

            if input_ui.has_focus() {
                *add_todo_input_focused = true;
            }

            if ui.add_enabled(!*is_adding, egui::Button::new("Add")).clicked() {
                should_add = true;
            }

            if *is_adding {
                ui.spinner();
            }
        });

        should_add
    }
}