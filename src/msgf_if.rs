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
    part: Vec<msgf_part::Part>,
    in_number_frames: u32,
}

impl Msgf {
    pub fn new() -> Self {
        let mut msgf = Self {
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
    pub fn process(&mut self, abuf: &mut [f32; general::MAX_BUFFER_SIZE], in_number_frames: u32) {
        if self.in_number_frames != in_number_frames {
            println!("Audio Buffer: {}",in_number_frames);
            self.in_number_frames = in_number_frames;
        }
        let audio_buffer = &mut msgf_afrm::AudioFrame::new(in_number_frames as usize);
        for i in 0..general::MAX_PART_NUM {
            &self.part[i].process(audio_buffer, in_number_frames as usize);
        };
        audio_buffer.copy_from_abuf(abuf);
    }
}
