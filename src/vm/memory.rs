use super::error::Error;
use std::fmt;

pub struct Memory<'a>(pub &'a mut [u8; 20]);

impl<'a> Memory<'a> {
    pub fn new(data: &'a mut [u8; 20]) -> Self {
        Memory(data)
    }

    pub fn get_mut(&mut self, i: u8) -> Result<&mut u8, Error> {
        self.0
            .get_mut(usize::from(i))
            .ok_or(Error::OutOfBounds(u16::from(i)))
    }

    pub fn size(&self) -> u8 {
        // TODO: This probably should be a constant.
        20
    }
}

impl<'a> fmt::Display for Memory<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut acc = String::new();

        for i in 0..20 {
            acc.push_str(&format!("{:02x} ", i));
        }
        acc.push_str("\n-----------------------------------------------------------\n");

        for value in self.0.iter() {
            acc.push_str(&format!("{:02x} ", value));
        }
        acc.push_str("\nINSTRUCTIONS ---------------------------^ OUT-^ IN-1^ IN-2^");
        write!(f, "{}", acc)
    }
}
