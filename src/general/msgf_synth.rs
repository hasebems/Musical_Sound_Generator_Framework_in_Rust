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
    Attack,     //  A
    Decay,      //  D
    Sustain,    //  S
    Release,    //  R
    EgDone,
    _Damp,
}
#[derive(PartialEq)]
enum WvType {
    Sine,
    _Saw,
    _Square,
    _Pulse,
}
const PITCH_OF_A: [f32; 11] =
[
//	-3     9     21    33     45     57     69     81      93      105     117 : note number
    13.75, 27.5, 55.0, 110.0, 220.0, 440.0, 880.0, 1760.0, 3520.0, 7040.0, 14080.0  // [Hz]
];
//  Voice Parameter
const EG_ATTACK_TIME: f32 = 0.02;    //  [sec]
const EG_DECAY_TIME: f32 = 0.0;     //  [sec] 0 means no decay and no sustain level
const EG_SUSTAIN_LEVEL: f32 = 0.5;  //  1 means same value as Attack Level
const EG_RELEASE_TIME: f32 = 0.1;   //  [sec]
const WV_TYPE: WvType = WvType::Sine;

const ABORT_FREQUENCY: f32 = 12000.0;
//---------------------------------------------------------
//		Class
//---------------------------------------------------------
pub struct Synth {
    base_pitch: f32,
    next_phase: f32,
    eg_state: EgState,
    eg_tgt_value: f32,
    eg_src_value: f32,
    eg_crnt: f32,
    eg_time: f32,
    eg_dac_count: usize,
    max_note_vol: f32,
    pi: f32,
    wv_type: WvType,
}

