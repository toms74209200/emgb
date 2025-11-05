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
                STEP.store(1, std::sync::atomic::Ordering::Relaxed);
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
impl IO8<Indirect> for cpu::Cpu {
    fn read8(&mut self, bus: &peripherals::Peripherals, src: Indirect) -> Option<u8> {
        static STEP: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
        static VAL8: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

        match STEP.load(std::sync::atomic::Ordering::Relaxed) {
            0 => {
                VAL8.store(
                    match src {
                        Indirect::BC => bus.read(self.regs.bc()),
                        Indirect::DE => bus.read(self.regs.de()),
                        Indirect::HL => bus.read(self.regs.hl()),
                        Indirect::CFF => bus.read(0xff00 | self.regs.c as u16),
                        Indirect::HLD => {
                            let addr = self.regs.hl();
                            self.regs.write_hl(addr.wrapping_sub(1));
                            bus.read(addr)
                        }
                        Indirect::HLI => {
                            let addr = self.regs.hl();
                            self.regs.write_hl(addr.wrapping_add(1));
                            bus.read(addr)
                        }
                    },
                    std::sync::atomic::Ordering::Relaxed,
                );
                STEP.store(1, std::sync::atomic::Ordering::Relaxed);
                None
            }
            1 => {
                STEP.store(0, std::sync::atomic::Ordering::Relaxed);
                Some(VAL8.load(std::sync::atomic::Ordering::Relaxed))
            }
            _ => unreachable!(),
        }
    }

    fn write8(&mut self, bus: &mut peripherals::Peripherals, dst: Indirect, val: u8) -> Option<()> {
        static STEP: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

        match STEP.load(std::sync::atomic::Ordering::Relaxed) {
            0 => {
                match dst {
                    Indirect::BC => bus.write(self.regs.bc(), val),
                    Indirect::DE => bus.write(self.regs.de(), val),
                    Indirect::HL => bus.write(self.regs.hl(), val),
                    Indirect::CFF => bus.write(0xff00 | self.regs.c as u16, val),
                    Indirect::HLD => {
                        let addr = self.regs.hl();
                        self.regs.write_hl(addr.wrapping_sub(1));
                        bus.write(addr, val);
                    }
                    Indirect::HLI => {
                        let addr = self.regs.hl();
                        self.regs.write_hl(addr.wrapping_add(1));
                        bus.write(addr, val);
                    }
                }
                STEP.store(1, std::sync::atomic::Ordering::Relaxed);
                None
            }
            1 => {
                STEP.store(0, std::sync::atomic::Ordering::Relaxed);
                Some(())
            }
            _ => unreachable!(),
        }
    }
}
impl IO8<Direct8> for cpu::Cpu {
    fn read8(&mut self, bus: &peripherals::Peripherals, src: Direct8) -> Option<u8> {
        static STEP: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
        static VAL8: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
        static VAL16: std::sync::atomic::AtomicU16 = std::sync::atomic::AtomicU16::new(0);

        match STEP.load(std::sync::atomic::Ordering::Relaxed) {
            0 => {
                if let Some(lo) = self.read8(bus, Imm8) {
                    VAL8.store(lo, std::sync::atomic::Ordering::Relaxed);
                    STEP.store(1, std::sync::atomic::Ordering::Relaxed);
                    if let Direct8::DFE = src {
                        VAL16.store(0xff00 | (lo as u16), std::sync::atomic::Ordering::Relaxed);
                        STEP.store(2, std::sync::atomic::Ordering::Relaxed);
                    }
                }
                None
            }
            1 => {
                if let Some(hi) = self.read8(bus, Imm8) {
                    VAL16.store(
                        u16::from_le_bytes([VAL8.load(std::sync::atomic::Ordering::Relaxed), hi]),
                        std::sync::atomic::Ordering::Relaxed,
                    );
                    STEP.store(2, std::sync::atomic::Ordering::Relaxed);
                }
                None
            }
            2 => {
                VAL8.store(
                    bus.read(VAL16.load(std::sync::atomic::Ordering::Relaxed)),
                    std::sync::atomic::Ordering::Relaxed,
                );
                STEP.store(3, std::sync::atomic::Ordering::Relaxed);
                None
            }
            3 => {
                STEP.store(0, std::sync::atomic::Ordering::Relaxed);
                Some(VAL8.load(std::sync::atomic::Ordering::Relaxed))
            }
            _ => unreachable!(),
        }
    }

