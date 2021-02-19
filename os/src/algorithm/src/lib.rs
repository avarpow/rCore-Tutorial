#![no_std]
#![feature(llvm_asm)]
#![feature(global_asm)]


#[macro_use]
mod print;
extern crate alloc;

mod allocator;

pub use allocator::*;
