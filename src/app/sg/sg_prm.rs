//
//  sg_prm.rs
//	Musical Sound Generator Framework
//      Instruments Parameter
//
//  Created by Hasebe Masahiko on 2022/03/15.
//  Copyright (c) 2022 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::engine::msgf_additive::*;
use crate::engine::msgf_aeg::*;
use crate::engine::msgf_lfo::*;

#[derive(Copy, Clone)]
pub struct SynthParameter {
    pub osc: AdditiveParameter,
    pub aeg: AegParameter,
    pub lfo: LfoParameter,
}

pub const SG_MAX_TONE_COUNT:usize = 1;
pub const SG_TONE_PRM: [SynthParameter; SG_MAX_TONE_COUNT] = [
    SynthParameter {
        osc: AdditiveParameter {
            coarse_tune: 0,     //  i32 : 0 means tuning of A=440[Hz]
            fine_tune: 0.0,     //  f32 : 1.0 means 1[cent]
        },
        aeg: AegParameter {
            attack_rate: 0.6,   //  0.0-1.0
            decay_rate: 0.05,   //  0.0-1.0 : 1.0 means no decay and no sustain level
            sustain_level: 0.5, //  1 means same value as Attack Level
            release_rate: 0.1, //  0.0-1.0
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