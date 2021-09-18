pub mod compiler;
pub mod error;
pub mod instruction;
pub mod memory;

use error::Error;
use instruction::Instruction;
use memory::Memory;

pub fn run<'a>(memory: &mut Memory) -> Result<(), Error> {
    let mut registers: [i16; 3] = [0; 3];

    loop {
        let pc = registers[0];
        if pc < 0 {
            return Err(Error::NegativePc);
        }
        // Halt will change our pc to the memory size.
        // TODO: Consider other unsave instructions that do this and return an
        // Error instead.
        if pc >= memory.size().into() {
            return Ok(());
        }

        // Don't keep a mutable reference on pc, so the other registers can be
        // accessed mutably.
        registers[0] = match memory.0[usize::from(pc as u16)..] {
            [0x01, reg, addr, ..] => {
                Instruction::LoadWord { reg, addr }.execute(&mut registers, memory, pc)?
            }
            [0x02, reg, addr, ..] => {
                Instruction::StoreWord { reg, addr }.execute(&mut registers, memory, pc)?
            }
            [0x03, first_reg, second_reg, ..] => Instruction::Add {
                first_reg,
                second_reg,
            }
            .execute(&mut registers, memory, pc)?,
            [0x04, first_reg, second_reg, ..] => Instruction::Sub {
                first_reg,
                second_reg,
            }
            .execute(&mut registers, memory, pc)?,
            [0x05, reg, constant, ..] => {
                Instruction::AddImmediate { reg, constant }.execute(&mut registers, memory, pc)?
            }
            [0x06, first_reg, second_reg, new_pc, ..] => Instruction::BranchIfEq {
                first_reg,
                second_reg,
                new_pc,
            }
            .execute(&mut registers, memory, pc)?,
            [0xff, ..] => Instruction::Halt.execute(&mut registers, memory, pc)?,
            _ => return Err(Error::UnknownInstruction),
        };
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
