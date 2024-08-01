
use std::num::NonZeroU32;
use std::fs;
use std::collections::HashMap;

use wgpu::{BufferUsages, Extent3d, TextureView, util::DeviceExt};

use crate::status::{Status,GEN_BUFFER_SIZE,FIL_BUFFER_SIZE};

pub struct ComputeModel{
    pub pipeline_init : wgpu::ComputePipeline,
    pub pipelines : HashMap<String,wgpu::ComputePipeline>,
    pub bindgroup_odd : wgpu::BindGroup,
    pub bindgroup_even : wgpu::BindGroup,
    pub status_buffer : wgpu::Buffer,
    pub intermediate_buffer_1 : wgpu::Buffer,
    pub intermediate_buffer_2 : wgpu::Buffer,
    pub filterinfo_buffer : wgpu::Buffer,
    pub key_lists : Vec<String>,
}

struct KeyShaderModule {
    key : String,
    shader_module : wgpu::ShaderModule,
}

impl ComputeModel {
    pub fn new(
        device : &wgpu::Device,
        input_tx_views_b : &Vec<&TextureView>,
        output_tx_view : &TextureView,
        status : Status,
    ) -> ComputeModel{

        //this shader is identity filter
        let shader_module_init = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("copy_tx2bf.wgsl").into()),
        });

        let key_lists = Vec::new();

        let bindgroup_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[

                //it storage current status (window size, now which frames, which steps)
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                //render this texture
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
                //storage sequential texture (raw movie data)
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: NonZeroU32::new(status.source.frame_len()),
                },
                //ping-pong buffer 1
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: (true) },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },

                //ping-pong buffer 2
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: (false) },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                //it storage filter parametor
                wgpu::BindGroupLayoutEntry {
                    binding: 5,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bindgroup_layout],
            push_constant_ranges: &[],
        });

        let pipeline_init = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            module: &shader_module_init,
            entry_point: "main",
            compilation_options: Default::default(),
        });

        //this vec contain all filter's piplines
        let pipelines = HashMap::new();

        /*------------------------------------
                initialize buffers
        ------------------------------------*/
        
        let status_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Status"),
            size: GEN_BUFFER_SIZE,
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE | BufferUsages::UNIFORM,
            mapped_at_creation: false,
        });

        let init_vec = vec![[0.7f32;4]; (status.mov_width * status.mov_height) as usize];

        let intermediate_buffer_1 = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("intermediate_read"),
            contents : bytemuck::cast_slice(&init_vec),
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
        });

        let intermediate_buffer_2 = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("intermediate_write"),
            size: (init_vec.len() * std::mem::size_of::<f32>() * 4) as u64,
            usage:  BufferUsages::COPY_DST | BufferUsages::STORAGE,
            mapped_at_creation: false,
        });

        let filterinfo_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Status"),
            size: FIL_BUFFER_SIZE,
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE | BufferUsages::UNIFORM,
            mapped_at_creation: false,
        });
        
        /*------------------------------------
                create bindgroups
        ------------------------------------*/

        let bindgroup_odd = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bindgroup_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: status_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(output_tx_view),
                },
                
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureViewArray(input_tx_views_b),
                }, 
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: intermediate_buffer_1.as_entire_binding(),
                },

                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: intermediate_buffer_2.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: filterinfo_buffer.as_entire_binding(),
                },
    
            ],
        });

        let bindgroup_even = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bindgroup_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: status_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(output_tx_view),
                },
                
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureViewArray(input_tx_views_b),
                }, 
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: intermediate_buffer_2.as_entire_binding(),
                },

                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: intermediate_buffer_1.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: filterinfo_buffer.as_entire_binding(),
                },
    
            ],
        });

        ComputeModel{
            pipeline_init,
            pipelines,
            bindgroup_odd,
            bindgroup_even,
            status_buffer,
            intermediate_buffer_1,
            intermediate_buffer_2,
            filterinfo_buffer,
            key_lists,
        }
    }

    //recreate computemodel : new input texture
    //もっと簡潔に書きたい。
    pub fn update_inputs (
        &mut self, 
        input_tx_views_b : &Vec<&TextureView>,
        output_tx_view : &TextureView,
        device : &wgpu::Device,
        status : &mut Status,
    ) {

        /*
        step 0 : inpute_texture -> buffer_1(w) : pipeline_init  use bindgroup_even
        step 1 : buffer_1(r)    -> buffer_2(w) : pipelines[0]   use bindgroup_odd
        step 2 : buffer_2(r)    -> buffer_1(w) : pipelines[1]   use bindgroup_even
        ...
        */

        let bindgroup_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[

                //it storage current status (window size, now which frames, which steps)
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                //render this texture
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
                //storage sequential texture (raw movie data)
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: NonZeroU32::new(status.source.frame_len()),
                },
                //ping-pong buffer 1
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: (true) },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },

                //ping-pong buffer 2
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: (false) },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                //it storage filter parametor
                wgpu::BindGroupLayoutEntry {
                    binding: 5,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let shader_module_init = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("copy_tx2bf.wgsl").into()),
        });

        let paths = fs::read_dir("./src/filters/").unwrap();

        let mut shader_modules = Vec::new();

        let mut key_lists = Vec::new();

        for path in paths {
            let pathbuf = path.unwrap().path();
            let key = pathbuf.file_stem().unwrap().to_str().unwrap().to_string();

            let path_string = format!("./src/filters/{}.wgsl",key);

            let path_read_to_string = fs::read_to_string(path_string).unwrap();

            let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(path_read_to_string.into()),
            });

            key_lists.push(key.clone());
            shader_modules.push(KeyShaderModule{key,shader_module});
            
        }

        self.key_lists = key_lists;

        
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bindgroup_layout],
            push_constant_ranges: &[],
        });

        self.pipeline_init = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            module: &shader_module_init,
            entry_point: "main",
            compilation_options: Default::default(),
        });

        //this vec contain all filter's piplines
        let mut pipelines = HashMap::new();

        for key_shader_module in &shader_modules{
            let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                module: &key_shader_module.shader_module,
                entry_point: "main",
                compilation_options: Default::default(),
            });

            pipelines.insert(key_shader_module.key.clone(), pipeline);
        }

        self.pipelines = pipelines;

        self.bindgroup_odd = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bindgroup_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.status_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(output_tx_view),
                },
                
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureViewArray(input_tx_views_b),
                }, 
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: self.intermediate_buffer_1.as_entire_binding(),
                },

                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: self.intermediate_buffer_2.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: self.filterinfo_buffer.as_entire_binding(),
                },
    
            ],
        });

        self.bindgroup_even = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bindgroup_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.status_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(output_tx_view),
                },
                
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureViewArray(input_tx_views_b),
                }, 
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: self.intermediate_buffer_2.as_entire_binding(),
                },

                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: self.intermediate_buffer_1.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: self.filterinfo_buffer.as_entire_binding(),
                },
    
            ],
        });

        
    }
}

