use crate::registers;

#[derive(Default)]
pub struct Ctx {
    pub opcode: u8,
    pub cb: bool,
}

pub struct Cpu {
    pub regs: registers::Registers,
    pub ctx: Ctx,
}
