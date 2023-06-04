#![no_std]

pub mod panic; // Lightweight implementation of panic handler
pub use e310x as pac; // Peripheral Access Crate. Generated from SVD file
pub use riscv_rt::entry; // Application entry point
#[allow(unused_imports)]
pub use rtt_target::{rprintln as log, rtt_init_print as log_init};
