use crate::gui::WindowShowStatus;

#[derive(Debug, Copy, Clone)]
pub enum PinPongStatus{
    FtT2,
    F1T2,
    F2T1,
}

pub const GEN_BUFFER_SIZE : u64 = 20;
pub const FIL_BUFFER_SIZE : u64 = 80;


//様々な用途の物が存在しているので、整理する必要がある。
#[derive(Debug, Clone)]
pub struct Status{
    //userが変更することを想定している
    pub frame_len_max : u32,
    pub frame_rate : u32,
    pub source : Source,
    pub win_show_status : WindowShowStatus,
    //userは変更しないが、内部情報として変化する。出力する必要がある
    pub elapsed_frame : u32,
    pub mov_width : u32,
    pub mov_height : u32,
    //userは変更しないが、内部情報として変化する。出力する必要はない。
    pub next_frame_index : u32,
    pub start_time : std::time::Instant,
    pub ping_pong : PinPongStatus,
}

#[derive(Debug, Clone)]
pub struct Source{
    pub dir : String,
    pub filename : String,
    pub digit : u32,
    pub from : u32,
    pub to : u32,
    pub extension : String,
}

impl Source{
    pub fn frame_len(&self) -> u32 {
        self.to + 1 +  self.from 
    }
}

impl Status {
    pub fn new() -> Status{

        let frame_rate = 20;
        let elapsed_frame = 0;
        let next_frame_index = 0; 
        let start_time = std::time::Instant::now();
        let ping_pong = PinPongStatus::FtT2;
        let (mov_width,mov_height) = (1920,1080);
        let win_show_status = WindowShowStatus::Source;
        let source = Source{
            dir : "./assets/dendrite/".to_string(),
            filename : "dendrite".to_string(),
            digit : 5,
            from : 0,
            to : 100,
            extension : "png".to_string(),
        };

        let frame_len_max = source.frame_len();

        Status{
            //buffer_size,
            frame_len_max,
            frame_rate,
            elapsed_frame,
            next_frame_index,
            start_time,
            ping_pong,
            //filterinfo_size,
            mov_width,
            mov_height,
            win_show_status,
            source,
        }
    } 
}
