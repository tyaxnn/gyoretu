use egui::{Context, Visuals};
use egui_wgpu::ScreenDescriptor;
use egui_wgpu::Renderer;

use egui_winit::State;
use wgpu::{CommandEncoder, Device, Queue, TextureFormat, TextureView};
use winit::event::WindowEvent;
use winit::window::Window;


use crate::filters::LayerType;
use crate::model::Id;
use crate::status::Status;
use crate::compute::ComputeModel;

mod control_filter;
use control_filter::gui_filters;

mod control_source;
use control_source::gui_source;

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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WindowShowStatus{
    Source,
    Filter,
}

pub fn gui (ui: &Context, layer_infos : &mut Vec<LayerType>,id_last : &mut Id, status : &mut Status, compute_model : &mut ComputeModel, device : &wgpu::Device, queue : &wgpu::Queue, output_tx_view : &wgpu::TextureView) {

    egui::Window::new("Controller")
        // .vscroll(true)
        .default_open(true)
        .scroll(true)
        .default_width(500.)
        .default_pos(egui::Pos2::new(0.,1000.))
        .show(&ui, |ui| {

            ui.horizontal(|ui| {
                ui.selectable_value(&mut status.win_show_status, WindowShowStatus::Source, "Source");
                ui.selectable_value(&mut status.win_show_status, WindowShowStatus::Filter, "Filter");
            });

            ui.add(egui::Separator::default());

            match status.win_show_status {
                WindowShowStatus::Source => {
                    gui_source(ui, status, compute_model, device, queue, output_tx_view)
                }
                WindowShowStatus::Filter => {

                    gui_filters(ui,layer_infos,&mut compute_model.key_lists, id_last)
                }
            }


         

        }
    );
}


