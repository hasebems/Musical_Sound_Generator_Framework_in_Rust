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
const DEFAULT_F1:f32 = 800.0;
const DEFAULT_F2:f32 = 1200.0;
const BPF_RESO:f32 = 3.0;
const NOTE_OFFSET:u8 = 24;
//---------------------------------------------------------
pub struct VoiceSgf {
    // Note
    note: u8,
    vel: u8,
    status: NoteStatus,
    damp_counter: u32,
    lvl_check_buf: msgf_afrm::AudioFrame,
    // Synth
    vcl: msgf_vocal::Vocal,
    lpf: msgf_biquad::Biquad,
    frm1: msgf_biquad::Biquad,
    frm2: msgf_biquad::Biquad,
    aeg: msgf_aeg::Aeg,
    lfo: msgf_lfo::Lfo,
    max_note_vol: f32,
    ended: bool,
    vowel_x: f32,   // -1..0..1
    vowel_y: f32,   // -1..0..1
    fmnt_adjust_vol: f32,   //  0..1
    scl_adjust_vol: f32,    // 0..1
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
        self.lpf.set_thru();
        self.frm1.set_bpf(DEFAULT_F1, BPF_RESO);
        self.frm2.set_bpf(DEFAULT_F2, BPF_RESO);
        self.lfo.start();
    }
    fn slide(&mut self, note:u8, vel:u8) {
        let real_note = note - NOTE_OFFSET;
        self.note = real_note;
        self.vel = vel;
        self.status = NoteStatus::DuringNoteOn;
        self.damp_counter = 0;
        self.vcl.change_note(note- NOTE_OFFSET);
        self.aeg.move_to_attack();
        self.lfo.start();
        self.scl_adjust_vol = VoiceSgf::calc_scaling_vol(real_note);
    }
    fn note_off(&mut self) {
        self.status = NoteStatus::AfterNoteOff;
        self.aeg.move_to_release()
    }
    fn note_num(&self) -> u8 {self.note + NOTE_OFFSET}
    fn velocity(&self) -> u8 {self.vel}
    fn change_pmd(&mut self, value: f32) {
        self.vcl.change_pmd(value);
    }
    fn amplitude(&mut self, volume: u8, expression: u8) {
        self.max_note_vol = VoiceSgf::calc_vol(volume, expression);
    }
    fn pitch(&mut self, pitch:f32) {
        self.vcl.change_pitch(pitch);
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
        self.vcl.process(abuf, lbuf);

        //  Filter
        self.lpf.process(abuf);
        self.frm1.process(abuf);
        self.frm2.process(abuf);

        //  AEG
        let aegbuf = &mut msgf_cfrm::CtrlFrame::new(cbuf_size);
        self.aeg.process(aegbuf);

        //  Volume
        let tmpvol = self.max_note_vol*self.fmnt_adjust_vol*self.scl_adjust_vol;
        for i in 0..abuf.sample_number {
            let aeg = aegbuf.ctrl_for_audio(i);
            abuf.mul_rate(i, tmpvol*aeg);
        }
        msgf_voice::manage_note_level(self, abuf, aegbuf)
    }
    fn set_prm(&mut self, prm_type: u8, value: u8) {
        match prm_type {
            0 => self.lpf.set_lpf((value as f32)*20.0,1.0),  // 16 : LPF cutoff
            //1 => self.lfo.set_wave(value),  // 17 : LFO Wave
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
    pub fn new(org_note:u8, vel:u8, pmd:f32, pit:f32, vol:u8, exp:u8,
        inst_prm: Rc<Cell<sgf_prm::SynthParameter>>) -> Self {
        let tprm: &sgf_prm::SynthParameter = &inst_prm.get();
        let real_note = org_note - NOTE_OFFSET;
        Self {
            note: real_note,
            vel,
            status: NoteStatus::DuringNoteOn,
            damp_counter: 0,
            lvl_check_buf: msgf_afrm::AudioFrame::new((msgf_if::SAMPLING_FREQ/100.0) as usize, msgf_if::MAX_BUFFER_SIZE),
            vcl: msgf_vocal::Vocal::new(&tprm.osc, real_note, pmd, pit),
            lpf: msgf_biquad::Biquad::new(),
            frm1: msgf_biquad::Biquad::new(),
            frm2: msgf_biquad::Biquad::new(),
            aeg: msgf_aeg::Aeg::new(&tprm.aeg),
            lfo: msgf_lfo::Lfo::new(&tprm.lfo),
            max_note_vol: VoiceSgf::calc_vol(vol, exp),
            ended: false,
            vowel_x: 0.0,
            vowel_y: 0.0,
            fmnt_adjust_vol: 1.0,
            scl_adjust_vol: VoiceSgf::calc_scaling_vol(real_note),
        }
    }
    fn calc_scaling_vol(note:u8) -> f32 {
        1.0 - 0.01*((note as f32)-60.0)
    }
    fn calc_vol(vol:u8, exp:u8) -> f32 {
        let exp_sq = exp as f32;
        let vol_sq = vol as f32;
        let total_vol = 2.0; //0.5f32.powf(4.0);    // 4bit margin
        (total_vol*vol_sq*exp_sq)/16384.0
    }
    fn calc_formant(&mut self) {
        //  (0,0): a, (1,0):e, (-1,0):i, (0,1):u, (0,-1):o
        let mut f1 = DEFAULT_F1;
        let mut f2 = DEFAULT_F2;
        if self.vowel_x == 0.0 && self.vowel_y == 0.0 {
            self.fmnt_adjust_vol = 1.0;
        }
        else if self.vowel_y > self.vowel_x {
            if self.vowel_y > -self.vowel_x {   /*a->u*/
                f1-=500.0*self.vowel_y; // y:0->1, 800->300
                self.fmnt_adjust_vol = 1.0 + self.vowel_y*0.3;
            }
            else {                              /*a->i*/
                f1+=500.0*self.vowel_x; // x:0->-1, 800->300
                f2-=1100.0*self.vowel_x;// 1200->2300
                self.fmnt_adjust_vol = 1.0 - self.vowel_x*0.3;
            }
        } else {
            if self.vowel_y > -self.vowel_x {   /*a->e*/
                f1-=300.0*self.vowel_x; // x:0->1, 800->500
                f2+=700.0*self.vowel_x; // 1200->1900
                self.fmnt_adjust_vol = 1.0;
            }
            else {                              /*a->o*/
                f1+=300.0*self.vowel_y;  // y:0->-1, 800->500
                f2+=400.0*self.vowel_y;  // 1200->800
                self.fmnt_adjust_vol = 1.0 + self.vowel_y*0.5;
            }
        }
        self.frm1.set_bpf(f1, BPF_RESO);
        self.frm2.set_bpf(f2, BPF_RESO);
    }
}
