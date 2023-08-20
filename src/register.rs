union RegisterRaw {
    wide: u16,
    parts: (u8, u8),
}

pub(crate) struct Register {
    inner: RegisterRaw,
}

impl Register {
    pub(crate) fn wide(&self) -> u16 {
        unsafe { self.inner.wide }
    }
    pub(crate) fn wide_mut(&mut self) -> &mut u16 {
        unsafe { &mut self.inner.wide }
    }
    pub(crate) fn high(&self) -> u8 {
        unsafe { self.inner.parts.0 }
    }
    pub(crate) fn high_mut(&mut self) -> &mut u8 {
        unsafe { &mut self.inner.parts.0 }
    }
    pub(crate) fn low(&self) -> u8 {
        unsafe { self.inner.parts.1 }
    }
    pub(crate) fn low_mut(&mut self) -> &mut u8 {
        unsafe { &mut self.inner.parts.1 }
    }
}

pub(crate) struct Registers {
    af: Register,
    bc: Register,
    de: Register,
    hl: Register,
}

impl Registers {
    pub(crate) fn a(&self) -> u8 {
        self.af.high()
    }
    pub(crate) fn b(&self) -> u8 {
        self.bc.high()
    }
    pub(crate) fn c(&self) -> u8 {
        self.bc.low()
    }
    pub(crate) fn d(&self) -> u8 {
        self.de.high()
    }
    pub(crate) fn e(&self) -> u8 {
        self.de.low()
    }
    pub(crate) fn h(&self) -> u8 {
        self.hl.high()
    }
    pub(crate) fn l(&self) -> u8 {
        self.hl.low()
    }
}

impl Registers {
    pub(crate) fn flags(&self) -> Flags {
        Flags(self.af.low())
    }
    pub(crate) fn flags_mut(&mut self) -> FlagsMut<'_> {
        FlagsMut(self.af.low_mut())
    }
}

#[repr(transparent)]
pub(crate) struct Flags(u8);

impl Flags {
    const FLAGS_MASKS: [u8; 8] = [
        0,
        0,
        0,
        0,
        0b0001_0000,
        0b0010_0000,
        0b0100_0000,
        0b1000_0000,
    ];

    fn flags_bit_at(&self, index: usize) -> u8 {
        assert!(index < 8);
        let flags = self.0;
        (flags & Self::FLAGS_MASKS[index]) >> index
    }
    pub(crate) fn zero(&self) -> bool {
        self.flags_bit_at(7) == 1
    }
    pub(crate) fn addsub(&self) -> bool {
        self.flags_bit_at(6) == 1
    }
    pub(crate) fn half_carry(&self) -> bool {
        self.flags_bit_at(5) == 1
    }
    pub(crate) fn carry(&self) -> bool {
        self.flags_bit_at(4) == 1
    }
}

pub(crate) struct FlagsMut<'a>(&'a mut u8);

impl<'a> FlagsMut<'a> {
    const ON_MASKS: [u8; 8] = [
        0,
        0,
        0,
        0,
        0b0001_0000,
        0b0010_0000,
        0b0100_0000,
        0b1000_0000,
    ];

    const OFF_MASKS: [u8; 8] = [
        0,
        0,
        0,
        0,
        0b1110_1111,
        0b1101_1111,
        0b1011_1111,
        0b0111_1111,
    ];

    fn set_bit_on_at(&mut self, index: usize) {
        *self.0 |= Self::ON_MASKS[index]
    }

    fn set_bit_off_at(&mut self, index: usize) {
        *self.0 &= Self::OFF_MASKS[index]
    }

    pub fn zero_on(&mut self) {
        self.set_bit_on_at(7)
    }
    pub fn zero_off(&mut self) {
        self.set_bit_off_at(7)
    }
    pub fn addsub_on(&mut self) {
        self.set_bit_on_at(7)
    }
    pub fn addsub_off(&mut self) {
        self.set_bit_off_at(7)
    }
    pub fn half_carry_on(&mut self) {
        self.set_bit_on_at(7)
    }
    pub fn half_carry_off(&mut self) {
        self.set_bit_off_at(7)
    }
    pub fn carry_on(&mut self) {
        self.set_bit_on_at(7)
    }
    pub fn carry_off(&mut self) {
        self.set_bit_off_at(7)
    }
}

#[cfg(test)]
mod test {
    use crate::register::Flags;

    use super::FlagsMut;

    #[test]
    fn can_manipulate_zero_flag() {
        let mut value = 0b0011_1111;
        let mut flags = FlagsMut(&mut value);

        flags.zero_on();
        assert_eq!(0b1011_1111, value);
        assert!(Flags(value).zero());

        let mut flags = FlagsMut(&mut value);
        flags.zero_off();
        assert_eq!(0b0011_1111, value);
        assert!(!Flags(value).zero());
    }
}
