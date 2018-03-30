#![feature(i128_type, asm, proc_macro, lang_items, core, core_intrinsics, const_fn, untagged_unions, arbitrary_self_types)]
#![no_std]
#![allow(dead_code)]

extern crate gbalib;
use gbalib::ptr::Ptr;

fn draw_rect(left: u16, top: u16, right: u16, bottom: u16, color: u16) {
    let mut ix = left as u32;
    let mut address = Ptr::null();
    while ix < right as u32 {
        let mut iy = top as u32;
        while iy < bottom as u32 {
            let index = 2*(240 * iy + ix);
            address = Ptr::<u16>::from_u32(0x06000000 + (index as u32));
            *address = color;
            iy += 1
        }
        ix += 1;
    }
    unsafe { address.num += 4; }
    *address = 0xDEAD;
}

#[no_mangle]
pub unsafe extern "C" fn main(_: i32, _: *const *const i8) -> i32 {
    let mut vmode = Ptr::<u16>::from_u32(0x04000000);
    *vmode = 0x0403;

    draw_rect(0, 80, 240, 120, 0x001F);
    loop {}
}
