//
//  msgf_part.rs
//	Musical Sound Generator Framework
//      Part Class
//
//  Created by Hasebe Masahiko on 2021/09/16.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::*;
use crate::core::*;
use crate::core::msgf_inst::*;
use crate::app::InstComposite;

//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub struct Part {
    //	Part Latest Parameter Value
    cc0_msb: u8,
    cc1_modulation_wheel: u8,
    cc5_portamento_time: u8,
    cc7_volume: u8,
    cc10_pan: u8,
    cc11_expression: u8,
    cc12_note_shift: u8,	//	out of MIDI
    cc13_tune: u8,			//	out of MIDI
    cc32_lsb: u8,
    cc64_sustain: u8,
    cc65_portamento: u8,
    cc66_sostenuto: u8,
    _cc126_mono: u8,
    program_number: u8,
    pitch_bend_value: i16,
    cc16_31_change_vprm: [u8; 16],
	
    //	Composite Object
    inst_comp: InstComposite,
}
//---------------------------------------------------------
//		Imprements
//---------------------------------------------------------
impl Part {
    pub fn new() -> Self {
        Self {
            cc0_msb: 0,
            cc1_modulation_wheel: 0,
            cc5_portamento_time: 0,
            cc7_volume: 100,
            cc10_pan: 64,
            cc11_expression: 127,
            cc12_note_shift: 64,
            cc13_tune: 64,
            cc32_lsb: 0,
            cc64_sustain: 0,
            cc65_portamento: 0,
            cc66_sostenuto: 0,
            _cc126_mono: 1,
            program_number: 0,
            pitch_bend_value: 0,
            cc16_31_change_vprm: [0; 16],
            inst_comp: InstComposite::new(100,64,127),
        }
    }
    fn get_inst(&mut self) -> &mut impl Inst {
        self.inst_comp.get_inst()
    }
    pub fn note_off(&mut self, dt2: u8, dt3: u8) {
        self.get_inst().note_off(dt2, dt3);
    }
    pub fn note_on(&mut self, dt2: u8, dt3: u8) {
        self.get_inst().note_on(dt2, dt3);
    }
    pub fn control_change(&mut self, controller: u8, value: u8) {
        match controller {
            0 => self.cc0_msb = value,
            1 => {
                self.cc1_modulation_wheel = value;
                self.get_inst().modulation(value);
            }
            5 => self.cc5_portamento_time = value,
            7 => {
                self.cc7_volume = value;
                self.get_inst().volume(value);
            }
            10 => {
                self.cc10_pan = value;
                self.get_inst().pan(value);
            }
            11 => {
                self.cc11_expression = value;
                self.get_inst().expression(value);
            }
            12 => {
                self.cc12_note_shift = value;
                let pb = self.pitch_bend_value;
                let tn = self.cc13_tune;
                self.get_inst().pitch(pb, value, tn);
            }
            13 => {
                self.cc13_tune = value;
                let pb = self.pitch_bend_value;
                let ns = self.cc12_note_shift;
                self.get_inst().pitch(pb, ns, value);
            }
            32 => self.cc32_lsb = value,
            64 => {
                self.cc64_sustain = value;
                self.get_inst().sustain(value);
            }
            65 => self.cc65_portamento = value,
            66 => self.cc66_sostenuto = value,
            120 => {
                if value == 0 {
                    self.get_inst().all_sound_off();
                }
            }
            16..=31 => {
                let vprm_num: u8 = controller-16;
                self.cc16_31_change_vprm[vprm_num as usize] = value;
                self.get_inst().set_prm(vprm_num, value);
            }
            _ => {}
        };
        println!("Control Change: {}",controller);
    }
    pub fn program_change(&mut self, dt2: u8) {
        self.program_number = dt2;
        let vol = self.cc7_volume;
        let pan = self.cc10_pan;
        let exp = self.cc11_expression;
        let pb = self.pitch_bend_value;
        let ns = self.cc12_note_shift;
        let tn = self.cc13_tune;
        self.inst_comp.change_inst(dt2 as usize, vol, pan, exp);
        self.get_inst().pitch(pb, ns, tn);
        println!("Program Change: {}",dt2);
    }
    pub fn pitch_bend(&mut self, bend: i16) {
        self.pitch_bend_value = bend;
        let ns = self.cc12_note_shift;
        let tn = self.cc13_tune;
        println!("Pitch Bend: {}",bend);
        self.get_inst().pitch(bend, ns, tn);
    }
    pub fn process(&mut self,
                   abuf_l: &mut msgf_afrm::AudioFrame,
                   abuf_r: &mut msgf_afrm::AudioFrame,
                   in_number_frames: usize) {
        let mut audio_buffer_l = msgf_afrm::AudioFrame::new(0,msgf_if::MAX_BUFFER_SIZE);
        let mut audio_buffer_r = msgf_afrm::AudioFrame::new(0,msgf_if::MAX_BUFFER_SIZE);
        audio_buffer_l.set_sample_number(in_number_frames as usize);
        audio_buffer_l.clr_abuf();
        audio_buffer_r.set_sample_number(in_number_frames as usize);
        audio_buffer_r.clr_abuf();
        let inst = self.get_inst();
        inst.process(&mut audio_buffer_l, &mut audio_buffer_r, in_number_frames);
        // Part Volume, Part Pan, Effect 
        audio_buffer_l.copy_to_abuf(abuf_l);
        audio_buffer_r.copy_to_abuf(abuf_r);
    }
}