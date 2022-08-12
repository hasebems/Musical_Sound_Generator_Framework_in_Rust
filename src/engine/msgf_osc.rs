//
//  msgf_osc.rs
//	Musical Sound Generator Framework
//      Osc Class
//
//  Created by Hasebe Masahiko on 2021/10/15.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::msgf_if;
use crate::core::*;
use crate::engine::msgf_table;

//---------------------------------------------------------
//		Synth. Parameter
//---------------------------------------------------------
#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum WvType {
    Sine,
    Saw,
    Square,
    Pulse,
}
#[derive(Copy, Clone)]
pub struct OscParameter {
    pub coarse_tune: i32,   //  [semitone]
    pub fine_tune: f32,     //  [cent]
    pub lfo_depth: f32,     //  1.0 means +-1oct.
    pub wv_type: WvType,
}
type WvFn = fn(f32, usize) -> f32;
//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub struct Osc {
    prms_variable: OscParameter,
    pmd: f32,
    base_pitch: f32,    //  [Hz]
    cnt_ratio: f32,     //  ratio of Hz
    next_phase: f32,    //  0.0 - 1.0
}
//---------------------------------------------------------
//		Implements
//---------------------------------------------------------
impl Osc {
    pub fn new(prms:&OscParameter, note:u8, pmd:f32, cnt_pitch:f32) -> Osc {
        Osc {
            prms_variable: *prms,
            pmd,
            base_pitch: Osc::calc_base_pitch(prms.coarse_tune, prms.fine_tune, note),
            cnt_ratio: Osc::calc_cnt_pitch(cnt_pitch),
            next_phase: 0.0,
        }
    }
    pub fn change_pmd(&mut self, value:f32) {self.pmd = value;} // value: 1.0:1oct,-1.0:-1oct
    pub fn _change_note(&mut self, note:u8) {
        self.base_pitch = Osc::calc_base_pitch(self.prms_variable.coarse_tune,
                                            self.prms_variable.fine_tune, note);
    }
    pub fn limit_note(calculated_note:i32) -> u8 {
        let mut note = calculated_note;
        while note < 0 { note += 12;}
        while note >= 128 { note -= 12;}
        note as u8
    }
    pub fn calc_base_pitch(coarse_tune:i32, fine_tune:f32, note:u8) -> f32 {
        let tune_note: u8 = Osc::limit_note(note as i32 + coarse_tune);
        let solfa_name: u8 = (tune_note + 3)%12;
        let octave: usize = ((tune_note as usize) + 3)/12;
        let mut ap = msgf_table::PITCH_OF_A[octave];
        let ratio = (2_f32.ln()/12_f32).exp();
        for _ in 0..solfa_name {
            ap *= ratio;
        }
        ap *= (fine_tune*(2_f32.ln()/1200_f32)).exp();
        ap
    }
    pub fn pseudo_sine(mut phase:f32) -> f32 {
        // Lagrange interpolation
        while phase > 1.0 { phase -= 1.0 }
        let nrm_phase:f32 = phase * 256.0;
        let phase_locate = nrm_phase.round() as usize;
        let x1 = nrm_phase - phase_locate as f32;
        //let x0 = x1 + 1.0; // cubic interpolation
        //let x2 = x1 - 1.0;
        //let x3 = x1 - 2.0;
        //let mut y = -(x1*x2*x3*SIN_TABLE[phase_locate+1]/6.0) + (x0*x2*x3*SIN_TABLE[phase_locate+2]/2.0)
        //            -(x0*x1*x3*SIN_TABLE[phase_locate+3]/2.0) + (x0*x1*x2*SIN_TABLE[phase_locate+4]/6.0);
        //assert!(phase_locate < 258, "{},{},{},{}:{}->{}", x0,x1,x2,x3,phase_locate,y);
        let y0 = msgf_table::SIN_TABLE[phase_locate+2];                    //  linear interpolation
        let mut y = (msgf_table::SIN_TABLE[phase_locate+3] - y0)*x1 + y0;  //
        if y > 1.0 { y = 1.0 }
        else if y < -1.0 { y = -1.0 }
        y
    }
    pub fn calc_cnt_pitch(pitch: f32) -> f32 {    //  pitch : [cent]
        let mut pt: f32 = 1.0;
        if pitch != 0.0 {
            pt = (pitch*(2_f32.ln()/1200_f32)).exp();
        }
        pt
    }
    pub fn change_pitch(&mut self, cnt_pitch:f32) {
        self.cnt_ratio = Osc::calc_cnt_pitch(cnt_pitch);
    }
    fn get_wave_func(&self) -> WvFn {
        match self.prms_variable.wv_type {
            WvType::Sine => {
                //wave_func = |x, _y| {
                //  let phase = x * 2.0 * msgf_if::PI;
                //  phase.sin()
                //}
                return |x, _y| Osc::pseudo_sine(x);
            }
            WvType::Saw => {
                return |x, y| {
                    let mut saw: f32 = 0.0;
                    for j in 1..y {
                        let ot:f32 = j as f32;
                        let phase:f32 = x * ot;
                        saw += 0.5*Osc::pseudo_sine(phase)/ot;
                    }
                    saw
                };
            }
            WvType::Square => {
                return |x, y| {
                    let mut sq: f32 = 0.0;
                    for j in (1..y).step_by(2) {
                        let ot:f32 = j as f32;
                        let phase:f32 = x * ot;
                        sq += 0.25*Osc::pseudo_sine(phase)/ot;
                    }
                    sq
                };
            }
            WvType::Pulse => {
                return |x, y| {
                    let mut pls: f32 = 0.1;
                    let mut oti = y;
                    if oti > 32 {oti = 32;}
                    for j in 1..oti {
                        let ot:f32 = j as f32;
                        let phase:f32 = x * ot;
                        pls += 0.5*msgf_table::PULSE0_1[j]*Osc::pseudo_sine(phase);
                    }
                    pls
                }
            }
        }
    }
    pub fn process(&mut self, abuf: &mut msgf_afrm::AudioFrame, lbuf: &mut msgf_cfrm::CtrlFrame) {
        let delta_phase = self.base_pitch*self.cnt_ratio/msgf_if::SAMPLING_FREQ;
        let mut phase = self.next_phase;
        let max_overtone: usize = (msgf_table::ABORT_FREQUENCY/self.base_pitch) as usize;
        let wave_func: WvFn = self.get_wave_func();
        for i in 0..abuf.sample_number {
            abuf.set_val(i, wave_func(phase, max_overtone));
            let magnitude = lbuf.ctrl_for_audio(i)*self.pmd;
            phase += delta_phase*(2.0_f32.powf(magnitude));
            while phase > 1.0 { phase -= 1.0 }
        }
        self.next_phase = phase;
    }
}
