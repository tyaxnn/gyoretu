use egui::{Context, Visuals};
use egui_wgpu::ScreenDescriptor;
use egui_wgpu::Renderer;

use egui_winit::State;
use wgpu::{CommandEncoder, Device, Queue, TextureFormat, TextureView};
use winit::event::WindowEvent;
use winit::window::Window;

use crate::filters::{FilterInfo,Ptype};
use crate::model::Id;

pub struct EguiRenderer {
    pub context: Context,
    state: State,
    renderer: Renderer,
}

impl EguiRenderer {
    pub fn new(
        device: &Device,
        output_color_format: TextureFormat,
        output_depth_format: Option<TextureFormat>,
        msaa_samples: u32,
        window: &Window,
    ) -> EguiRenderer {
        let egui_context = Context::default();
        let id = egui_context.viewport_id();

        let visuals = Visuals::light();

        egui_context.set_visuals(visuals);

        let egui_state = egui_winit::State::new(egui_context.clone(), id, &window, None, None);

        // egui_state.set_pixels_per_point(window.scale_factor() as f32);
        let egui_renderer = egui_wgpu::Renderer::new(
            device,
            output_color_format,
            output_depth_format,
            msaa_samples,
        );

        EguiRenderer {
            context: egui_context,
            state: egui_state,
            renderer: egui_renderer,
        }
    }

    pub fn handle_input(&mut self, window: &Window, event: &WindowEvent) {
        let _ = self.state.on_window_event(window, event);
    }

    pub fn draw(
        &mut self,
        device: &Device,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        window: &Window,
        window_surface_view: &TextureView,
        screen_descriptor: ScreenDescriptor,
        run_ui: impl FnOnce(&Context),
    ) {
        // self.state.set_pixels_per_point(window.scale_factor() as f32);
        let raw_input = self.state.take_egui_input(&window);
        let full_output = self.context.run(raw_input, |_ui| {
            run_ui(&self.context);
        });

        self.state
            .handle_platform_output(&window, full_output.platform_output);

        let tris = self
            .context
            .tessellate(full_output.shapes, full_output.pixels_per_point);
        for (id, image_delta) in &full_output.textures_delta.set {
            self.renderer
                .update_texture(&device, &queue, *id, &image_delta);
        }
        self.renderer
            .update_buffers(&device, &queue, encoder, &tris, &screen_descriptor);
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &window_surface_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            label: Some("egui main render pass"),
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        self.renderer.render(&mut rpass, &tris, &screen_descriptor);
        drop(rpass);
        for x in &full_output.textures_delta.free {
            self.renderer.free_texture(x)
        }
    }
}



pub fn gui_filters (ui: &Context, filter_infos : &mut Vec<FilterInfo>, key_lists : &Vec<String>, id_last : &mut Id) {
    egui::Window::new("Controller")
        // .vscroll(true)
        .default_open(true)
        .scroll(true)
        .default_width(250.)
        .show(&ui, |ui| {

            ui.menu_button("Add filter", |ui| {

                for key in key_lists{
                    if ui.button(key).clicked() {

                        id_last.id += 1;
                        filter_infos.push(FilterInfo::new(key,id_last.id));
                        ui.close_menu();
                    };
                }
                if ui.button("cancel").clicked() {
                    ui.close_menu();
                }
            });
            
            let mut delete_lists = Vec::new();
            let mut swap_lists = Vec::new();

            let filter_infos_len = filter_infos.len();

            for i in (0..filter_infos_len).rev(){

                let filter_info = &mut filter_infos[i];

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
                        });
                }
                
            }

            for i in delete_lists.into_iter().rev(){
                filter_infos.remove(i);
            }

            for (i,j) in swap_lists {
                filter_infos.swap(i, j);
            }
            
            ui.end_row();

            // proto_scene.egui(ui);
        });
}