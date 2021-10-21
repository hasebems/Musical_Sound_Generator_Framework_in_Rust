//
//  msgf_aeg.rs
//	Musical Sound Generator Framework
//      Aeg Class
//
//  Created by Hasebe Masahiko on 2021/10/15.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::general::msgf_afrm;

#[derive(PartialEq, Clone, Copy)]
pub enum EgState {
    NotYet,
    Attack,     //  A
    Decay,      //  D
    Sustain,    //  S
    Release,    //  R
    EgDone,
    _Damp,
}
//---------------------------------------------------------
//		Synth. Parameter
//---------------------------------------------------------
pub struct AegParameter {
    attack_rate: f32,
    decay_rate: f32,
    sustain_level: f32,
    release_rate: f32,
}
//  Voice Parameter
const AEG_PRM: AegParameter = AegParameter {
    attack_rate: 0.01,  //  0.0-1.0
    decay_rate: 0.0001, //  0.0-1.0 : 1.0 means no decay and no sustain level
    sustain_level: 0.1, //  1 means same value as Attack Level
    release_rate: 0.02, //  0.0-1.0
};
//---------------------------------------------------------
pub struct Aeg {
    state: EgState,
    tgt_value: f32,
    src_value: f32,
    crnt: f32,
    rate: f32,
    nrlz_value: f32,
}

impl Aeg {
    pub fn new() -> Aeg {
        Aeg {
            state: EgState::NotYet,
            tgt_value: 0.0,
            src_value: 0.0,
            crnt: 0.0,
            rate: 1.0,
            nrlz_value: 0.0,
        }
    }
    pub fn move_to_attack(&mut self) {
        self.src_value = 0.0;
        self.tgt_value = 1.0;
        self.rate = AEG_PRM.attack_rate;
        self.state = EgState::Attack;
        self.nrlz_value = 0.0;
    }
    fn move_to_decay(&mut self, eg_crnt: f32) {
        if AEG_PRM.decay_rate == 1.0 {
            self.move_to_sustain(eg_crnt);
        } else {
            self.src_value = eg_crnt;
            self.tgt_value = AEG_PRM.sustain_level;
            self.rate = AEG_PRM.decay_rate;
            self.state = EgState::Decay;
            self.nrlz_value = 0.0;
        }
    }
    fn move_to_sustain(&mut self, eg_crnt: f32) {
        if AEG_PRM.sustain_level == 0.0 {
            self.move_to_egdone();
        } else {
            self.src_value = eg_crnt;
            self.tgt_value = AEG_PRM.sustain_level;
            self.rate = 0.0;
            self.state = EgState::Sustain;
            self.nrlz_value = 0.0;
        }
    }
    pub fn move_to_release(&mut self) {
        self.src_value = self.crnt;
        self.tgt_value = 0.0;
        self.rate = AEG_PRM.release_rate;
        self.state = EgState::Release;
        self.nrlz_value = 0.0;
    }
    fn move_to_egdone(&mut self) {
        self.src_value = 0.0;
        self.tgt_value = 0.0;
        self.rate = 0.0;
        self.state = EgState::EgDone;
        self.nrlz_value = 0.0;
    }
    fn calc_delta_eg(&mut self, eg_diff: f32) -> f32 {
        let mut nrlz = self.nrlz_value;
        if nrlz > 0.99 {
            nrlz += 0.001;
            if nrlz > 1.01 {nrlz = 1.01;}
        } else {
            nrlz += (1.0-self.nrlz_value)*(self.rate);
        }
        self.nrlz_value = nrlz;
        eg_diff*nrlz + self.src_value
    }
    pub fn process(&mut self, abuf: &mut msgf_afrm::AudioFrame) {
        let eg_diff: f32 = self.tgt_value - self.src_value;
        for i in 0..abuf.sample_number {
            let mut eg_crnt: f32 = self.tgt_value;
            match self.state {
                EgState::Attack => {
                    eg_crnt = self.calc_delta_eg(eg_diff);
                    if eg_diff > 0.0 && self.tgt_value <= eg_crnt {
                        self.move_to_decay(self.tgt_value);
                    }
                },
                EgState::Decay => {
                    eg_crnt = self.calc_delta_eg(eg_diff);
                    if eg_diff < 0.0 && self.tgt_value >= eg_crnt {
                        self.move_to_sustain(self.tgt_value);
                    }
                },
                EgState::Release => {
                    eg_crnt = self.calc_delta_eg(eg_diff);
                    if eg_diff < 0.0 && self.tgt_value >= eg_crnt {
                        self.move_to_egdone();
                    }
                },
                _ => {},
            }
            abuf.mul_abuf(i, eg_crnt);
            self.crnt = eg_crnt;
        }
    }
}