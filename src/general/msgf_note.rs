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
use crate::general::msgf_cfrm;
use crate::engine::msgf_aeg;
use crate::engine::msgf_osc;
use crate::engine::msgf_lfo;

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
    // Note
    note: u8,
    vel: u8,
    status: NoteStatus,
    damp_counter: u32,
    lvl_check_buf: msgf_afrm::AudioFrame,
    // Synth
    osc: msgf_osc::Osc,
    aeg: msgf_aeg::Aeg,
    lfo: msgf_lfo::Lfo,
    max_note_vol: f32,
}

const DAMP_RATE: u32 = 400;		// * dac time(22.68usec)

impl PartialEq for Note {
    fn eq(&self, other: &Self) -> bool {
        self.note == other.note && self.vel == other.vel
    }
}

impl Note {
    pub fn new(note: u8, vel: u8, inst_set: usize) -> Note {
        Self {
            note,
            vel,
            status: NoteStatus::DuringNoteOn,
            damp_counter: 0,
            lvl_check_buf: msgf_afrm::AudioFrame::new((general::SAMPLING_FREQ/100.0) as usize),
            osc: msgf_osc::Osc::new(note, inst_set),
            aeg: msgf_aeg::Aeg::new(inst_set),
            lfo: msgf_lfo::Lfo::new(inst_set),
            max_note_vol: 0.5f32.powf(4.0), // 4bit margin
        }
    }
    pub fn start_sound(&mut self) {
        self.aeg.move_to_attack();
    }
    pub fn note_off(&mut self) {
        self.status = NoteStatus::AfterNoteOff;
        self.aeg.move_to_release()
    }
    pub fn note_num(&self) -> u8 {self.note}
    pub fn _velocity(&self) -> u8 {self.vel}
    pub fn status(&self) -> NoteStatus {self.status}
    pub fn damp(&mut self) {
        self.status = NoteStatus::DuringDamp;
        self.damp_counter = 0;
    }
    fn manage_note_level(&mut self, abuf: &mut msgf_afrm::AudioFrame, aegbuf: &mut msgf_cfrm::CtrlFrame) {
        if self.status != NoteStatus::DuringDamp {
            //	Check Level
            let level = aegbuf.get_max_level();
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
    pub fn process(&mut self, abuf: &mut msgf_afrm::AudioFrame, in_number_frames: usize) {
        //  Pitch Control
        let cbuf_size = msgf_cfrm::CtrlFrame::get_cbuf_size(in_number_frames);
        let lbuf = &mut msgf_cfrm::CtrlFrame::new(cbuf_size);

        //  LFO
        self.lfo.process(lbuf);

        //  Oscillator
        self.osc.process(abuf, lbuf);

        //  AEG
        let aegbuf = &mut msgf_cfrm::CtrlFrame::new(cbuf_size);
        self.aeg.process(aegbuf);

        //  Volume
        for i in 0..abuf.sample_number {
            let aeg = aegbuf.ctrl_for_audio(i);
            abuf.mul_abuf(i, self.max_note_vol*aeg);
        }
        self.manage_note_level(abuf, aegbuf);
    }
}