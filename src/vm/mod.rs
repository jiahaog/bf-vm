use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    // Out of bounds while accessing memory at offset.
    OutOfBounds(u16),
}

pub fn run<'a>(memory: &mut Memory) -> Result<(), Error> {
    let mut registers: [u16; 3] = [0; 3];

    loop {
        let pc = registers[0];

        // Don't keep a mutable reference on pc, so the other registers can be
        // easily accessed.
        registers[0] = match memory.0[usize::from(pc)..] {
            // load_word
            [0x01, reg, addr, ..] => {
                registers[usize::from(reg)] =
                    u16::from_le_bytes([*memory.get_mut(addr)?, *memory.get_mut(addr + 1)?]);
                pc + 3
            }
            // store_word
            [0x02, reg, addr, ..] => {
                let le_bytes = registers[usize::from(reg)].to_le_bytes();

                *memory.get_mut(addr)? = le_bytes[0];
                *memory.get_mut(addr + 1)? = le_bytes[1];

                pc + 3
            }
            // add
            [0x03, first_reg, second_reg, ..] => {
                registers[usize::from(first_reg)] =
                    registers[usize::from(first_reg)] + registers[usize::from(second_reg)];

                pc + 3
            }
            // sub
            [0x04, first_reg, second_reg, ..] => {
                registers[usize::from(first_reg)] =
                    registers[usize::from(first_reg)] - registers[usize::from(second_reg)];

                pc + 3
            }
            // halt
            [0xff, ..] => {
                return Ok(());
            }
            _ => {
                unimplemented!("Unexpected instruction");
            }
        };
    }
}

pub struct Memory<'a>(&'a mut [u8; 20]);

impl<'a> Memory<'a> {
    pub fn new(data: &'a mut [u8; 20]) -> Self {
        Memory(data)
    }

    fn get_mut(&mut self, i: u8) -> Result<&mut u8, Error> {
        self.0
            .get_mut(usize::from(i))
            .ok_or(Error::OutOfBounds(u16::from(i)))
    }
}

impl<'a> fmt::Display for Memory<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_helper(raw_memory: [u8; 20]) -> Result<u16, Error> {
        let mut raw_memory = raw_memory;

        let mut memory = Memory::new(&mut raw_memory);

        run(&mut memory)?;

        Ok(u16::from_le_bytes([raw_memory[0x0e], raw_memory[0x0f]]))
    }

    #[test]
    fn can_add() {
        #[rustfmt::skip]
        assert_eq!(Ok(5293), test_helper([
            0x01, 0x01, 0x10,
            0x01, 0x02, 0x12,
            0x03, 0x01, 0x02,
            0x02, 0x01, 0x0e,
            0xff,
            0x00,
            0x00, 0x00,
            0xa1, 0x14,
            0x0c, 0x00
        ]));
    }

    // TODO: Add more tests.
}
