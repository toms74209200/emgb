use crate::registers;

#[derive(Default)]
struct Ctx {
    opcode: u8,
    cb: bool,
}

pub struct Cpu {
    regs: registers::Registers,
    ctx: Ctx,
}
