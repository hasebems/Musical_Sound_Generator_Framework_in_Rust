//
//  va_prm.rs
//	Musical Sound Generator Framework
//      Instruments Parameter
//
//  Created by Hasebe Masahiko on 2021/10/25.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::engine::msgf_osc::*;
use crate::engine::msgf_aeg::*;
use crate::engine::msgf_lfo::*;

pub struct SynthParameter {
    pub osc: OscParameter,
    pub aeg: AegParameter,
    pub lfo: LfoParameter,
}

pub const MAX_TONE_COUNT:usize = 3;
pub const TONE_PRM: [SynthParameter; MAX_TONE_COUNT] = [
    SynthParameter {
        osc: OscParameter {
            coarse_tune: 0,     //  i32 : 0 means tuning of A=440[Hz]
            fine_tune: 0.0,     //  f32 : 1.0 means 1[cent]
            lfo_depth: 0.02,    //  f32 : 1.0 means +-1oct.
            wv_type: WvType::Sine,
        },
        aeg: AegParameter {
            attack_rate: 0.5,   //  0.0-1.0
            decay_rate: 0.01,   //  0.0-1.0 : 1.0 means no decay and no sustain level
            sustain_level: 0.1, //  1 means same value as Attack Level
            release_rate: 0.01, //  0.0-1.0
        },
        lfo: LfoParameter {
            freq: 5.0,          //  [Hz]
            wave: LfoWave::Tri,
            direction: LfoDirection::LfoBoth,
            fadein_time: 0,     //  1: AUDIO_FRAME_PER_CONTROL / SAMPLING_FREQ (=3msec)
            delay_time: 0,      //    same as above
        },
    },
    SynthParameter {
        osc: OscParameter {
            coarse_tune: 0,
            fine_tune: 0.0,
            lfo_depth: 0.0,
            wv_type: WvType::Saw,
        },
        aeg: AegParameter {
            attack_rate: 0.9,
            decay_rate: 1.0,
            sustain_level: 1.0,
            release_rate: 0.2,
        },
        lfo: LfoParameter {
            freq: 5.0,
            wave: LfoWave::Tri,
            direction: LfoDirection::LfoBoth,
            fadein_time: 0,
            delay_time: 0,
        },
    },
    SynthParameter {
        osc: OscParameter {
            coarse_tune: 0,
            fine_tune: 0.0,
            lfo_depth: 0.02,
            wv_type: WvType::Square,
        },
        aeg: AegParameter {
            attack_rate: 0.5,
            decay_rate: 0.01,
            sustain_level: 0.5,
            release_rate: 0.1,
        },
        lfo: LfoParameter {
            freq: 5.0,
            wave: LfoWave::Tri,
            direction: LfoDirection::LfoBoth,
            fadein_time: 300,
            delay_time: 600,
        },
    },
];