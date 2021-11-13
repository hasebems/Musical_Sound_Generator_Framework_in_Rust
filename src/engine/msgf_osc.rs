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
use crate::general::*;
use crate::app::*;

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
    pub lfo_depth: f32,     //  1.0 means +-1oct.
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
const SINE_TABLE: [f32; 261] = //   index should be used by adding 2.
[
-0.049067674,   -0.024541229,
0.0,            0.024541229,    0.049067674,    0.073564564,    0.09801714,     0.122410675,    0.146730474,    0.170961889,
0.195090322,    0.21910124,     0.24298018,     0.266712757,    0.290284677,    0.31368174,     0.336889853,    0.359895037,
0.382683432,    0.405241314,    0.427555093,    0.44961133,     0.471396737,    0.492898192,    0.514102744,    0.53499762,
0.555570233,    0.575808191,    0.595699304,    0.615231591,    0.634393284,    0.653172843,    0.671558955,    0.689540545,
0.707106781,    0.724247083,    0.740951125,    0.757208847,    0.773010453,    0.788346428,    0.803207531,    0.817584813,
0.831469612,    0.844853565,    0.85772861,     0.870086991,    0.881921264,    0.893224301,    0.903989293,    0.914209756,
0.923879533,    0.932992799,    0.941544065,    0.949528181,    0.956940336,    0.963776066,    0.970031253,    0.97570213,
0.98078528,     0.985277642,    0.98917651,     0.992479535,    0.995184727,    0.997290457,    0.998795456,    0.999698819,
1.0,            0.999698819,    0.998795456,    0.997290457,    0.995184727,    0.992479535,    0.98917651,     0.985277642,
0.98078528,     0.97570213,     0.970031253,    0.963776066,    0.956940336,    0.949528181,    0.941544065,    0.932992799,
0.923879533,    0.914209756,    0.903989293,    0.893224301,    0.881921264,    0.870086991,    0.85772861,     0.844853565,
0.831469612,    0.817584813,    0.803207531,    0.788346428,    0.773010453,    0.757208847,    0.740951125,    0.724247083,
0.707106781,    0.689540545,    0.671558955,    0.653172843,    0.634393284,    0.615231591,    0.595699304,    0.575808191,
0.555570233,    0.53499762,     0.514102744,    0.492898192,    0.471396737,    0.44961133,     0.427555093,    0.405241314,
0.382683432,    0.359895037,    0.336889853,    0.31368174,     0.290284677,    0.266712757,    0.24298018,     0.21910124,
0.195090322,    0.170961889,    0.146730474,    0.122410675,    0.09801714,     0.073564564,    0.049067674,    0.024541229,
0.0,            -0.024541229,   -0.049067674,   -0.073564564,   -0.09801714,    -0.122410675,   -0.146730474,   -0.170961889,
-0.195090322,   -0.21910124,    -0.24298018,    -0.266712757,   -0.290284677,   -0.31368174,    -0.336889853,   -0.359895037,
-0.382683432,   -0.405241314,   -0.427555093,   -0.44961133,    -0.471396737,   -0.492898192,   -0.514102744,   -0.53499762,
-0.555570233,   -0.575808191,   -0.595699304,   -0.615231591,   -0.634393284,   -0.653172843,   -0.671558955,   -0.689540545,
-0.707106781,   -0.724247083,   -0.740951125,   -0.757208847,   -0.773010453,   -0.788346428,   -0.803207531,   -0.817584813,
-0.831469612,   -0.844853565,   -0.85772861,    -0.870086991,   -0.881921264,   -0.893224301,   -0.903989293,   -0.914209756,
-0.923879533,   -0.932992799,   -0.941544065,   -0.949528181,   -0.956940336,   -0.963776066,   -0.970031253,   -0.97570213,
-0.98078528,    -0.985277642,   -0.98917651,    -0.992479535,   -0.995184727,   -0.997290457,   -0.998795456,   -0.999698819,
-1.0,           -0.999698819,   -0.998795456,   -0.997290457,   -0.995184727,   -0.992479535,   -0.98917651,    -0.985277642,
-0.98078528,    -0.97570213,    -0.970031253,   -0.963776066,   -0.956940336,   -0.949528181,   -0.941544065,   -0.932992799,
-0.923879533,   -0.914209756,   -0.903989293,   -0.893224301,   -0.881921264,   -0.870086991,   -0.85772861,    -0.844853565,
-0.831469612,   -0.817584813,   -0.803207531,   -0.788346428,   -0.773010453,   -0.757208847,   -0.740951125,   -0.724247083,
-0.707106781,   -0.689540545,   -0.671558955,   -0.653172843,   -0.634393284,   -0.615231591,   -0.595699304,   -0.575808191,
-0.555570233,   -0.53499762,    -0.514102744,   -0.492898192,   -0.471396737,   -0.44961133,    -0.427555093,   -0.405241314,
-0.382683432,   -0.359895037,   -0.336889853,   -0.31368174,    -0.290284677,   -0.266712757,   -0.24298018,    -0.21910124,
-0.195090322,   -0.170961889,   -0.146730474,   -0.122410675,   -0.09801714,    -0.073564564,   -0.049067674,   -0.024541229,
0.0,            0.024541229,    0.049067674,
];
//---------------------------------------------------------
pub struct Osc {
    base_pitch: f32,    //  [Hz]
    next_phase: f32,    //  0.0 - 1.0
    lfo_depth: f32,
    wv_type: WvType,
}
//---------------------------------------------------------
impl Osc {
    pub fn new(note:u8, inst_set:usize, pmd:f32) -> Osc {
        Osc {
            base_pitch: Osc::calc_pitch(note, inst_set),
            next_phase: 0.0,
            lfo_depth: pmd,
            wv_type: msgf_prm::TONE_PRM[inst_set].osc.wv_type,
        }
    }
    pub fn change_pmd(&mut self, value:f32) {self.lfo_depth = value;}
    fn limit_note(calculated_note:i32) -> u8 {
        let mut note = calculated_note;
        while note < 0 { note += 12;}
        while note >= 128 { note -= 12;}
        note as u8
    }
    fn calc_pitch(note:u8, inst_set:usize) -> f32 {
        let tune_note: u8 = Osc::limit_note(note as i32 + msgf_prm::TONE_PRM[inst_set].osc.coarse_tune);
        let solfa_name: u8 = (tune_note + 3)%12;
        let octave: usize = ((tune_note as usize) + 3)/12;
        let mut ap = PITCH_OF_A[octave];
        let ratio = (2_f32.ln()/12_f32).exp();
        for _ in 0..solfa_name {
            ap *= ratio;
        }
        ap *= (msgf_prm::TONE_PRM[inst_set].osc.fine_tune*(2_f32.ln()/1200_f32)).exp();
        ap
    }
    fn pseudo_sine(mut phase:f32) -> f32 {
        // Lagrange interpolation
        while phase > 1.0 { phase -= 1.0 }
        let nrm_phase:f32 = phase * 256.0;
        let phase_locate = nrm_phase.round() as usize;
        let x1 = nrm_phase - phase_locate as f32;
        //let x0 = x1 + 1.0; // cubic interpolation
        //let x2 = x1 - 1.0;
        //let x3 = x1 - 2.0;
        //let mut y = -(x1*x2*x3*SINE_TABLE[phase_locate+1]/6.0) + (x0*x2*x3*SINE_TABLE[phase_locate+2]/2.0)
        //            -(x0*x1*x3*SINE_TABLE[phase_locate+3]/2.0) + (x0*x1*x2*SINE_TABLE[phase_locate+4]/6.0);
        //assert!(phase_locate < 258, "{},{},{},{}:{}->{}", x0,x1,x2,x3,phase_locate,y);
        let y0 = SINE_TABLE[phase_locate+2];                    //  linear interpolation
        let mut y = (SINE_TABLE[phase_locate+3] - y0)*x1 + y0;  //
        if y > 1.0 { y = 1.0 }
        else if y < -1.0 { y = -1.0 }
        y
    }
    pub fn process(&mut self, abuf: &mut msgf_afrm::AudioFrame, lbuf: &mut msgf_cfrm::CtrlFrame) {
        let wave_func: fn(f32, usize) -> f32;
        match self.wv_type {
            WvType::Sine => {
                //  Case of using sine()
                //wave_func = |x, _y| {
                //  let phase = x * 2.0 * general::PI;
                //  phase.sin()
                //}
                wave_func = |x, _y| Osc::pseudo_sine(x);
            }
            WvType::Saw => {
                wave_func = |x, y| {
                    let mut saw: f32 = 0.0;
                    for j in 1..y {
                        let ot:f32 = j as f32;
                        let phase:f32 = x * ot;
                        saw += 0.25*Osc::pseudo_sine(phase)/ot;
                    }
                    saw
                };
            }
            WvType::Square => {
                wave_func = |x, y| {
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
                wave_func = |x, _y| {
                    let mut pls: f32 = 0.0;
                    let ps: f32 = x;
                    if ps < 0.1 { pls = 0.5;}
                    else if ps < 0.2 { pls = -0.5;}
                    pls
                }
            }
        }
        let delta_phase = self.base_pitch/general::SAMPLING_FREQ;
        let mut phase = self.next_phase;
        let max_overtone: usize = (ABORT_FREQUENCY/self.base_pitch) as usize;
        for i in 0..abuf.sample_number {
            abuf.set_abuf(i, wave_func(phase, max_overtone));
            let magnitude = lbuf.ctrl_for_audio(i)*self.lfo_depth;
            phase += delta_phase*(2.0_f32.powf(magnitude));
            while phase > 1.0 { phase -= 1.0 }
        }
        self.next_phase = phase;
    }
}