    fn write8(&mut self, bus: &mut peripherals::Peripherals, dst: Direct8, val: u8) -> Option<()> {
        static STEP: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
        static VAL8: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
        static VAL16: std::sync::atomic::AtomicU16 = std::sync::atomic::AtomicU16::new(0);

        match STEP.load(std::sync::atomic::Ordering::Relaxed) {
            0 => {
                if let Some(lo) = self.read8(bus, Imm8) {
                    VAL8.store(lo, std::sync::atomic::Ordering::Relaxed);
                    STEP.store(1, std::sync::atomic::Ordering::Relaxed);
                    if let Direct8::DFE = dst {
                        VAL16.store(0xff00 | (lo as u16), std::sync::atomic::Ordering::Relaxed);
                        STEP.store(2, std::sync::atomic::Ordering::Relaxed);
                    }
                }
                None
            }
            1 => {
                if let Some(hi) = self.read8(bus, Imm8) {
                    VAL16.store(
                        u16::from_le_bytes([VAL8.load(std::sync::atomic::Ordering::Relaxed), hi]),
                        std::sync::atomic::Ordering::Relaxed,
                    );
                    STEP.store(2, std::sync::atomic::Ordering::Relaxed);
                }
                None
            }
            2 => {
                bus.write(VAL16.load(std::sync::atomic::Ordering::Relaxed), val);
                STEP.store(3, std::sync::atomic::Ordering::Relaxed);
                None
            }
            3 => {
                bus.write(
                    VAL16
                        .load(std::sync::atomic::Ordering::Relaxed)
                        .wrapping_add(1),
                    val.checked_shr(8).unwrap_or(0),
                );
                STEP.store(4, std::sync::atomic::Ordering::Relaxed);
                None
            }
            4 => Some(STEP.store(0, std::sync::atomic::Ordering::Relaxed)),
            _ => unreachable!(),
        }
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
impl IO16<Imm16> for cpu::Cpu {
    fn read16(&mut self, bus: &peripherals::Peripherals, _: Imm16) -> Option<u16> {
        static STEP: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
        static VAL8: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
        static VAL16: std::sync::atomic::AtomicU16 = std::sync::atomic::AtomicU16::new(0);

        match STEP.load(std::sync::atomic::Ordering::Relaxed) {
            0 => {
                let v = bus.read(self.regs.pc);
                self.regs.pc = self.regs.pc.wrapping_add(1);
                VAL8.store(v, std::sync::atomic::Ordering::Relaxed);
                STEP.store(1, std::sync::atomic::Ordering::Relaxed);
                None
            }
            1 => {
                let v = bus.read(self.regs.pc);
                self.regs.pc = self.regs.pc.wrapping_add(1);
                VAL16.store(
                    u16::from_le_bytes([VAL8.load(std::sync::atomic::Ordering::Relaxed), v]),
                    std::sync::atomic::Ordering::Relaxed,
                );
                STEP.store(2, std::sync::atomic::Ordering::Relaxed);
                None
            }
            2 => {
                STEP.store(0, std::sync::atomic::Ordering::Relaxed);
                Some(VAL16.load(std::sync::atomic::Ordering::Relaxed))
            }
            _ => unreachable!(),
        }
    }

    fn write16(&mut self, _: &mut peripherals::Peripherals, _: Imm16, _: u16) -> Option<()> {
        unreachable!()
    }
}

impl IO16<Direct16> for cpu::Cpu {
    fn read16(&mut self, _: &peripherals::Peripherals, _: Direct16) -> Option<u16> {
        unreachable!()
    }

    fn write16(&mut self, bus: &mut peripherals::Peripherals, _: Direct16, val: u16) -> Option<()> {
        static STEP: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
        static VAL8: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
        static VAL16: std::sync::atomic::AtomicU16 = std::sync::atomic::AtomicU16::new(0);

        match STEP.load(std::sync::atomic::Ordering::Relaxed) {
            0 => {
                if let Some(lo) = self.read8(bus, Imm8) {
                    VAL8.store(lo, std::sync::atomic::Ordering::Relaxed);
                    STEP.store(1, std::sync::atomic::Ordering::Relaxed);
                }
                None
            }
            1 => {
                if let Some(hi) = self.read8(bus, Imm8) {
                    VAL16.store(
                        u16::from_le_bytes([VAL8.load(std::sync::atomic::Ordering::Relaxed), hi]),
                        std::sync::atomic::Ordering::Relaxed,
                    );
                    STEP.store(2, std::sync::atomic::Ordering::Relaxed);
                }
                None
            }
            2 => {
                bus.write(VAL16.load(std::sync::atomic::Ordering::Relaxed), val as u8);
                STEP.store(3, std::sync::atomic::Ordering::Relaxed);
                None
            }
            3 => {
                bus.write(
                    VAL16
                        .load(std::sync::atomic::Ordering::Relaxed)
                        .wrapping_add(1),
                    (val >> 8) as u8,
                );
                STEP.store(4, std::sync::atomic::Ordering::Relaxed);
                None
            }
            4 => Some(STEP.store(0, std::sync::atomic::Ordering::Relaxed)),
            _ => unreachable!(),
        }
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
    CFF,
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
    NZ,
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
    fn test_io8_read_indirect() {
        let addr = rand::rng().random_range(0xc000..0xfdff);
        let val_expected = rand::rng().random();
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let mut bootrom = vec![0; 256];
        bootrom[0] = 0;
        let mut bootrom = crate::bootrom::Bootrom::new(bootrom.into_boxed_slice());
        bootrom.write(addr, val_expected);
        let mut peripherals = peripherals::Peripherals::new(bootrom);
        peripherals.write(addr, val_expected);
        cpu.regs.write_hl(addr);
        assert_eq!(cpu.read8(&peripherals, Indirect::HL), None);
        assert_eq!(cpu.read8(&peripherals, Indirect::HL), Some(val_expected));
    }

    #[test]
    fn test_io8_write_indirect() {
        let addr = rand::rng().random_range(0xc000..0xfdff);
        let val_expected = rand::rng().random();
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let mut bootrom = vec![0; 256];
        bootrom[0] = 0;
        let bootrom = crate::bootrom::Bootrom::new(bootrom.into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);
        cpu.regs.write_hl(addr);
        assert_eq!(
            cpu.write8(&mut peripherals, Indirect::HL, val_expected),
            None
        );
        assert_eq!(
            cpu.write8(&mut peripherals, Indirect::HL, val_expected),
            Some(())
        );
        assert_eq!(peripherals.read(addr), val_expected);
    }
    #[test]
    fn test_io8_read_direct() {
        let lo = rand::rng().random();
        let addr = u16::from_le_bytes([lo, 0]);
        let val_expected = rand::rng().random();
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let mut bootrom_data = vec![0; 256];
        bootrom_data[0] = lo;
        bootrom_data[1] = 0;
        bootrom_data[addr as usize] = val_expected;
        let bootrom = crate::bootrom::Bootrom::new(bootrom_data.into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);
        peripherals.write(addr, val_expected);
        cpu.regs.pc = 0;
        assert_eq!(cpu.read8(&peripherals, Direct8::D), None);
        assert_eq!(cpu.read8(&peripherals, Direct8::D), None);
        assert_eq!(cpu.read8(&peripherals, Direct8::D), None);
        assert_eq!(cpu.read8(&peripherals, Direct8::D), None);
        assert_eq!(cpu.read8(&peripherals, Direct8::D), None);
        assert_eq!(cpu.read8(&peripherals, Direct8::D), Some(val_expected));
        assert_eq!(cpu.regs.pc, 2);
    }
    #[test]
    fn test_io8_write_direct() {
        let lo = rand::rng().random();
        let addr = u16::from_le_bytes([lo, 0xc0]);
        let val_expected = rand::rng().random();
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let mut bootrom_data = vec![0; 256];
        bootrom_data[0] = lo;
        bootrom_data[1] = 0xc0;
        let bootrom = crate::bootrom::Bootrom::new(bootrom_data.into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);
        cpu.regs.pc = 0;
        assert_eq!(cpu.write8(&mut peripherals, Direct8::D, val_expected), None);
        assert_eq!(cpu.write8(&mut peripherals, Direct8::D, val_expected), None);
        assert_eq!(cpu.write8(&mut peripherals, Direct8::D, val_expected), None);
        assert_eq!(cpu.write8(&mut peripherals, Direct8::D, val_expected), None);
        assert_eq!(cpu.write8(&mut peripherals, Direct8::D, val_expected), None);
        assert_eq!(cpu.write8(&mut peripherals, Direct8::D, val_expected), None);
        assert_eq!(
            cpu.write8(&mut peripherals, Direct8::D, val_expected),
            Some(())
        );
        assert_eq!(cpu.regs.pc, 2);
        assert_eq!(peripherals.read(addr), val_expected);
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
    #[test]
    fn test_io16_read_imm() {
        let lo = rand::rng().random();
        let hi = rand::rng().random();
        let val_expected = u16::from_le_bytes([lo, hi]);
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let mut bootrom_data = vec![0; 256];
        bootrom_data[0] = lo;
        bootrom_data[1] = hi;
        let bootrom = crate::bootrom::Bootrom::new(bootrom_data.into_boxed_slice());
        let peripherals = peripherals::Peripherals::new(bootrom);
        cpu.regs.pc = 0;
        assert_eq!(cpu.read16(&peripherals, Imm16), None);
        assert_eq!(cpu.read16(&peripherals, Imm16), None);
        assert_eq!(cpu.read16(&peripherals, Imm16), Some(val_expected));
        assert_eq!(cpu.regs.pc, 2);
    }

    #[test]
    fn test_io16_write_direct() {
        let lo = rand::rng().random();
        let hi = rand::rng().random();
        let val_expected = rand::rng().random();
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let mut bootrom_data = vec![0; 256];
        bootrom_data[0] = lo;
        bootrom_data[1] = hi;
        let bootrom = crate::bootrom::Bootrom::new(bootrom_data.into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);
        cpu.regs.pc = 0;
        assert_eq!(cpu.write16(&mut peripherals, Direct16, val_expected), None);
        assert_eq!(cpu.write16(&mut peripherals, Direct16, val_expected), None);
        assert_eq!(cpu.write16(&mut peripherals, Direct16, val_expected), None);
        assert_eq!(cpu.write16(&mut peripherals, Direct16, val_expected), None);
        assert_eq!(cpu.write16(&mut peripherals, Direct16, val_expected), None);
    }
}
