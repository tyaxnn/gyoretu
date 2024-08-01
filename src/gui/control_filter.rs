use crate::filters::{FilterInfo,Ptype,LayerType};
use crate::model::Id;

pub fn gui_filters (ui: &mut egui::Ui, layer_infos : &mut Vec<LayerType>, key_lists : &Vec<String>, id_last : &mut Id) {
    ui.menu_button("Add filter", |ui| {

        for key in key_lists{
            if ui.button(key).clicked() {

                id_last.id += 1;
                layer_infos.push(LayerType::Filter(FilterInfo::new(key,id_last.id)));
                ui.close_menu();
            };
        }
        if ui.button("cancel").clicked() {
            ui.close_menu();
        }
    });

    egui::ScrollArea::vertical().show(ui, |ui| {

        ui.separator();
    
        let mut delete_lists = Vec::new();
        let mut swap_lists = Vec::new();

        let filter_infos_len = layer_infos.len();

        for i in (0..filter_infos_len).rev(){

            let layer_type =  &mut layer_infos[i];

            match layer_type {
                LayerType::Source(source_info) => {
                    ui.horizontal(|ui| {

                        ui.checkbox(&mut source_info.active, "");
        
                        ui.label(format!("Source #{}",&source_info.id));
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
                }
                LayerType::Filter(filter_info) => {
                    ui.horizontal(|ui| {

                        ui.checkbox(&mut filter_info.active, "");
        
                        ui.label(&filter_info.key);
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
                            .id_source(format!("{}",filter_info.id)).default_open(true)
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
            }

            
            
        }

        for i in delete_lists.into_iter().rev(){
            layer_infos.remove(i);
        }

        for (i,j) in swap_lists {
            layer_infos.swap(i, j);
        }
        
        ui.end_row();

        // proto_scene.egui(ui);

    });

}