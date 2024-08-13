use std::f32::consts::PI;

use serde::{Serialize, Deserialize};

use strum_macros::EnumIter;

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Fluctus {
    pub figura : Figura,
    pub cycle : f32,
    pub phase : f32,
}

impl Fluctus{
    pub fn new() -> Fluctus{
        Fluctus{
            figura : Figura::Constant,
            cycle : 1.,
            phase : 0.,
        }
    }

    pub fn set_fluctus(&self, input : f32, time : f32) -> f32{

        match self.figura{
            Figura::Constant => {
                input
            }
            Figura::Sin => {
                input * ((time / self.cycle + self.phase) * 2. * PI ).sin()
            }
            Figura::Square => {
                let rem = (time / self.cycle + self.phase).rem_euclid(1.);
                if rem > 0.5{
                    input
                }
                else {
                    0.
                }
            }
            Figura::Triangle => {
                let rem = (time / self.cycle + self.phase).rem_euclid(1.);
                
                input * rem
            }
            Figura::Sinln => {
                let phase = time / self.cycle + self.phase;

                input * (1. - ((phase * 2. * PI).sin() + 1. + 0.00001).ln().sin()) * 0.5
            }
            Figura::Delta => {
                let rem = (time / self.cycle + self.phase).rem_euclid(1.);

                input * ((rem - 0.5).powf(2.) * -200. * self.cycle.powf(2.)).exp() 
            }
            Figura::InverseTri => {
                let rem = (time / self.cycle + self.phase).rem_euclid(1.);
                
                input * (1. - rem)
            }
            Figura::Trifrom1 => {
                let rem = (time / self.cycle + self.phase).rem_euclid(1.);
                
                input * rem + 1. * (1. - rem)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumIter)]
pub enum Figura {
    Constant,
    Sin,
    Square,
    Triangle,
    Sinln,
    Delta,
    InverseTri,
    Trifrom1,
}

impl Figura{
    pub fn display(self) -> String {
        let out_str ;
        match self{
            Figura::Constant => {
                out_str = "const";
            }
            Figura::Sin => {
                out_str = " sin "
            }
            Figura::Square => {
                out_str = "squar"
            }
            Figura::Triangle => {
                out_str = "trian"
            }
            Figura::Sinln => {
                out_str = "sinln"
            }
            Figura::Delta => {
                out_str = "delta"
            }
            Figura::InverseTri => {
                out_str = "invtr"
            }
            Figura::Trifrom1 => {
                out_str = "tri 1"
            }
        };
        
        out_str.to_string()
    }

}