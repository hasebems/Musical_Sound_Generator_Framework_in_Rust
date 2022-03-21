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
pub mod sg;
use crate::app::va::*;
use crate::app::sg::*;
pub fn get_inst(inst_number:usize, vol:u8, pan:u8, exp:u8) -> Box<dyn msgf_inst::Inst> {
    if inst_number < 8 {
        println!("Changed Va: {}",inst_number);
        Box::new(va_inst::InstVa::new(inst_number,vol,pan,exp))
    }
    else if inst_number < 16 {
        println!("Changed Sg: {}",inst_number);
        Box::new(sg_inst::InstSg::new(inst_number-8,vol,pan,exp))
    }
    else {
        Box::new(va_inst::InstVa::new(0,vol,pan,exp))
    }
} 
