use bf_vm::*;

fn main() {
    #[rustfmt::skip]
    let mut raw_memory = [
        0x01, 0x01, 0x10,
        0x01, 0x02, 0x12,
        0x03, 0x01, 0x02,
        0x02, 0x01, 0x0e,
        0xff,
        0x00,
        0x00, 0x00,
        0xa1, 0x14,
        0x0c, 0x00,
    ];

    let decompiled = vm::compiler::decompile(&raw_memory).expect("Can decompile the memory");
    eprintln!("Decompiled:\n\n{}\n", decompiled.join("\n"));

    eprintln!("Executing...\n");

    let mut memory = vm::Memory::new(&mut raw_memory);

    vm::run(&mut memory).expect("Should complete successfully");

    eprintln!("Final memory:\n\n{}\n", memory);
    eprintln!("Output:");

    println!(
        "{}",
        u16::from_le_bytes([raw_memory[0x0e], raw_memory[0x0f]])
    );
}
