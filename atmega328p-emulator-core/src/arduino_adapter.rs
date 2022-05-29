use avr::{Addon, Instruction};

#[derive(Debug, Default)]
pub struct ArduinoUno {
    old_value: u8,
}

impl Addon for ArduinoUno {
    fn tick(&mut self, core: &mut avr::Core, _inst: Instruction, _: u32) -> Result<(), avr::Error> {
        let value = core.register_file().gpr(25)?;
        if value != self.old_value {
            println!("Some pin has changed! {} -> {}", self.old_value, value);
            self.old_value = value;
        }
        Ok(())
    }
}

pub struct BreakpointAddon<A: Addon> {
    pub pc_values: Vec<u32>,
    pub base_addon: A,
}

impl<A: Addon> Addon for BreakpointAddon<A> {
    fn tick(&mut self, core: &mut avr::Core, inst: Instruction, pc: u32) -> Result<(), avr::Error> {
        if self.pc_values.contains(&pc) {
            println!("Hit breakpoint on 0x{:x}: {:?}", pc, inst);
            self.base_addon.tick(core, inst, pc)
        } else {
            Ok(())
        }
    }
}

pub struct DumpRegistersAddon {}

impl Addon for DumpRegistersAddon {
    fn tick(&mut self, core: &mut avr::Core, _: Instruction, _: u32) -> Result<(), avr::Error> {
        for (index, register) in core.register_file().registers().enumerate() {
            print!("{:3}: {:3}      ", register.name, register.value);
            if index % 8 == 7 {
                println!();
            }
        }
        println!();
        Ok(())
    }
}