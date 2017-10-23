//! The purpose of this crate is to extend the `UpperHex` and `LowerHex`
//! traits to slices, as well as the integers it is currently implemented for.
//!
//! # Examples
//!
//! ```rust
//! extern crate hex_slice;
//! use hex_slice::AsHex;
//!
//! fn main() {
//!     let foo = vec![0u32, 1 ,2 ,3];
//!     println!("{:x}", foo.as_hex());
//! }
//! ```


use std::fmt;
<<<<<<< HEAD
use std::mem::size_of;
=======
use std::fmt::Write;
>>>>>>> master

pub struct Hex<'a, T: 'a>(&'a [T]);

pub trait AsHex {
    type Item;
    fn as_hex<'a>(&'a self) -> Hex<'a, Self::Item>;
}

impl<'a, T> Hex<'a, T> {
    pub fn hex(slice: &'a [T]) -> Hex<'a, T> {
        Hex(slice)
    }
}

impl<'a, T: fmt::LowerHex> fmt::LowerHex for Hex<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "["));
        for (i, val) in self.0.iter().enumerate() {
            if i > 0 {
                try!(f.write_char(' '));
            }
            try!(fmt::LowerHex::fmt(val, f));
        }
        try!(write!(f, "]"));
        Ok(())
    }
}

impl<'a, T: fmt::UpperHex> fmt::UpperHex for Hex<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "["));
        for (i, val) in self.0.iter().enumerate() {
            if i > 0 {
                try!(f.write_char(' '));
            }
            try!(fmt::UpperHex::fmt(val, f));
        }
        try!(write!(f, "]"));
        Ok(())
    }
}

impl<T> AsHex for [T] {
    type Item = T;
    fn as_hex<'a>(&'a self) -> Hex<'a, Self::Item> {
        Hex::hex(self)
    }
}