pub fn input_tx_views_factory (
    device : &wgpu::Device,
    queue : &wgpu::Queue,
    status : Status,
) -> Vec<wgpu::TextureView> {

    let mut input_tx_views = Vec::new();

    for i in (status.source.from)..(status.source.to + 1){

        let file_name = format!("{}{}_00{}{}.{}",status.source.dir,status.source.filename,{
            if i < 10 {"00"}
            else if i < 100 {"0"}
            else {""}
        },i,status.source.extension);

        print!("\r\x1B[K");
        print!("reading {}",file_name);

        //prepare for input picture
        let missing = image::open("./assets/missing.png").unwrap();
        let diffuse_image = {
            match image::open(file_name){
                Ok(img) => {img}
                _ => {
                    missing
                }
            }
        };

        let diffuse_rgba = diffuse_image.to_rgba8();
    
        use image::GenericImageView;
        let dimensions = diffuse_image.dimensions();
    
        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
    
        //create input texture for wgpu
        let input_texture = device.create_texture(
            &wgpu::TextureDescriptor {
                size: texture_size,
                mip_level_count: 1, // We'll talk about this a little later
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label: Some("input_texture"),
                view_formats: &vec![],
            }
        );

        //read picture to texture
        queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::ImageCopyTexture {
                texture: &input_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // The actual pixel data
            &diffuse_rgba,
            // The layout of the texture
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            texture_size,
        );
    
        let input_texture_view = input_texture.create_view(&wgpu::TextureViewDescriptor::default());

        input_tx_views.push(input_texture_view);

    }


    input_tx_views


}

pub fn output_tx_view_factory(
    device : &wgpu::Device,
    status : Status,
) -> wgpu::TextureView{
    let output_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size: Extent3d {
            width: status.mov_width,
            height: status.mov_height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &vec![]
    });
    output_texture.create_view(&Default::default())
}
