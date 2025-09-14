use crate::peripherals;

pub trait IO8<T: Copy> {
    fn read8(&mut self, bus: &peripherals::Peripherals, src: T) -> Option<u8>;
    fn write8(&mut self, bus: &mut peripherals::Peripherals, dst: T, val: u8) -> Option<()>;
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
