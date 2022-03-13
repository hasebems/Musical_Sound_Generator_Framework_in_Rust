//
//  msgf_if.rs
//	Musical Sound Generator Framework
//      Interface for C / Objective-C
//
//  Created by Hasebe Masahiko on 2021/09/12.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::core::*;

//---------------------------------------------------------
//		Constants
//---------------------------------------------------------
//  configuration
pub const MAX_PART_NUM: usize = 5;
pub const MAX_BUFFER_SIZE: usize = 1024;
pub const SAMPLING_FREQ: f32 = 44100.0;
pub const PI: f32 = std::f32::consts::PI;
pub const AUDIO_FRAME_PER_CONTROL: usize = 128;
pub const DAMP_LIMIT_DEPTH: f32 = 0.0001;
//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub struct Msgf {
    msg_buf: Vec<(u8,usize,u8,u8)>,
    part: Vec<msgf_part::Part>,
    audio_buffer_l: msgf_afrm::AudioFrame,
    audio_buffer_r: msgf_afrm::AudioFrame,
    audio_buffer_send_effect_l: msgf_afrm::AudioFrame,
    audio_buffer_send_effect_r: msgf_afrm::AudioFrame,
    audio_buffer_total_effect_l: msgf_afrm::AudioFrame,
    audio_buffer_total_effect_r: msgf_afrm::AudioFrame,
    in_number_frames: u32,
}
//---------------------------------------------------------
//		Implements
//---------------------------------------------------------
impl Msgf {
    pub fn new() -> Self {
        let mut msgf = Self {
            msg_buf: Vec::new(),
            part: Vec::new(),
            audio_buffer_l: msgf_afrm::AudioFrame::new(0,MAX_BUFFER_SIZE),
            audio_buffer_r: msgf_afrm::AudioFrame::new(0,MAX_BUFFER_SIZE),
            audio_buffer_send_effect_l: msgf_afrm::AudioFrame::new(0,MAX_BUFFER_SIZE),
            audio_buffer_send_effect_r: msgf_afrm::AudioFrame::new(0,MAX_BUFFER_SIZE),
            audio_buffer_total_effect_l: msgf_afrm::AudioFrame::new(0,MAX_BUFFER_SIZE),
            audio_buffer_total_effect_r: msgf_afrm::AudioFrame::new(0,MAX_BUFFER_SIZE),
            in_number_frames: 0,
        };
        for _ in 0..MAX_PART_NUM {
            msgf.part.push(msgf_part::Part::new());
        };
        msgf
    }
    pub fn recieve_midi_message(&mut self, dt1: u8, dt2: u8, dt3: u8) {
        let ch: usize = (dt1 & 0x0f).into();
        let status = dt1 & 0xf0;

        if ch >= MAX_PART_NUM {
            return;
        };

        let msg: (u8,usize,u8,u8) = (status, ch, dt2, dt3);
        self.msg_buf.push(msg);
    }
    fn parse_msg(&mut self) {
        if self.msg_buf.len() == 0 { return }
        let (status, ch, dt2, dt3) = self.msg_buf.remove(0);
        let pt = &mut self.part[ch];
        match status {
            0x80 => pt.note_off(dt2, dt3),
            0x90 => if dt3 == 0 { pt.note_off(dt2, dt3);} else { pt.note_on(dt2, dt3);},
            0xb0 => pt.control_change(dt2, dt3),
            0xc0 => pt.program_change(dt2),
            0xe0 => {
                let mut bend: i16 = dt2.into();
                bend += dt3 as i16*128;
                bend -= 8192;
                pt.pitch_bend(bend);
            }
            _ => {}
        };
    }
    pub fn process(&mut self,
      abuf_l: &mut [f32; MAX_BUFFER_SIZE],
      abuf_r: &mut [f32; MAX_BUFFER_SIZE],
      in_number_frames: u32) {
        self.parse_msg();   // MIDI message
        if self.in_number_frames != in_number_frames {
            println!("Audio Buffer: {}",in_number_frames);
            self.in_number_frames = in_number_frames;
        }
        self.audio_buffer_l.set_sample_number(in_number_frames as usize);
        self.audio_buffer_r.set_sample_number(in_number_frames as usize);
        // init effect buffer
        self.audio_buffer_send_effect_l.set_sample_number(in_number_frames as usize);
        self.audio_buffer_send_effect_r.set_sample_number(in_number_frames as usize);
        self.audio_buffer_total_effect_l.set_sample_number(in_number_frames as usize);
        self.audio_buffer_total_effect_r.set_sample_number(in_number_frames as usize);
        if MAX_PART_NUM >= 1 {      //  Part 1 は copy
            self.part[0].process(
                &mut self.audio_buffer_l,
                &mut self.audio_buffer_r,
                &mut self.audio_buffer_total_effect_l,
                &mut self.audio_buffer_total_effect_r,
                in_number_frames as usize);
            self.audio_buffer_l.copy_to_sysbuf(abuf_l);  // L
            self.audio_buffer_r.copy_to_sysbuf(abuf_r);  // R
        }
        for i in 1..MAX_PART_NUM { //  Part 2 以降は add
            self.part[i].process(
                &mut self.audio_buffer_l,
                &mut self.audio_buffer_r,
                &mut self.audio_buffer_send_effect_l,
                &mut self.audio_buffer_send_effect_r,
                in_number_frames as usize);
            self.audio_buffer_l.add_to_sysbuf(abuf_l);  // L
            self.audio_buffer_r.add_to_sysbuf(abuf_r);  // R
            self.audio_buffer_total_effect_l.mix_and_check_no_sound(&mut self.audio_buffer_send_effect_l);  // L
            self.audio_buffer_total_effect_r.mix_and_check_no_sound(&mut self.audio_buffer_send_effect_r);  // R
        };
        // Effect
        self.audio_buffer_total_effect_l.clr_abuf();
        self.audio_buffer_total_effect_r.clr_abuf();
        //  sysbuf に足す
        self.audio_buffer_total_effect_l.add_to_sysbuf(abuf_l);  // L
        self.audio_buffer_total_effect_r.add_to_sysbuf(abuf_r);  // R
    }
}
