//
//  sgf_voice.rs
//	Musical Sound Generator Framework
//      Sing Voice Class
//
//  Created by Hasebe Masahiko on 2022/06/11.
//  Copyright (c) 2022 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use std::rc::Rc;
use std::cell::Cell;
use crate::msgf_if;
use crate::core::*;
use crate::core::msgf_voice::*;
use crate::engine::*;
use crate::app::sgf::*;

//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub struct VoiceSgf {
    // Note
    note: u8,
    vel: u8,
    status: NoteStatus,
    damp_counter: u32,
    lvl_check_buf: msgf_afrm::AudioFrame,
    // Synth
    osc: msgf_additive::Additive,
    flt: msgf_biquad::Biquad,
    aeg: msgf_aeg::Aeg,
    lfo: msgf_lfo::Lfo,
    max_note_vol: f32,
    ended: bool,
    vowel_x: f32,   // -1..0..1
    vowel_y: f32,   // -1..0..1
}
//---------------------------------------------------------
//		Implements
//---------------------------------------------------------
impl PartialEq for VoiceSgf {
    fn eq(&self, other: &Self) -> bool {
        self.note == other.note && self.vel == other.vel
    }
}
//---------------------------------------------------------
impl msgf_voice::Voice for VoiceSgf {
    fn start_sound(&mut self) {
        self.aeg.move_to_attack();
        self.flt.set_lpf(1000.0, 8.0);
        self.lfo.start();
    }
    fn slide(&mut self, note:u8, vel:u8) {
        self.note = note;
        self.vel = vel;
        self.status = NoteStatus::DuringNoteOn;
        self.damp_counter = 0;
        self.osc.change_note(note);
        self.aeg.move_to_attack();
        self.lfo.start();
    }
    fn note_off(&mut self) {
        self.status = NoteStatus::AfterNoteOff;
        self.aeg.move_to_release()
    }
    fn note_num(&self) -> u8 {self.note}
    fn velocity(&self) -> u8 {self.vel}
    fn change_pmd(&mut self, value: f32) {
        self.osc.change_pmd(value);
    }
    fn amplitude(&mut self, volume: u8, expression: u8) {
        self.max_note_vol = VoiceSgf::calc_vol(volume, expression);
    }
    fn pitch(&mut self, pitch:f32) {
        self.osc.change_pitch(pitch);
    }
    fn status(&self) -> NoteStatus {self.status}
    fn damp(&mut self) {
        self.status = NoteStatus::DuringDamp;
        self.damp_counter = 0;
    }
    fn process(&mut self, abuf: &mut msgf_afrm::AudioFrame, in_number_frames: usize) -> bool {
        if self.ended {return self.ended;}

        //  Pitch Control
        let cbuf_size = msgf_cfrm::CtrlFrame::get_cbuf_size(in_number_frames);
        let lbuf = &mut msgf_cfrm::CtrlFrame::new(cbuf_size);

        //  LFO
        self.lfo.process(lbuf);

        //  Oscillator
        self.osc.process(abuf, lbuf);

        //  Filter
        self.flt.process(abuf);

        //  AEG
        let aegbuf = &mut msgf_cfrm::CtrlFrame::new(cbuf_size);
        self.aeg.process(aegbuf);

        //  Volume
        for i in 0..abuf.sample_number {
            let aeg = aegbuf.ctrl_for_audio(i);
            abuf.mul_rate(i, self.max_note_vol*aeg);
        }
        msgf_voice::manage_note_level(self, abuf, aegbuf)
    }
    fn set_prm(&mut self, prm_type: u8, value: u8) {
        match prm_type {
            0 => self.lfo.set_freq(value),  // 16 : LFO freq.
            1 => self.lfo.set_wave(value),  // 17 : LFO Wave
            2 => {self.vowel_x = (value as f32-64.0)/64.0; self.calc_formant();}
            3 => {self.vowel_y = (value as f32-64.0)/64.0; self.calc_formant();}
            _ => ()
        }
    }
    fn put_lvl_check_buf(&mut self, lvl: f32) {self.lvl_check_buf.put_into_abuf(lvl);}
    fn damp_counter(&self) -> u32 {self.damp_counter}
    fn inc_damp_counter(&mut self) {self.damp_counter+=1;}
    fn ended(&self) -> bool {self.ended}
    fn set_ended(&mut self, which: bool) {self.ended = which;}
}

impl VoiceSgf {
    pub fn new(note:u8, vel:u8, _pmd:f32, pit:f32, vol:u8, exp:u8,
        inst_prm: Rc<Cell<sgf_prm::SynthParameter>>) -> Self {
        let tprm: &sgf_prm::SynthParameter = &inst_prm.get();
        Self {
            note,
            vel,
            status: NoteStatus::DuringNoteOn,
            damp_counter: 0,
            lvl_check_buf: msgf_afrm::AudioFrame::new((msgf_if::SAMPLING_FREQ/100.0) as usize, msgf_if::MAX_BUFFER_SIZE),
            osc: msgf_additive::Additive::new(&tprm.osc, note, pit),
            flt: msgf_biquad::Biquad::new(),
            aeg: msgf_aeg::Aeg::new(&tprm.aeg),
            lfo: msgf_lfo::Lfo::new(&tprm.lfo),
            max_note_vol: VoiceSgf::calc_vol(vol, exp),
            ended: false,
            vowel_x: 0.0,
            vowel_y: 0.0,
        }
    }
    fn calc_vol(vol:u8, exp:u8) -> f32 {
        let exp_sq = exp as f32;
        let vol_sq = vol as f32;
        let total_vol = 0.5f32.powf(4.0);    // 4bit margin
        (total_vol*vol_sq*exp_sq)/16384.0
    }
    fn calc_formant(&mut self) {
        //  (0,0): a, (1,0):e, (-1,0):i, (0,1):u, (0,-1):o
        let mut f1 = 800.0;
        let mut f2 = 1200.0;
        if self.vowel_x == 0.0 && self.vowel_y == 0.0 {}
        else if self.vowel_y > self.vowel_x {
            if self.vowel_y > -self.vowel_x {       /*a-u*/
                f1-=500.0*self.vowel_y;
            }
            else {  /*a-i*/
                f1+=500.0*self.vowel_x;
                f2+=1100.0*self.vowel_x;
            }
        } else {
            if self.vowel_y > -self.vowel_x {       /*a-e*/
                f1-=300.0*self.vowel_x;
                f2+=700.0*self.vowel_x;
            }
            else {  /*a-o*/
                f1+=300.0*self.vowel_y;
                f2+=300.0*self.vowel_y;
            }
        }
        self.osc.change_f1(f1);
        self.osc.change_f2(f2);
    }
}