impl Synth {
    pub fn new(note:u8) -> Synth {
        Synth {
            base_pitch: Synth::calc_pitch(note),
            next_phase: 0.0,
            eg_state: EgState::NotYet,
            eg_tgt_value: 0.0,
            eg_src_value: 0.0,
            eg_crnt: 0.0,
            eg_time: 0.0,
            eg_dac_count: 0,
            max_note_vol: 0.5f32.powf(4.0), // 4bit margin
            pi: std::f32::consts::PI,
            wv_type: WV_TYPE,
        }
    }
    pub fn process(&mut self, abuf: &mut msgf_afrm::AudioFrame) {
        let this_time = self.periodic_once_every_process();
        self.generate_wave(abuf, this_time.0);

        let mut phase = self.next_phase;
        for i in 0..abuf.sample_number {
            let mut smpl = abuf.get_abuf(i);
            smpl *= self.calc_eg_level(this_time.1);    // AEG
            smpl *= self.max_note_vol;                  // Max Volume
            abuf.set_abuf(i, smpl);                     // Set Buffer
            phase += this_time.0;
            self.eg_dac_count += 1;
        }
        while phase > 2.0*self.pi {
            phase -= 2.0*self.pi;
        }
        self.next_phase = phase;
    }
    pub fn move_to_attack(&mut self) {
        self.eg_src_value = 0.0;
        self.eg_tgt_value = 1.0;
        self.eg_dac_count = 0;
        self.eg_time = EG_ATTACK_TIME*general::SAMPLING_FREQ;
        self.eg_state = EgState::Attack;
    }
    pub fn move_to_decay(&mut self) {
        if EG_DECAY_TIME == 0.0 {
            self.move_to_sustain();
        } else {
            self.eg_src_value = self.eg_crnt;
            self.eg_tgt_value = EG_SUSTAIN_LEVEL;
            self.eg_dac_count = 0;
            self.eg_time = EG_DECAY_TIME*general::SAMPLING_FREQ;
            self.eg_state = EgState::Decay;
        }
    }
    pub fn move_to_sustain(&mut self) {
        if EG_SUSTAIN_LEVEL == 0.0 {
            self.move_to_egdone();
        } else {
            self.eg_src_value = self.eg_crnt;
            self.eg_tgt_value = EG_SUSTAIN_LEVEL;
            self.eg_dac_count = 0;
            self.eg_time = 0.0;
            self.eg_state = EgState::Sustain;
        }
    }
    pub fn move_to_release(&mut self) {
        self.eg_src_value = self.eg_crnt;
        self.eg_tgt_value = 0.0;
        self.eg_dac_count = 0;
        self.eg_time = EG_RELEASE_TIME*general::SAMPLING_FREQ;
        self.eg_state = EgState::Release;
    }
    pub fn move_to_egdone(&mut self) {
        self.eg_src_value = 0.0;
        self.eg_tgt_value = 0.0;
        self.eg_dac_count = 0;
        self.eg_time = 0.0;
        self.eg_state = EgState::EgDone;
    }
    //---------------------------------------------------------
    fn periodic_once_every_process(&self) -> (f32, f32) {
        let delta_phase: f32 = (2.0 * self.pi * self.base_pitch)/general::SAMPLING_FREQ;
        let eg_diff: f32 = self.eg_tgt_value - self.eg_src_value;
        (delta_phase,eg_diff)
    }
    fn calc_delta_eg(&self, eg_diff: f32) -> f32 {
        eg_diff*(10f32.powf((self.eg_dac_count as f32)/self.eg_time))/10.0
    }
    fn calc_eg_level(&mut self, eg_diff: f32) -> f32 {
        match self.eg_state {
            EgState::Attack => {
                let delta_eg = self.calc_delta_eg(eg_diff);
                self.eg_crnt = self.eg_src_value + delta_eg;
                if eg_diff > 0.0 && self.eg_tgt_value < self.eg_crnt {
                    self.eg_crnt = self.eg_tgt_value;
                    self.move_to_decay();
                }
            },
            EgState::Decay => {
                let delta_eg = self.calc_delta_eg(eg_diff);
                self.eg_crnt = self.eg_src_value + delta_eg;
                if eg_diff < 0.0 && self.eg_tgt_value > self.eg_crnt {
                    self.eg_crnt = self.eg_tgt_value;
                    self.move_to_sustain();
                }
            }
            EgState::Sustain => {}
            EgState::Release => {
                let delta_eg = self.calc_delta_eg(eg_diff);
                self.eg_crnt = self.eg_src_value + delta_eg;
                if eg_diff < 0.0 && self.eg_tgt_value > self.eg_crnt {
                    self.move_to_egdone();
                }
            },
            _ => {},
        }
        self.eg_crnt
    }
    fn calc_pitch(note:u8) -> f32 {
        let solfa_name: u8 = (note + 3)%12;
        let octave: usize = ((note as usize) + 3)/12;
        let mut ap: f32 = PITCH_OF_A[octave];
        let ratio: f32 = (2_f32.ln()/12_f32).exp();
        for _ in 0..solfa_name {
            ap *= ratio;
        }
        ap
    }
    fn generate_wave(&self, abuf: &mut msgf_afrm::AudioFrame, delta_phase: f32) {
        let max_overtone: usize = (ABORT_FREQUENCY/self.base_pitch) as usize;
        let mut phase = self.next_phase;
        match self.wv_type {
            WvType::Sine => {
                for i in 0..abuf.sample_number {
                    abuf.set_abuf(i, phase.sin());
                    phase += delta_phase;
                }
            }
            WvType::_Saw => {
                for i in 0..abuf.sample_number {
                    let mut saw: f32 = 0.0;
                    for j in 1..max_overtone {
                        let ot:f32 = j as f32;
                        saw += 0.25*(phase*ot).sin()/ot;
                    }
                    abuf.set_abuf(i, saw);
                    phase += delta_phase;
                }
            }
            WvType::_Square => {
                for i in 0..abuf.sample_number {
                    let mut sq: f32 = 0.0;
                    for j in (1..max_overtone).step_by(2) {
                        let ot:f32 = j as f32;
                        sq += 0.25*(phase*ot).sin()/ot;
                    }
                    abuf.set_abuf(i, sq);
                    phase += delta_phase;
                }
            }
            WvType::_Pulse => {
                for i in 0..abuf.sample_number {
                    let mut pls: f32 = 0.0;
                    let mut ps: f32 = phase;
                    ps %= 2.0*self.pi;
                    ps /= 2.0*self.pi;
                    if ps < 0.1 { pls = 0.5;}
                    else if ps < 0.2 { pls = -0.5;}
                    abuf.set_abuf(i, pls);
                    phase += delta_phase;
                }
            }
        }
    }
}