//
//  app/mod.rs
//	Musical Sound Generator Framework
//
//  Created by Hasebe Masahiko on 2021/10/25.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
pub mod va;

use crate::core::msgf_inst::Inst;
use crate::app::va::va_inst::*;

//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub struct InstComposite {
    va: InstVa,
}
//---------------------------------------------------------
//		Implements
//---------------------------------------------------------
impl InstComposite {
    pub fn new(vol:u8, pan:u8, exp:u8) -> Self {
        Self {
            va: InstVa::new(0, vol, pan, exp),
        }
    }
    pub fn get_inst(&mut self) -> &mut impl Inst {
        &mut self.va
    }
    pub fn change_inst(&mut self, inst_number: usize, vol:u8, pan:u8, exp:u8) {
        self.va.change_inst(inst_number, vol, pan, exp);
        println!("Inst Changed!")
    }
}
