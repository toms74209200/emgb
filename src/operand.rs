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

pub trait IO16<T: Copy> {
    fn read16(&mut self, bus: &peripherals::Peripherals, src: T) -> Option<u16>;
    fn write16(&mut self, bus: &mut peripherals::Peripherals, dst: T, val: u16) -> Option<()>;
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
}
