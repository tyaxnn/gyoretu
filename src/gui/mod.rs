use egui::{Context, Visuals};
use egui_wgpu::ScreenDescriptor;
use egui_wgpu::Renderer;

use egui_winit::State;
use wgpu::{CommandEncoder, Device, Queue, TextureFormat, TextureView};
use winit::event::WindowEvent;
use winit::window::Window;

use serde::{Serialize, Deserialize};

use crate::status::Status;
use crate::filters::FilterKeys;

mod control_filter;
use control_filter::gui_filters;

mod control_source;
use control_source::gui_source;

mod control_setting;
use control_setting::gui_setting;

mod control_info;
use control_info::gui_info;

mod control_write;
use control_write::gui_write;

mod control_save;
use control_save::gui_save;

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

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum WindowShowStatus{
    Source,
    Filter,
    Setting,
    Info,
    Write,
    Save,
}

pub fn gui (ui: &Context, status : &mut Status, key_lists : &mut FilterKeys) {

    egui::Window::new("Controller")
        // .vscroll(true)
        .default_open(true)
        .scroll(true)
        .default_width(625.)
        .default_height(1080.)
        .default_pos(egui::Pos2::new(1295.,0.))
        .resizable(false)
        .movable(false)
        .title_bar(false)
        .show(&ui, |ui| {

            ui.horizontal(|ui| {
                ui.selectable_value(&mut status.win_show_status, WindowShowStatus::Source, "Source");
                ui.selectable_value(&mut status.win_show_status, WindowShowStatus::Filter, "Filter");
                ui.selectable_value(&mut status.win_show_status, WindowShowStatus::Setting, "Setting");
                ui.selectable_value(&mut status.win_show_status, WindowShowStatus::Info, "Info");
                ui.selectable_value(&mut status.win_show_status, WindowShowStatus::Write, "Write");
                ui.selectable_value(&mut status.win_show_status, WindowShowStatus::Save, "Save");
            });

            ui.add(egui::Separator::default());

            match status.win_show_status {
                WindowShowStatus::Source => {
                    gui_source(ui, status)
                }
                WindowShowStatus::Filter => {
                    gui_filters(ui, key_lists, status)
                }
                WindowShowStatus::Setting => {
                    gui_setting(ui, &mut status.setting)
                }
                WindowShowStatus::Info => {
                    gui_info(ui, status)
                }
                WindowShowStatus::Write => {
                    gui_write(ui, status)
                }
                WindowShowStatus::Save => {
                    gui_save(ui, status)
                }

            }


         

        }
    );
}


