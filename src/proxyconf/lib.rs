#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate byteorder;
extern crate winreg;

pub mod ie;
mod registry_helpers;
mod string_serialization;

#[cfg(test)]
mod hex;