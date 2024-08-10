use crate::status::Status;

use std::fs::File;
use std::io::BufReader;
use std::fs;

pub fn gui_save (ui: &mut egui::Ui, status : &mut Status) {

    ui.label("project name");

        ui.add(egui::TextEdit::singleline(&mut status.project_name).hint_text("Write something here").desired_width(300.));

    if ui.button("save").clicked() { 

        let dir = format!("./assets/save/{}",status.project_name);

        fs::create_dir_all(dir)
            .expect("Failed to create directory");

        use chrono::{DateTime,Local};
        let runtime : DateTime<Local> = Local::now();
        let runtime_string = runtime.format("%Y%m%d%H%M%S");

        let path = format!("./assets/save/{}/{}.yaml",status.project_name,runtime_string);

        let file = File::create(path).unwrap();
    
        match serde_yaml::to_writer(file,status) {
            Err(err) => {
                ui.label(format!("{}",err));
            }
            _ => {
                ui.label("OK");
            }
        };

    }

    ui.menu_button("Read", |ui| {
        let paths = fs::read_dir("./assets/save/").unwrap();
        
        for path in paths {
            let pathbuf = path.unwrap().path();
            let project_name = pathbuf.file_stem().unwrap().to_str().unwrap().to_string();

            ui.menu_button(project_name.clone(), |ui|{
                let paths_time = fs::read_dir(format!("./assets/save/{}",project_name)).unwrap();

                for path_time in paths_time{

                    let pathbuf_time = path_time.unwrap().path();
                    let runtime_name = pathbuf_time.file_stem().unwrap().to_str().unwrap().to_string();

                    if ui.button(runtime_name.clone()).clicked() {

                        status.update_input = true;
                        //read data here
                        let read_path = File::open(format!("./assets/save/{}/{}.yaml",project_name,runtime_name)).unwrap();

                        let reader = BufReader::new(read_path);

                        let new_status : Status = serde_yaml::from_reader(reader).unwrap();

                        status.setting = new_status.setting;
                        status.source_infos = new_status.source_infos;
                        status.layer_infos = new_status.layer_infos;
                        status.output_setting = new_status.output_setting;
                        status.project_name = new_status.project_name;
                    }
                }
            });
            
        }
    });
}