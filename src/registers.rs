#[derive(Clone, Copy, Debug, Default)]
pub struct Registers {
    pub pc: u16,
    pub sp: u16,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }
    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }
    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }
    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn write_af(&mut self, val: u16) {
        self.a = (val >> 8) as u8;
        self.f = (val & 0xf0) as u8;
    }
    pub fn write_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = val as u8;
    }
    pub fn write_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = val as u8;
    }
    pub fn write_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = val as u8;
    }

    pub fn zf(&self) -> bool {
        (self.f & 0b_1000_0000) > 0
    }
    pub fn nf(&self) -> bool {
        (self.f & 0b_0100_0000) > 0
    }
    pub fn hf(&self) -> bool {
        (self.f & 0b_0010_0000) > 0
    }
    pub fn cf(&self) -> bool {
        (self.f & 0b_0001_0000) > 0
    }

    pub fn set_zf(&mut self, zf: bool) {
        if zf {
            self.f |= 0b_1000_0000;
        } else {
            self.f &= 0b_0111_1111;
        }
    }
    pub fn set_nf(&mut self, nf: bool) {
        if nf {
            self.f |= 0b_0100_0000;
        } else {
            self.f &= 0b_1011_1111;
        }
    }
    pub fn set_hf(&mut self, hf: bool) {
        if hf {
            self.f |= 0b_0010_0000;
        } else {
            self.f &= 0b_1101_1111;
        }
    }
    pub fn set_cf(&mut self, cf: bool) {
        if cf {
            self.f |= 0b_0001_0000;
        } else {
            self.f &= 0b_1110_1111;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_af() {
        let mut regs = Registers::default();
        regs.a = 0x5a;
        regs.f = 0xa5;
        assert_eq!(regs.af(), 0x5aa5);
        regs.write_af(0x5aa5);
        assert_eq!(regs.a, 0x5a);
        assert_eq!(regs.f, 0xa0);
    }

    #[test]
    fn test_bc() {
        let mut regs = Registers::default();
        regs.b = 0x5a;
        regs.c = 0xa5;
        assert_eq!(regs.bc(), 0x5aa5);
        regs.write_bc(0x5aa5);
        assert_eq!(regs.b, 0x5a);
        assert_eq!(regs.c, 0xa5);
    }

    #[test]
    fn test_de() {
        let mut regs = Registers::default();
        regs.d = 0x5a;
        regs.e = 0xa5;
        assert_eq!(regs.de(), 0x5aa5);
        regs.write_de(0x5aa5);
        assert_eq!(regs.d, 0x5a);
        assert_eq!(regs.e, 0xa5);
    }

    #[test]
    fn test_hl() {
        let mut regs = Registers::default();
        regs.h = 0x5a;
        regs.l = 0xa5;
        assert_eq!(regs.hl(), 0x5aa5);
        regs.write_hl(0x5aa5);
        assert_eq!(regs.h, 0x5a);
        assert_eq!(regs.l, 0xa5);
    }

    #[test]
    fn test_zf() {
        let mut regs = Registers::default();
        regs.f = 0b_1000_0000;
        assert!(regs.zf());
        regs.set_zf(false);
        assert!(!regs.zf());
        regs.set_zf(true);
        assert!(regs.zf());
    }
    #[test]
    fn test_nf() {
        let mut regs = Registers::default();
        regs.f = 0b_0100_0000;
        assert!(regs.nf());
        regs.set_nf(false);
        assert!(!regs.nf());
        regs.set_nf(true);
        assert!(regs.nf());
    }
    #[test]
    fn test_hf() {
        let mut regs = Registers::default();
        regs.f = 0b_0010_0000;
        assert!(regs.hf());
        regs.set_hf(false);
        assert!(!regs.hf());
        regs.set_hf(true);
        assert!(regs.hf());
    }
    #[test]
    fn test_cf() {
        let mut regs = Registers::default();
        regs.f = 0b_0001_0000;
        assert!(regs.cf());
        regs.set_cf(false);
        assert!(!regs.cf());
        regs.set_cf(true);
        assert!(regs.cf());
    }
}
