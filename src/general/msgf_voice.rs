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
use crate::general;
use crate::general::*;
use crate::engine::*;

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
//		Class
//---------------------------------------------------------
pub struct Voice {
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
    ended: bool,
}

impl PartialEq for Voice {
    fn eq(&self, other: &Self) -> bool {
        self.note == other.note && self.vel == other.vel
    }
}

impl Voice {
    pub fn new(note:u8, vel:u8, inst_set:usize, pmd:f32, vol:u8, exp:u8) -> Voice {
        Self {
            note,
            vel,
            status: NoteStatus::DuringNoteOn,
            damp_counter: 0,
            lvl_check_buf: msgf_afrm::AudioFrame::new((general::SAMPLING_FREQ/100.0) as usize),
            osc: msgf_osc::Osc::new(note, inst_set, pmd),
            aeg: msgf_aeg::Aeg::new(inst_set),
            lfo: msgf_lfo::Lfo::new(inst_set),
            max_note_vol: Voice::calc_vol(vol, exp),
            ended: false,
        }
    }
    fn calc_vol(vol:u8, exp:u8) -> f32 {
        let exp_sq = exp as f32;
        let vol_sq = vol as f32;
        let total_vol = 0.5f32.powf(4.0);    // 4bit margin
        (total_vol*vol_sq*exp_sq)/16384.0
    }
    pub fn start_sound(&mut self) {
        self.aeg.move_to_attack();
        self.lfo.start();
    }
    pub fn note_off(&mut self) {
        self.status = NoteStatus::AfterNoteOff;
        self.aeg.move_to_release()
    }
    pub fn note_num(&self) -> u8 {self.note}
    pub fn _velocity(&self) -> u8 {self.vel}
    pub fn change_pmd(&mut self, value: f32) {
        self.osc.change_pmd(value);
    }
    pub fn amplitude(&mut self, volume: u8, expression: u8) {
        self.max_note_vol = Voice::calc_vol(volume, expression);
    }
    pub fn status(&self) -> NoteStatus {self.status}
    pub fn damp(&mut self) {
        self.status = NoteStatus::DuringDamp;
        self.damp_counter = 0;
    }
    fn manage_note_level(&mut self, abuf: &mut msgf_afrm::AudioFrame, aegbuf: &mut msgf_cfrm::CtrlFrame) -> bool {
        if self.status != NoteStatus::DuringDamp {
            //	Check Level
            let level = aegbuf.get_max_level();
            self.lvl_check_buf.put_abuf(level);
            if general::DAMP_LIMIT_DEPTH > level {
                println!("Damped!");
                self.damp();
            }
        } else {    //	Damp
            for snum in 0..abuf.sample_number {
                let mut rate: f32 = 0.0;
                if self.damp_counter <= DAMP_TIME {
                    let cntdwn = DAMP_TIME - self.damp_counter;
                    rate = (cntdwn as f32)/(DAMP_TIME as f32);
                    rate *= rate;
                }
                abuf.mul_abuf(snum, rate);
                self.damp_counter += 1;
                if self.damp_counter > DAMP_TIME {
                    self.ended = true;
                    break;
                }
            }
        }
        self.ended
    }
    pub fn process(&mut self, abuf: &mut msgf_afrm::AudioFrame, in_number_frames: usize) -> bool {
        if self.ended {return self.ended;}

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
        self.manage_note_level(abuf, aegbuf)
    }
}