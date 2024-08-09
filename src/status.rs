use crate::gui::WindowShowStatus;
use crate::filters::{SourceInfo,LayerType,LayerId,LayerInfos};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum PinPongStatus{
    F1T2,
    F2T1,
}

pub const MAX_SAMPLED_TEXTURES_PER_SHADER_STAGE : u32 = 1024;

pub const GEN_BUFFER_SIZE : u64 = 20;
pub const FIL_BUFFER_SIZE : u64 = 80;
pub const FIRST_SOURCE_ID : usize = 1;


//様々な用途の物が存在しているので、整理する必要がある。
#[derive(Debug, Clone)]
pub struct Status{
    //userが変更することを想定している
    pub setting : Setting,
    pub source_infos : SourceInfos,
    pub layer_infos : LayerInfos,
    pub win_show_status : WindowShowStatus,
    pub mode : Mode,
    pub output_setting : OutputSetting,
    //userは変更しないが、内部情報として変化する。出力する必要がある
    pub elapsed_frame : u32,
    pub previous_elapsed_frame : u32,
    pub mov_width : u32,
    pub mov_height : u32,
    pub actual_fps : Option<f32>,
    //userは変更しないが、内部情報として変化する。出力する必要はない。
    pub next_frame_index : u32,
    pub one_before_frame_index : u32,
    pub start_time : std::time::Instant,
    pub previous_elapsed_time : f32,
    pub ping_pong : PinPongStatus,
    pub update_input : bool,
    pub offset_id_map : HashMap<SourceId, SourceIdentity>,
}

#[derive(Debug, Clone)]
pub struct OutputSetting{
    pub filename : String,
}

#[derive(Debug, Clone)]
pub enum Mode{
    Preview,
    Render(u32),
}

#[derive(Debug, Clone)]
pub struct SourceIdentity{
    pub offset_array_texture : u32,
    pub len : u32,
}

impl SourceIdentity{
    pub fn new(offset_array_texture : u32, len :u32) -> SourceIdentity{
        SourceIdentity{
            offset_array_texture,
            len,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Setting{
    pub frame_len : u32,
    pub frame_rate : u32,
    pub clear_intensity : f32,
}

#[derive(Debug, Clone)]
pub struct SourceInfos{
    pub sources : Vec<Source>,
    pub id_last : SourceId
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct SourceId{
    pub num : usize
}

impl SourceId{
    pub fn new(num : usize) -> SourceId{
        SourceId{num}
    }
}

#[derive(Debug, Clone)]
pub struct Source{
    pub dir : String,
    pub filename : String,
    pub digit : u32,
    pub from : u32,
    pub to : u32,
    pub extension : String,
    pub id : SourceId,
}

impl Source{
    pub fn new(id : SourceId) -> Source {
        Source{
            dir : "./assets/".to_string(),
            filename : "".to_string(),
            digit : 5,
            from : 0,
            to : 10,
            extension : "png".to_string(),
            id,
        }
    }
    pub fn frame_len(&self) -> u32 {
        if self.to < self.from{
            0
        }
        else{
            self.to + 1 - self.from 
        }
    }
}

pub fn sources_len(sources : &Vec<Source>) -> u32{

    let mut sum = 0;
    for source in sources{
        sum += source.frame_len()
    }

    sum
}

impl Status {
    pub fn new() -> Status{

        let frame_rate = 20;
        let elapsed_frame = 0;
        let previous_elapsed_frame = 0;
        let next_frame_index = 0;
        let one_before_frame_index = 0;
        let start_time = std::time::Instant::now();
        let previous_elapsed_time = 0f32;
        let ping_pong = PinPongStatus::F1T2;
        let (mov_width,mov_height) = (1920,1080);
        let actual_fps = Some(0f32);
        let win_show_status = WindowShowStatus::Source;

        let offset_id_map = HashMap::new();

        let source = Source{
            dir : "./assets/".to_string(),
            filename : "seed".to_string(),
            digit : 5,
            from : 0,
            to : 0,
            extension : "png".to_string(),
            id : SourceId::new(FIRST_SOURCE_ID),
        };

        let ini_source_layer = LayerType::Source(SourceInfo::new(LayerId::new(FIRST_SOURCE_ID), SourceId::new(FIRST_SOURCE_ID), source.frame_len()));

        let types = vec![ini_source_layer];

        let frame_len = source.frame_len();

        let setting = Setting{frame_len, frame_rate, clear_intensity : 1.};

        let sources = vec![source];
        let id_last = SourceId::new(FIRST_SOURCE_ID);

        let source_infos = SourceInfos{sources,id_last};

        let update_input = false;

        
        let id_last = LayerId{num : types.len()};

        let layer_infos = LayerInfos{
            types,
            id_last,
        };

        let mode = Mode::Preview;

        let output_setting = OutputSetting{filename : "image".to_string()};

        Status{
            setting,
            elapsed_frame,
            previous_elapsed_frame,
            next_frame_index,
            one_before_frame_index,
            start_time,
            previous_elapsed_time,
            ping_pong,
            mov_width,
            mov_height,
            actual_fps,
            win_show_status,
            source_infos,
            update_input,
            offset_id_map,
            layer_infos,
            mode,
            output_setting,
        }
    } 
}

