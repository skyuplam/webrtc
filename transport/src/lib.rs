#![warn(rust_2018_idioms)]
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

pub mod buffer;
pub mod fixed_big_int;

pub use crate::buffer::Buffer;