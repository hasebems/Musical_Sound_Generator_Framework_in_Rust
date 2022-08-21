//
//  msgf_disp.rs
//	Musical Sound Generator Framework
//      Display
//
//  Created by Hasebe Masahiko on 2022/08/21.
//  Copyright (c) 2022 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//

//---------------------------------------------------------
//		Definition
//---------------------------------------------------------
pub trait MsgfDisplay {
    fn print_str(&self, string: &str) {println!("{}",string)}
    fn print_prm<T: std::fmt::Display>(&self, string: &str, prm: T) {
        let mut all_str = String::from(string);
        let prm_str: &str = &prm.to_string();   //  to_string で、String型、&を付けると &str型
        all_str += prm_str;                     // '+' は String + &str のみ可能
        println!("{}", all_str)
    }
}