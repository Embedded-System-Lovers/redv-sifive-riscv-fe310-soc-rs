//! Example blinky
//!
//! Blinks onboard LED
//!

#![no_std]
#![no_main]

use board::{entry, pac};
use redv_sifive_riscv_fe310_soc_rs as board;

#[entry] // Macro which handles startup phase and jumps to application entry point.
fn main() -> ! {
    // GPIO0 register block. TODO: Add proper abstraction
    let gpio0_reg_blk = unsafe { &*pac::GPIO0::ptr() };
    // Set initial value and enable output of LED
    gpio0_reg_blk.output_val.write(|w| w.pin5().set_bit());
    gpio0_reg_blk.output_en.write(|w| w.pin5().set_bit());

    loop {
        // Basic software blocking delay. TODO: Use hardware mechanism for time base
        for _ in 0..100_000 {}
        // Toggles LED
        gpio0_reg_blk
            .output_val
            .modify(|r, w| w.pin5().bit(!r.pin5().bit()));
    }
}
