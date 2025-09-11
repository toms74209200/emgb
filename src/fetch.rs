use crate::cpu;
use crate::peripherals;

impl cpu::Cpu {
    pub fn fetch(&mut self, bus: &peripherals::Peripherals) {
        self.ctx.opcode = bus.read(self.regs.pc);
        self.regs.pc = self.regs.pc.wrapping_add(1);
        self.ctx.cb = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0; 256].into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);

        cpu.regs.pc = 0xc000;
        peripherals.write(0xc000, 42);

        cpu.fetch(&peripherals);

        assert_eq!(cpu.ctx.opcode, 42);
        assert_eq!(cpu.regs.pc, 0xc001);
        assert_eq!(cpu.ctx.cb, false);
    }
}
