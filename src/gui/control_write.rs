use crate::status::{Status,Mode};

pub fn gui_write (ui: &mut egui::Ui, status : &mut Status) {

    ui.add(egui::TextEdit::singleline(&mut status.output_setting.filename).hint_text("Write something here").desired_width(300.));

    if ui.button("start rendering").clicked() {
        status.mode = Mode::Render(0);
    }

    match status.mode{
        Mode::Render(index) => {
            let progress = index as f32 / status.setting.frame_len as f32;
            let progress_bar = egui::widgets::ProgressBar::new(progress).show_percentage();

            ui.add(progress_bar);
        }
        _ => {}
    }
}