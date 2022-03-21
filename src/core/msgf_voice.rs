//
//  msgf_voice.rs
//	Musical Sound Generator Framework
//      Voice Class
//
//  Created by Hasebe Masahiko on 2021/09/18.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::core::*;

//---------------------------------------------------------
//		Constants
//---------------------------------------------------------
#[derive(PartialEq, Clone, Copy)]
pub enum NoteStatus {
    DuringNoteOn,
    AfterNoteOff,
    DuringDamp,
}

//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub trait Voice {
    fn start_sound(&mut self);
    fn slide(&mut self, _note: u8, _vel: u8){}
    fn note_off(&mut self);
    fn damp(&mut self);
    fn change_pmd(&mut self, value: f32);
    fn amplitude(&mut self, volume: u8, expression: u8);
    fn pitch(&mut self, pitch:f32);
    fn status(&self) -> NoteStatus;
    fn process(&mut self, abuf: &mut msgf_afrm::AudioFrame, in_number_frames: usize) -> bool;
    fn note_num(&self) -> u8;
    fn velocity(&self) -> u8;
    fn set_prm(&mut self, prm_type: u8, value: u8);
}