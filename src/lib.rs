#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    non_snake_case,
    unused
)]
#![deny(
    deprecated
)]

#[macro_use]
extern crate lazy_static;

pub mod imports;

mod acmd;
mod frame;
mod status;
mod agent;
pub mod data;
use data::gamemode::*;
//pub mod util;
//use util::*;

std::arch::global_asm!(
    r#"
    .section .nro_header
    .global __nro_header_start
    .word 0
    .word _mod_header
    .word 0
    .word 0
    
    .section .rodata.module_name
        .word 0
        .word 5
        .ascii "dropdash"
    .section .rodata.mod0
    .global _mod_header
    _mod_header:
        .ascii "MOD0"
        .word __dynamic_start - _mod_header
        .word __bss_start - _mod_header
        .word __bss_end - _mod_header
        .word __eh_frame_hdr_start - _mod_header
        .word __eh_frame_hdr_end - _mod_header
        .word __nx_module_runtime - _mod_header // runtime-generated module object offset
    .global IS_NRO
    IS_NRO:
        .word 1
    
    .section .bss.module_runtime
    __nx_module_runtime:
    .space 0xD0
    "#
);
#[no_mangle]
pub extern "C" fn main() {
    println!("[smashline_dropdash::main] Loading...");

    //data::install();
    data::gamemode::set_gamemode();
    acmd::install();
    //frame::install();
    status::install();
    agent::install();
    println!("[smashline_dropdash::main] Loaded!");
}