//
//  app/mod.rs
//	Musical Sound Generator Framework
//
//  Created by Hasebe Masahiko on 2021/10/25.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
use crate::core::msgf_inst;

//  You can select a specific app.
pub mod va;
use crate::app::va::*;
pub fn get_inst(inst_number:usize, vol:u8, pan:u8, exp:u8) -> Box<dyn msgf_inst::Inst> {
    Box::new(va_inst::InstVa::new(inst_number,vol,pan,exp))
}