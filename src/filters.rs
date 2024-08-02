#[derive(Debug, Clone)]

pub enum LayerType {
    Source(SourceInfo),
    Filter(FilterInfo),
    Bg(BgInfo)
}

#[derive(Debug,Clone)]
pub struct SourceInfo {
    pub id : usize,
    pub active : bool,
}

#[derive(Debug,Clone)]
pub struct FilterInfo {
    pub key : String,
    pub parameter : [u32; 20],
    pub label : Vec<Pinfo>,
    pub id : usize,
    pub active : bool,
}


impl FilterInfo{
    pub fn new(key_str : &str, id : usize) -> FilterInfo{
        let mut init = [(0f32).to_bits();20];
        let active = true;

        match key_str{
            "monochrome" => {
                init[0] = (1f32).to_bits();
                init[1] = (0.5f32).to_bits();
                init[2] = (0.5f32).to_bits();
                init[3] = (0.5f32).to_bits();
                FilterInfo{key : key_str.to_string(), parameter : init, label : vec![Pinfo::new(Ptype::Float,"strongness"),Pinfo::new(Ptype::Color3,"input_rgb")],id,active}
            },
            "flip_x" => {
                FilterInfo{key : key_str.to_string(), parameter : init, label : vec![],id, active }
            },
            "flip_y" => {
                FilterInfo{key : key_str.to_string(), parameter : init, label : vec![],id, active}
            },
            "threshold" => {
                init[0] = (0.5f32).to_bits();
                init[1] = (0f32).to_bits();
                init[2] = (0f32).to_bits();
                init[3] = (0f32).to_bits();
                init[4] = (1f32).to_bits();
                init[5] = (1f32).to_bits();
                init[6] = (1f32).to_bits();
                FilterInfo{key : key_str.to_string(), parameter : init, label : vec![Pinfo::new(Ptype::Integer,"threshold"),Pinfo::new(Ptype::Color3,"color1"),Pinfo::new(Ptype::Color3,"color2")],id, active}
            },
            "bayer_16" => {
                init[0] = (0f32).to_bits();
                init[1] = (0f32).to_bits();
                init[2] = (0f32).to_bits();
                init[3] = (1f32).to_bits();
                init[4] = (1f32).to_bits();
                init[5] = (1f32).to_bits();
                FilterInfo{key : key_str.to_string(), parameter : init, label : vec![Pinfo::new(Ptype::Color3,"color1"),Pinfo::new(Ptype::Color3,"color2")],id, active}
            },
            _ => {panic!("{} doesn't exist",key_str )}
        }
    }
}

#[derive(Debug,Clone)]
pub struct BgInfo {
    pub parameter : [u32; 20],
    pub id : usize,
    pub active : bool,
}

impl BgInfo{
    pub fn new(id : usize) -> BgInfo{
        let parameter = [(1f32).to_bits();20];
        let active = true;

        BgInfo{
            parameter,
            id, 
            active,
        }
    }
}

#[derive(Debug,Clone)]
pub enum Ptype {
    Integer,
    Float,
    Color3,
}

#[derive(Debug,Clone)]
pub struct Pinfo {
    pub ptype : Ptype,
    pub plabel : String,
}


impl Pinfo{
    fn new(ptype : Ptype, label : &str) -> Pinfo {
        Pinfo { ptype, plabel : label.to_string() }
    }
}

