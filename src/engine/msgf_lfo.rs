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
use crate::general;
use crate::general::msgf_cfrm;

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

//---------------------------------------------------------
//		Synth. Parameter
//---------------------------------------------------------
pub struct LfoParameter {
    freq: f32,
    depth: f32,
    wave: LfoWave,
    direction: LfoDirection,
}
//  Voice Parameter
const LFO_PRM: LfoParameter = LfoParameter {
    freq: 5.0,
    depth: 0.02,
    wave: LfoWave::Tri,
    direction: LfoDirection::LfoBoth,
};
//---------------------------------------------------------
pub struct Lfo {
    depth: f32,
    next_phase: f32,
    delta_phase: f32,
    direction: LfoDirection,
    x1: f32,
    x2: f32,
    y: f32,
    z: f32,
    dac_counter: u64,
}

impl Lfo {
    pub fn new() -> Lfo {
        let coef = Lfo::set_lfo(LFO_PRM.wave, LFO_PRM.direction);
        Lfo {
            depth: LFO_PRM.depth,
            next_phase: 0.0,
            delta_phase: (LFO_PRM.freq*(general::AUDIO_FRAME_PER_CONTROL as f32))/general::SAMPLING_FREQ,
            direction: LFO_PRM.direction,
            x1: coef.0,
            x2: coef.1,
            y: coef.2,
            z: coef.3,
            dac_counter: 0,
        }
    }
    fn set_lfo(wv: LfoWave, _dir: LfoDirection) -> (f32, f32, f32, f32) {
        let (x1, x2, y, z): (f32, f32, f32, f32);
        match wv {
            LfoWave::Tri => {x1=0.5; x2=1.5; y=4.0; z=0.0;}
            LfoWave::Saw => {x1=0.0; x2=2.0; y=2.0; z=0.0;}
            LfoWave::Squ => {x1=0.5; x2=1.5; y=100000.0; z=0.0;}
            LfoWave::Sin => {x1=0.5; x2=1.5; y=2.0*general::PI; z=1.0/6.78;}
        };
        (x1, x2, y, z)
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
            //if ( _dacCounter < _fadeInDacCount ) lvl = 0;
            //else if ( _dacCounter < _fadeInDacCount+_delayDacCount ){
            //    lvl = (_dacCounter-_fadeInDacCount)/_delayDacCount;
            //}
        
            //	Direction
            if self.direction == LfoDirection::LfoUpper {
                lvl /= 2.0;
                ofs = lvl/2.0;
            } else if self.direction == LfoDirection::LfoLower {
                lvl /= 2.0;
                ofs = -lvl/2.0;
            }
            value = value*lvl + ofs;
    
            abuf.set_cbuf(i, value*self.depth);
            self.dac_counter += 1;
        }
        while phase > 2.0*general::PI {
            phase -= 2.0*general::PI;
        }
        self.next_phase = phase;
    }
}
