use crate::status::{Setting,MAX_SAMPLED_TEXTURES_PER_SHADER_STAGE};

pub fn gui_setting (ui: &mut egui::Ui, setting : &mut Setting) {

    ui.add(egui::Slider::new(&mut setting.frame_len, 1..= MAX_SAMPLED_TEXTURES_PER_SHADER_STAGE).text("frame length"));

    ui.add(egui::Slider::new(&mut setting.frame_rate, 1..= 60).text("frame rate"));

    ui.add(egui::Slider::new(&mut setting.clear_intensity, 0f32..= 1f32).text("clear intensity"));
}