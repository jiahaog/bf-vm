use super::error::Error;
use super::memory::Memory;
use std::fmt;

// TODO: Consider single source of truth that maps addresses to the instruction.

pub enum Instruction {
    LoadWord {
        reg: u8,
        addr: u8,
    },
    StoreWord {
        reg: u8,
        addr: u8,
    },
    Add {
        first_reg: u8,
        second_reg: u8,
    },
    Sub {
        first_reg: u8,
        second_reg: u8,
    },
    AddImmediate {
        reg: u8,
        constant: u8,
    },
    /// If first_reg and second_reg contain same value, pc jumps to new_pc.
    BranchIfEq {
        first_reg: u8,
        second_reg: u8,
        new_pc: u8,
    },
    Halt,
    Unknown(u8),
}

impl Instruction {
    /// Returns the new pc.
    pub fn execute(
        self,
        registers: &mut [i16; 3],
        memory: &mut Memory,
        pc: i16,
    ) -> Result<i16, Error> {
        use Instruction::*;

        match self {
            LoadWord { reg, addr } => {
                registers[usize::from(reg)] =
                    i16::from_le_bytes([*memory.get_mut(addr)?, *memory.get_mut(addr + 1)?]);
                Ok(pc + 3)
            }
            StoreWord { reg, addr } => {
                let le_bytes = registers[usize::from(reg)].to_le_bytes();

                *memory.get_mut(addr)? = le_bytes[0];
                *memory.get_mut(addr + 1)? = le_bytes[1];

                Ok(pc + 3)
            }
            Add {
                first_reg,
                second_reg,
            } => {
                registers[usize::from(first_reg)] =
                    registers[usize::from(first_reg)] + registers[usize::from(second_reg)];

                Ok(pc + 3)
            }
            Sub {
                first_reg,
                second_reg,
            } => {
                registers[usize::from(first_reg)] =
                    registers[usize::from(first_reg)] - registers[usize::from(second_reg)];

                Ok(pc + 3)
            }
            AddImmediate { reg, constant } => {
                registers[usize::from(reg)] = registers[usize::from(reg)] + i16::from(constant);

                Ok(pc + 3)
            }
            BranchIfEq {
                first_reg,
                second_reg,
                new_pc,
            } => {
                if registers[usize::from(first_reg)] == registers[usize::from(second_reg)] {
                    Ok(i16::from(new_pc))
                } else {
                    Ok(pc + 4)
                }
            }
            Halt => Ok(memory.size().into()),
            Unknown(_) => Err(Error::UnknownInstruction),
        }
    }

    pub fn size(&self) -> u8 {
        match self {
            Instruction::LoadWord { reg: _, addr: _ } => 3,
            Instruction::StoreWord { reg: _, addr: _ } => 3,
            Instruction::Add {
                first_reg: _,
                second_reg: _,
            } => 3,
            Instruction::Sub {
                first_reg: _,
                second_reg: _,
            } => 3,
            Instruction::AddImmediate {
                reg: _,
                constant: _,
            } => 3,
            Instruction::BranchIfEq {
                first_reg: _,
                second_reg: _,
                new_pc: _,
            } => 4,
            Instruction::Halt => 1,
            Instruction::Unknown(_) => 1,
        }
    }
}

pub fn instructions_to_string(instructions: Vec<Instruction>) -> String {
    instructions
        .into_iter()
        .map(|x| format!("{}", x))
        .collect::<Vec<String>>()
        .join("\n")
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Instruction::*;

        match self {
            LoadWord { reg, addr } => write!(f, "load_word r{} ({:#04x})", reg, addr),
            StoreWord { reg, addr } => write!(f, "store_word r{} ({:#04x})", reg, addr),
            Add {
                first_reg,
                second_reg,
            } => write!(f, "add r{} r{}", first_reg, second_reg),
            Sub {
                first_reg,
                second_reg,
            } => write!(f, "sub r{} r{}", first_reg, second_reg),
            AddImmediate { reg, constant } => {
                write!(f, "add_immediate r{} ({:#04x})", reg, constant)
            }
            BranchIfEq {
                first_reg,
                second_reg,
                new_pc,
            } => write!(
                f,
                "branch_if_eq r{} r{} {:#04x}",
                first_reg, second_reg, new_pc
            ),
            Halt => write!(f, "halt"),
            Unknown(x) => write!(f, "{:#04x}", x),
        }
    }
}
