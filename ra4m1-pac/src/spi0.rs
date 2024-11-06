#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI Control Register"]
    pub spcr: SPCR,
    #[doc = "0x01 - SPI Slave Select Polarity Register"]
    pub sslp: SSLP,
    #[doc = "0x02 - SPI Pin Control Register"]
    pub sppcr: SPPCR,
    #[doc = "0x03 - SPI Status Register"]
    pub spsr: SPSR,
    _reserved_4_spdr: [u8; 0x04],
    _reserved5: [u8; 0x02],
    #[doc = "0x0a - SPI Bit Rate Register"]
    pub spbr: SPBR,
    #[doc = "0x0b - SPI Data Control Register"]
    pub spdcr: SPDCR,
    #[doc = "0x0c - SPI Clock Delay Register"]
    pub spckd: SPCKD,
    #[doc = "0x0d - SPI Slave Select Negation Delay Register"]
    pub sslnd: SSLND,
    #[doc = "0x0e - SPI Next-Access Delay Register"]
    pub spnd: SPND,
    #[doc = "0x0f - SPI Control Register 2"]
    pub spcr2: SPCR2,
    #[doc = "0x10 - SPI Command Register 0"]
    pub spcmd0: SPCMD0,
}
impl RegisterBlock {
    #[doc = "0x04 - SPI Data Register ( halfword access )"]
    #[inline(always)]
    pub const fn spdr_ha(&self) -> &SPDR_HA {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - SPI Data Register"]
    #[inline(always)]
    pub const fn spdr(&self) -> &SPDR {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
}
#[doc = "SPCR (rw) register accessor: SPI Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spcr`]
module"]
pub type SPCR = crate::Reg<spcr::SPCR_SPEC>;
#[doc = "SPI Control Register"]
pub mod spcr;
#[doc = "SSLP (rw) register accessor: SPI Slave Select Polarity Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sslp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sslp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sslp`]
module"]
pub type SSLP = crate::Reg<sslp::SSLP_SPEC>;
#[doc = "SPI Slave Select Polarity Register"]
pub mod sslp;
#[doc = "SPPCR (rw) register accessor: SPI Pin Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sppcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sppcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sppcr`]
module"]
pub type SPPCR = crate::Reg<sppcr::SPPCR_SPEC>;
#[doc = "SPI Pin Control Register"]
pub mod sppcr;
#[doc = "SPSR (rw) register accessor: SPI Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spsr`]
module"]
pub type SPSR = crate::Reg<spsr::SPSR_SPEC>;
#[doc = "SPI Status Register"]
pub mod spsr;
#[doc = "SPDR (rw) register accessor: SPI Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdr`]
module"]
pub type SPDR = crate::Reg<spdr::SPDR_SPEC>;
#[doc = "SPI Data Register"]
pub mod spdr;
#[doc = "SPDR_HA (rw) register accessor: SPI Data Register ( halfword access )\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdr_ha::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdr_ha::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdr_ha`]
module"]
pub type SPDR_HA = crate::Reg<spdr_ha::SPDR_HA_SPEC>;
#[doc = "SPI Data Register ( halfword access )"]
pub mod spdr_ha;
#[doc = "SPBR (rw) register accessor: SPI Bit Rate Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spbr`]
module"]
pub type SPBR = crate::Reg<spbr::SPBR_SPEC>;
#[doc = "SPI Bit Rate Register"]
pub mod spbr;
#[doc = "SPDCR (rw) register accessor: SPI Data Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdcr`]
module"]
pub type SPDCR = crate::Reg<spdcr::SPDCR_SPEC>;
#[doc = "SPI Data Control Register"]
pub mod spdcr;
#[doc = "SPCKD (rw) register accessor: SPI Clock Delay Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spckd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spckd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spckd`]
module"]
pub type SPCKD = crate::Reg<spckd::SPCKD_SPEC>;
#[doc = "SPI Clock Delay Register"]
pub mod spckd;
#[doc = "SSLND (rw) register accessor: SPI Slave Select Negation Delay Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sslnd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sslnd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sslnd`]
module"]
pub type SSLND = crate::Reg<sslnd::SSLND_SPEC>;
#[doc = "SPI Slave Select Negation Delay Register"]
pub mod sslnd;
#[doc = "SPND (rw) register accessor: SPI Next-Access Delay Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spnd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spnd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spnd`]
module"]
pub type SPND = crate::Reg<spnd::SPND_SPEC>;
#[doc = "SPI Next-Access Delay Register"]
pub mod spnd;
#[doc = "SPCR2 (rw) register accessor: SPI Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spcr2`]
module"]
pub type SPCR2 = crate::Reg<spcr2::SPCR2_SPEC>;
#[doc = "SPI Control Register 2"]
pub mod spcr2;
#[doc = "SPCMD0 (rw) register accessor: SPI Command Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spcmd0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spcmd0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spcmd0`]
module"]
pub type SPCMD0 = crate::Reg<spcmd0::SPCMD0_SPEC>;
#[doc = "SPI Command Register 0"]
pub mod spcmd0;
