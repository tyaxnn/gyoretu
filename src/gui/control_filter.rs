use crate::filters::{FilterInfo, LayerType, Ptype, SourceInfo};
use crate::status::Status;
use crate::filters::LayerId;

pub fn gui_filters (ui: &mut egui::Ui, key_lists : &Vec<String>, status : &mut Status) {

    ui.horizontal(|ui| {

        ui.menu_button("Add filter", |ui| {

            for key in key_lists{
                if ui.button(key).clicked() {

                    status.layer_infos.id_last.num += 1;
                    status.layer_infos.types.push(LayerType::Filter(FilterInfo::new(key,LayerId::new(status.layer_infos.id_last.num))));
                    ui.close_menu();
                };
            }
            if ui.button("cancel").clicked() {
                ui.close_menu();
            }
        });

        ui.menu_button("Add Source", |ui| {

            for source in &status.source_infos.sources{
                if ui.button(format!("# {}",source.id.num)).clicked() {

                    status.layer_infos.id_last.num += 1;
                    status.layer_infos.types.push(LayerType::Source(SourceInfo::new(status.layer_infos.id_last,source.id, source.frame_len())));
                    ui.close_menu();
                };
            }
            if ui.button("cancel").clicked() {
                ui.close_menu();
            }
        });

    });

    egui::ScrollArea::vertical().show(ui, |ui| {

        ui.separator();
    
        let mut delete_lists = Vec::new();
        let mut swap_lists = Vec::new();

        let filter_infos_len = status.layer_infos.types.len();

        for i in (0..filter_infos_len).rev(){

            let layer_type =  &mut status.layer_infos.types[i];

            match layer_type {
                LayerType::Source(source_info) => {
                    ui.horizontal(|ui| {

                        ui.checkbox(&mut source_info.active, "");
        
                        ui.label(format!("Source #{} ",&source_info.source_id.num));
                    });

                    ui.horizontal(|ui| {
        
                        if ui.button("up").clicked() {
                            if i != filter_infos_len-1{
                                swap_lists.push((i,i+1));
                            }
                        }
                        else if ui.button("down").clicked() {
                            if i != 0{
                                swap_lists.push((i,i-1));
                            }
                        }

                        else if ui.button("delete").clicked() {
                            delete_lists.push(i);
                        }
                    });

                    //フレームのオフセットをいじる
                    egui::CollapsingHeader::new("")
                            .id_source(format!("{}",source_info.id.num)).default_open(true)
                            .show(ui, |ui| {    
                        ui.add(egui::Slider::new(&mut source_info.offset, 0..= status.setting.frame_len).text("offset"));
                    });
                }
                LayerType::Filter(filter_info) => {
                    ui.horizontal(|ui| {

                        ui.checkbox(&mut filter_info.active, "");
        
                        ui.label(format!("{}",&filter_info.key));
                    });
        
                    
                    ui.horizontal(|ui| {
        
                        if ui.button("up").clicked() {
                            if i != filter_infos_len-1{
                                swap_lists.push((i,i+1));
                            }
                        }
                        else if ui.button("down").clicked() {
                            if i != 0{
                                swap_lists.push((i,i-1));
                            }
                        }
                        else if ui.button("delete").clicked() {
                            delete_lists.push(i);
                        }
                    });
                    
                    //パラメータを調節するUIを作る
                    if filter_info.label.len() != 0{   
        
                        egui::CollapsingHeader::new("")
                            .id_source(format!("{}",filter_info.id.num)).default_open(true)
                            .show(ui, |ui| {
        
                                let mut count = 0;
        
                                
                                for p_info in &filter_info.label{
        
                                    match p_info.ptype {
                                        Ptype::Integer => {
                                            let mut value= (f32::from_bits(filter_info.parameter[count]) * 256.) as u32;
        
                                            ui.add(egui::Slider::new(&mut value, 0..=255).text(&p_info.plabel));
                            
                                            filter_info.parameter[count] = (value as f32 / 256.).to_bits();
        
                                            count += 1;
                                        }
                                        Ptype::Float => {
                                            let mut value= f32::from_bits(filter_info.parameter[count]);
        
                                            ui.add(egui::Slider::new(&mut value, 0f32..=1f32).text(&p_info.plabel));
                            
                                            filter_info.parameter[count] = value.to_bits();
        
                                            count += 1;
                                        }
                                        Ptype::Color3 => {
                                            let r= f32::from_bits(filter_info.parameter[count]);
        
                                            let g= f32::from_bits(filter_info.parameter[count + 1]);
        
                                            let b= f32::from_bits(filter_info.parameter[count + 2]);
        
                                            let mut rgb = [r, g, b];
        
                                            ui.horizontal(|ui| {
                                                egui::widgets::color_picker::color_edit_button_rgb(ui,&mut rgb);
        
                                                ui.label(&p_info.plabel);
                                            });
        
        
                                            filter_info.parameter[count] = rgb[0].to_bits();
        
                                            filter_info.parameter[count + 1] = rgb[1].to_bits();
        
                                            filter_info.parameter[count + 2] = rgb[2].to_bits();
        
                                            count += 3;
                                        }
                                    }
                
                                    
                                }
                            }
                        );
                    }
                }
                LayerType::Bg(bg_info) => {
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut bg_info.active, "");

                        ui.label(format!("Back ground"));
                    });

                    ui.horizontal(|ui| {
        
                        if ui.button("up").clicked() {
                            if i != filter_infos_len-1{
                                swap_lists.push((i,i+1));
                            }
                        }
                        else if ui.button("down").clicked() {
                            if i != 0{
                                swap_lists.push((i,i-1));
                            }
                        }
                    });

                    egui::CollapsingHeader::new("")
                            .id_source(format!("bg{}",bg_info.id.num)).default_open(true)
                            .show(ui, |ui| {

                        let r= f32::from_bits(bg_info.parameter[0]);

                        let g= f32::from_bits(bg_info.parameter[1]);

                        let b= f32::from_bits(bg_info.parameter[2]);

                        let mut rgb = [r, g, b];

                        ui.horizontal(|ui| {
                            egui::widgets::color_picker::color_edit_button_rgb(ui,&mut rgb);

                            ui.label("Back ground color");
                        });


                        bg_info.parameter[0] = rgb[0].to_bits();

                        bg_info.parameter[1] = rgb[1].to_bits();

                        bg_info.parameter[2] = rgb[2].to_bits();

                    });
                }
            }

            
            
        }

        for i in delete_lists.into_iter().rev(){
            status.layer_infos.types.remove(i);
        }

        for (i,j) in swap_lists {
            status.layer_infos.types.swap(i, j);
        }
        
        ui.end_row();

        // proto_scene.egui(ui);

    });

}