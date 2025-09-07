use crate::bootrom;
use crate::hram;
use crate::wram;

pub struct Peripherals {
    bootrom: bootrom::Bootrom,
    wram: wram::WRam,
    hram: hram::HRam,
}
impl Peripherals {
    pub fn new(bootrom: bootrom::Bootrom) -> Self {
        Self {
            bootrom,
            wram: wram::WRam::new(),
            hram: hram::HRam::new(),
        }
    }
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x00ff => {
                if self.bootrom.is_active() {
                    self.bootrom.read(addr)
                } else {
                    0xff
                }
            }
            0xc000..=0xfdff => self.wram.read(addr),
            0xff80..=0xfffe => self.hram.read(addr),
            _ => 0xff,
        }
    }
    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0xc000..=0xfdff => self.wram.write(addr, val),
            0xff50 => self.bootrom.write(addr, val),
            0xff80..=0xfffe => self.hram.write(addr, val),
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_peripherals_wram() {
        let bootrom = bootrom::Bootrom::new(vec![0; 256].into_boxed_slice());
        let mut peripherals = Peripherals::new(bootrom);
        peripherals.write(0xc000, 42);
        assert_eq!(peripherals.read(0xc000), 42);
    }

    #[test]
    fn test_peripherals_hram() {
        let bootrom = bootrom::Bootrom::new(vec![0; 256].into_boxed_slice());
        let mut peripherals = Peripherals::new(bootrom);
        peripherals.write(0xff80, 84);
        assert_eq!(peripherals.read(0xff80), 84);
    }

    #[test]
    fn test_peripherals_bootrom() {
        let bootrom = bootrom::Bootrom::new(vec![0; 256].into_boxed_slice());
        let mut peripherals = Peripherals::new(bootrom);
        peripherals.write(0xff50, 0);
        assert!(peripherals.bootrom.is_active());
    }
}
