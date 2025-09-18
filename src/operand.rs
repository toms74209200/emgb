use crate::cpu;
use crate::peripherals;

pub trait IO8<T: Copy> {
    fn read8(&mut self, bus: &peripherals::Peripherals, src: T) -> Option<u8>;
    fn write8(&mut self, bus: &mut peripherals::Peripherals, dst: T, val: u8) -> Option<()>;
}
impl IO8<Reg8> for cpu::Cpu {
    fn read8(&mut self, _bus: &peripherals::Peripherals, src: Reg8) -> Option<u8> {
        Some(match src {
            Reg8::A => self.regs.a,
            Reg8::B => self.regs.b,
            Reg8::C => self.regs.c,
            Reg8::D => self.regs.d,
            Reg8::E => self.regs.e,
            Reg8::H => self.regs.h,
            Reg8::L => self.regs.l,
        })
    }
    fn write8(&mut self, _bus: &mut peripherals::Peripherals, dst: Reg8, val: u8) -> Option<()> {
        Some(match dst {
            Reg8::A => self.regs.a = val,
            Reg8::B => self.regs.b = val,
            Reg8::C => self.regs.c = val,
            Reg8::D => self.regs.d = val,
            Reg8::E => self.regs.e = val,
            Reg8::H => self.regs.h = val,
            Reg8::L => self.regs.l = val,
        })
    }
}
impl IO8<Imm8> for cpu::Cpu {
    fn read8(&mut self, bus: &peripherals::Peripherals, _: Imm8) -> Option<u8> {
        static STEP: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
        static VAL8: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
        match STEP.load(std::sync::atomic::Ordering::Relaxed) {
            0 => {
                VAL8.store(bus.read(self.regs.pc), std::sync::atomic::Ordering::Relaxed);
                self.regs.pc = self.regs.pc.wrapping_add(1);
                STEP.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                None
            }
            1 => {
                STEP.store(0, std::sync::atomic::Ordering::Relaxed);
                Some(VAL8.load(std::sync::atomic::Ordering::Relaxed))
            }
            _ => unreachable!(),
        }
    }

    fn write8(&mut self, _: &mut peripherals::Peripherals, _: Imm8, _: u8) -> Option<()> {
        unreachable!()
    }
}

