use wgpu::util::DeviceExt;
use wgpu::BufferUsages;

use winit::{
    event::*,
    window::Window,
};

use egui_wgpu::ScreenDescriptor;


use crate::compute::{input_tx_views_factory, output_tx_view_factory, ComputeModel};
use crate::render::RenderModel;
use crate::status::{PinPongStatus, Status, FIL_BUFFER_SIZE, GEN_BUFFER_SIZE};
use crate::filters::{SourceInfo,LayerType,BgInfo};
use crate::gui::{EguiRenderer,gui};

pub struct Model<'a>{
    pub window: &'a Window,

    pub pv: WindowChildren<'a>,

    pub output_tx_view : wgpu::TextureView,

    pub compute_model : ComputeModel,
    pub render_model : RenderModel,
    pub status : Status,

    pub egui : EguiRenderer,

    pub layer_infos : Vec<LayerType>,
    pub id_last : Id,
}

pub struct WindowChildren<'a> {
    pub size: winit::dpi::PhysicalSize<u32>,
    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_format: wgpu::TextureFormat,
    pub config: wgpu::SurfaceConfiguration,
}

pub struct Id{
    pub id : usize,
}

impl<'a> WindowChildren<'a> {
    pub async fn new(window: &'a Window) -> WindowChildren<'a> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(
            wgpu::InstanceDescriptor{
                backends: wgpu::Backends::PRIMARY,
                ..Default::default()
            }
        );

        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: Default::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .expect("error finding adapter");

        let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::TEXTURE_BINDING_ARRAY,
                required_limits: {
                    let mut default_lim = wgpu::Limits::default();
                    
                    default_lim.max_sampled_textures_per_shader_stage = 1024;

                    default_lim
                },
            },
            //Some(&std::path::Path::new("trace")), // Trace path
            None,
        )
        .await
        .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        WindowChildren{
            size,
            surface,
            device,
            queue,
            surface_format,
            config,
        }
    }
}

impl<'a> Model<'a> {
    pub async fn new(window: &'a Window, status : Status) -> Model<'a> {

        /*------------------------------------
                surface device etc ...
        ------------------------------------*/

        let pv = WindowChildren::new(&window).await;

        /*------------------------------------
                in/output textures
        ------------------------------------*/
        

        //create input_texture_views
        //  load images here
        let input_tx_views = input_tx_views_factory(&pv.device, &pv.queue,status.clone());
        
        //create output_texture_view
        let output_tx_view = output_tx_view_factory(&pv.device, status.clone());

        let mut input_tx_views_b = Vec::new();

        for i in 0..input_tx_views.len(){
            input_tx_views_b.push(&input_tx_views[i])
        }


        /*------------------------------------
                compute model
        ------------------------------------*/

        let mut compute_model = ComputeModel::new(&pv.device,&input_tx_views_b,&output_tx_view,status.clone());

        compute_model.update_inputs(&input_tx_views_b, &output_tx_view, &pv.device, &mut status.clone());

        /*------------------------------------
                render model
        ------------------------------------*/

        let render_model = RenderModel ::new(&pv.device, pv.surface_format, &output_tx_view);

        /*------------------------------------
                gui model
        ------------------------------------*/
        
        let egui = EguiRenderer::new(
            &pv.device,       // wgpu Device
            pv.config.format, // TextureFormat
            None,          // this can be None
            1,             // samples
            &window,       // winit Window
        );

        let ini_source = LayerType::Source(SourceInfo{id : 1, active : true});
        let layer_infos = vec![LayerType::Bg(BgInfo::new(1)),ini_source];

        let id_last = Id{id : layer_infos.len()};
        
        /*------------------------------------
                Return Model
        ------------------------------------*/
        Self {
            window,
            pv,
            output_tx_view,
            compute_model,
            render_model,
            status,
            egui,
            layer_infos,
            id_last,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    
    pub fn input(&mut self, _event: &WindowEvent) -> bool {
        self.window().request_redraw();
        false
    }
    
    //update status
    pub fn update_pre(&mut self) {
        let elapsed_time: f32 = 0.5 + self.status.start_time.elapsed().as_micros() as f32 * 1e-6;

        self.status.next_frame_index = (elapsed_time * self.status.frame_rate as f32) as u32 % self.status.frame_len_max;
    }

    pub fn update_post(&mut self) {
        self.status.elapsed_frame += 1;

        //self.status.ping_pong = PinPongStatus::F1T2;
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.pv.size = new_size;
            self.pv.config.width = new_size.width;
            self.pv.config.height = new_size.height;
            self.pv.surface.configure(&self.pv.device, &self.pv.config);
        }
    }

    
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.pv.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        //
        let status_buffer_data = [self.status.mov_width, self.status.mov_height, self.status.next_frame_index,0,(0f32).to_bits()];
        

        let status_buffer_host = self.pv.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::bytes_of(&status_buffer_data),
            usage: BufferUsages::COPY_SRC,
        });
        let mut encoder = self.pv.device.create_command_encoder(&Default::default());
        encoder.copy_buffer_to_buffer(&status_buffer_host, 0, &self.compute_model.status_buffer, 0, GEN_BUFFER_SIZE);

