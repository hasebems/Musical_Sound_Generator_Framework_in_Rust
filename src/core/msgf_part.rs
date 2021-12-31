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

//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub struct Part {
    audio_buffer_l: msgf_afrm::AudioFrame,
    audio_buffer_r: msgf_afrm::AudioFrame,

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
	
    //	Composite Object
	inst: Box<dyn msgf_inst::Inst>,
}
//---------------------------------------------------------
//		Imprements
//---------------------------------------------------------
impl Part {
    pub fn new() -> Self {
        let inst_instance = app::get_inst(0,100,64,127); //pgn,vol,pan,exp,
        Self {
            audio_buffer_l: msgf_afrm::AudioFrame::new(0,msgf_if::MAX_BUFFER_SIZE),
            audio_buffer_r: msgf_afrm::AudioFrame::new(0,msgf_if::MAX_BUFFER_SIZE),
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
            inst: inst_instance,
        }
    }
    pub fn note_off(&mut self, dt2: u8, dt3: u8) {
        self.inst.note_off(dt2, dt3);
    }
    pub fn note_on(&mut self, dt2: u8, dt3: u8) {
        self.inst.note_on(dt2, dt3);
    }
    pub fn control_change(&mut self, controller: u8, value: u8) {
        match controller {
            0 => self.cc0_msb = value,
            1 => {
                self.cc1_modulation_wheel = value;
                self.inst.modulation(value);
            }
            5 => self.cc5_portamento_time = value,
            7 => {
                self.cc7_volume = value;
                self.inst.volume(value);
            }
            10 => {
                self.cc10_pan = value;
                self.inst.pan(value);
            }
            11 => {
                self.cc11_expression = value;
                self.inst.expression(value);
            }
            12 => {
                self.cc12_note_shift = value;
                self.inst.pitch(self.pitch_bend_value,value,self.cc13_tune);
            }
            13 => {
                self.cc13_tune = value;
                self.inst.pitch(self.pitch_bend_value,self.cc12_note_shift,value);
            }
            32 => self.cc32_lsb = value,
            64 => {
                self.cc64_sustain = value;
                self.inst.sustain(value);
            }
            65 => self.cc65_portamento = value,
            66 => self.cc66_sostenuto = value,
            120 => {
                if value == 0 {
                    self.inst.all_sound_off();
                }
            }
            _ => {}
        };
        println!("Control Change: {}",controller);
    }
    pub fn program_change(&mut self, dt2: u8) {
        self.program_number = dt2;
        self.inst.change_inst(
            dt2 as usize, 
            self.cc7_volume,
            self.cc10_pan,
            self.cc11_expression);
        self.inst.pitch(
            self.pitch_bend_value,
            self.cc12_note_shift,
            self.cc13_tune);
        println!("Program Change: {}",dt2);
    }
    pub fn pitch_bend(&mut self, bend: i16) {
        self.pitch_bend_value = bend;
        self.inst.pitch(bend,self.cc12_note_shift,self.cc13_tune);
    }
    pub fn process(&mut self,
                   abuf_l: &mut msgf_afrm::AudioFrame,
                   abuf_r: &mut msgf_afrm::AudioFrame,
                   in_number_frames: usize) {
        self.audio_buffer_l.set_sample_number(in_number_frames as usize);
        self.audio_buffer_l.clr_abuf();
        self.audio_buffer_r.set_sample_number(in_number_frames as usize);
        self.audio_buffer_r.clr_abuf();
        self.inst.process(&mut self.audio_buffer_l, &mut self.audio_buffer_r, in_number_frames);
        // Part Volume, Part Pan, Effect 
        self.audio_buffer_l.copy_to_abuf(abuf_l);
        self.audio_buffer_r.copy_to_abuf(abuf_r);
    }
}