use std::fs;
use wgpu::util::DeviceExt;
use wgpu::BufferUsages;

use winit::{
    event::*,
    window::Window,
};

use egui_wgpu::ScreenDescriptor;


use crate::compute::{input_tx_views_factory, output_tx_factory, ComputeModel};
use crate::render::RenderModel;
use crate::status::{PinPongStatus, SourceIdentity, Status, FIL_BUFFER_SIZE, GEN_BUFFER_SIZE, MAX_SAMPLED_TEXTURES_PER_SHADER_STAGE ,Mode};
use crate::filters::LayerType;
use crate::gui::{EguiRenderer,gui};
use crate::write::output_buffer_factory;

pub struct Model<'a>{
    pub window: &'a Window,

    pub pv: WindowChildren<'a>,

    pub output_tx_view : wgpu::TextureView,

    pub compute_model : ComputeModel,
    pub render_model : RenderModel,
    pub status : Status,

    pub egui : EguiRenderer,

    pub output_tx : wgpu::Texture,
    pub output_buffer : wgpu::Buffer,

    pub save_dir : String,

    pub start_time : std::time::Instant,

    //pub sink : rodio::Sink,
}

pub struct WindowChildren<'a> {
    pub size: winit::dpi::PhysicalSize<u32>,
    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_format: wgpu::TextureFormat,
    pub config: wgpu::SurfaceConfiguration,
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
                    
                    default_lim.max_sampled_textures_per_shader_stage = MAX_SAMPLED_TEXTURES_PER_SHADER_STAGE;

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
    pub async fn new(window: &'a Window, mut status : Status) -> Model<'a> {

        /*------------------------------------
                surface device etc ...
        ------------------------------------*/

        let pv = WindowChildren::new(&window).await;

        /*------------------------------------
                in/output textures
        ------------------------------------*/
        

        //create input_texture_views
        //  load images here
        let input_tx_views = input_tx_views_factory(&pv.device, &pv.queue,&mut status);
        
        //create output_texture_view
        let output_tx = output_tx_factory(&pv.device, status.clone());

        let output_tx_view = output_tx.create_view(&Default::default());

        let mut input_tx_views_b = Vec::new();

        for i in 0..input_tx_views.len(){
            input_tx_views_b.push(&input_tx_views[i])
        }


        /*------------------------------------
                compute model
        ------------------------------------*/

        let mut compute_model = ComputeModel::new(&pv.device,&input_tx_views_b,&output_tx_view,&status);

        compute_model.update_inputs(&input_tx_views_b, &output_tx_view, &pv.device, &status);

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

        let output_buffer = output_buffer_factory(&pv.device, &status);

        let save_dir = "untitled".to_string();

        let start_time = std::time::Instant::now();
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
            output_tx,
            output_buffer,
            save_dir,
            start_time,
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
        let elapsed_time: f32 = 0.5 + self.start_time.elapsed().as_micros() as f32 * 1e-6;

        //store previous frame
        self.status.one_before_frame_index = self.status.next_frame_index;

        match self.status.mode{
            Mode::Preview => {
                self.status.next_frame_index = (elapsed_time * self.status.setting.frame_rate as f32) as u32 % self.status.setting.frame_len;
            }
            Mode::Render(index) => {
                self.status.next_frame_index = index;
                
                if index > self.status.setting.frame_len - 2{
                    self.status.mode = Mode::Preview;
                }
                else {
                    self.status.mode = Mode::Render(index + 1)
                }
            }
        }
        

        //calculate fps
        let delta_t = elapsed_time - self.status.previous_elapsed_time;
        
        if delta_t > 1f32 {
            self.status.actual_fps = {
                let delta_frame = self.status.elapsed_frame - self.status.previous_elapsed_frame;
                 
                if delta_t == 0f32{
                    None
                }
                else {
                    Some(delta_frame as f32 / delta_t)
                }
            };
    
            //after that, update previous_elapsed_time,frame
            self.status.previous_elapsed_time = elapsed_time;
            self.status.previous_elapsed_frame = self.status.elapsed_frame;
        }
        

        //if you chick GUI/Source/import, update texture array which store source files.
        if self.status.update_input{
            //update offset_id_map
            for source in &self.status.source_infos.sources{
                
                match self.status.offset_id_map.get(&source.id){
                    Some(identity) => {
                        self.status.offset_id_map.insert(source.id, SourceIdentity::new(identity.offset_array_texture,source.frame_len()));
                    }
                    None => {
                        
                    }
                };

                
            }

            //update layer length
            for layer in &mut self.status.layer_infos.types{
                match layer{
                    LayerType::Source(source_infos) => {
                        let source_id = source_infos.source_id;
                        match self.status.offset_id_map.get(&source_id){
                            Some(identity) => {
                                source_infos.len = identity.len;
                            }
                            None => {
                                // /panic!("No such a source id");
                            }
                        };
                    }
                    _ => {}
                }
            }
            let input_tx_views = input_tx_views_factory(&self.pv.device, &self.pv.queue, &mut self.status);
    
            let mut input_tx_views_b = Vec::new();

            for i in 0..input_tx_views.len(){
                input_tx_views_b.push(&input_tx_views[i])
            }

            self.compute_model.update_inputs(&input_tx_views_b, &self.output_tx_view, &self.pv.device, &self.status);

            self.status.update_input = false;
        }
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
        let status_buffer_data = [self.status.mov_width, self.status.mov_height, self.status.next_frame_index,self.status.elapsed_frame,(0f32).to_bits()];
        

        let status_buffer_host = self.pv.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::bytes_of(&status_buffer_data),
            usage: BufferUsages::COPY_SRC,
        });
        let mut encoder = self.pv.device.create_command_encoder(&Default::default());
        encoder.copy_buffer_to_buffer(&status_buffer_host, 0, &self.compute_model.status_buffer, 0, GEN_BUFFER_SIZE);

        {   
            let mut parameter = [0f32;20];
            parameter[3] = self.status.setting.clear_intensity; 
            let parameter_buffer_host = self.pv.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::bytes_of(&parameter),
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
        for layer in &self.status.layer_infos.types
        {   
            
            match layer{
                LayerType::Source(infos) => {

                    if infos.active{
                        //copy texture date to buffer
                        match self.status.offset_id_map.get(&infos.source_id){
                            Some(identity) => {

                                if infos.offset <= self.status.next_frame_index && self.status.next_frame_index < infos.offset + infos.len {
                                    let read_here = self.status.next_frame_index + identity.offset_array_texture - infos.offset;

                                    let status_buffer_data = [self.status.mov_width, self.status.mov_height, read_here ,self.status.elapsed_frame,(infos.alpha).to_bits()];
                                    
    
    
                                    let status_buffer_host = self.pv.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                        label: None,
                                        contents: bytemuck::bytes_of(&status_buffer_data),
                                        usage: BufferUsages::COPY_SRC,
                                    });
            
                                    encoder.copy_buffer_to_buffer(&status_buffer_host, 0, &self.compute_model.status_buffer, 0, GEN_BUFFER_SIZE);
            
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
                            None => {

                            }
                        }
                        
                    }
                }
                LayerType::Filter(infos) => {
                    if infos.active{


                        {   
                            let time = self.status.next_frame_index as f32 / self.status.setting.frame_rate as f32;
                            let parameter_buffer_host = self.pv.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                label: None,
                                contents: bytemuck::bytes_of(&infos.set_fluctus(time)),
                                usage: BufferUsages::COPY_SRC,
                            });
            
                            encoder.copy_buffer_to_buffer(&parameter_buffer_host, 0, &self.compute_model.filterinfo_buffer, 0, FIL_BUFFER_SIZE);


                            //clear old information
                            {
                                let pipeline = self.compute_model.pipelines.get("clear_old_buffer").unwrap();
                                
                                let mut compute_pass = encoder.begin_compute_pass(&Default::default());
                                compute_pass.set_pipeline(pipeline);

                                match self.status.ping_pong{
                                    PinPongStatus::F2T1 => {
                                        compute_pass.set_bind_group(0, &self.compute_model.bindgroup_even, &[]);
                                    }
                                    PinPongStatus::F1T2 => {
                                        compute_pass.set_bind_group(0, &self.compute_model.bindgroup_odd, &[]);
                                    }
                                }
                            }
                            
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

        match self.status.mode{
            Mode::Render(_) => {

                let u32_size = std::mem::size_of::<u32>() as u32;

                encoder.copy_texture_to_buffer(
                    wgpu::ImageCopyTexture {
                        aspect: wgpu::TextureAspect::All,
                                texture: &self.output_tx,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                    },
                    wgpu::ImageCopyBuffer {
                        buffer: &self.output_buffer,
                        layout: wgpu::ImageDataLayout {
                            offset: 0,
                            bytes_per_row: Some(u32_size * self.status.mov_width),
                            rows_per_image: Some(self.status.mov_height),
                        },
                    },
                    wgpu::Extent3d {
                        width : self.status.mov_width,
                        height : self.status.mov_height,
                        depth_or_array_layers: 1,
                    },
                );
        
            }_ => {}
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
            |ui| gui(ui, &mut self.status, &mut self.compute_model.key_lists),
        );

        self.pv.queue.submit(Some(encoder.finish()));
            output.present();

        Ok(())
    }

    pub fn write(&mut self) {
        match self.status.mode{
            Mode::Render(index) => {

                if index == 0 {

                    use chrono::{DateTime,Local};
                    let runtime : DateTime<Local> = Local::now();
                    self.save_dir = format!("{}_{}",runtime.format("%Y%m%d%H%M").to_string(),self.status.output_setting.filename);

                    fs::create_dir(format!("./assets/outputs/{}",self.save_dir))
                    .expect("Failed to create directory");
                }

                {   
                    let file_path = format!("./assets/outputs/{}/{}_00{}{}.png",self.save_dir,self.status.output_setting.filename,
                        {
                        
                            if index < 10 {"00"}
                            else if index < 100 {"0"}
                            else {""}
                        
                        },index);
                    let buffer_slice = self.output_buffer.slice(..);
                
                    // NOTE: We have to create the mapping THEN device.poll() before await
                    // the future. Otherwise the application will freeze.
                    let (tx, _rx) = futures_intrusive::channel::shared::oneshot_channel();
                    buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
                        tx.send(result).unwrap();
                    });
                    self.pv.device.poll(wgpu::Maintain::Wait);
                    //rx.receive();
                
                    let data = buffer_slice.get_mapped_range();
                
                    use image::{ImageBuffer, Rgba};

                    // sRGBからリニアRGBへの変換
                    let result = data.to_vec();
                    let mut srgb_data: Vec<u8> = Vec::with_capacity(result.len());
                    for chunk in result.chunks(4) {
                        let r = linear_to_srgb(chunk[0]);
                        let g = linear_to_srgb(chunk[1]);
                        let b = linear_to_srgb(chunk[2]);
                        let a = chunk[3];
                        srgb_data.push(r);
                        srgb_data.push(g);
                        srgb_data.push(b);
                        srgb_data.push(a);
                    }

                    let srgb_image_buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(self.status.mov_width, self.status.mov_height, srgb_data).unwrap();
                    srgb_image_buffer.save(file_path).unwrap();
                
                }
                self.output_buffer.unmap();
            }
            _ => {}
        }
    }
    
}

fn linear_to_srgb(value: u8) -> u8 {
    let v = value as f32 / 255.0;
    let srgb = if v <= 0.0031308 {
        12.92 * v
    } else {
        1.055 * v.powf(1.0 / 2.4) - 0.055
    };
    (srgb * 255.0).round() as u8
}