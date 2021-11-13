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
use crate::general::*;

//---------------------------------------------------------
//		Class
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
	
    //	Composite Object
    //inst_factory: &InstrumentFactory,
	inst: msgf_inst::Inst,
}

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
            inst: msgf_inst::Inst::new(0,100,64,127),//pgn,vol,pan,exp
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
            10 => self.cc10_pan = value,
            11 => {
                self.cc11_expression = value;
                self.inst.expression(value);
            }
            12 => self.cc12_note_shift = value,
            13 => self.cc13_tune = value,
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
        self.inst = msgf_inst::Inst::new(
            dt2 as usize, 
            self.cc7_volume,
            self.cc10_pan,
            self.cc11_expression);
        println!("Program Change: {}",dt2);
    }
    pub fn pitch_bend(&mut self, bend: i16) {
        self.pitch_bend_value = bend;
    }
    pub fn process(&mut self,
                   abuf_l: &mut msgf_afrm::AudioFrame,
                   abuf_r: &mut msgf_afrm::AudioFrame,
                   in_number_frames: usize) {
        let inst_audio_buffer = &mut msgf_afrm::AudioFrame::new(in_number_frames as usize);
        self.inst.process(inst_audio_buffer, in_number_frames);
        // Part Volume, Part Pan, Effect 
        inst_audio_buffer.copy_to_abuf(abuf_l);
        inst_audio_buffer.copy_to_abuf(abuf_r);
    }
}