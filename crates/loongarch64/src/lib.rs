#![no_std]
#![allow(unused)]
pub mod asm;
pub mod cpu;
pub mod mem;
pub mod register;
pub mod tlb;
pub const VALEN: usize = 48;
pub const PALEN: usize = 48;
