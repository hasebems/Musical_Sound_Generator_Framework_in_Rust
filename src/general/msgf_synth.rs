//
//  msgf_synth.rs
//	Musical Sound Generator Framework
//      Synth Class
//
//  Created by Hasebe Masahiko on 2021/09/26.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::general;
use crate::general::msgf_afrm;

//---------------------------------------------------------
//		Constants
//---------------------------------------------------------
#[derive(PartialEq, Clone, Copy)]
enum EgState {
    NotYet,
    Attack,
    _Decay,
    KeyOnSteady,
    Release,
    KeyOffSteady,
    _Damp,
}
const PITCH_OF_A: [f32; 11] =
[
//	-3     9     21    33     45     57     69     81      93      105     117 : note number
    13.75, 27.5, 55.0, 110.0, 220.0, 440.0, 880.0, 1760.0, 3520.0, 7040.0, 14080.0
];
const EG_ATTACK_TIME: f32 = 0.1;    //  [sec]
const EG_RELEASE_TIME: f32 = 0.1;   //  [sec]
//---------------------------------------------------------
//		Class
//---------------------------------------------------------
pub struct Synth {
    pitch: f32,
    crnt_phase: f32,
    eg_state: EgState,
    eg_tgt_value: f32,
    eg_src_value: f32,
    eg_crnt: f32,
    eg_time: f32,
    eg_dac_count: usize,
    max_note_vol: f32, 
}

impl Synth {
    pub fn new(note:u8) -> Synth {
        Synth {
            pitch: Synth::calc_pitch(note),
            crnt_phase: 0.0,
            eg_state: EgState::NotYet,
            eg_tgt_value: 0.0,
            eg_src_value: 0.0,
            eg_crnt: 0.0,
            eg_time: 0.0,
            eg_dac_count: 0,
            max_note_vol: 0.5f32.powf(4.0), // 4bit margin
        }
    }
    pub fn process(&mut self, abuf: &mut msgf_afrm::AudioFrame) {
        let this_time = self.periodic_once_every_process();
        let mut phase = self.crnt_phase;
        for i in 0..abuf.sample_number {
            let mut smpl: f32 = phase.sin();
            smpl *= self.calc_eg_level(this_time.1);
            smpl *= self.max_note_vol;    // Max Volume
            abuf.set_abuf(i, smpl);
            phase += this_time.0;
            self.eg_dac_count += 1;
        }
        self.crnt_phase = phase;
    }
    pub fn move_to_attack(&mut self) {
        self.eg_tgt_value = 1.0;
        self.eg_src_value = 0.0;
        self.eg_dac_count = 0;
        self.eg_time = EG_ATTACK_TIME*general::SAMPLING_FREQ;
        self.eg_state = EgState::Attack;
    }
    pub fn move_to_release(&mut self) {
        self.eg_tgt_value = 0.0;
        self.eg_src_value = self.eg_crnt;
        self.eg_dac_count = 0;
        self.eg_time = EG_RELEASE_TIME*general::SAMPLING_FREQ;
        self.eg_state = EgState::Release;
    }
    //---------------------------------------------------------
    fn periodic_once_every_process(&self) -> (f32, f32) {
        let delta_phase: f32 = (2.0 * std::f32::consts::PI * self.pitch)/general::SAMPLING_FREQ;
        let eg_diff: f32 = self.eg_tgt_value - self.eg_src_value;
        (delta_phase,eg_diff)
    }
    fn calc_eg_level(&mut self, eg_diff: f32) -> f32 {
        match self.eg_state {
            EgState::Attack => {
                let eg_val = eg_diff*(10f32.powf((self.eg_dac_count as f32)/self.eg_time))/10.0;
                self.eg_crnt = self.eg_src_value + eg_val;
                if eg_diff > 0.0 && self.eg_tgt_value < self.eg_crnt {
                    self.eg_crnt = self.eg_tgt_value;
                    self.eg_state = EgState::KeyOnSteady;
                }
            },
            EgState::Release => {
                let eg_val = eg_diff*(10f32.powf((self.eg_dac_count as f32)/self.eg_time))/10.0;
                self.eg_crnt = self.eg_src_value + eg_val;
                if eg_diff < 0.0 && self.eg_tgt_value > self.eg_crnt {
                    self.eg_crnt = self.eg_tgt_value;
                    self.eg_state = EgState::KeyOffSteady;
                }
            },
            _ => {},
        }
        self.eg_crnt
    }
    fn calc_pitch(note:u8) -> f32 {
        let mut tone_name: u8 = note + 3;
        let mut octave: usize = 0;
        if note >= 9 {
            tone_name = (note - 9)%12;
            octave = ((note as usize) - 9)/12 + 1;
        }
        let mut ap: f32 = PITCH_OF_A[octave];
        let ratio: f32 = (2_f32.ln()/12_f32).exp();
        for _ in 0..tone_name {
            ap *= ratio;
        }
        ap
    }
}