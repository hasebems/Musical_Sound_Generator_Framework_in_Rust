//
//  msgf_inst.rs
//	Musical Sound Generator Framework
//      Instrument Class
//
//  Created by Hasebe Masahiko on 2021/09/18.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::general::*;
use crate::general::msgf_voice::NoteStatus;
use crate::app::msgf_prm;

//---------------------------------------------------------
//		Class
//---------------------------------------------------------
pub struct Inst {
    ntvec: Vec<msgf_voice::Voice>,
    inst_number: usize,
    mdlt: u8,
    vol: u8,
    pan: u8,
    exp: u8,
}

impl Inst {
    pub fn new(mut inst_number: usize, mdlt: u8, vol: u8, pan: u8, exp: u8) -> Self {
        let max_tone = msgf_prm::MAX_TONE_COUNT;
        if inst_number >= max_tone {
            inst_number = max_tone-1;
        }
        Self {
            ntvec: Vec::new(),
            inst_number,
            mdlt,
            vol,
            pan,
            exp,
        }
    }
    pub fn note_off(&mut self, dt2: u8, _dt3: u8) {
        let nt_opt = self.search_note(dt2, NoteStatus::DuringNoteOn);
        if let Some(nt) = nt_opt {
            nt.note_off();
        }
    }
    pub fn note_on(&mut self, dt2: u8, dt3: u8) {
        let mut new_voice = msgf_voice::Voice::new(dt2, dt3, self.inst_number, 
            self.mdlt, self.vol, self.pan, self.exp);
        new_voice.start_sound();
        self.ntvec.push(new_voice);
    }
    pub fn volume(&mut self, value: u8) {
        self.vol = value;
        for ntobj in self.ntvec.iter_mut() {
            ntobj.amplitude(self.vol, value);
        }
    }
    pub fn expression(&mut self, value: u8) {
        self.exp = value;
        for ntobj in self.ntvec.iter_mut() {
            ntobj.amplitude(self.vol, value);
        }
    }
    pub fn sustain(&mut self, _value: u8) {}
    pub fn all_sound_off(&mut self) {
        for ntobj in self.ntvec.iter_mut() {
            ntobj.damp();
        }
    }
    pub fn _release_note(&mut self, nt: &msgf_voice::Voice){
        let ntcmp = self.ntvec.iter_mut();
        for (i, ntobj) in ntcmp.enumerate() {
            if ntobj == nt {
                //  一つ消去したら、ループから抜ける
                self.ntvec.remove(i);
                break;
            }
        }
    }
    pub fn process(&mut self, abuf: &mut msgf_afrm::AudioFrame, in_number_frames: usize) {
        let sz = self.ntvec.len();
        let mut ch_ended = vec![false; sz];
        for i in 0..sz {
            if let Some(nt) = self.ntvec.get_mut(i) {
                let nt_audio = &mut msgf_afrm::AudioFrame::new(in_number_frames);
                ch_ended[i] = nt.process(nt_audio, in_number_frames);
                abuf.mix(nt_audio);
            }
        }
        for i in 0..sz {
            if ch_ended[i] {
                //  一つ消去したら、ループから抜ける
                self.ntvec.remove(i);
                break;
            }
        }
    }
    fn search_note(&mut self, note_num: u8, sts: NoteStatus) -> Option<&mut msgf_voice::Voice> {
        let max_note = self.ntvec.len();
        let mut return_num = max_note;
        for i in 0..max_note {
            if self.ntvec[i].note_num() == note_num && sts == self.ntvec[i].status() {
                return_num = i;
                break;
            }
        };
        if return_num == max_note {
            None
        } else {
            Some(&mut self.ntvec[return_num])
        }
    }

    fn _debug(&mut self) {
        let sz = self.ntvec.len();
        println!("Debug!: {}",sz);
    }
}