use crate::cpu;
use crate::operand::IO8;
use crate::peripherals;

impl cpu::Cpu {
    pub fn nop(&mut self, bus: &mut peripherals::Peripherals) {
        self.fetch(bus);
    }
    pub fn ld<D: Copy, S: Copy>(&mut self, bus: &mut peripherals::Peripherals, dst: D, src: S)
    where
        Self: crate::operand::IO8<D> + IO8<S>,
    {
        static STEP: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
        static VAL8: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

        match STEP.load(std::sync::atomic::Ordering::Relaxed) {
            0 => {
                if let Some(v) = self.read8(bus, src) {
                    VAL8.store(v, std::sync::atomic::Ordering::Relaxed);
                    STEP.store(1, std::sync::atomic::Ordering::Relaxed);
                }
            }
            1 => {
                if self
                    .write8(bus, dst, VAL8.load(std::sync::atomic::Ordering::Relaxed))
                    .is_some()
                {
                    STEP.store(2, std::sync::atomic::Ordering::Relaxed);
                }
            }
            2 => {
                STEP.store(0, std::sync::atomic::Ordering::Relaxed);
                self.fetch(bus);
            }
            _ => unreachable!(),
        }
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

    #[test]
    fn test_ld() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0x42; 256].into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);

        cpu.regs.pc = 0;

        cpu.ld(
            &mut peripherals,
            crate::operand::Reg8::A,
            crate::operand::Imm8,
        );
        cpu.ld(
            &mut peripherals,
            crate::operand::Reg8::A,
            crate::operand::Imm8,
        );
        cpu.ld(
            &mut peripherals,
            crate::operand::Reg8::A,
            crate::operand::Imm8,
        );

        assert_eq!(cpu.regs.a, 0x42);
    }
}
