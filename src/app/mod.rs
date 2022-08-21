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
pub mod sgf;
use crate::app::va::*;
use crate::app::sg::*;
use crate::app::sgf::*;
pub fn get_inst(inst_number:usize, vol:u8, pan:u8, exp:u8) -> Box<dyn msgf_inst::Inst> {
    if inst_number < 8 {
        println!("Changed Va: {}",inst_number);
        Box::new(va_inst::InstVa::new(inst_number,vol,pan,exp))
    }
    else if inst_number == 8 {
        println!("Changed Sg: {}",inst_number);
        Box::new(sg_inst::InstSg::new(0,vol,pan,exp))
    }
    else if inst_number == 9 {
        println!("Changed Sgf {}",inst_number);
        Box::new(sgf_inst::InstSgf::new(0,vol,pan,exp))
    }
    else if inst_number == 10 { //  for Touch Keyboard
        println!("Changed Sgf {}",inst_number);
        Box::new(sgf_inst::InstSgf::new(0,vol,pan,exp))
    }
    //  for Touch Keyboard MFT2022 version
    else if inst_number == 16 {
        println!("Changed Va: {}",inst_number);
        Box::new(va_inst::InstVa::new(4,vol,pan,exp))
    }
    else if inst_number == 17 {
        println!("Changed Va: {}",inst_number);
        Box::new(va_inst::InstVa::new(5,vol,pan,exp))
    }
    else if inst_number == 18 {
        println!("Changed Sgf: {}",inst_number);
        Box::new(sgf_inst::InstSgf::new(0,vol,pan,exp))
    }

    else {
        Box::new(va_inst::InstVa::new(0,vol,pan,exp))
    }
} 