pub trait IO16<T: Copy> {
    fn read16(&mut self, bus: &peripherals::Peripherals, src: T) -> Option<u16>;
    fn write16(&mut self, bus: &mut peripherals::Peripherals, dst: T, val: u16) -> Option<()>;
}
impl IO16<Reg16> for cpu::Cpu {
    fn read16(&mut self, _bus: &peripherals::Peripherals, src: Reg16) -> Option<u16> {
        Some(match src {
            Reg16::AF => self.regs.af(),
            Reg16::BC => self.regs.bc(),
            Reg16::DE => self.regs.de(),
            Reg16::HL => self.regs.hl(),
            Reg16::SP => self.regs.sp,
        })
    }
    fn write16(&mut self, _bus: &mut peripherals::Peripherals, dst: Reg16, val: u16) -> Option<()> {
        Some(match dst {
            Reg16::AF => self.regs.write_af(val),
            Reg16::BC => self.regs.write_bc(val),
            Reg16::DE => self.regs.write_de(val),
            Reg16::HL => self.regs.write_hl(val),
            Reg16::SP => self.regs.sp = val,
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}
#[derive(Clone, Copy, Debug)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
}
#[derive(Clone, Copy, Debug)]
pub struct Imm8;
#[derive(Clone, Copy, Debug)]
pub struct Imm16;
#[derive(Clone, Copy, Debug)]
pub enum Indirect {
    BC,
    DE,
    HL,
    CEF,
    HLD,
    HLI,
}
#[derive(Clone, Copy, Debug)]
pub enum Direct8 {
    D,
    DFE,
}
#[derive(Clone, Copy, Debug)]
pub struct Direct16;
#[derive(Clone, Copy, Debug)]
pub enum Cond {
    Nz,
    Z,
    NC,
    C,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_io8_read() {
        let a_expected = rand::rng().random();
        let b_expected = rand::rng().random();
        let c_expected = rand::rng().random();
        let d_expected = rand::rng().random();
        let e_expected = rand::rng().random();
        let f_expected = rand::rng().random();
        let l_expected = rand::rng().random();

        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0; 256].into_boxed_slice());
        let peripherals = peripherals::Peripherals::new(bootrom);
        cpu.regs.a = a_expected;
        cpu.regs.b = b_expected;
        cpu.regs.c = c_expected;
        cpu.regs.d = d_expected;
        cpu.regs.e = e_expected;
        cpu.regs.h = f_expected;
        cpu.regs.l = l_expected;
        assert_eq!(cpu.read8(&peripherals, Reg8::A), Some(a_expected));
        assert_eq!(cpu.read8(&peripherals, Reg8::B), Some(b_expected));
        assert_eq!(cpu.read8(&peripherals, Reg8::C), Some(c_expected));
        assert_eq!(cpu.read8(&peripherals, Reg8::D), Some(d_expected));
        assert_eq!(cpu.read8(&peripherals, Reg8::E), Some(e_expected));
        assert_eq!(cpu.read8(&peripherals, Reg8::H), Some(f_expected));
        assert_eq!(cpu.read8(&peripherals, Reg8::L), Some(l_expected));
    }
    #[test]
    fn test_io8_write() {
        let a_expected = rand::rng().random();
        let b_expected = rand::rng().random();
        let c_expected = rand::rng().random();
        let d_expected = rand::rng().random();
        let e_expected = rand::rng().random();
        let f_expected = rand::rng().random();
        let l_expected = rand::rng().random();

        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0; 256].into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);
        cpu.regs.a = a_expected;
        cpu.regs.b = b_expected;
        cpu.regs.c = c_expected;
        cpu.regs.d = d_expected;
        cpu.regs.e = e_expected;
        cpu.regs.h = f_expected;
        cpu.regs.l = l_expected;
        cpu.write8(&mut peripherals, Reg8::A, a_expected);
        cpu.write8(&mut peripherals, Reg8::B, b_expected);
        cpu.write8(&mut peripherals, Reg8::C, c_expected);
        cpu.write8(&mut peripherals, Reg8::D, d_expected);
        cpu.write8(&mut peripherals, Reg8::E, e_expected);
        cpu.write8(&mut peripherals, Reg8::H, f_expected);
        cpu.write8(&mut peripherals, Reg8::L, l_expected);
        assert_eq!(cpu.regs.a, a_expected);
        assert_eq!(cpu.regs.b, b_expected);
        assert_eq!(cpu.regs.c, c_expected);
        assert_eq!(cpu.regs.d, d_expected);
        assert_eq!(cpu.regs.e, e_expected);
        assert_eq!(cpu.regs.h, f_expected);
        assert_eq!(cpu.regs.l, l_expected);
    }

    #[test]
    fn test_io16_read() {
        let af_expected = rand::rng().random();
        let bc_expected = rand::rng().random();
        let de_expected = rand::rng().random();
        let hl_expected = rand::rng().random();
        let sp_expected = rand::rng().random();
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0; 256].into_boxed_slice());
        let peripherals = peripherals::Peripherals::new(bootrom);
        cpu.regs.write_af(af_expected);
        cpu.regs.write_bc(bc_expected);
        cpu.regs.write_de(de_expected);
        cpu.regs.write_hl(hl_expected);
        cpu.regs.sp = sp_expected;
        assert_eq!(
            cpu.read16(&peripherals, Reg16::AF),
            Some(af_expected & 0xfff0)
        );
        assert_eq!(cpu.read16(&peripherals, Reg16::BC), Some(bc_expected));
        assert_eq!(cpu.read16(&peripherals, Reg16::DE), Some(de_expected));
        assert_eq!(cpu.read16(&peripherals, Reg16::HL), Some(hl_expected));
        assert_eq!(cpu.read16(&peripherals, Reg16::SP), Some(sp_expected));
    }

    #[test]
    fn test_io8_read_imm() {
        let val_expected = rand::rng().random();
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let mut bootrom_data = vec![0; 256];
        bootrom_data[0] = val_expected;
        let bootrom = crate::bootrom::Bootrom::new(bootrom_data.into_boxed_slice());
        let peripherals = peripherals::Peripherals::new(bootrom);
        cpu.regs.pc = 0;
        assert_eq!(cpu.read8(&peripherals, Imm8), None);
        assert_eq!(cpu.read8(&peripherals, Imm8), Some(val_expected));
        assert_eq!(cpu.regs.pc, 1);
    }

    #[test]
    fn test_io16_write() {
        let af_expected = rand::rng().random();
        let bc_expected = rand::rng().random();
        let de_expected = rand::rng().random();
        let hl_expected = rand::rng().random();
        let sp_expected = rand::rng().random();
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0; 256].into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);
        cpu.write16(&mut peripherals, Reg16::AF, af_expected);
        cpu.write16(&mut peripherals, Reg16::BC, bc_expected);
        cpu.write16(&mut peripherals, Reg16::DE, de_expected);
        cpu.write16(&mut peripherals, Reg16::HL, hl_expected);
        cpu.write16(&mut peripherals, Reg16::SP, sp_expected);
        assert_eq!(cpu.regs.af(), af_expected & 0xfff0);
        assert_eq!(cpu.regs.bc(), bc_expected);
        assert_eq!(cpu.regs.de(), de_expected);
        assert_eq!(cpu.regs.hl(), hl_expected);
        assert_eq!(cpu.regs.sp, sp_expected);
    }
}
