#![no_std]

extern crate msp430fr2355;
extern crate panic_msp430;

use core::cell::Cell;
use core::cell::UnsafeCell;

fn main() {
    let n = UnsafeCell::new(5);
    unsafe {
        *n.get() = 5;
    }

    let c = Cell::new(3);
    c.set(4);
}
