use crate::filters::{FilterInfo, LayerType, Ptype, SourceInfo, FilterKeys};
use crate::fluctus::Figura;
use crate::status::Status;
use crate::filters::LayerId;

pub fn gui_filters (ui: &mut egui::Ui, key_lists : &FilterKeys, status : &mut Status) {

    ui.horizontal(|ui| {

        ui.menu_button("Add filter", |ui| {

            ui.menu_button("transform", |ui|

                for key in &key_lists.transforms{
                    if ui.button(key).clicked() {

                        status.layer_infos.id_last.num += 1;
                        status.layer_infos.types.push(LayerType::Filter(FilterInfo::new(key,LayerId::new(status.layer_infos.id_last.num))));
                        ui.close_menu();
                    };
                }

            );
            ui.menu_button("color adjustment", |ui|

                for key in &key_lists.color_adjustments{
                    if ui.button(key).clicked() {

                        status.layer_infos.id_last.num += 1;
                        status.layer_infos.types.push(LayerType::Filter(FilterInfo::new(key,LayerId::new(status.layer_infos.id_last.num))));
                        ui.close_menu();
                    };
                }

            );
            ui.menu_button("dithering", |ui|

                for key in &key_lists.ditherings{
                    if ui.button(key).clicked() {

                        status.layer_infos.id_last.num += 1;
                        status.layer_infos.types.push(LayerType::Filter(FilterInfo::new(key,LayerId::new(status.layer_infos.id_last.num))));
                        ui.close_menu();
                    };
                }

            );
            ui.menu_button("generating", |ui|

                for key in &key_lists.generatings{
                    if ui.button(key).clicked() {

                        status.layer_infos.id_last.num += 1;
                        status.layer_infos.types.push(LayerType::Filter(FilterInfo::new(key,LayerId::new(status.layer_infos.id_last.num))));
                        ui.close_menu();
                    };
                }

            );
            ui.menu_button("buffer handling", |ui|

                for key in &key_lists.buffer_handlings{
                    if ui.button(key).clicked() {

                        status.layer_infos.id_last.num += 1;
                        status.layer_infos.types.push(LayerType::Filter(FilterInfo::new(key,LayerId::new(status.layer_infos.id_last.num))));
                        ui.close_menu();
                    };
                }

            );
            ui.menu_button("simulation", |ui|

                for key in &key_lists.simulations{
                    if ui.button(key).clicked() {

                        status.layer_infos.id_last.num += 1;
                        status.layer_infos.types.push(LayerType::Filter(FilterInfo::new(key,LayerId::new(status.layer_infos.id_last.num))));
                        ui.close_menu();
                    };
                }

            );
            ui.menu_button("distortion", |ui|

                for key in &key_lists.distortions{
                    if ui.button(key).clicked() {

                        status.layer_infos.id_last.num += 1;
                        status.layer_infos.types.push(LayerType::Filter(FilterInfo::new(key,LayerId::new(status.layer_infos.id_last.num))));
                        ui.close_menu();
                    };
                }

            );
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
        
                        if ui.button("⬆").clicked() {
                            if i != filter_infos_len-1{
                                swap_lists.push((i,i+1));
                            }
                        }
                        else if ui.button("⬇").clicked() {
                            if i != 0{
                                swap_lists.push((i,i-1));
                            }
                        }

                        else if ui.button("🗑").clicked() {
                            delete_lists.push(i);
                        }

                        ui.label(format!("Source #{} ",&source_info.source_id.num));
                    });

                    //フレームのオフセットをいじる
                    egui::CollapsingHeader::new("")
                            .id_source(format!("{}",source_info.id.num)).default_open(true)
                            .show(ui, |ui| {    
                        ui.add(egui::Slider::new(&mut source_info.offset, status.setting.frame_len as i32 * -1..= status.setting.frame_len as i32).text("offset"));

                        ui.add(egui::Slider::new(&mut source_info.alpha, 0f32..= 1f32).text("alpha"));

                        ui.add(egui::Slider::new(&mut source_info.speed, 0.001f32..= 10f32).text("speed"));
                    });
                }
                LayerType::Filter(filter_info) => {
                    ui.horizontal(|ui| {

                        ui.checkbox(&mut filter_info.active, "");
        
                        if ui.button("⬆").clicked() {
                            if i != filter_infos_len-1{
                                swap_lists.push((i,i+1));
                            }
                        }
                        else if ui.button("⬇").clicked() {
                            if i != 0{
                                swap_lists.push((i,i-1));
                            }
                        }
                        else if ui.button("🗑").clicked() {
                            delete_lists.push(i);
                        }

                        ui.label(format!("{}",&filter_info.key));
                    });
                    
                    //パラメータを調節するUIを作る
                    if filter_info.label.len() != 0{   
        
                        egui::CollapsingHeader::new("")
                            .id_source(format!("{}",filter_info.id.num)).default_open(true)
                            .show(ui, |ui| {
        
                                control_parameter(ui, filter_info, status.setting.quantization_cycle);
                            }
                        );
                    }
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

use strum::IntoEnumIterator;

fn control_parameter(ui: &mut egui::Ui, filter_info : &mut FilterInfo, quantization : Option<f32>) {
    let mut count = 0;
        
                                
    for p_info in &mut filter_info.label{

        match p_info.ptype {
            Ptype::Integer(range) => {
                let mut value= (f32::from_bits(filter_info.parameter[count])) as u32;

                ui.add(egui::Slider::new(&mut value, range.from..=range.to).text(&p_info.plabel));

                filter_info.parameter[count] = (value as f32).to_bits();

                count += 1;
            }
            Ptype::Float(range) => {
                let mut value= f32::from_bits(filter_info.parameter[count]);

                ui.horizontal(|ui| {

                    ui.menu_button(egui::RichText::new(p_info.fluctus.figura.display()).monospace(), |ui|{
                            
                        for fluctus in Figura::iter(){

                            if ui.button(fluctus.display()).clicked() {
                                p_info.fluctus.figura = fluctus;
                            }
                        }

                    });
                    ui.label("T");
                    ui.add(egui::Slider::new(&mut p_info.fluctus.cycle, 0.0001f32..=60f32));

                    match quantization{
                        Some(cycle) => {
                            p_info.fluctus.cycle = {
                                let rem = p_info.fluctus.cycle.rem_euclid(cycle);

                                if cycle * 0.5 < rem {
                                    p_info.fluctus.cycle + cycle - rem
                                }
                                else {
                                    p_info.fluctus.cycle - rem
                                }
                            };
                        }
                        None => {}
                    }
                    

                    ui.label("θ");
                    ui.add(egui::Slider::new(&mut p_info.fluctus.phase, 0f32..=1f32));

                    ui.label("amp");

                    ui.add(egui::Slider::new(&mut value, range.from..=range.to).text(&p_info.plabel));

                    filter_info.parameter[count] = value.to_bits();

                });

                

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
            Ptype::Color4 => {
                let r= f32::from_bits(filter_info.parameter[count]);

                let g= f32::from_bits(filter_info.parameter[count + 1]);

                let b= f32::from_bits(filter_info.parameter[count + 2]);

                let mut a= f32::from_bits(filter_info.parameter[count + 3]);

                let mut rgb = [r, g, b];

                ui.horizontal(|ui| {
                    egui::widgets::color_picker::color_edit_button_rgb(ui,&mut rgb);

                    ui.add(egui::Slider::new(&mut a, 0f32..=1f32).text(&p_info.plabel));
                });


                filter_info.parameter[count] = rgb[0].to_bits();

                filter_info.parameter[count + 1] = rgb[1].to_bits();

                filter_info.parameter[count + 2] = rgb[2].to_bits();

                filter_info.parameter[count + 3] = a.to_bits();

                count += 4;
            }
            Ptype::Bool => {

                let mut bool_checkbox;

                if filter_info.parameter[count] == 0 {
                    bool_checkbox = false;
                }
                else {
                    bool_checkbox = true;
                }
                

                ui.checkbox(&mut bool_checkbox, &p_info.plabel);

                if bool_checkbox{
                    filter_info.parameter[count] = 1;
                }
                else{
                    filter_info.parameter[count] = 0;
                }
            }
        }

        
    }    
}