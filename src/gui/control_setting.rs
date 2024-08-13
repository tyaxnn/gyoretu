use crate::status::{Setting,MAX_SAMPLED_TEXTURES_PER_SHADER_STAGE};

pub fn gui_setting (ui: &mut egui::Ui, setting : &mut Setting) {

    ui.add(egui::Slider::new(&mut setting.frame_len, 1..= MAX_SAMPLED_TEXTURES_PER_SHADER_STAGE * 2).text("frame length"));

    ui.add(egui::Slider::new(&mut setting.frame_rate, 1..= 60).text("frame rate"));

    ui.horizontal(|ui| {
        let mut quantization = {setting.quantization_cycle != None};

        ui.checkbox(&mut quantization, "quantization");

        if quantization{

            let mut cycle;
            match setting.quantization_cycle{
                
                Some(pre) => {
                    cycle = pre;
                }
                None => {
                    cycle = 1f32;
                }
            }

            ui.add(egui::Slider::new(&mut cycle, 0.0001f32..=5f32));

            setting.quantization_cycle = Some(cycle);
        }
        else {
            setting.quantization_cycle = None
        }
    });
}