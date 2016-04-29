use std::fmt;

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
            try!(write!(f, "{}{:x}", if i > 0 { " " } else { "" }, val ))
        }
        try!(write!(f, "]"));
        Ok(())
    }
}

impl<'a, T: fmt::UpperHex> fmt::UpperHex for Hex<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "["));
        for (i, val) in self.0.iter().enumerate() {
            try!(write!(f, "{}{:X}", if i > 0 { " " } else { "" }, val ))
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
