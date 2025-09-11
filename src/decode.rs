use crate::cpu;
use crate::peripherals;

impl cpu::Cpu {
    pub fn decode(&mut self, bus: &mut peripherals::Peripherals) {
        match self.ctx.opcode {
            0x00 => self.nop(bus),
            _ => unimplemented!("opcode {:02x} not implemented", self.ctx.opcode),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_decode_nop() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0; 256].into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);
        cpu.regs.pc = 0xc000;
        peripherals.write(0xc000, 0x00);
        cpu.fetch(&peripherals);
        let initial_pc = cpu.regs.pc;
        cpu.decode(&mut peripherals);
        assert_eq!(cpu.regs.pc, initial_pc + 1);
    }
}
