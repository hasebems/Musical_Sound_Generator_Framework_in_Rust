//
//  msgf_inst.rs
//	Musical Sound Generator Framework
//      Instrument Trait
//
//  Created by Hasebe Masahiko on 2021/09/18.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::general::*;

pub trait Inst {
    fn change_inst(&mut self, inst_number: usize, vol: u8, pan: u8, exp: u8);
    fn note_off(&mut self, dt2: u8, dt3: u8);
    fn note_on(&mut self, dt2: u8, dt3: u8);
    fn modulation(&mut self, value: u8);
    fn volume(&mut self, value: u8);
    fn pan(&mut self, value: u8);
    fn expression(&mut self, value: u8);
    fn pitch(&mut self, bend:i16, tune_coarse:u8, tune_fine:u8);
    fn sustain(&mut self, value: u8);
    fn all_sound_off(&mut self);
    //fn release_note(&mut self, nt: &msgf_voice::Voice);
    fn process(&mut self,
        abuf_l: &mut msgf_afrm::AudioFrame,
        abuf_r: &mut msgf_afrm::AudioFrame,
        in_number_frames: usize);
}