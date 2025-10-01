use crate::cpu;
use crate::operand::{IO8, IO16};
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
    pub fn ld16<D: Copy, S: Copy>(&mut self, bus: &mut peripherals::Peripherals, dst: D, src: S)
    where
        Self: crate::operand::IO16<D> + IO16<S>,
    {
        static STEP: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
        static VAL16: std::sync::atomic::AtomicU16 = std::sync::atomic::AtomicU16::new(0);

        match STEP.load(std::sync::atomic::Ordering::Relaxed) {
            0 => {
                if let Some(v) = self.read16(bus, src) {
                    self.regs.pc = self.regs.pc.wrapping_add(1);
                    VAL16.store(v, std::sync::atomic::Ordering::Relaxed);
                    STEP.store(1, std::sync::atomic::Ordering::Relaxed);
                }
            }
            1 => {
                if self
                    .write16(bus, dst, VAL16.load(std::sync::atomic::Ordering::Relaxed))
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
    pub fn cp<S: Copy>(&mut self, bus: &peripherals::Peripherals, src: S)
    where
        Self: IO8<S>,
    {
        if let Some(v) = self.read8(bus, src) {
            let (result, carry) = self.regs.a.overflowing_sub(v);
            self.regs.set_zf(result == 0);
            self.regs.set_nf(true);
            self.regs.set_hf((self.regs.a & 0x0f) < (v & 0x0f));
            self.regs.set_cf(carry);
            self.fetch(bus);
        }
    }
    pub fn inc<S: Copy>(&mut self, bus: &mut peripherals::Peripherals, src: S)
    where
        Self: IO8<S>,
    {
        static STEP: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
        static VAL8: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

        match STEP.load(std::sync::atomic::Ordering::Relaxed) {
            0 => {
                if let Some(v) = self.read8(bus, src) {
                    let result = v.wrapping_add(1);
                    self.regs.set_zf(result == 0);
                    self.regs.set_nf(false);
                    self.regs.set_hf(v & 0x0f == 0x0f);
                    VAL8.store(result, std::sync::atomic::Ordering::Relaxed);
                    STEP.store(1, std::sync::atomic::Ordering::Relaxed);
                }
            }
            1 => {
                if self
                    .write8(bus, src, VAL8.load(std::sync::atomic::Ordering::Relaxed))
                    .is_some()
                {
                    STEP.store(0, std::sync::atomic::Ordering::Relaxed);
                    self.fetch(bus);
                }
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

    #[test]
    fn test_ld16() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let mut bootrom_data = Vec::new();
        for _ in 0..128 {
            bootrom_data.push(0x34);
            bootrom_data.push(0x12);
        }
        let bootrom = crate::bootrom::Bootrom::new(bootrom_data.into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);

        cpu.regs.pc = 0;

        for _ in 0..5 {
            cpu.ld16(
                &mut peripherals,
                crate::operand::Reg16::BC,
                crate::operand::Imm16,
            );
        }

        assert_eq!(cpu.regs.bc(), 0x1234);
    }

    #[test]
    fn test_cp() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0x42; 256].into_boxed_slice());
        let peripherals = peripherals::Peripherals::new(bootrom);

        cpu.regs.pc = 0;
        cpu.regs.a = 0x50;

        for _ in 0..3 {
            cpu.cp(&peripherals, crate::operand::Imm8);
            if cpu.regs.nf() {
                break;
            }
        }
        assert!(!cpu.regs.zf());
        assert!(cpu.regs.nf());
        assert!(cpu.regs.hf());
        assert!(!cpu.regs.cf());
    }

    #[test]
    fn test_cp_equal() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0x42; 256].into_boxed_slice());
        let peripherals = peripherals::Peripherals::new(bootrom);

        cpu.regs.pc = 0;
        cpu.regs.a = 0x42;

        for _ in 0..3 {
            cpu.cp(&peripherals, crate::operand::Imm8);
            if cpu.regs.zf() {
                break;
            }
        }
        assert!(cpu.regs.zf());
        assert!(cpu.regs.nf());
        assert!(!cpu.regs.hf());
        assert!(!cpu.regs.cf());
    }

    #[test]
    fn test_cp_underflow() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0x42; 256].into_boxed_slice());
        let peripherals = peripherals::Peripherals::new(bootrom);

        cpu.regs.pc = 0;
        cpu.regs.a = 0x30;

        for _ in 0..3 {
            cpu.cp(&peripherals, crate::operand::Imm8);
            if cpu.regs.cf() {
                break;
            }
        }
        assert!(!cpu.regs.zf());
        assert!(cpu.regs.nf());
        assert!(cpu.regs.hf());
        assert!(cpu.regs.cf());
    }

    #[test]
    fn test_inc() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0x42; 256].into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);

        cpu.regs.pc = 0;
        cpu.regs.a = 0x42;

        for _ in 0..2 {
            cpu.inc(&mut peripherals, crate::operand::Reg8::A);
        }

        assert_eq!(cpu.regs.a, 0x43);
        assert!(!cpu.regs.zf());
        assert!(!cpu.regs.nf());
        assert!(!cpu.regs.hf());
    }

    #[test]
    fn test_inc_half_carry() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0x42; 256].into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);

        cpu.regs.pc = 0;
        cpu.regs.a = 0x0F;

        for _ in 0..2 {
            cpu.inc(&mut peripherals, crate::operand::Reg8::A);
        }

        assert_eq!(cpu.regs.a, 0x10);
        assert!(!cpu.regs.zf());
        assert!(!cpu.regs.nf());
        assert!(cpu.regs.hf());
    }

    #[test]
    fn test_inc_zero_flag() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0x42; 256].into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);

        cpu.regs.pc = 0;
        cpu.regs.a = 0xFF;

        for _ in 0..2 {
            cpu.inc(&mut peripherals, crate::operand::Reg8::A);
        }

        assert_eq!(cpu.regs.a, 0x00);
        assert!(cpu.regs.zf());
        assert!(!cpu.regs.nf());
        assert!(cpu.regs.hf());
    }
}
