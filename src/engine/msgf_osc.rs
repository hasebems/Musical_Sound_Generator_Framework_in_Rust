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

#[allow(dead_code)]
#[derive(PartialEq)]
enum WvType {
    Sine,
    Saw,
    Square,
    Pulse,
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
pub struct OscParameter {
    coarse_tune: i32,   //  [semitone]
    fine_tune: f32,     //  [cent]
    wv_type: WvType,
}
//  Voice Parameter
const OSC_PRM: OscParameter = OscParameter {
    coarse_tune: 0,     //  i32
    fine_tune: 0.0,   //  f32
    wv_type: WvType::Pulse,
};
//---------------------------------------------------------
//		Class
//---------------------------------------------------------
pub struct Osc {
    base_pitch: f32,
    next_phase: f32,
    pi: f32,
    wv_type: WvType,
}

impl Osc {
    pub fn new(note:u8) -> Osc {
        Osc {
            base_pitch: Osc::calc_pitch(note),
            next_phase: 0.0,
            pi: std::f32::consts::PI,
            wv_type: OSC_PRM.wv_type,
        }
    }
    fn limit_note(calculated_note:i32) -> u8 {
        let mut note = calculated_note;
        while note < 0 { note += 12;}
        while note >= 128 { note -= 12;}
        note as u8
    }
    fn calc_pitch(note:u8) -> f32 {
        let tune_note: u8 = Osc::limit_note(note as i32 + OSC_PRM.coarse_tune);
        let solfa_name: u8 = (tune_note + 3)%12;
        let octave: usize = ((tune_note as usize) + 3)/12;
        let mut ap = PITCH_OF_A[octave];
        let ratio = (2_f32.ln()/12_f32).exp();
        for _ in 0..solfa_name {
            ap *= ratio;
        }
        let cratio = (OSC_PRM.fine_tune*(2_f32.ln()/1200_f32)).exp();
        ap *= cratio;
        ap
    }
    pub fn process(&mut self, abuf: &mut msgf_afrm::AudioFrame) {
        let delta_phase = (2.0 * self.pi * self.base_pitch)/general::SAMPLING_FREQ;
        let max_overtone: usize = (ABORT_FREQUENCY/self.base_pitch) as usize;
        let mut phase = self.next_phase;
        match self.wv_type {
            WvType::Sine => {
                for i in 0..abuf.sample_number {
                    abuf.set_abuf(i, phase.sin());
                    phase += delta_phase;
                }
            }
            WvType::Saw => {
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
            WvType::Square => {
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
            WvType::Pulse => {
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
        //  Update next_phase
        let mut phase = self.next_phase + delta_phase*(abuf.sample_number as f32);
        while phase > 2.0*self.pi {
            phase -= 2.0*self.pi;
        }
        self.next_phase = phase;
    }
}
