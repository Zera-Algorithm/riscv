//! RISC-V Counter Enable Register

pub const ENABLE_CYCLE_REG: usize = 1 << 0;
pub const ENABLE_TIME_REG: usize = 1 << 1;
pub const ENABLE_INSTRET_REG: usize = 1 << 2;

pub struct Hcounteren {
    bits: usize,
}

impl Hcounteren {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// VM cycle register enable  
    #[inline]
    pub fn cycle(&self) -> bool {
        (self.bits & ENABLE_CYCLE_REG) != 0
    }

    /// VM time register enable
    #[inline]
    pub fn time(&self) -> bool {
        (self.bits & ENABLE_TIME_REG) != 0
    }

    /// VM instret register enable
    #[inline]
    pub fn instret(&self) -> bool {
        (self.bits & ENABLE_INSTRET_REG) != 0
    }

    /// VM hpmcounter i enable
    #[inline]
    pub fn hpmcounter(&self, idx: usize) -> bool {
        if idx < 3 || idx > 31 {
            return false;
        }

        (self.bits & (1 << idx)) != 0
    }
}

pub fn enable_hpmcounter(idx: usize) {
    if idx < 3 || idx > 31 {
        return;
    }

    let hcounteren = read();
    write(hcounteren.bits() | (1 << idx));
}

pub fn disable_hpmcounter(idx: usize) {
    if idx < 3 || idx > 31 {
        return;
    }

    let hcounteren = read();
    write(hcounteren.bits() & !(1 << idx));
}

read_csr_as!(Hcounteren, 0x606);
write_csr_as_usize!(0x606);
set!(0x606);
clear!(0x606);

set_clear_csr!(
    /// Hypervisor cycle Enable
    , set_cycle, clear_cycle, ENABLE_CYCLE_REG);
set_clear_csr!(
    /// Hypervisor time Enable
    , set_time, clear_time, ENABLE_TIME_REG);
set_clear_csr!(
    /// Hypervisor instret Enable
    , set_instret, clear_instret, ENABLE_INSTRET_REG);
