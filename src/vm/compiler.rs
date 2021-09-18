use super::instruction::*;
use super::Error;

pub fn decompile(memory: &[u8]) -> Result<Vec<Instruction>, Error> {
    let instruction = match memory {
        [0x01, reg, addr, ..] => Instruction::LoadWord {
            reg: *reg,
            addr: *addr,
        },
        [0x02, reg, addr, ..] => Instruction::StoreWord {
            reg: *reg,
            addr: *addr,
        },
        [0x03, first_reg, second_reg, ..] => Instruction::Add {
            first_reg: *first_reg,
            second_reg: *second_reg,
        },
        [0x04, first_reg, second_reg, ..] => Instruction::Sub {
            first_reg: *first_reg,
            second_reg: *second_reg,
        },
        [0x05, reg, constant, ..] => Instruction::AddImmediate {
            reg: *reg,
            constant: *constant,
        },
        [0x06, first_reg, second_reg, new_pc, ..] => Instruction::BranchIfEq {
            first_reg: *first_reg,
            second_reg: *second_reg,
            new_pc: *new_pc,
        },
        [0xff, ..] => Instruction::Halt,
        [x, ..] => Instruction::Unknown(*x),
        [] => return Ok(vec![]),
    };

    let instruction_size = usize::from(instruction.size());
    let mut result = vec![instruction];

    let next_result = decompile(&memory[instruction_size..])?;
    result.extend(next_result);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decompiles_1() {
        #[rustfmt::skip]
        assert_eq!(
            Ok(
"load_word r1 (0x01)
store_word r1 (0x02)
add r1 r2
sub r2 r1
add_immediate r1 (0x01)
halt
0xa1
0x14
0x00
0x00"
            .to_string()),
            decompile(&[
                0x01, 0x01, 0x01,
                0x02, 0x01, 0x02,
                0x03, 0x01, 0x02,
                0x04, 0x02, 0x01,
                0x05, 0x01, 0x01,
                0xff,
                0xa1, 0x14,
                0x00, 0x00,
            ])
            .map(|x| instructions_to_string(x)),
        );
    }

    #[test]
    fn decompiles_2() {
        #[rustfmt::skip]
        assert_eq!(
            Ok(
"branch_if_eq r1 r2 0x06
0x00
0x00
0x00
0x00
0x00
0x00
0x00
0x00
0x00
0x00
0x00
0x00
0x00
0x00
0x00
0x00"
            .to_string()),
            decompile(&[
                0x06, 0x01, 0x02, 0x06,
                0x00,
                0x00, 0x00, 0x00,
                0x00, 0x00, 0x00,
                0x00, 0x00, 0x00,
                0x00, 0x00, 0x00,
                0x00, 0x00, 0x00,
            ])
            .map(|x| instructions_to_string(x)),
        );
    }
}
