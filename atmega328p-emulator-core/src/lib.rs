use avr::addons::instruction_listener::InstructionListener;
use elf_rs::*;

mod arduino_adapter;

pub fn emulate_from_elf(data: &[u8]) -> Result<(), Error> {
    let elf = Elf::from_bytes(data)?;
    let content = elf.content();
    let text_section = elf.lookup_section(b".text").unwrap();
    let content = &content[text_section.offset() as usize..][..text_section.size() as usize];

    let content = content.to_vec();
    emulate_program(&content);
    Ok(())
}

pub fn emulate_program(program: &[u8]) {
    let mut core = avr::Core::new::<avr::chips::atmega328p::Chip>();

    core.load_program_space(program.iter().map(|b| *b));

    core.pc = 0;

    let mut mcu = avr::Mcu::new(core);

    // let uart = avr::addons::Uart::new(
    //     16000000, // CPU frequency
    //     187000,   // Baud rate
    //     avr::io::Port::new(0x24), // Tx
    //     avr::io::Port::new(0x25), // Rx
    // );

    let arduino = arduino_adapter::ArduinoUno::default();
    // let breakpoint = arduino_adapter::BreakpointAddon {
    //     pc_values: vec![
    //         0xe0, 0xe2, 0xe4, 0xe6, 0xe8, 0xea, 0xec, 0xee, 0xf0, 0xf2, 0xf4, 0xf6, 0xf8, 0xfa,
    //         0xfc, 0xfe, 0x100, 0x102, 0x104, 0x106, 0x108, 0x10a, 0x10c, 0x10e, 0x110, 0x112,
    //         0x114, 0x116, 0x118, 0x11a, 0x11c, 0x11e, 0x120, 0x122, 0x124, 0x126, 0x128, 0x12a,
    //         0x12c, 0x12e, 0x130, 0x132, 0x134, 0x136, 0x13a, 0x13c, 0x13e, 0x142, 0x144, 0x148,
    //         0x14a, 0x14c, 0x14e, 0x150, 0x152, 0x154, 0x156, 0x158, 0x15c, 0x15e, 0x162, 0x164,
    //         0x168, 0x16a, 0x16c, 0x16e,
    //     ],
    //     base_addon: arduino_adapter::DumpRegistersAddon {},
    // };
    // mcu.attach(Box::new(arduino));
    mcu.attach(Box::new(InstructionListener {}));

    // mcu.attach(Box::new(uart));

    for _ in 0.. {
        mcu.tick().expect("failed while ticking");
    }
}
