use crate::peripherals;

pub trait IO8<T: Copy> {
    fn read8(&mut self, bus: &peripherals::Peripherals, src: T) -> Option<u8>;
    fn write8(&mut self, bus: &mut peripherals::Peripherals, dst: T, val: u8) -> Option<()>;
}
