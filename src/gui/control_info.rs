use crate::status::Status;

pub fn gui_info (ui: &mut egui::Ui, status : & Status) {

    ui.label(format!("FPS : {}",{
        match status.actual_fps{
            Some(fps) => {fps.to_string()}
            None => {"NaN".to_string()}
        }
    }));

    ui.label(format!("elapsed frames : {}",status.elapsed_frame));

    ui.label(format!("frame number : {}",status.next_frame_index));

    ui.label(format!("width : {}",status.mov_width));

    ui.label(format!("height : {}",status.mov_height));
}