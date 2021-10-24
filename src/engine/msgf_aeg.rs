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
use crate::general::msgf_cfrm;

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
    attack_rate: 0.5,  //  0.0-1.0
    decay_rate: 0.01, //  0.0-1.0 : 1.0 means no decay and no sustain level
    sustain_level: 0.1, //  1 means same value as Attack Level
    release_rate: 0.01, //  0.0-1.0
};
//---------------------------------------------------------
pub struct Aeg {
    state: EgState,
    tgt_value: f32,
    src_value: f32,
    crnt_value: f32,
    crnt_rate: f32,
    interpolate_value: f32,
    release_rsv: bool,
}

impl Aeg {
    pub fn new() -> Aeg {
        Aeg {
            state: EgState::NotYet,
            tgt_value: 0.0,
            src_value: 0.0,
            crnt_value: 0.0,
            crnt_rate: 1.0,
            interpolate_value: 0.0,
            release_rsv: false,
        }
    }
    pub fn move_to_attack(&mut self) {
        self.src_value = 0.0;
        self.tgt_value = 1.0;
        self.crnt_rate = AEG_PRM.attack_rate;
        self.state = EgState::Attack;
        self.interpolate_value = 0.0;
    }
    fn move_to_decay(&mut self, eg_crnt: f32) {
        if AEG_PRM.decay_rate == 1.0 {
            self.move_to_sustain(eg_crnt);
        } else {
            self.src_value = eg_crnt;
            self.tgt_value = AEG_PRM.sustain_level;
            self.crnt_rate = AEG_PRM.decay_rate;
            self.state = EgState::Decay;
            self.interpolate_value = 0.0;
        }
    }
    fn move_to_sustain(&mut self, eg_crnt: f32) {
        if AEG_PRM.sustain_level == 0.0 {
            self.move_to_egdone();
        } else {
            self.src_value = eg_crnt;
            self.tgt_value = AEG_PRM.sustain_level;
            self.crnt_rate = 0.0;
            self.state = EgState::Sustain;
            self.interpolate_value = 0.0;
        }
    }
    pub fn move_to_release(&mut self) {
        if self.state == EgState::Decay {
            //  Decay 中であれば、Decay が終わるまで release は保留
            self.release_rsv = true;
        }
        else {
            self.src_value = self.crnt_value;
            self.tgt_value = 0.0;
            self.crnt_rate = AEG_PRM.release_rate;
            self.state = EgState::Release;
            self.interpolate_value = 0.0;
        }
    }
    fn move_to_egdone(&mut self) {
        self.src_value = 0.0;
        self.tgt_value = 0.0;
        self.crnt_rate = 0.0;
        self.state = EgState::EgDone;
        self.interpolate_value = 0.0;
    }
    fn calc_delta_eg(&mut self, eg_diff: f32) -> f32 {
          // 0.0 -> 1.01 の動きを作り出し、interpolate_value に格納
          // その値を eg_diff にかけて、現在の到達値を返す
        let mut intplt = self.interpolate_value;
        if intplt > 0.9 {
            intplt += 0.01;
            if intplt > 1.01 {intplt = 1.01;}
        } else {
            intplt += (1.0-intplt)*(self.crnt_rate);
        }
        self.interpolate_value = intplt;
        eg_diff*intplt + self.src_value
    }
    pub fn process(&mut self, cbuf: &mut msgf_cfrm::CtrlFrame) {
        let eg_diff: f32 = self.tgt_value - self.src_value;
        for i in 0..cbuf.sample_number {
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
                        if self.release_rsv {
                            self.state = EgState::Sustain;
                            self.move_to_release();
                        } else {
                            self.move_to_sustain(self.tgt_value);
                        }
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
            cbuf.set_cbuf(i, eg_crnt);
            self.crnt_value = eg_crnt;
        }
    }
}