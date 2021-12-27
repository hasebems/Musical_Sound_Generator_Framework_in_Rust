//
//  lib.rs
//	Musical Sound Generator Framework
//      Interface for C / Objective-C
//
//  Created by Hasebe Masahiko on 2021/09/12.
//  Copyright (c) 2021 Hasebe Masahiko.
//  Released under the MIT license
//  https://opensource.org/licenses/mit-license.php
//
//  How to generate msgf.h
//      cbindgen --config cbindgen.toml --crate msgf --output msgf.h --lang=c -q
//      (-q: Report errors only )
mod msgf_if;
mod core;
mod engine;
mod app;

#[no_mangle]
pub extern "C" fn rust_msgf_new() -> *mut msgf_if::Msgf {
    let ptr = Box::new(msgf_if::Msgf::new());
    Box::into_raw(ptr)
}
#[no_mangle]
pub extern "C" fn rust_recieve_midi_message(rust_msgf: &mut msgf_if::Msgf, dt1: u8, dt2: u8, dt3: u8) {
    rust_msgf.recieve_midi_message(dt1, dt2, dt3);
}
#[no_mangle]
pub extern "C" fn rust_process(rust_msgf: &mut msgf_if::Msgf, abuf_l: &mut [f32; msgf_if::MAX_BUFFER_SIZE], abuf_r: &mut [f32; msgf_if::MAX_BUFFER_SIZE],in_number_frames: u32) {
    rust_msgf.process(abuf_l, abuf_r, in_number_frames);
}
#[no_mangle]
pub extern "C" fn say_hello() {
    println!("Hello, World!");
}
#[no_mangle]
pub extern "C" fn rust_msgf_destroy(rust_msgf: *mut msgf_if::Msgf) {
    unsafe { Box::from_raw(rust_msgf) };
}