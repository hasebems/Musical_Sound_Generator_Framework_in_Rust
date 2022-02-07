//
//  msgf_lfo.rs
//	Musical Sound Generator Framework
//      LFO Class
//
//  Created by Hasebe Masahiko on 2021/10/17.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::msgf_if;
use crate::core::*;

//---------------------------------------------------------
//		Synth. Parameter
//---------------------------------------------------------
#[derive(PartialEq, Clone, Copy)]
#[allow(dead_code)]
pub enum LfoDirection {
    LfoBoth,
    LfoUpper,
    LfoLower,
}
#[derive(PartialEq, Clone, Copy)]
#[allow(dead_code)]
pub enum LfoWave {
    Tri,
    Saw,
    Squ,
    Sin,
}
#[derive(Copy, Clone)]
pub struct LfoParameter {
    pub freq: f32,              // RTP, prm#0
    pub wave: LfoWave,          // NKP, prm#1:bit 7-6
    pub direction: LfoDirection,// NKP, prm#1:bit 2-0
    pub fadein_time: u64,       // NKP, prm#2
    pub delay_time: u64,        // NKP, prm#3
}
//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub struct Lfo {
    fadein_time: u64,
    delay_time: u64,
    next_phase: f32,
    delta_phase: f32,
    direction: LfoDirection,
    x1: f32,
    x2: f32,
    y: f32,
    z: f32,
    dac_counter: u64,
}
//---------------------------------------------------------
//		Imprements
//---------------------------------------------------------
impl Lfo {
    pub fn new(ref_prms: &LfoParameter) -> Lfo {
        let coef = Lfo::calc_wave(ref_prms.wave, ref_prms.direction);
        Lfo {
            fadein_time: ref_prms.fadein_time,
            delay_time: ref_prms.delay_time,
            next_phase: 0.0,
            delta_phase: Lfo::calc_freq(ref_prms.freq),
            direction: coef.4,
            x1: coef.0,
            x2: coef.1,
            y: coef.2,
            z: coef.3,
            dac_counter: 0,
        }
    }
    fn calc_wave(wv: LfoWave, dir: LfoDirection) -> (f32, f32, f32, f32, LfoDirection) {
        let (x1, x2, y, z): (f32, f32, f32, f32);
        match wv {
            LfoWave::Tri => {x1=0.5; x2=1.5; y=4.0; z=0.0;}
            LfoWave::Saw => {x1=0.0; x2=2.0; y=2.0; z=0.0;}
            LfoWave::Squ => {x1=0.5; x2=1.5; y=100000.0; z=0.0;}
            LfoWave::Sin => {x1=0.5; x2=1.5; y=2.0*msgf_if::PI; z=1.0/6.78;}
        };
        (x1, x2, y, z, dir)
    }
    fn calc_freq(freq: f32) -> f32 {
        freq*(msgf_if::AUDIO_FRAME_PER_CONTROL as f32)/msgf_if::SAMPLING_FREQ
    }
    pub fn set_wave(&mut self, value: u8) {
        let dir_num: u8 = value&0x30;
        let dir: LfoDirection;
        match dir_num {
            0 => dir = LfoDirection::LfoBoth,
            1 => dir = LfoDirection::LfoUpper,
            2 => dir = LfoDirection::LfoLower,
            _ => dir = LfoDirection::LfoBoth,
        }
        let wv_num: u8 = value&0x60;
        let wv: LfoWave;
        match wv_num {
            0x00 => wv = LfoWave::Tri,
            0x20 => wv = LfoWave::Saw,
            0x40 => wv = LfoWave::Squ,
            0x60 => wv = LfoWave::Sin,
            _ => wv = LfoWave::Sin,
        }
        let coef = Lfo::calc_wave(wv, dir);
        self.direction = coef.4;
        self.x1 = coef.0;
        self.x2 = coef.1;
        self.y =  coef.2;
        self.z = coef.3;
    }
    pub fn set_freq(&mut self, value: u8) {
        self.delta_phase = Lfo::calc_freq((value as f32)/10.0);
    }
    pub fn start(&mut self) {
        self.dac_counter = 0;
    }
    pub fn process(&mut self, abuf: &mut msgf_cfrm::CtrlFrame) {
        let mut phase = self.next_phase;
        for i in 0..abuf.sample_number {
            let mut value = phase;
            if value < self.x1-phase {
                value = self.x1-phase;
            }
            if value > self.x2-phase {
                value = self.x2-phase;
            }
            value -= 0.5;
            value *= self.y;
            value = value - value*value*value*self.z;
            
            phase += self.delta_phase;
            if phase >= 1.0 {
                phase -= 1.0;
            }
            //	Limit
            if value > 1.0 {
                value = 1.0;
            } else if value < -1.0 {
                value = -1.0;
            }
            //	Fadein, Delay
            let mut lvl = 1.0;
            let mut ofs = 0.0;
            if self.dac_counter < self.fadein_time {
                lvl = 0.0;
            } else if self.dac_counter < self.fadein_time+self.delay_time {
                let tm = (self.dac_counter-self.fadein_time) as f32;
                lvl = tm/(self.delay_time as f32);
            }
        
            //	Direction
            if self.direction == LfoDirection::LfoUpper {
                lvl /= 2.0;
                ofs = lvl/2.0;
            } else if self.direction == LfoDirection::LfoLower {
                lvl /= 2.0;
                ofs = -lvl/2.0;
            }
            value = value*lvl + ofs;
    
            abuf.set_cbuf(i, value);
            self.dac_counter += 1;
        }
        while phase > 2.0*msgf_if::PI {
            phase -= 2.0*msgf_if::PI;
        }
        self.next_phase = phase;
    }
}
