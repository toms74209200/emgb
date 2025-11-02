use crate::cpu;
use crate::operand::{IO8, IO16, Reg16};
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
    pub fn inc16<S: Copy>(&mut self, bus: &mut peripherals::Peripherals, src: S)
    where
        Self: IO16<S>,
    {
        static STEP: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
        static VAL16: std::sync::atomic::AtomicU16 = std::sync::atomic::AtomicU16::new(0);

        match STEP.load(std::sync::atomic::Ordering::Relaxed) {
            0 => {
                if let Some(v) = self.read16(bus, src) {
                    VAL16.store(v.wrapping_add(1), std::sync::atomic::Ordering::Relaxed);
                    STEP.store(1, std::sync::atomic::Ordering::Relaxed);
                }
            }
            1 => {
                if self
                    .write16(bus, src, VAL16.load(std::sync::atomic::Ordering::Relaxed))
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
    pub fn dec<S: Copy>(&mut self, bus: &mut peripherals::Peripherals, src: S)
    where
        Self: IO8<S>,
    {
        static STEP: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
        static VAL8: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

        match STEP.load(std::sync::atomic::Ordering::Relaxed) {
            0 => {
                if let Some(v) = self.read8(bus, src) {
                    let result = v.wrapping_sub(1);
                    self.regs.set_zf(result == 0);
                    self.regs.set_nf(true);
                    self.regs.set_hf((v & 0x0f) == 0x00);
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
    pub fn dec16<S: Copy>(&mut self, bus: &mut peripherals::Peripherals, src: S)
    where
        Self: IO16<S>,
    {
        static STEP: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
        static VAL16: std::sync::atomic::AtomicU16 = std::sync::atomic::AtomicU16::new(0);

        match STEP.load(std::sync::atomic::Ordering::Relaxed) {
            0 => {
                if let Some(v) = self.read16(bus, src) {
                    VAL16.store(v.wrapping_sub(1), std::sync::atomic::Ordering::Relaxed);
                    STEP.store(1, std::sync::atomic::Ordering::Relaxed);
                }
            }
            1 => {
                if self
                    .write16(bus, src, VAL16.load(std::sync::atomic::Ordering::Relaxed))
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
    pub fn rl<S: Copy>(&mut self, bus: &mut peripherals::Peripherals, src: S)
    where
        Self: IO8<S>,
    {
        static STEP: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
        static VAL8: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

        match STEP.load(std::sync::atomic::Ordering::Relaxed) {
            0 => {
                if let Some(v) = self.read8(bus, src) {
                    let result = (v << 1) | self.regs.cf() as u8;
                    self.regs.set_zf(result == 0);
                    self.regs.set_nf(false);
                    self.regs.set_hf(false);
                    self.regs.set_cf(v & 0x80 > 0);
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
    pub fn bit<S: Copy>(&mut self, bus: &peripherals::Peripherals, bit: u8, src: S)
    where
        Self: IO8<S>,
    {
        if let Some(mut v) = self.read8(bus, src) {
            v &= 1 << bit;
            self.regs.set_zf(v == 0);
            self.regs.set_nf(false);
            self.regs.set_hf(true);
            self.fetch(bus);
        }
    }
    pub fn push16(&mut self, bus: &mut peripherals::Peripherals, val: u16) -> Option<()> {
        static STEP: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
        static VAL8: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
        match STEP.load(std::sync::atomic::Ordering::Relaxed) {
            0 => {
                STEP.store(1, std::sync::atomic::Ordering::Relaxed);
                None
            }
            1 => {
                let [lo, hi] = u16::to_le_bytes(val);
                self.regs.sp = self.regs.sp.wrapping_sub(1);
                bus.write(self.regs.sp, hi);
                VAL8.store(lo, std::sync::atomic::Ordering::Relaxed);
                STEP.store(2, std::sync::atomic::Ordering::Relaxed);
                return None;
            }
            2 => {
                self.regs.sp = self.regs.sp.wrapping_sub(1);
                bus.write(
                    self.regs.sp,
                    VAL8.load(std::sync::atomic::Ordering::Relaxed),
                );
                STEP.store(3, std::sync::atomic::Ordering::Relaxed);
                return None;
            }
            3 => {
                return Some(STEP.store(0, std::sync::atomic::Ordering::Relaxed));
            }
            _ => unreachable!(),
        }
    }
    pub fn push(&mut self, bus: &mut peripherals::Peripherals, src: Reg16) {
        static STEP: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
        static VAL16: std::sync::atomic::AtomicU16 = std::sync::atomic::AtomicU16::new(0);
        match STEP.load(std::sync::atomic::Ordering::Relaxed) {
            0 => {
                VAL16.store(
                    self.read16(bus, src).unwrap(),
                    std::sync::atomic::Ordering::Relaxed,
                );
                STEP.store(1, std::sync::atomic::Ordering::Relaxed);
            }
            1 => {
                if self
                    .push16(bus, VAL16.load(std::sync::atomic::Ordering::Relaxed))
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

    #[test]
    fn test_inc16() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0x42; 256].into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);

        cpu.regs.pc = 0;
        cpu.regs.write_bc(0x1234);

        for _ in 0..3 {
            cpu.inc16(&mut peripherals, crate::operand::Reg16::BC);
        }

        assert_eq!(cpu.regs.bc(), 0x1235);
        assert!(!cpu.regs.zf());
        assert!(!cpu.regs.nf());
        assert!(!cpu.regs.hf());
    }

    #[test]
    fn test_inc16_overflow() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0x42; 256].into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);

        cpu.regs.pc = 0;
        cpu.regs.write_bc(0xFFFF);

        for _ in 0..3 {
            cpu.inc16(&mut peripherals, crate::operand::Reg16::BC);
        }

        assert_eq!(cpu.regs.bc(), 0x0000);
        assert!(!cpu.regs.zf());
        assert!(!cpu.regs.nf());
        assert!(!cpu.regs.hf());
    }

    #[test]
    fn test_inc16_half_carry() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0x42; 256].into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);

        cpu.regs.pc = 0;
        cpu.regs.write_bc(0x12FF);

        for _ in 0..3 {
            cpu.inc16(&mut peripherals, crate::operand::Reg16::BC);
        }

        assert_eq!(cpu.regs.bc(), 0x1300);
        assert!(!cpu.regs.zf());
        assert!(!cpu.regs.nf());
        assert!(!cpu.regs.hf());
    }

    #[test]
    fn test_dec() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0x42; 256].into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);

        cpu.regs.pc = 0;
        cpu.regs.a = 0x42;

        for _ in 0..2 {
            cpu.dec(&mut peripherals, crate::operand::Reg8::A);
        }

        assert_eq!(cpu.regs.a, 0x41);
        assert!(!cpu.regs.zf());
        assert!(cpu.regs.nf());
        assert!(!cpu.regs.hf());
    }

    #[test]
    fn test_dec_half_carry() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0x42; 256].into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);

        cpu.regs.pc = 0;
        cpu.regs.a = 0x10;

        for _ in 0..2 {
            cpu.dec(&mut peripherals, crate::operand::Reg8::A);
        }

        assert_eq!(cpu.regs.a, 0x0F);
        assert!(!cpu.regs.zf());
        assert!(cpu.regs.nf());
        assert!(cpu.regs.hf());
    }

    #[test]
    fn test_dec_zero_flag() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0x42; 256].into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);

        cpu.regs.pc = 0;
        cpu.regs.a = 0x01;

        for _ in 0..2 {
            cpu.dec(&mut peripherals, crate::operand::Reg8::A);
        }

        assert_eq!(cpu.regs.a, 0x00);
        assert!(cpu.regs.zf());
        assert!(cpu.regs.nf());
        assert!(!cpu.regs.hf());
    }

    #[test]
    fn test_dec16() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0x42; 256].into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);

        cpu.regs.pc = 0;
        cpu.regs.write_bc(0x1234);

        cpu.dec16(&mut peripherals, crate::operand::Reg16::BC);
        cpu.dec16(&mut peripherals, crate::operand::Reg16::BC);
        cpu.dec16(&mut peripherals, crate::operand::Reg16::BC);

        assert_eq!(cpu.regs.bc(), 0x1233);
    }

    #[test]
    fn test_dec16_underflow() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0x42; 256].into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);

        cpu.regs.pc = 0;
        cpu.regs.write_bc(0x0000);

        cpu.dec16(&mut peripherals, crate::operand::Reg16::BC);
        cpu.dec16(&mut peripherals, crate::operand::Reg16::BC);
        cpu.dec16(&mut peripherals, crate::operand::Reg16::BC);

        assert_eq!(cpu.regs.bc(), 0xFFFF);
    }

    #[test]
    fn test_rl() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0x42; 256].into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);

        cpu.regs.pc = 0;
        cpu.regs.b = 0x7F;
        cpu.regs.set_cf(false);

        cpu.rl(&mut peripherals, crate::operand::Reg8::B);
        cpu.rl(&mut peripherals, crate::operand::Reg8::B);

        assert_eq!(cpu.regs.b, 0xFE);
        assert!(!cpu.regs.zf());
        assert!(!cpu.regs.nf());
        assert!(!cpu.regs.hf());
        assert!(!cpu.regs.cf());
    }

    #[test]
    fn test_rl_with_carry() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0x42; 256].into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);

        cpu.regs.pc = 0;
        cpu.regs.b = 0x80;
        cpu.regs.set_cf(true);

        cpu.rl(&mut peripherals, crate::operand::Reg8::B);
        cpu.rl(&mut peripherals, crate::operand::Reg8::B);

        assert_eq!(cpu.regs.b, 0x01);
        assert!(!cpu.regs.zf());
        assert!(!cpu.regs.nf());
        assert!(!cpu.regs.hf());
        assert!(cpu.regs.cf());
    }

    #[test]
    fn test_rl_zero_flag() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0x42; 256].into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);

        cpu.regs.pc = 0;
        cpu.regs.b = 0x80;
        cpu.regs.set_cf(false);

        cpu.rl(&mut peripherals, crate::operand::Reg8::B);
        cpu.rl(&mut peripherals, crate::operand::Reg8::B);

        assert_eq!(cpu.regs.b, 0x00);
        assert!(cpu.regs.zf());
        assert!(!cpu.regs.nf());
        assert!(!cpu.regs.hf());
        assert!(cpu.regs.cf());
    }

    #[test]
    fn test_bit() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0x42; 256].into_boxed_slice());
        let peripherals = peripherals::Peripherals::new(bootrom);

        cpu.regs.pc = 0;
        cpu.regs.b = 0b0100_0010;

        cpu.bit(&peripherals, 6, crate::operand::Reg8::B);
        assert!(!cpu.regs.zf());
        assert!(!cpu.regs.nf());
        assert!(cpu.regs.hf());

        cpu.bit(&peripherals, 5, crate::operand::Reg8::B);
        assert!(cpu.regs.zf());
        assert!(!cpu.regs.nf());
        assert!(cpu.regs.hf());
    }

    #[test]
    fn test_bit_all_positions() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0x42; 256].into_boxed_slice());
        let peripherals = peripherals::Peripherals::new(bootrom);

        cpu.regs.pc = 0;
        cpu.regs.a = 0b1010_1010;

        for bit in 0..8 {
            let expected_set = (cpu.regs.a >> bit) & 1 == 1;
            cpu.bit(&peripherals, bit, crate::operand::Reg8::A);

            if expected_set {
                assert!(!cpu.regs.zf(), "Bit {} should be set", bit);
            } else {
                assert!(cpu.regs.zf(), "Bit {} should be clear", bit);
            }
            assert!(!cpu.regs.nf());
            assert!(cpu.regs.hf());
        }
    }

    #[test]
    fn test_bit_carry_unchanged() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0x42; 256].into_boxed_slice());
        let peripherals = peripherals::Peripherals::new(bootrom);

        cpu.regs.pc = 0;
        cpu.regs.b = 0xFF;

        cpu.regs.set_cf(true);

        cpu.bit(&peripherals, 7, crate::operand::Reg8::B);
        assert!(cpu.regs.cf());

        cpu.regs.set_cf(false);

        cpu.bit(&peripherals, 0, crate::operand::Reg8::B);
        assert!(!cpu.regs.cf());
    }

    #[test]
    fn test_push16() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0x00; 256].into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);

        cpu.regs.sp = 0xFFFE;

        let val = 0x1234;

        cpu.push16(&mut peripherals, val);
        cpu.push16(&mut peripherals, val);
        cpu.push16(&mut peripherals, val);
        cpu.push16(&mut peripherals, val);

        let lo = peripherals.read(0xFFFC);
        let hi = peripherals.read(0xFFFD);
        let pushed_val = u16::from_le_bytes([lo, hi]);

        assert_eq!(pushed_val, val);
        assert_eq!(cpu.regs.sp, 0xFFFC);
    }

    #[test]
    fn test_push16_register() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0x00; 256].into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);

        cpu.regs.sp = 0xFFFE;
        cpu.regs.write_bc(0x5678);

        cpu.push16(&mut peripherals, cpu.regs.bc());
        cpu.push16(&mut peripherals, cpu.regs.bc());
        cpu.push16(&mut peripherals, cpu.regs.bc());
        cpu.push16(&mut peripherals, cpu.regs.bc());

        let lo = peripherals.read(0xFFFC);
        let hi = peripherals.read(0xFFFD);
        let pushed_val = u16::from_le_bytes([lo, hi]);

        assert_eq!(pushed_val, 0x5678);
        assert_eq!(cpu.regs.sp, 0xFFFC);
    }

    #[test]
    fn test_push16_multiple() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0x00; 256].into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);

        cpu.regs.sp = 0xFFFE;

        let val1 = 0x1234;
        cpu.push16(&mut peripherals, val1);
        cpu.push16(&mut peripherals, val1);
        cpu.push16(&mut peripherals, val1);
        cpu.push16(&mut peripherals, val1);

        let val2 = 0x5678;
        cpu.push16(&mut peripherals, val2);
        cpu.push16(&mut peripherals, val2);
        cpu.push16(&mut peripherals, val2);
        cpu.push16(&mut peripherals, val2);

        let lo2 = peripherals.read(0xFFFA);
        let hi2 = peripherals.read(0xFFFB);
        let pushed_val2 = u16::from_le_bytes([lo2, hi2]);

        let lo1 = peripherals.read(0xFFFC);
        let hi1 = peripherals.read(0xFFFD);
        let pushed_val1 = u16::from_le_bytes([lo1, hi1]);

        assert_eq!(pushed_val1, val1);
        assert_eq!(pushed_val2, val2);
        assert_eq!(cpu.regs.sp, 0xFFFA);
    }

    #[test]
    fn test_push() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0x00; 256].into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);

        cpu.regs.sp = 0xFFFE;
        cpu.regs.write_bc(0x9ABC);

        cpu.push(&mut peripherals, Reg16::BC);
        cpu.push(&mut peripherals, Reg16::BC);
        cpu.push(&mut peripherals, Reg16::BC);
        cpu.push(&mut peripherals, Reg16::BC);
        let lo = peripherals.read(0xFFFC);
        let hi = peripherals.read(0xFFFD);
        let pushed_val = u16::from_le_bytes([lo, hi]);
        assert_eq!(pushed_val, 0x9ABC);
        assert_eq!(cpu.regs.sp, 0xFFFC);
    }
}
