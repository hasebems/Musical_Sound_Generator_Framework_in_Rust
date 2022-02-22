//
//  msgf_cfrm.rs
//	Musical Sound Generator Framework
//      Control Frame Class
//
//  Created by Hasebe Masahiko on 2021/10/24.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::msgf_if;

//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub struct CtrlFrame {
    cbuf: Vec<f32>,
    pub sample_number: usize,
}
//---------------------------------------------------------
//		Implements
//---------------------------------------------------------
impl CtrlFrame {
    pub fn new(sample_number: usize) -> Self {
        Self {
            cbuf: vec![0.0; sample_number],
            sample_number,
        }
    }
    pub fn get_cbuf_size(in_number_frames: usize) -> usize {
        in_number_frames/msgf_if::AUDIO_FRAME_PER_CONTROL
    }
    pub fn set_cbuf(&mut self, num: usize, val: f32) {
        self.cbuf[num] = val;
    }
    pub fn _add_cbuf(&mut self, num: usize, val: f32) {
        self.cbuf[num] += val;
    }
    pub fn _mul_cbuf(&mut self, num: usize, rate: f32) {
        self.cbuf[num] *= rate;
    }
    pub fn _get_cbuf(&self, num: usize) -> f32 { self.cbuf[num]}
    pub fn ctrl_for_audio(&self, num: usize) -> f32 { // for Audio Buffer
        self.cbuf[num/msgf_if::AUDIO_FRAME_PER_CONTROL]
    }
    pub fn get_max_level(&self) -> f32 {
        let mut max_val: f32 = 0.0;
        for i in 0..self.sample_number {
            let val = self.cbuf[i];
            if max_val < val {
                max_val = val;
            }
        }
        max_val
    }
}
