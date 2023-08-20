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
    fn flags(&self) -> u8 {
        self.af.low()
    }

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
        let flags = self.flags();
        flags & Self::FLAGS_MASKS[index] >> index
    }
    pub(crate) fn zero_flag(&self) -> bool {
        self.flags_bit_at(7) == 1
    }
    pub(crate) fn addsub_flag(&self) -> bool {
        self.flags_bit_at(6) == 1
    }
    pub(crate) fn half_carry_flag(&self) -> bool {
        self.flags_bit_at(5) == 1
    }
    pub(crate) fn carry_flag(&self) -> bool {
        self.flags_bit_at(4) == 1
    }
}
