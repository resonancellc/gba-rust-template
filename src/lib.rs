#![feature(i128_type, asm, proc_macro, lang_items, core, core_intrinsics, const_fn, untagged_unions, arbitrary_self_types)]
#![no_std]
#![allow(dead_code)]

extern crate gbalib;

#[no_mangle]
pub unsafe extern "C" fn main(_: i32, _: *const *const i8) -> i32 {
    return 5;
}
