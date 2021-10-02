//
//  msgf_afrm.rs
//	Musical Sound Generator Framework
//      Audio Frame Class
//
//  Created by Hasebe Masahiko on 2021/09/24.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::general;

//---------------------------------------------------------
//		Constants
//---------------------------------------------------------
const DAMP_LIMIT_DEPTH: f32 = 0.0001;

//---------------------------------------------------------
//		Class
//---------------------------------------------------------
pub struct AudioFrame {
    abuf: Vec<f32>,
    pub sample_number: usize,
    index: usize,
}

impl AudioFrame {
    pub fn new(sample_number: usize) -> Self {
        Self {
            abuf: vec![0.0; sample_number],
            sample_number,
            index: 0,
        }
    }
    pub fn copy_from_abuf(&self, ab: &mut [f32; general::MAX_BUFFER_SIZE]) {
        for i in 0..self.sample_number {
            ab[i] = self.abuf[i];
        }
    }
    pub fn put_abuf(&mut self, val: f32) {
        self.abuf[self.index] = val;
        self.index += 1;
        if self.index >= self.sample_number {
            self.index = 0;
        }
    }
    pub fn set_abuf(&mut self, num: usize, val: f32) {
        self.abuf[num] = val;
    }
    pub fn add_abuf(&mut self, num: usize, val: f32) {
        self.abuf[num] += val;
    }
    pub fn mul_abuf(&mut self, num: usize, rate: f32) {
        self.abuf[num] *= rate;
    }
    pub fn get_abuf(&self, num: usize) -> f32 { self.abuf[num]}

    pub fn get_max_level(&self) -> f32 {
        let mut max_val: f32 = 0.0;
        for i in 0..self.sample_number {
            let val = self.abuf[i];
            if max_val < val {
                max_val = val;
            }
        }
        max_val
    }
    pub fn _mix(&mut self, srcbuf: &AudioFrame) {
        for i in 0..self.sample_number {
            let val: f32 = srcbuf.get_abuf(i);
            self.add_abuf(i, val);
        }
    }

    pub fn mix_and_check_no_sound(&mut self, srcbuf: &AudioFrame) -> bool {
        let mut cnt: usize = 0;
        for i in 0..self.sample_number {
            let val: f32 = srcbuf.get_abuf(i);
            self.add_abuf(i, val);
            if val < DAMP_LIMIT_DEPTH {
                cnt += 1;
            }
        }
        if cnt >= self.sample_number {
            true
        } else {
            false
        }
    }
}
