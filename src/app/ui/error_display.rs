use egui::Id;
use crate::models::ErrorMessage;

pub struct ErrorDisplayComponent;

impl ErrorDisplayComponent {
    pub fn render(errors: &mut Vec<ErrorMessage>, ctx: &egui::Context) {
        let now = std::time::Instant::now();

        // Удаляем старые ошибки (старше 4 секунд)
        errors.retain(|err| now.duration_since(err.created_at).as_secs() < 4);

        if !errors.is_empty() {
            egui::Area::new(Id::from("errors"))
                .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-10.0, 10.0))
                .show(ctx, |ui| {
                    for error in errors {
                        ui.colored_label(egui::Color32::RED, &error.message);
                    }
                });
        }
    }
}