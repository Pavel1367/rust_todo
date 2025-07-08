use crate::models::Todo;

pub struct TodoListComponent;

impl TodoListComponent {
    pub fn render(todos: &mut Vec<Todo>, ctrl_pressed: bool, ui: &mut egui::Ui) {
        for todo in todos {
            let text = if todo.completed {
                egui::RichText::new(&todo.text).strikethrough()
            } else {
                egui::RichText::new(&todo.text)
            };

            let response = ui.selectable_label(todo.selected, text);

            if todo.selected {
                ui.painter().rect_filled(
                    response.rect,
                    egui::Rounding::same(4),
                    egui::Color32::from_rgba_unmultiplied(100, 150, 255, 50),
                );
            }

            if response.hovered() && ctrl_pressed {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }

            if response.clicked() && ctrl_pressed {
                todo.selected = !todo.selected;
            }
            ui.add_space(5.0)
        }
    }
}