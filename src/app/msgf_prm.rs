//
//  msgf_prm.rs
//	Musical Sound Generator Framework
//      Instruments Parameter
//
//  Created by Hasebe Masahiko on 2021/10/25.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::engine::msgf_osc::OscParameter;
use crate::engine::msgf_osc::WvType;
use crate::engine::msgf_aeg::AegParameter;
use crate::engine::msgf_lfo::LfoParameter;
use crate::engine::msgf_lfo::LfoWave;
use crate::engine::msgf_lfo::LfoDirection;

pub struct SynthParameter {
    pub osc: OscParameter,
    pub aeg: AegParameter,
    pub lfo: LfoParameter,
}

pub const MAX_TONE_COUNT:usize = 3;
pub const TONE_PRM: [SynthParameter; MAX_TONE_COUNT] = [
SynthParameter {
    osc: OscParameter {
        coarse_tune: 0,     //  i32
        fine_tune: 0.0,   //  f32
        wv_type: WvType::Sine,
    },
    aeg: AegParameter {
        attack_rate: 0.5,  //  0.0-1.0
        decay_rate: 0.01, //  0.0-1.0 : 1.0 means no decay and no sustain level
        sustain_level: 0.1, //  1 means same value as Attack Level
        release_rate: 0.01, //  0.0-1.0
    },
    lfo: LfoParameter {
        freq: 5.0,
        depth: 0.02,
        wave: LfoWave::Tri,
        direction: LfoDirection::LfoBoth,
    },
},
SynthParameter {
    osc: OscParameter {
        coarse_tune: 0,     //  i32
        fine_tune: 0.0,   //  f32
        wv_type: WvType::Saw,
    },
    aeg: AegParameter {
        attack_rate: 0.9,  //  0.0-1.0
        decay_rate: 1.0, //  0.0-1.0 : 1.0 means no decay and no sustain level
        sustain_level: 1.0, //  1 means same value as Attack Level
        release_rate: 0.2, //  0.0-1.0
    },
    lfo: LfoParameter {
        freq: 5.0,
        depth: 0.0,
        wave: LfoWave::Tri,
        direction: LfoDirection::LfoBoth,
    },
},
SynthParameter {
    osc: OscParameter {
        coarse_tune: 0,     //  i32
        fine_tune: 0.0,   //  f32
        wv_type: WvType::Square,
    },
    aeg: AegParameter {
        attack_rate: 0.5,  //  0.0-1.0
        decay_rate: 0.01, //  0.0-1.0 : 1.0 means no decay and no sustain level
        sustain_level: 0.5, //  1 means same value as Attack Level
        release_rate: 0.1, //  0.0-1.0
    },
    lfo: LfoParameter {
        freq: 5.0,
        depth: 0.02,
        wave: LfoWave::Tri,
        direction: LfoDirection::LfoBoth,
    },
},
];