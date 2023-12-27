//! This is the driver of ats-intc
//! 

#![no_std]
#![allow(unused)]

extern crate alloc;

#[cfg(any(feature = "locked-simul", feature = "lock-free-simul"))]
mod executor;
#[cfg(any(feature = "locked-simul", feature = "lock-free-simul"))]
mod handler;

mod driver;
mod task;
mod waker;

pub use task::*;
pub use driver::*;
pub use waker::*;