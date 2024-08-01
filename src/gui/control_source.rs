use wgpu;
use crate::status::Status;
use crate::compute::{ComputeModel,input_tx_views_factory};

pub fn gui_source(ui: &mut egui::Ui, status : &mut Status, compute_model : &mut ComputeModel, device : &wgpu::Device, queue : &wgpu::Queue, output_tx_view : &wgpu::TextureView) {

    let source = &mut status.source; 

    ui.label("Source #1");
    ui.horizontal(|ui| {
        ui.add(egui::TextEdit::singleline(&mut source.dir).hint_text("Write something here").desired_width(200.));

        ui.add(egui::TextEdit::singleline(&mut source.filename).hint_text("Write something here").desired_width(100.));
        ui.label("_#5.");
        ui.add(egui::TextEdit::singleline(&mut source.extension).hint_text("Write something here").desired_width(50.));
    });

    ui.horizontal(|ui| {
        let mut from_string = source.from.to_string();
        let mut to_string = source.to.to_string();

        ui.label("from");
        ui.add(egui::TextEdit::singleline(&mut from_string).hint_text("Write something here").desired_width(50.));
        ui.label("to");
        ui.add(egui::TextEdit::singleline(&mut to_string).hint_text("Write something here").desired_width(50.));

        match from_string.parse::<u32>(){
            Ok(int) => {
                source.from = int
            }
            _ => {}
        }

        match to_string.parse::<u32>(){
            Ok(int) => {
                source.to = int
            }
            _ => {}
        }
    });

    if ui.button("import").clicked() {
        //create input_texture_views
        //  load images here
        let input_tx_views = input_tx_views_factory(device, queue,status.clone());

        let mut input_tx_views_b = Vec::new();

        for i in 0..input_tx_views.len(){
            input_tx_views_b.push(&input_tx_views[i])
        }

        status.frame_len_max = status.source.frame_len();

        compute_model.update_inputs(&input_tx_views_b, &output_tx_view, device, status);

        
    }
}