#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC Control Register0"]
    pub crccr0: CRCCR0,
    #[doc = "0x01 - CRC Control Register1"]
    pub crccr1: CRCCR1,
    _reserved2: [u8; 0x02],
    _reserved_2_crcdir: [u8; 0x04],
    _reserved_3_crcdor: [u8; 0x04],
    #[doc = "0x0c - Snoop Address Register"]
    pub crcsar: CRCSAR,
}
impl RegisterBlock {
    #[doc = "0x04 - CRC Data Input Register (byte access)"]
    #[inline(always)]
    pub const fn crcdir_by(&self) -> &CRCDIR_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - CRC Data Input Register"]
    #[inline(always)]
    pub const fn crcdir(&self) -> &CRCDIR {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x08 - CRC Data Output Register(byte access)"]
    #[inline(always)]
    pub const fn crcdor_by(&self) -> &CRCDOR_BY {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x08 - CRC Data Output Register (halfword access)"]
    #[inline(always)]
    pub const fn crcdor_ha(&self) -> &CRCDOR_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x08 - CRC Data Output Register"]
    #[inline(always)]
    pub const fn crcdor(&self) -> &CRCDOR {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
}
#[doc = "CRCCR0 (rw) register accessor: CRC Control Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crccr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crccr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crccr0`]
module"]
pub type CRCCR0 = crate::Reg<crccr0::CRCCR0_SPEC>;
#[doc = "CRC Control Register0"]
pub mod crccr0;
#[doc = "CRCCR1 (rw) register accessor: CRC Control Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crccr1`]
module"]
pub type CRCCR1 = crate::Reg<crccr1::CRCCR1_SPEC>;
#[doc = "CRC Control Register1"]
pub mod crccr1;
#[doc = "CRCDIR (rw) register accessor: CRC Data Input Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcdir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcdir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcdir`]
module"]
pub type CRCDIR = crate::Reg<crcdir::CRCDIR_SPEC>;
#[doc = "CRC Data Input Register"]
pub mod crcdir;
#[doc = "CRCDIR_BY (rw) register accessor: CRC Data Input Register (byte access)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcdir_by::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcdir_by::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcdir_by`]
module"]
pub type CRCDIR_BY = crate::Reg<crcdir_by::CRCDIR_BY_SPEC>;
#[doc = "CRC Data Input Register (byte access)"]
pub mod crcdir_by;
#[doc = "CRCDOR (rw) register accessor: CRC Data Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcdor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcdor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcdor`]
module"]
pub type CRCDOR = crate::Reg<crcdor::CRCDOR_SPEC>;
#[doc = "CRC Data Output Register"]
pub mod crcdor;
#[doc = "CRCDOR_HA (rw) register accessor: CRC Data Output Register (halfword access)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcdor_ha::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcdor_ha::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcdor_ha`]
module"]
pub type CRCDOR_HA = crate::Reg<crcdor_ha::CRCDOR_HA_SPEC>;
#[doc = "CRC Data Output Register (halfword access)"]
pub mod crcdor_ha;
#[doc = "CRCDOR_BY (rw) register accessor: CRC Data Output Register(byte access)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcdor_by::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcdor_by::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcdor_by`]
module"]
pub type CRCDOR_BY = crate::Reg<crcdor_by::CRCDOR_BY_SPEC>;
#[doc = "CRC Data Output Register(byte access)"]
pub mod crcdor_by;
#[doc = "CRCSAR (rw) register accessor: Snoop Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcsar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcsar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcsar`]
module"]
pub type CRCSAR = crate::Reg<crcsar::CRCSAR_SPEC>;
#[doc = "Snoop Address Register"]
pub mod crcsar;
