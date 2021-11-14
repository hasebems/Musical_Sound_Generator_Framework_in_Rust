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
use crate::general;
use crate::general::*;

//---------------------------------------------------------
//		Class
//---------------------------------------------------------
pub struct Msgf {
    msg_buf: Vec<(u8,usize,u8,u8)>,
    part: Vec<msgf_part::Part>,
    in_number_frames: u32,
}

impl Msgf {
    pub fn new() -> Self {
        let mut msgf = Self {
            msg_buf: Vec::new(),
            part: Vec::new(),
            in_number_frames: 0,
        };
        for _ in 0..general::MAX_PART_NUM {
            msgf.part.push(msgf_part::Part::new());
        };
        msgf
    }
    pub fn recieve_midi_message(&mut self, dt1: u8, dt2: u8, dt3: u8) {
        let ch: usize = (dt1 & 0x0f).into();
        let status = dt1 & 0xf0;

        if ch >= general::MAX_PART_NUM {
            return;
        };

        let msg: (u8,usize,u8,u8) = (status, ch, dt2, dt3);
        self.msg_buf.push(msg);
    }
    fn parse_msg(&mut self) {
        if let Some(msg) = self.msg_buf.pop() {
            let (status, ch, dt2, dt3) = msg;
            let pt = &mut self.part[ch];
            match status {
                0x80 => pt.note_off(dt2, dt3),
                0x90 => if dt3 == 0 { pt.note_off(dt2, dt3);} else { pt.note_on(dt2, dt3);},
                0xb0 => pt.control_change(dt2, dt3),
                0xc0 => pt.program_change(dt2),
                0xe0 => {
                    let mut bend: i16 = dt3.into();
                    bend += dt2 as i16*128;
                    bend -= 8192;
                    pt.pitch_bend(bend);
                }
                _ => {}
            };
        }
    }
    pub fn process(&mut self,
      abuf_l: &mut [f32; general::MAX_BUFFER_SIZE],
      abuf_r: &mut [f32; general::MAX_BUFFER_SIZE],
      in_number_frames: u32) {
        self.parse_msg();   // MIDI message

        if self.in_number_frames != in_number_frames {
            println!("Audio Buffer: {}",in_number_frames);
            self.in_number_frames = in_number_frames;
        }
        let audio_buffer_l = &mut msgf_afrm::AudioFrame::new(in_number_frames as usize);
        let audio_buffer_r = &mut msgf_afrm::AudioFrame::new(in_number_frames as usize);
        for i in 0..general::MAX_PART_NUM {
            &self.part[i].process(audio_buffer_l, audio_buffer_r, in_number_frames as usize);
        };
        audio_buffer_l.copy_to_sysbuf(abuf_l);  // L
        audio_buffer_r.copy_to_sysbuf(abuf_r);  // R
    }
}
