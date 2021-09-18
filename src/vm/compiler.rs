use super::Error;

pub fn decompile(memory: &[u8]) -> Result<Vec<String>, Error> {
    let (current, next) = match memory {
        [0x01, reg, addr, ..] => (format!("load_word r{} ({:#04x})", reg, addr), &memory[3..]),
        [0x02, reg, addr, ..] => (format!("store_word r{} ({:#04x})", reg, addr), &memory[3..]),
        // add
        [0x03, first_reg, second_reg, ..] => {
            (format!("add r{} r{}", first_reg, second_reg), &memory[3..])
        }
        // sub
        [0x04, first_reg, second_reg, ..] => {
            (format!("sub r{} r{}", first_reg, second_reg), &memory[3..])
        }
        // add_immediate
        [0x05, reg, constant, ..] => (
            format!("add_immediate r{} ({:#04x})", reg, constant),
            &memory[3..],
        ),
        // branch_if_eq
        [0x06, first_reg, second_reg, new_pc, ..] => (
            format!(
                "branch_if_eq r{} r{} {:#04x}",
                first_reg, second_reg, new_pc
            ),
            &memory[4..],
        ),
        // halt
        [0xff, ..] => ("halt".to_string(), &memory[1..]),
        [x, ..] => (format!("{:#04x}", x), &memory[1..]),
        [] => return Ok(vec![]),
    };

    let mut result = vec![current];

    let next_result = decompile(next)?;
    result.extend(next_result);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decompiles() {
        #[rustfmt::skip]
        assert_eq!(
            Ok(
"load_word r1 (0x10)
add_immediate r1 (0x02)
store_word r1 (0x0e)
halt
0x00
0x00
0x00
0x00
0x00
0x00
0xa1
0x14
0x00
0x00"
            .to_string()),
            decompile(&[
                0x01, 0x01, 0x10,
                0x05, 0x01, 0x02,
                0x02, 0x01, 0x0e,
                0xff,
                0x00, 0x00, 0x00,
                0x00, 0x00, 0x00,
                0xa1, 0x14,
                0x00, 0x00,
            ])
            .map(|x| x.join("\n")),
        );
    }
}
