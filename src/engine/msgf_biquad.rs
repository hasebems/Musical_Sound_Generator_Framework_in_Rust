//
//  msgf_biquad.rs
//	Musical Sound Generator Framework
//      Biquad Class
//
//  Created by Hasebe Masahiko on 2022/06/04.
//  Copyright (c) 2022 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::msgf_if;
use crate::core::*;
use crate::engine::msgf_gen::*;

//---------------------------------------------------------
//		Synth. Parameter
//---------------------------------------------------------
#[derive(Copy, Clone)]
pub struct BiquadParameter {
    pub freq: f32,
    pub resonance: f32,
}
//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub struct Biquad {
    a1: f32,
    a2: f32,
    b0: f32,
    b1: f32,
    b2: f32,
    x_z1: f32,
    x_z2: f32,
    y_z1: f32,
    y_z2: f32,
}
//---------------------------------------------------------
//		Implements
//---------------------------------------------------------
impl Biquad {
    pub fn new() -> Biquad {
        Self {
            a1: 0.0,
            a2: 0.0,
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,
            x_z1: 0.0,
            x_z2: 0.0,
            y_z1: 0.0,
            y_z2: 0.0,
        }
    }
    fn calc_analog_cutoff(&self, fd: f32) -> f32 {
        // cutoff : fd/fs = digital cutoff/sampling freq
        (msgf_if::PI*fd/msgf_if::SAMPLING_FREQ).tan()/(2.0*msgf_if::PI)
    }
    pub fn set_thru(&mut self) {
        self.a1 = 0.0;
        self.a2 = 0.0;
        self.b0 = 1.0;
        self.b1 = 0.0;
        self.b2 = 0.0;
    }
    pub fn set_lpf(&mut self, cutoff:f32, reso:f32) {
        let fa = self.calc_analog_cutoff(cutoff);
        let sqfc = fa * fa;
        let sqpi = msgf_if::PI * msgf_if::PI;
        let a0 = 1.0 + (2.0 * msgf_if::PI * fa)/reso + 4.0 * sqpi * sqfc;
        self.a1 = (8.0 * sqpi * sqfc - 2.0)/a0;
        self.a2 = (1.0 - 2.0 * msgf_if::PI * fa/reso + 4.0 * sqpi * sqfc)/a0;
        self.b0 = (4.0 * sqpi * sqfc)/a0;
        self.b1 = (8.0 * sqpi * sqfc)/a0;
        self.b2 = self.b0;
    }
    pub fn set_bpf(&mut self, cutoff:f32, reso:f32) {
        let fa = self.calc_analog_cutoff(cutoff);
        let sqfc = fa * fa;
        let sqpi = msgf_if::PI * msgf_if::PI;
        let a0 = 1.0 + (2.0 * msgf_if::PI * fa)/reso + 4.0 * sqpi * sqfc;
        self.a1 = (8.0 * sqpi * sqfc - 2.0)/a0;
        self.a2 = (1.0 - (2.0 * msgf_if::PI * fa/reso) + 4.0 * sqpi * sqfc)/a0;
        self.b0 = (2.0 * msgf_if::PI * fa/reso)/a0;
        self.b1 = 0.0;
        self.b2 = -self.b0;
    }
    fn core_job(&mut self, input: f32) -> f32 {
		let mut output: f32 = self.b0*input + self.b1*self.x_z1 + self.b2*self.x_z2;
		output += - self.a1*self.y_z1 - self.a2*self.y_z2;
        self.x_z2 = self.x_z1;
        self.x_z1 = input;
        self.y_z2 = self.y_z1;
        self.y_z1 = output;
        output
    }
}
impl Engine for Biquad {
    fn process_a(&mut self, abuf: &mut msgf_afrm::AudioFrame) {
        for i in 0..abuf.sample_number {
            if let Some(x) = abuf.get_from_abuf(i){
                abuf.set_val(i, self.core_job(x));
            }
        }
    }
}