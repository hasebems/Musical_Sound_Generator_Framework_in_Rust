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
use crate::msgf_if;

//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub struct AudioFrame {
    abuf: Vec<f32>,
    pub sample_number: usize,
    index: usize,
}
//---------------------------------------------------------
//		Implements
//---------------------------------------------------------
impl AudioFrame {
    pub fn new(mut sample_number: usize, total_size: usize) -> Self {
        if sample_number > total_size {
            sample_number = total_size;
        }
        Self {
            abuf: vec![0.0; total_size],
            sample_number,
            index: 0,
        }
    }
    pub fn set_sample_number(&mut self, snum: usize) {
        self.sample_number = snum;      
    }
    pub fn copy_to_sysbuf(&self, ab: &mut [f32; msgf_if::MAX_BUFFER_SIZE]) {
        for i in 0..self.sample_number {
            ab[i] = self.abuf[i];
        }
    }
    pub fn add_to_sysbuf(&self, ab: &mut [f32; msgf_if::MAX_BUFFER_SIZE]) {
        for i in 0..self.sample_number {
            ab[i] += self.abuf[i];
        }
    }
    pub fn copy_to_abuf(&self, ab: &mut AudioFrame) {
        for i in 0..self.sample_number {
            ab.abuf[i] = self.abuf[i];
        }
    }
    pub fn put_into_abuf(&mut self, val: f32) {
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
    pub fn clr_abuf(&mut self) {
        for i in 0..self.sample_number {
            self.abuf[i] = 0.0;
        }
    }
    pub fn set_abuf(&mut self, num: usize, val: f32) {
        let newval = Self::limit_check(val, 0.0);
        self.abuf[num] = newval;
    }
    pub fn add_to_abuf(&mut self, num: usize, val: f32) {
        let newval = Self::limit_check(self.abuf[num], val);
        self.abuf[num] = newval;
    }
    pub fn mul_abuf(&mut self, num: usize, rate: f32) {
        let newval = Self::limit_check(self.abuf[num]*rate, 0.0);
        self.abuf[num] = newval;
    }
    pub fn get_from_abuf(&self, num: usize) -> Option<f32> {
        if num >= self.sample_number {
            return None;
        }
        Some(self.abuf[num])
    }
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
            if let Some(src_dt) = srcbuf.get_from_abuf(i) {
                let val: f32 = src_dt*mul_value;
                self.add_to_abuf(i, val);
            }
        }
    }
    pub fn _mix_and_check_no_sound(&mut self, srcbuf: &AudioFrame) -> bool {
        let mut cnt: usize = 0;
        for i in 0..self.sample_number {
            if let Some(val) = srcbuf.get_from_abuf(i) {
                self.add_to_abuf(i, val);
                if val < msgf_if::DAMP_LIMIT_DEPTH {
                    cnt += 1;
                }
            }
        }
        if cnt >= self.sample_number {
            true
        } else {
            false
        }
    }
}
