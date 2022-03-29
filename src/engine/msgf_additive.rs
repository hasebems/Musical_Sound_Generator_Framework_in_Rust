//
//  msgf_additive.rs
//	Musical Sound Generator Framework
//      Additive Class
//
//  Created by Hasebe Masahiko on 2022/03/21.
//  Copyright (c) 2022 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::msgf_if;
use crate::core::*;
use crate::engine::msgf_osc::*;
use crate::engine::msgf_table;

//---------------------------------------------------------
//		Synth. Parameter
//---------------------------------------------------------
#[derive(Copy, Clone)]
pub struct AdditiveParameter {
    pub coarse_tune: i32,   //  [semitone]
    pub fine_tune: f32,     //  [cent]
    pub pmd: f32,
    pub prtm_spd: f32,      //  speed of portamento: 0.0(fastest)-?
    pub magnitude: f32,
}
//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub struct Additive {
    prms_variable: AdditiveParameter,
    pmd: f32,
    base_pitch: f32,    //  [Hz]
    next_phase: f32,    //  0.0 - 1.0
    //  for Portamento
    target_pitch: f32,  //  [Hz]
    real_prtm_spd: f32,
    //  for Pitch Bend
    cnt_ratio: f32,     //  ratio of Hz
}
//---------------------------------------------------------
//		Implements
//---------------------------------------------------------
impl Additive {
    pub fn new(prms:&AdditiveParameter, note:u8, cnt_pitch:f32) -> Additive {
        let pit = Osc::calc_base_pitch(prms.coarse_tune, prms.fine_tune, note);
        Self {
            prms_variable: *prms,
            pmd: prms.pmd,
            base_pitch: pit,
            next_phase: 0.0,
            target_pitch: pit,
            real_prtm_spd: 0.0,
            cnt_ratio: Osc::calc_cnt_pitch(cnt_pitch),
        }
    }
    pub fn change_pmd(&mut self, value:f32) {self.pmd = value;}
    pub fn change_note(&mut self, note:u8) {
        self.target_pitch = Osc::calc_base_pitch(self.prms_variable.coarse_tune,
                                                 self.prms_variable.fine_tune, note);
        self.real_prtm_spd = self.prms_variable.prtm_spd;//(diff_note as f32).powf(0.25);
    }
    pub fn change_pitch(&mut self, cnt_pitch:f32) {
        self.cnt_ratio = Osc::calc_cnt_pitch(cnt_pitch);
    }
    fn wave_func(&self, phase: f32, y: usize) -> f32 {
        let mut pls: f32 = 0.1;
        let oti = if y <= 32 {y} else {32};
        for j in 1..oti {
            let ot:f32 = j as f32;
            pls += msgf_table::PULSE0_1[j]*Osc::pseudo_sine(phase*ot+1.0);
        }
        pls
    }
    fn pitch_interporation(&mut self, diff: f32) {
        //  Pitch Operation for Portamento
        self.base_pitch += diff*self.real_prtm_spd;
        if self.target_pitch*1.01 > self.base_pitch &&
          self.target_pitch*0.99 < self.base_pitch {
            self.base_pitch = self.target_pitch;
        }
    }
    pub fn process(&mut self, abuf: &mut msgf_afrm::AudioFrame, lbuf: &mut msgf_cfrm::CtrlFrame) {
        let lvl_variable = self.prms_variable.magnitude;
        if self.target_pitch != self.base_pitch {
            //  Portameneto Operation
            let diff = self.target_pitch - self.base_pitch;
            self.pitch_interporation(diff);
        }
        let delta_phase = self.base_pitch*self.cnt_ratio/msgf_if::SAMPLING_FREQ;
        let mut phase = self.next_phase;
        let max_overtone: usize = (msgf_table::ABORT_FREQUENCY/self.target_pitch) as usize;
        //let wave_func: WvFn = self.get_wave_func();
        for i in 0..abuf.sample_number {
            let sample = self.wave_func(phase, max_overtone);
            abuf.set_abuf(i, sample*lvl_variable);
            let magnitude = lbuf.ctrl_for_audio(i)*self.pmd;
            phase += delta_phase*(2.0_f32.powf(magnitude));
            while phase > 1.0 { phase -= 1.0 }
        }
        self.next_phase = phase;
    }
}