fn main() {
    let elf = std::fs::read("../blink.elf").unwrap();
    atmega328p_emulator_core::emulate_from_elf(&elf).unwrap();
}
