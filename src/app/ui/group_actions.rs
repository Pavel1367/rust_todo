use crate::models::Todo;
use crate::services::TodoService;

#[derive(Debug, PartialEq)]
pub enum GroupAction {
    Complete,
    Delete,
    None,
}
pub struct GroupActionsComponent;
impl GroupActionsComponent {
    pub fn render(todos: &mut Vec<Todo>, ui: &mut egui::Ui) -> GroupAction {
        let has_selected = TodoService::has_selected(todos);
        let mut action = GroupAction::None;  // По умолчанию ничего

        ui.horizontal(|ui| {
            ui.add_enabled_ui(has_selected, |ui| {
                if ui.button("Complete").clicked() {
                    action = GroupAction::Complete;
                }

                if ui.button("Delete Selected").clicked() {
                    action = GroupAction::Delete;
                }
            });
        });

        action
    }
}