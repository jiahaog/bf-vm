use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    // Out of bounds while accessing memory at offset.
    OutOfBounds(u16),
    // PC was somehow negative.
    NegativePc,
}

pub fn run<'a>(memory: &mut Memory) -> Result<(), Error> {
    let mut registers: [i16; 3] = [0; 3];

    loop {
        let pc = registers[0];
        if pc < 0 {
            return Err(Error::NegativePc);
        }

        // Don't keep a mutable reference on pc, so the other registers can be
        // easily accessed.
        registers[0] = match memory.0[usize::from(pc as u16)..] {
            // load_word
            [0x01, reg, addr, ..] => {
                registers[usize::from(reg)] =
                    i16::from_le_bytes([*memory.get_mut(addr)?, *memory.get_mut(addr + 1)?]);
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
            // add_immediate
            [0x05, reg, constant, ..] => {
                registers[usize::from(reg)] = registers[usize::from(reg)] + i16::from(constant);

                pc + 3
            }
            // branch_if_eq
            // If first_reg and second_reg contain same value, pc goes to new_pc.
            [0x06, first_reg, second_reg, new_pc, ..] => {
                if registers[usize::from(first_reg)] == registers[usize::from(second_reg)] {
                    i16::from(new_pc)
                } else {
                    pc + 4
                }
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

    fn test_helper(raw_memory: [u8; 20]) -> Result<i16, Error> {
        let mut raw_memory = raw_memory;

        let mut memory = Memory::new(&mut raw_memory);

        run(&mut memory)?;

        Ok(i16::from_le_bytes([raw_memory[0x0e], raw_memory[0x0f]]))
    }

    #[test]
    fn add() {
        #[rustfmt::skip]
        assert_eq!(Ok(5281 + 12), test_helper([
            0x01, 0x01, 0x10,
            0x01, 0x02, 0x12,
            0x03, 0x01, 0x02,
            0x02, 0x01, 0x0e,
            0xff,
            0x00, 0x00, 0x00,
            // 161 + 20 * 2^8
            0xa1, 0x14,
            // 12
            0x0c, 0x00,
        ]));
    }

    #[test]
    fn add_immediate() {
        #[rustfmt::skip]
        assert_eq!(Ok(5281 + 2), test_helper([
            0x01, 0x01, 0x10,
            0x05, 0x01, 0x02,
            0x02, 0x01, 0x0e,
            0xff,
            0x00, 0x00, 0x00,
            0x00, 0x00, 0x00,
            // 161 + 20 * 2^8
            0xa1, 0x14,
            0x00, 0x00,
        ]));
    }

    #[test]
    fn sub() {
        #[rustfmt::skip]
        assert_eq!(Ok(5281 - 12), test_helper([
            0x01, 0x01, 0x10,
            0x01, 0x02, 0x12,
            0x04, 0x01, 0x02,
            0x02, 0x01, 0x0e,
            0xff,
            0x00, 0x00, 0x00,
            // 161 + 20 * 2^8
            0xa1, 0x14,
            // 12
            0x0c, 0x00,
        ]));
    }

    #[test]
    fn sub_negative() {
        #[rustfmt::skip]
        assert_eq!(Ok(12 - 5281), test_helper([
            0x01, 0x01, 0x10,
            0x01, 0x02, 0x12,
            0x04, 0x01, 0x02,
            0x02, 0x01, 0x0e,
            0xff,
            0x00, 0x00, 0x00,
            // 12
            0x0c, 0x00,
            // 161 + 20 * 2^8
            0xa1, 0x14,
        ]));
    }

    #[test]
    fn branch_if_equal() {
        // Set the output to 1, expect the output to be 0.
        // If r1 and r0 is equal (true since both initialized to 0), jump and
        // store r1 to the output. Otherwise halt.
        #[rustfmt::skip]
        assert_eq!(Ok(0), test_helper([
            0x06, 0x01, 0x02, 0x05,
            0xff,
            0x02, 0x01, 0x0e,
            0xff, 0x00, 0x00,
            0x00, 0x00, 0x00,
            0x00, 0x01,
            0x00, 0x00,
            0x00, 0x00,
        ]));
    }
}
