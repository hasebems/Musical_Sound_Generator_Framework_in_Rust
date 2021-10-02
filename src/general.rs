//
//  general::mod.rs
//	Musical Sound Generator Framework
//
//  Created by Hasebe Masahiko on 2021/09/18.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
pub mod msgf_part;
pub mod msgf_inst;
pub mod msgf_note;
pub mod msgf_afrm;
pub mod msgf_synth;
//---------------------------------------------------------
//		Constants
//---------------------------------------------------------
//  configuration
pub const MAX_PART_NUM: usize = 1;
pub const MAX_BUFFER_SIZE: usize = 1024;
pub const SAMPLING_FREQ: f32 = 44100.0;