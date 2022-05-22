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
use crate::msgf_if;

//---------------------------------------------------------
//		Constants
//---------------------------------------------------------
#[derive(PartialEq, Clone, Copy)]
pub enum NoteStatus {
    DuringNoteOn,
    AfterNoteOff,
    DuringDamp,
}
const DAMP_TIME: u32 = 300;		// * dac time(22.68usec)

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

    //  Setter/Getter
    fn put_lvl_check_buf(&mut self, lvl: f32);
    fn damp_counter(&self) -> u32;
    fn add_damp_counter(&mut self, num: u32);
    fn ended(&self) -> bool;
    fn set_ended(&mut self, which: bool);
}
//---------------------------------------------------------
//		Trait Bound
//---------------------------------------------------------
pub fn manage_note_level<T: Voice>(t: &mut T, 
    abuf:   &mut msgf_afrm::AudioFrame,
    aegbuf: &mut msgf_cfrm::CtrlFrame) -> bool {
    if t.status() != NoteStatus::DuringDamp {
        //	Check Level
        let level = aegbuf.get_max_level();
        t.put_lvl_check_buf(level);
        if msgf_if::DAMP_LIMIT_DEPTH > level {
            println!("Damped!");
            t.damp();
        }
    } else {    //	Damp
        for snum in 0..abuf.sample_number {
            let mut rate: f32 = 0.0;
            if t.damp_counter() <= DAMP_TIME {
                let cntdwn = DAMP_TIME - t.damp_counter();
                rate = (cntdwn as f32)/(DAMP_TIME as f32);
                rate *= rate;
            }
            abuf.mul_rate(snum, rate);
            t.add_damp_counter(1);
            if t.damp_counter() > DAMP_TIME {
                t.set_ended(true);
                break;
            }
        }
    }
    t.ended()
}
