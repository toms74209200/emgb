use crate::cpu;
use crate::peripherals;

impl cpu::Cpu {
    pub fn nop(&mut self, bus: &mut peripherals::Peripherals) {
        self.fetch(bus);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_nop() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0; 256].into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);

        cpu.regs.pc = 0xc000;
        peripherals.write(0xc000, 0x00);
        let initial_pc = cpu.regs.pc;
        cpu.nop(&mut peripherals);
        assert_eq!(cpu.regs.pc, initial_pc + 1);
    }
}
