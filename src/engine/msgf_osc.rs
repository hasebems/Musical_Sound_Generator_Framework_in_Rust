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
use crate::general;
use crate::general::msgf_afrm;
use crate::general::msgf_cfrm;
use crate::app::msgf_prm;

//---------------------------------------------------------
//		Synth. Parameter
//---------------------------------------------------------
#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy)]
pub enum WvType {
    Sine,
    Saw,
    Square,
    Pulse,
}
pub struct OscParameter {
    pub coarse_tune: i32,   //  [semitone]
    pub fine_tune: f32,     //  [cent]
    pub wv_type: WvType,
}
//---------------------------------------------------------
//		Constants
//---------------------------------------------------------
const PITCH_OF_A: [f32; 11] =
[
//	-3     9     21    33     45     57     69     81      93      105     117 : note number
    13.75, 27.5, 55.0, 110.0, 220.0, 440.0, 880.0, 1760.0, 3520.0, 7040.0, 14080.0  // [Hz]
];
const ABORT_FREQUENCY: f32 = 12000.0;
//---------------------------------------------------------
pub struct Osc {
    base_pitch: f32,
    next_phase: f32,
    wv_type: WvType,
}
//---------------------------------------------------------
impl Osc {
    pub fn new(note:u8) -> Osc {
        Osc {
            base_pitch: Osc::calc_pitch(note),
            next_phase: 0.0,
            wv_type: msgf_prm::TONE_PRM[0].osc.wv_type,
        }
    }
    fn limit_note(calculated_note:i32) -> u8 {
        let mut note = calculated_note;
        while note < 0 { note += 12;}
        while note >= 128 { note -= 12;}
        note as u8
    }
    fn calc_pitch(note:u8) -> f32 {
        let tune_note: u8 = Osc::limit_note(note as i32 + msgf_prm::TONE_PRM[0].osc.coarse_tune);
        let solfa_name: u8 = (tune_note + 3)%12;
        let octave: usize = ((tune_note as usize) + 3)/12;
        let mut ap = PITCH_OF_A[octave];
        let ratio = (2_f32.ln()/12_f32).exp();
        for _ in 0..solfa_name {
            ap *= ratio;
        }
        ap *= (msgf_prm::TONE_PRM[0].osc.fine_tune*(2_f32.ln()/1200_f32)).exp();
        ap
    }
    pub fn process(&mut self, abuf: &mut msgf_afrm::AudioFrame, lbuf: &mut msgf_cfrm::CtrlFrame) {
        let wave_func: fn(f32, usize) -> f32;
        match self.wv_type {
            WvType::Sine => {
                wave_func = |x, _y| x.sin();
            }
            WvType::Saw => {
                wave_func = |x, y| {
                    let mut saw: f32 = 0.0;
                    for j in 1..y {
                        let ot:f32 = j as f32;
                        saw += 0.25*(x*ot).sin()/ot;
                    }
                    saw
                };
            }
            WvType::Square => {
                wave_func = |x, y| {
                    let mut sq: f32 = 0.0;
                    for j in (1..y).step_by(2) {
                        let ot:f32 = j as f32;
                        sq += 0.25*(x*ot).sin()/ot;
                    }
                    sq
                };
            }
            WvType::Pulse => {
                wave_func = |x, _y| {
                    let mut pls: f32 = 0.0;
                    let mut ps: f32 = x;
                    ps %= 2.0*general::PI;
                    ps /= 2.0*general::PI;
                    if ps < 0.1 { pls = 0.5;}
                    else if ps < 0.2 { pls = -0.5;}
                    pls
                }
            }
        }
        let piconst = (2.0 * general::PI)/general::SAMPLING_FREQ;
        let mut phase = self.next_phase;
        let max_overtone: usize = (ABORT_FREQUENCY/self.base_pitch) as usize;
        for i in 0..abuf.sample_number {
            abuf.set_abuf(i, wave_func(phase, max_overtone));
            phase += (self.base_pitch*piconst)*(1.0 + lbuf.ctrl_for_audio(i));
        }
        //  Update next_phase
        while phase > 2.0*general::PI {
            phase -= 2.0*general::PI;
        }
        self.next_phase = phase;
    }
}
