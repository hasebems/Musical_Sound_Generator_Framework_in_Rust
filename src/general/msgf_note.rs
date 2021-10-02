//
//  msgf_note.rs
//	Musical Sound Generator Framework
//      Note Class
//
//  Created by Hasebe Masahiko on 2021/09/18.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::general;
use crate::general::msgf_afrm;
use crate::general::msgf_synth;

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
//		Class
//---------------------------------------------------------
pub struct Note {
    note: u8,
    vel: u8,
    status: NoteStatus,
    damp_counter: u32,
    lvl_check_buf: msgf_afrm::AudioFrame,
    synth: msgf_synth::Synth,
}

const DAMP_RATE: u32 = 400;		// * dac time(22.68usec)

impl PartialEq for Note {
    fn eq(&self, other: &Self) -> bool {
        self.note == other.note && self.vel == other.vel
    }
}

impl Note {
    pub fn new(note: u8, vel: u8) -> Note {
        Self {
            note,
            vel,
            status: NoteStatus::DuringNoteOn,
            damp_counter: 0,
            lvl_check_buf: msgf_afrm::AudioFrame::new((general::SAMPLING_FREQ/100.0) as usize),
            synth: msgf_synth::Synth::new(note),
        }
    }
    pub fn start_sound(&mut self) {
        self.synth.move_to_attack();
    }
    pub fn note_off(&mut self) {
        self.status = NoteStatus::AfterNoteOff;
        self.synth.move_to_release();
    }
    pub fn note_num(&self) -> u8 {self.note}
    pub fn _velocity(&self) -> u8 {self.vel}
    pub fn status(&self) -> NoteStatus {self.status}
    pub fn damp(&mut self) {
        self.status = NoteStatus::DuringDamp;
        self.damp_counter = 0;
    }

    //---------------------------------------------------------
    //		Manage Note Level
    //---------------------------------------------------------
    pub fn manage_note_level(&mut self, abuf: &mut msgf_afrm::AudioFrame) {
        if self.status != NoteStatus::DuringDamp {
            //	Check Level
            let level = abuf.get_max_level();
            self.lvl_check_buf.put_abuf(level);
        } else {    //	Damp
            for snum in 0..abuf.sample_number {
                let mut rate: f32 = 0.0;
                if self.damp_counter <= DAMP_RATE {
                    rate = ((DAMP_RATE as f32) - (self.damp_counter as f32))/(DAMP_RATE as f32);
                    rate *= rate;
                }
                abuf.mul_abuf(snum, rate);
                self.damp_counter += 1;
            }
        }
    }
    pub fn process(&mut self, abuf: &mut msgf_afrm::AudioFrame) {
        self.synth.process(abuf);
        self.manage_note_level(abuf);
    }
}