        {
            let parameter_buffer_host = self.pv.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::bytes_of(&[0f32;20]),
                usage: BufferUsages::COPY_SRC,
            });

            encoder.copy_buffer_to_buffer(&parameter_buffer_host, 0, &self.compute_model.filterinfo_buffer, 0, FIL_BUFFER_SIZE);

            { 
                let mut compute_pass = encoder.begin_compute_pass(&Default::default());
                compute_pass.set_pipeline(&self.compute_model.pipeline_bg);
    
                match self.status.ping_pong{
                    PinPongStatus::F2T1 => {
                        compute_pass.set_bind_group(0, &self.compute_model.bindgroup_even, &[]);
    
                        self.status.ping_pong = PinPongStatus::F1T2
                    }
                    PinPongStatus::F1T2 => {
                        compute_pass.set_bind_group(0, &self.compute_model.bindgroup_odd, &[]);
    
                        self.status.ping_pong = PinPongStatus::F2T1
                    }
                }
                
                compute_pass.dispatch_workgroups(self.pv.size.width / 16, self.pv.size.height / 16, 1);
            }
        } 
        
        //Filter the image here
        for layer in &self.layer_infos
        {   
            match layer{
                LayerType::Source(infos) => {
                    if infos.active{
                        //copy texture date to buffer
                        {
                            let mut compute_pass = encoder.begin_compute_pass(&Default::default());
                            compute_pass.set_pipeline(&self.compute_model.pipeline_add_source);

                            match self.status.ping_pong{
                                PinPongStatus::F2T1 => {
                                    compute_pass.set_bind_group(0, &self.compute_model.bindgroup_even, &[]);
        
                                    self.status.ping_pong = PinPongStatus::F1T2
                                }
                                PinPongStatus::F1T2 => {
                                    compute_pass.set_bind_group(0, &self.compute_model.bindgroup_odd, &[]);
        
                                    self.status.ping_pong = PinPongStatus::F2T1
                                }
                            }
                            
                            compute_pass.dispatch_workgroups(self.pv.size.width / 16, self.pv.size.height / 16, 1);
                        }
                    }
                }
                LayerType::Filter(infos) => {
                    if infos.active{
            
                        let parameter_buffer_host = self.pv.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: None,
                            contents: bytemuck::bytes_of(&infos.parameter),
                            usage: BufferUsages::COPY_SRC,
                        });
        
                        encoder.copy_buffer_to_buffer(&parameter_buffer_host, 0, &self.compute_model.filterinfo_buffer, 0, FIL_BUFFER_SIZE);
                        
                        {
                            let pipeline = self.compute_model.pipelines.get(&infos.key).unwrap();
                            let mut compute_pass = encoder.begin_compute_pass(&Default::default());
                            compute_pass.set_pipeline(pipeline);
                            
                            match self.status.ping_pong{
                                PinPongStatus::F2T1 => {
                                    compute_pass.set_bind_group(0, &self.compute_model.bindgroup_even, &[]);
        
                                    self.status.ping_pong = PinPongStatus::F1T2
                                }
                                PinPongStatus::F1T2 => {
                                    compute_pass.set_bind_group(0, &self.compute_model.bindgroup_odd, &[]);
        
                                    self.status.ping_pong = PinPongStatus::F2T1
                                }
                            }
                            
                            compute_pass.dispatch_workgroups(self.pv.size.width / 16, self.pv.size.height / 16, 1);
        
                        }
                    }
                }
                LayerType::Bg(infos) => {
                    if infos.active{
                        let parameter_buffer_host = self.pv.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: None,
                            contents: bytemuck::bytes_of(&infos.parameter),
                            usage: BufferUsages::COPY_SRC,
                        });
            
                        encoder.copy_buffer_to_buffer(&parameter_buffer_host, 0, &self.compute_model.filterinfo_buffer, 0, FIL_BUFFER_SIZE);
            
                        { 
                            let mut compute_pass = encoder.begin_compute_pass(&Default::default());
                            compute_pass.set_pipeline(&self.compute_model.pipeline_bg);
                
                            match self.status.ping_pong{
                                PinPongStatus::F2T1 => {
                                    compute_pass.set_bind_group(0, &self.compute_model.bindgroup_even, &[]);
                
                                    self.status.ping_pong = PinPongStatus::F1T2
                                }
                                PinPongStatus::F1T2 => {
                                    compute_pass.set_bind_group(0, &self.compute_model.bindgroup_odd, &[]);
                
                                    self.status.ping_pong = PinPongStatus::F2T1
                                }
                            }
                            
                            compute_pass.dispatch_workgroups(self.pv.size.width / 16, self.pv.size.height / 16, 1);
                        }
                    }
                }
            }

            
        }

        {
            
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color{r: 0.8,g:0.8,b:0.8,a:1.0}),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_model.pipeline);
            render_pass.set_bind_group(0, &self.render_model.bindgroup, &[]);
            render_pass.draw(0..3, 0..2);
        }

        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [self.pv.config.width, self.pv.config.height],
            pixels_per_point: self.window().scale_factor() as f32,
        };

        self.egui.draw(
            &self.pv.device,
            &self.pv.queue,
            &mut encoder,
            &self.window,
            &view,
            screen_descriptor,
            |ui| gui(ui,&mut self.layer_infos, &mut self.id_last, &mut self.status, &mut self.compute_model, &self.pv.device, &self.pv.queue, &self.output_tx_view),
        );

        self.pv.queue.submit(Some(encoder.finish()));
            output.present();

        Ok(())
    }
    
}