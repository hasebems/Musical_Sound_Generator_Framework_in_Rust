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
use crate::general::msgf_afrm;

//---------------------------------------------------------
//		Synth. Parameter
//---------------------------------------------------------
pub struct LfoParameter {
    freq: f32,
    depth: f32,
}
//  Voice Parameter
const LFO_PRM: LfoParameter = LfoParameter {
    freq: 5.0,
    depth: 0.1,
};
//---------------------------------------------------------
pub struct Lfo {
    freq: f32,
    depth: f32,
    next_phase: f32,
}

impl Lfo {
    pub fn new() -> Lfo {
        Lfo {
            freq: LFO_PRM.freq,
            depth: LFO_PRM.depth,
            next_phase: 0.0,
        }
    }
    pub fn process(&mut self, abuf: &mut msgf_afrm::AudioFrame) {
        let delta_phase = (2.0 * general::PI * self.freq)/general::SAMPLING_FREQ;
        let mut phase = self.next_phase;
        for i in 0..abuf.sample_number {
            abuf.set_abuf(i, phase.sin()*self.depth);
            phase += delta_phase;
        }
        while phase > 2.0*general::PI {
            phase -= 2.0*general::PI;
        }
        self.next_phase = phase;
    }
}
