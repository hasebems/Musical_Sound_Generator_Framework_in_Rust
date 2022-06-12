//
//  sgf_prm.rs
//	Musical Sound Generator Framework
//      Instruments Parameter
//
//  Created by Hasebe Masahiko on 2022/06/11.
//  Copyright (c) 2022 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::engine::msgf_vocal::*;
use crate::engine::msgf_aeg::*;
use crate::engine::msgf_lfo::*;

#[derive(Copy, Clone)]
pub struct SynthParameter {
    pub osc: VocalParameter,
    pub aeg: AegParameter,
    pub lfo: LfoParameter,
}

pub const SGF_MAX_TONE_COUNT:usize = 1;
pub const SGF_TONE_PRM: [SynthParameter; SGF_MAX_TONE_COUNT] = [
    SynthParameter {
        osc: VocalParameter {
            coarse_tune: 0,     //  i32 : 0 means tuning of A=440[Hz]
            fine_tune: 0.0,     //  f32 : 1.0 means 1[cent]
            lfo_depth: 0.02,    //  f32 : 1.0 means +-1oct.
       },
        aeg: AegParameter {
            attack_rate: 0.6,   //  0.0-1.0
            decay_rate: 0.05,   //  0.0-1.0 : 1.0 means no decay and no sustain level
            sustain_level: 0.5, //  1 means same value as Attack Level
            release_rate: 0.02, //  0.0-1.0
        },
        lfo: LfoParameter {
            freq: 6.0,          //  [Hz]
            wave: LfoWave::Tri,
            direction: LfoDirection::LfoBoth,
            fadein_time: 100,   //  1: AUDIO_FRAME_PER_CONTROL / SAMPLING_FREQ (=3msec)
            delay_time: 100,    //    same as above
        },
    },
];