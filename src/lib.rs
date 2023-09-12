#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(not(feature="bindgen"))]
mod bindings;
#[cfg(not(feature="bindgen"))]
pub use bindings::*;

#[cfg(feature="bindgen")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
