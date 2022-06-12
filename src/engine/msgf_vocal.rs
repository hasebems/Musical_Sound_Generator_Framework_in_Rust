//
//  msgf_vocal.rs
//	Musical Sound Generator Framework
//      Vocal Class
//      声帯振動による波形を生成する
//
//  Created by Hasebe Masahiko on 2022/06/11.
//  Copyright (c) 2022 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::msgf_if;
use crate::core::*;
use crate::engine::msgf_table;

//---------------------------------------------------------
//		Synth. Parameter
//---------------------------------------------------------
#[derive(Copy, Clone)]
pub struct VocalParameter {
    pub coarse_tune: i32,   //  [semitone]
    pub fine_tune: f32,     //  [cent]
    pub lfo_depth: f32,     //  1.0 means +-1oct.
}
//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub struct Vocal {
    prms_variable: VocalParameter,
    pmd: f32,
    base_pitch: f32,    //  [Hz]
    cnt_ratio: f32,     //  ratio of Hz
    next_phase: f32,    //  0.0 - 1.0
}
//---------------------------------------------------------
//		Implements
//---------------------------------------------------------
impl Vocal {
    pub fn new(prms:&VocalParameter, note:u8, pmd:f32, cnt_pitch:f32) -> Vocal {
        Vocal {
            prms_variable: *prms,
            pmd,
            base_pitch: Vocal::calc_base_pitch(prms.coarse_tune, prms.fine_tune, note),
            cnt_ratio: Vocal::calc_cnt_pitch(cnt_pitch),
            next_phase: 0.0,
        }
    }
    pub fn change_pmd(&mut self, value:f32) {self.pmd = value;}
    pub fn change_note(&mut self, note:u8) {
        self.base_pitch = Vocal::calc_base_pitch(self.prms_variable.coarse_tune,
                                            self.prms_variable.fine_tune, note);
    }
    pub fn limit_note(calculated_note:i32) -> u8 {
        let mut note = calculated_note;
        while note < 0 { note += 12;}
        while note >= 128 { note -= 12;}
        note as u8
    }
    pub fn calc_base_pitch(coarse_tune:i32, fine_tune:f32, note:u8) -> f32 {
        let tune_note: u8 = Vocal::limit_note(note as i32 + coarse_tune);
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
    pub fn gen_wave(mut phase:f32) -> f32 {
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
        let y0 = msgf_table::VOCAL_TABLE[phase_locate+2];                    //  linear interpolation
        let mut y = (msgf_table::VOCAL_TABLE[phase_locate+3] - y0)*x1 + y0;  //
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
        self.cnt_ratio = Vocal::calc_cnt_pitch(cnt_pitch);
    }
    pub fn process(&mut self, abuf: &mut msgf_afrm::AudioFrame, lbuf: &mut msgf_cfrm::CtrlFrame) {
        let delta_phase = self.base_pitch*self.cnt_ratio/msgf_if::SAMPLING_FREQ;
        let mut phase = self.next_phase;
        for i in 0..abuf.sample_number {
            abuf.set_val(i, Vocal::gen_wave(phase));
            let magnitude = lbuf.ctrl_for_audio(i)*self.pmd;
            phase += delta_phase*(2.0_f32.powf(magnitude));
            while phase > 1.0 { phase -= 1.0 }
        }
        self.next_phase = phase;
    }
}
