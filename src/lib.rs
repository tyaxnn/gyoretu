use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{WindowBuilder,Icon},
    dpi::PhysicalSize,
};

mod model;
mod compute;
mod render;
mod status;
mod filters;
mod gui;
mod write;

use model::Model;
use status::Status;

const WIDTH : i32 = 1920;
const HEIGHT: i32 = 1620;


pub async fn run() {

    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(WIDTH, HEIGHT))
        .with_title("gyoretu")
        .with_window_icon(Some(Icon::from_rgba({
            let icon_bytes = include_bytes!("../assets/icons/icon_d.png");
            let icon_image = image::load_from_memory(icon_bytes).unwrap();
            let diffuse_rgba = icon_image.to_rgba8().into_raw();

            diffuse_rgba
        }, 436, 436).unwrap()))
        .build(&event_loop)
        .unwrap();




    window.set_max_inner_size(Some(PhysicalSize::new(WIDTH as f32, HEIGHT as f32)));
    window.set_min_inner_size(Some(PhysicalSize::new(WIDTH as f32, HEIGHT as f32)));

    let new_status = Status::new();

    // Model::new uses async code, so we're going to wait for it to finish
    let mut model = Model::new(&window, new_status).await;
    let mut surface_configured = false;

    

    event_loop
        .run(move |event, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == model.window().id() => {
                    if !model.input(event) {
                        // UPDATED!
                        match event {
                            WindowEvent::CloseRequested
                            | WindowEvent::KeyboardInput {
                                event:
                                    KeyEvent {
                                        state: ElementState::Pressed,
                                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                                        ..
                                    },
                                ..
                            } => control_flow.exit(),
                            WindowEvent::Resized(physical_size) => {
                                log::info!("physical_size: {physical_size:?}");
                                surface_configured = true;
                                model.resize(*physical_size);
                            }
                            WindowEvent::RedrawRequested => {
                                // This tells winit that we want another frame after this one
                                model.window().request_redraw();

                                if !surface_configured {
                                    return;
                                }

                                model.update_pre();

                                match model.render() {
                                    Ok(_) => {}
                                    // Reconfigure the surface if it's lost or outdated
                                    Err(
                                        wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,
                                    ) => model.resize(model.pv.size),
                                    // The system is out of memory, we should probably quit
                                    Err(wgpu::SurfaceError::OutOfMemory) => {
                                        log::error!("OutOfMemory");
                                        control_flow.exit();
                                    }

                                    // This happens when the a frame takes too long to present
                                    Err(wgpu::SurfaceError::Timeout) => {
                                        log::warn!("Surface timeout")
                                    }
                                }

                                model.update_post();
                                model.write()
                            }
                            _ => {}
                        
                        }

                        model.egui.handle_input(&mut model.window, &event);
                    }
                }
                _ => {}
            }
        })
        .unwrap();


}