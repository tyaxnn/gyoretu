use crate::status::SourceId;
use num::Num;

use serde::{Serialize, Deserialize};

use crate::fluctus::Fluctus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerInfos{
    pub types : Vec<LayerType>,
    pub id_last : LayerId
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayerType {
    Source(SourceInfo),
    Filter(FilterInfo),

}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LayerId{
    pub num : usize,
}

impl LayerId{
    pub fn new(num : usize) -> LayerId{
        LayerId{num}
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceInfo {
    pub id : LayerId,
    pub active : bool,
    pub source_id : SourceId,
    pub offset : u32,
    pub len : u32,
    pub alpha : f32,
}

impl SourceInfo {
    pub fn new(id : LayerId, source_id : SourceId, len : u32) -> SourceInfo{
        SourceInfo{
            id,
            active : true,
            source_id,
            offset : 0,
            len,
            alpha : 1.,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterInfo {
    pub key : String,
    pub parameter : [u32; 20],
    pub label : Vec<Pinfo>,
    pub id : LayerId,
    pub active : bool,
}


impl FilterInfo {
    pub fn new(key_str : &str, id : LayerId) -> FilterInfo{
        let mut init = [(0f32).to_bits();20];
        let active = true;

        let ran_f_ini = Range::new(0f32,1f32);
        let _ran_i_ini = Range::new(0,256);

        match key_str{
            "monochrome" => {
                init[0] = (1f32).to_bits();
                init[1] = (0.5f32).to_bits();
                init[2] = (0.5f32).to_bits();
                init[3] = (0.5f32).to_bits();
                FilterInfo{key : key_str.to_string(), parameter : init, label : vec![Pinfo::new(Ptype::Float(ran_f_ini),"strongness"),Pinfo::new(Ptype::Color3,"input_rgb")],id,active}
            },
            "flip_x" => {
                FilterInfo{key : key_str.to_string(), parameter : init, label : vec![],id, active }
            },
            "flip_y" => {
                FilterInfo{key : key_str.to_string(), parameter : init, label : vec![],id, active}
            },
            "threshold" => {
                init[0] = (0.5f32).to_bits();
                init[1] = (1f32).to_bits();
                init[2] = (1f32).to_bits();
                init[3] = (1f32).to_bits();
                init[3] = (1f32).to_bits();
                init[4] = (0f32).to_bits();
                init[5] = (0f32).to_bits();
                init[6] = (0f32).to_bits();
                init[6] = (0f32).to_bits();
                FilterInfo{key : key_str.to_string(), parameter : init, label : vec![Pinfo::new(Ptype::Float(ran_f_ini),"threshold"),Pinfo::new(Ptype::Color4,"color1"),Pinfo::new(Ptype::Color4,"color2")],id, active}
            },
            "bayer_16" => {
                init[0] = (1f32).to_bits();
                init[1] = (1f32).to_bits();
                init[2] = (1f32).to_bits();
                init[3] = (1f32).to_bits();
                init[4] = (0f32).to_bits();
                init[5] = (0f32).to_bits();
                init[6] = (0f32).to_bits();
                init[7] = (0f32).to_bits();
                FilterInfo{key : key_str.to_string(), parameter : init, label : vec![Pinfo::new(Ptype::Color4,"color1"),Pinfo::new(Ptype::Color4,"color2")],id, active}
            },
            "bg" => {
                init[0] = (0f32).to_bits();
                init[1] = (0f32).to_bits();
                init[2] = (0f32).to_bits();
                init[3] = (0.3f32).to_bits();
                FilterInfo{key : key_str.to_string(), parameter : init, label : vec![Pinfo::new(Ptype::Color4,"color1")],id, active }
            }
            "polar_coordinate" => {
                FilterInfo{key : key_str.to_string(), parameter : init, label : vec![Pinfo::new(Ptype::Float(ran_f_ini),"radius max"),Pinfo::new(Ptype::Float(ran_f_ini),"theta_offset")],id, active }
            }
            "peter_de_jong" => {
                init[0] = (1.641f32).to_bits();
                init[1] = (1.902f32).to_bits();
                init[2] = (0.316f32).to_bits();
                init[3] = (1.525f32).to_bits();
                init[4] = (1f32).to_bits();
                init[5] = (1f32).to_bits();
                init[6] = (0.4f32).to_bits();
                FilterInfo{key : key_str.to_string(), parameter : init, label : vec![
                    Pinfo::new(Ptype::Float(Range::new(-10f32,10f32)),"a"),
                    Pinfo::new(Ptype::Float(Range::new(-10f32,10f32)),"b"),
                    Pinfo::new(Ptype::Float(Range::new(-10f32,10f32)),"c"),
                    Pinfo::new(Ptype::Float(Range::new(-10f32,10f32)),"d"),
                    Pinfo::new(Ptype::Float(ran_f_ini),"mul_x"),
                    Pinfo::new(Ptype::Float(ran_f_ini),"mul_y"),
                    Pinfo::new(Ptype::Float(ran_f_ini),"clear strength"),
                ],id, active }
            }
            "noise_input" => {
                FilterInfo{key : key_str.to_string(), parameter : init, label : vec![Pinfo::new(Ptype::Float(ran_f_ini),"density")],id, active }
            }
            "seed" => {
                FilterInfo{key : key_str.to_string(), parameter : init, label : vec![Pinfo::new(Ptype::Integer(Range::new(0,1919)),"seed_w"),Pinfo::new(Ptype::Integer(Range::new(0,1079)),"seed_h")],id, active }
            }
            "displacement_map" => {
                FilterInfo{key : key_str.to_string(), parameter : init, label : vec![
                    Pinfo::new(Ptype::Float(Range::new(-5000f32,5000f32)),"displacement x"),
                    Pinfo::new(Ptype::Float(Range::new(-5000f32,5000f32)),"displacement y"),
                ],id, active }
            }
            "clear_old_buffer" => {
                FilterInfo{key : key_str.to_string(), parameter : init, label : vec![],id, active }
            }
            "mosaïque" => {
                FilterInfo{key : key_str.to_string(), parameter : init, label : vec![
                    Pinfo::new(Ptype::Integer(Range::new(1,1920)),"cell width"),
                    Pinfo::new(Ptype::Integer(Range::new(1,1080)),"cell heigght"),
                ],id, active }
            }
            _ => {panic!("{} doesn't exist",key_str )}
        }
    }

    pub fn set_fluctus(&self, time : f32) -> [u32; 20]{
        let mut out_parameter = self.parameter;
        let mut count = 0;
        for pinfo in &self.label{
            

            match pinfo.ptype{
                Ptype::Float(_) => {
                    let num = f32::from_bits(self.parameter[count]);
                    out_parameter[count] = pinfo.fluctus.set_fluctus(num, time).to_bits();
                    count += 1;
                }
                Ptype::Integer(_) => {
                    count += 1;
                }
                Ptype::Color3 => {
                    count += 3;
                }
                Ptype::Color4 => {
                    count += 4;
                }
            }
        }

        out_parameter
    }
}

#[derive(Debug,Clone, Serialize, Deserialize)]
pub enum Ptype {
    Integer(Range<u32>),
    Float(Range<f32>),
    Color3,
    Color4,
}

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct Pinfo {
    pub ptype : Ptype,
    pub plabel : String,
    pub fluctus : Fluctus,
}


impl Pinfo{
    fn new(ptype : Ptype, label : &str) -> Pinfo {
        Pinfo { ptype, plabel : label.to_string(), fluctus : Fluctus::new() }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Range<N : Num> {
    pub from : N,
    pub to : N,
}

impl<N : Num> Range<N> {
    pub fn new(from : N, to : N) -> Range<N> {
        Range{
            from,
            to,
        }
    }
}
