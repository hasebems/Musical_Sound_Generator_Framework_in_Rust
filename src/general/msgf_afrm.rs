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
    pub fn copy_to_sysbuf(&self, ab: &mut [f32; general::MAX_BUFFER_SIZE]) {
        for i in 0..self.sample_number {
            ab[i] = self.abuf[i];
        }
    }
    pub fn copy_to_abuf(&self, ab: &mut AudioFrame) {
        for i in 0..self.sample_number {
            ab.abuf[i] = self.abuf[i];
        }
    }
    pub fn put_abuf(&mut self, val: f32) {
        self.abuf[self.index] = val;
        self.index += 1;
        if self.index >= self.sample_number {
            self.index = 0;
        }
    }
    fn limit_check(val1: f32, val2: f32) -> f32 {
        let mut newval = val1 + val2;
        if newval > 1.0 {
            newval = 0.99;
        } else if newval < -1.0 {
            newval = -0.99;
        };
        newval
    }
    pub fn _clr_abuf(&mut self) {
        for i in 0..self.sample_number {
            self.abuf[i] = 0.0;
        }
    }
    pub fn set_abuf(&mut self, num: usize, val: f32) {
        let newval = Self::limit_check(val, 0.0);
        self.abuf[num] = newval;
    }
    pub fn add_abuf(&mut self, num: usize, val: f32) {
        let newval = Self::limit_check(self.abuf[num], val);
        self.abuf[num] = newval;
    }
    pub fn mul_abuf(&mut self, num: usize, rate: f32) {
        let newval = Self::limit_check(self.abuf[num]*rate, 0.0);
        self.abuf[num] = newval;
    }
    pub fn get_abuf(&self, num: usize) -> f32 { self.abuf[num]}
    pub fn _get_max_level(&self) -> f32 {
        let mut max_val: f32 = 0.0;
        for i in 0..self.sample_number {
            let val = self.abuf[i];
            if max_val < val {
                max_val = val;
            }
        }
        max_val
    }
    pub fn mul_and_mix(&mut self, srcbuf: &AudioFrame, mul_value:f32) {
        for i in 0..self.sample_number {
            let val: f32 = srcbuf.get_abuf(i)*mul_value;
            self.add_abuf(i, val);
        }
    }
    pub fn _mix_and_check_no_sound(&mut self, srcbuf: &AudioFrame) -> bool {
        let mut cnt: usize = 0;
        for i in 0..self.sample_number {
            let val: f32 = srcbuf.get_abuf(i);
            self.add_abuf(i, val);
            if val < general::DAMP_LIMIT_DEPTH {
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
