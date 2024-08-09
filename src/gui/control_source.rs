use crate::status::{Status,Source,MAX_SAMPLED_TEXTURES_PER_SHADER_STAGE};

pub fn gui_source(ui: &mut egui::Ui, status : &mut Status) {

    if ui.button("Add source").clicked() {        
        status.source_infos.id_last.num += 1;

        //Add to Status.sources
        status.source_infos.sources.push(Source::new(status.source_infos.id_last));
    };

    let sources = &mut status.source_infos.sources;

    let mut number_of_sources = 0;

    egui::ScrollArea::vertical().show(ui, |ui| {

        ui.separator();

        for i in 0..sources.len(){
            ui.label(format!("Source #{}",sources[i].id.num));
            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut sources[i].dir).hint_text("Write something here").desired_width(300.));
        
                ui.add(egui::TextEdit::singleline(&mut sources[i].filename).hint_text("Write something here").desired_width(100.));
                ui.label("_#");
        
                let mut digit_string = sources[i].digit.to_string();
        
                ui.add(egui::TextEdit::singleline(&mut digit_string).hint_text("Write something here").desired_width(20.));
                
                match digit_string.parse::<u32>(){
                    Ok(int) => {
                        sources[i].digit = int
                    }
                    _ => {}
                }
                
                ui.label(".");
                ui.add(egui::TextEdit::singleline(&mut sources[i].extension).hint_text("Write something here").desired_width(50.));
            });
        
            ui.horizontal(|ui| {
                let mut from_string = sources[i].from.to_string();
                let mut to_string = sources[i].to.to_string();
        
                ui.label("from");
                ui.add(egui::TextEdit::singleline(&mut from_string).hint_text("Write something here").desired_width(50.));
                ui.label("to");
                ui.add(egui::TextEdit::singleline(&mut to_string).hint_text("Write something here").desired_width(50.));
        
                match from_string.parse::<u32>(){
                    Ok(int) => {
                        sources[i].from = int;
                    }
                    _ => {}
                }
        
                match to_string.parse::<u32>(){
                    Ok(int) => {
                        sources[i].to = int;
                    }
                    _ => {}
                }
            });

            number_of_sources += sources[i].frame_len();
        }

        if ui.button("import").clicked() {
            //create input_texture_views
            //  load images here

            //in order not to crash the app, check how many sources 
            if number_of_sources > MAX_SAMPLED_TEXTURES_PER_SHADER_STAGE{
                println!("too many sources !")
            }
            else{
                status.update_input = true;
            } 
        }
    });
}