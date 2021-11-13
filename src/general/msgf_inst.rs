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
use crate::app::*;

//---------------------------------------------------------
//		Class
//---------------------------------------------------------
pub struct Inst {
    vcevec: Vec<msgf_voice::Voice>,
    inst_number: usize,
    mdlt: f32,
    vol: u8,
    pan: u8,
    exp: u8,
}

impl Inst {
    pub fn new(mut inst_number: usize, vol: u8, pan: u8, exp: u8) -> Self {
        let max_tone = msgf_prm::MAX_TONE_COUNT;
        if inst_number >= max_tone {
            inst_number = max_tone-1;
        }
        Self {
            vcevec: Vec::new(),
            inst_number,
            mdlt: msgf_prm::TONE_PRM[inst_number].osc.lfo_depth,
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
        self.vcevec.push(new_voice);
    }
    pub fn modulation(&mut self, value: u8) {
        self.mdlt = 0.5f32*(value as f32)/127.0;
        for vce in self.vcevec.iter_mut() {
            vce.change_pmd(self.mdlt);
        }
    }
    pub fn volume(&mut self, value: u8) {
        self.vol = value;
        for vce in self.vcevec.iter_mut() {
            vce.amplitude(self.vol, value);
        }
    }
    pub fn expression(&mut self, value: u8) {
        self.exp = value;
        for vce in self.vcevec.iter_mut() {
            vce.amplitude(self.vol, value);
        }
    }
    pub fn sustain(&mut self, _value: u8) {}
    pub fn all_sound_off(&mut self) {
        for vce in self.vcevec.iter_mut() {
            vce.damp();
        }
    }
    pub fn _release_note(&mut self, nt: &msgf_voice::Voice){
        let ntcmp = self.vcevec.iter_mut();
        for (i, vce) in ntcmp.enumerate() {
            if vce == nt {
                //  一つ消去したら、ループから抜ける
                self.vcevec.remove(i);
                break;
            }
        }
    }
    pub fn process(&mut self, abuf: &mut msgf_afrm::AudioFrame, in_number_frames: usize) {
        let sz = self.vcevec.len();
        let mut ch_ended = vec![false; sz];
        for i in 0..sz {
            if let Some(nt) = self.vcevec.get_mut(i) {
                let nt_audio = &mut msgf_afrm::AudioFrame::new(in_number_frames);
                ch_ended[i] = nt.process(nt_audio, in_number_frames);
                abuf.mix(nt_audio);
            }
        }
        for i in 0..sz {
            if ch_ended[i] {
                //  一つ消去したら、ループから抜ける
                self.vcevec.remove(i);
                break;
            }
        }
    }
    fn search_note(&mut self, note_num: u8, sts: NoteStatus) -> Option<&mut msgf_voice::Voice> {
        let max_note = self.vcevec.len();
        let mut return_num = max_note;
        for i in 0..max_note {
            if self.vcevec[i].note_num() == note_num && sts == self.vcevec[i].status() {
                return_num = i;
                break;
            }
        };
        if return_num == max_note {
            None
        } else {
            Some(&mut self.vcevec[return_num])
        }
    }

    fn _debug(&mut self) {
        let sz = self.vcevec.len();
        println!("Debug!: {}",sz);
    }
}