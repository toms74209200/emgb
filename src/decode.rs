use crate::cpu;
use crate::operand;
use crate::peripherals;

impl cpu::Cpu {
    pub fn decode(&mut self, bus: &mut peripherals::Peripherals) {
        match self.ctx.opcode {
            0x00 => self.nop(bus),
            0x20 => self.jr_c(bus, operand::Cond::NZ),
            0x30 => self.jr_c(bus, operand::Cond::NC),
            0x01 => self.ld16(bus, operand::Reg16::BC, operand::Imm16),
            0x11 => self.ld16(bus, operand::Reg16::DE, operand::Imm16),
            0x21 => self.ld16(bus, operand::Reg16::HL, operand::Imm16),
            0x31 => self.ld16(bus, operand::Reg16::SP, operand::Imm16),
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
    #[test]
    fn test_decode_jr_c_nz() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0; 256].into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);
        cpu.regs.pc = 0xc000;
        peripherals.write(0xc000, 0x20);
        peripherals.write(0xc001, 0x05);
        cpu.fetch(&peripherals);
        let initial_pc = cpu.regs.pc;
        cpu.decode(&mut peripherals);
        cpu.decode(&mut peripherals);
        cpu.decode(&mut peripherals);
        assert_eq!(cpu.regs.pc, initial_pc + 7);
    }
    #[test]
    fn test_decode_jr_c_nc() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0; 256].into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);
        cpu.regs.pc = 0xc000;
        peripherals.write(0xc000, 0x30);
        peripherals.write(0xc001, 0x05);
        cpu.fetch(&peripherals);
        let initial_pc = cpu.regs.pc;
        cpu.decode(&mut peripherals);
        cpu.decode(&mut peripherals);
        cpu.decode(&mut peripherals);
        assert_eq!(cpu.regs.pc, initial_pc + 7);
    }
    #[test]
    fn test_decode_ld16_bc() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0; 256].into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);
        cpu.regs.pc = 0xc000;
        peripherals.write(0xc000, 0x01);
        peripherals.write(0xc001, 0x34);
        peripherals.write(0xc002, 0x12);
        cpu.fetch(&peripherals);
        for _ in 0..7 {
            cpu.decode(&mut peripherals);
        }
        assert_eq!(cpu.regs.bc(), 0x1234);
        assert_eq!(cpu.regs.pc, 0xc005);
    }

    #[test]
    fn test_decode_ld16_de() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0; 256].into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);
        cpu.regs.pc = 0xc000;
        peripherals.write(0xc000, 0x11);
        peripherals.write(0xc001, 0x78);
        peripherals.write(0xc002, 0x56);
        cpu.fetch(&peripherals);
        for _ in 0..7 {
            cpu.decode(&mut peripherals);
        }
        assert_eq!(cpu.regs.de(), 0x5678);
        assert_eq!(cpu.regs.pc, 0xc005);
    }

    #[test]
    fn test_decode_ld16_hl() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0; 256].into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);
        cpu.regs.pc = 0xc000;
        peripherals.write(0xc000, 0x21);
        peripherals.write(0xc001, 0xbc);
        peripherals.write(0xc002, 0x9a);
        cpu.fetch(&peripherals);
        for _ in 0..7 {
            cpu.decode(&mut peripherals);
        }
        assert_eq!(cpu.regs.hl(), 0x9abc);
        assert_eq!(cpu.regs.pc, 0xc005);
    }

    #[test]
    fn test_decode_ld16_sp() {
        let mut cpu = cpu::Cpu {
            regs: crate::registers::Registers::default(),
            ctx: cpu::Ctx::default(),
        };
        let bootrom = crate::bootrom::Bootrom::new(vec![0; 256].into_boxed_slice());
        let mut peripherals = peripherals::Peripherals::new(bootrom);
        cpu.regs.pc = 0xc000;
        peripherals.write(0xc000, 0x31);
        peripherals.write(0xc001, 0xfe);
        peripherals.write(0xc002, 0xff);
        cpu.fetch(&peripherals);
        for _ in 0..7 {
            cpu.decode(&mut peripherals);
        }
        assert_eq!(cpu.regs.sp, 0xfffe);
        assert_eq!(cpu.regs.pc, 0xc005);
    }
}
