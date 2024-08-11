use rfd::FileDialog; 
use std::fs;
use std::path::PathBuf;
use regex::Regex;

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

        let mut delete_lists = Vec::new();

        for i in 0..sources.len(){
            ui.label(format!("Source #{}",sources[i].id.num));

            //select local files here
            if ui.button("Open File").clicked() {
                if let Some(file) = FileDialog::new().pick_file() {

                    //get file name
                    let dir = file.parent().unwrap();
                    let paths = fs::read_dir(dir).unwrap();

                    let (file_name_option,num_str_option) = get_path_stem(&file);

                    match file_name_option {
                        Some(file_name) => {
                            match num_str_option {
                                Some(num_str) => {
                                    let mut num_list = Vec::new();

                                    for other_path_dir in paths{
                
                                        let other_path = other_path_dir.unwrap().path();
                                        let (other_file_name_option,other_num_str_option) = get_path_stem(&other_path);

                                        match other_file_name_option {
                                            Some(other_file_name) => {
                                                match other_num_str_option {
                                                    Some(other_num_str) => {
                                                        if other_file_name == file_name {
                                                            if get_extention(&file) == get_extention(&other_path){
                                                                let num = other_num_str.parse::<u32>().unwrap();
                                
                                                                num_list.push(num);
                                                            }
                                                        }
                                                    }
                                                    None => {}
                                                }
                                            }
                                            None => {}
                                        }
                                        
                                    }
                
                                    num_list.sort();
                
                                    sources[i].dir = dir.to_str().unwrap().to_string();
                                    sources[i].filename = file_name.to_string();
                                    sources[i].digit = num_str.chars().count() as u32;
                                    sources[i].from = num_list[0];
                                    sources[i].to = *num_list.last().unwrap();
                                }
                                None => {}
                            }
                        }
                        None => {}
                    }



                    
                }
            }

            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut sources[i].dir).hint_text("Write something here").desired_width(200.));
        
                ui.add(egui::TextEdit::singleline(&mut sources[i].filename).hint_text("Write something here").desired_width(100.));
                ui.label("#");
        
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
            
            if ui.button("delete").clicked() {
                delete_lists.push(i);
            }
            else {
                number_of_sources += sources[i].frame_len();
            }

        }

        for _i in delete_lists.into_iter().rev(){
            //sources.remove(i);
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

fn get_path_stem(path : &PathBuf) -> (Option<&str>,Option<&str>) {
    let stem = path.file_stem().unwrap().to_str().unwrap();

    let not_seq = Regex::new(r"^(.*?)[0-9]+$").unwrap();
    let seq = Regex::new(r"(\d+)$").unwrap();

    let not_seq_str = {
        match not_seq.captures(stem) {
            Some(cap) => {
                match cap.get(1) {
                    Some(re) => {Some(re.as_str())}
                    None => None
                }
            }
            None => None
        }
    };

    let seq_str = {
        match seq.find(stem) {
            Some(re) => {Some(re.as_str())}
            None => None
        }
    };

    (not_seq_str,seq_str)
}

fn get_extention(path : &PathBuf) -> Option<&str> {

    match path.extension(){
        Some(os) => {Some(os.to_str().unwrap())}
        None => None
    }

}