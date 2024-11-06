#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ssicr: SSICR,
    #[doc = "0x04 - Status Register"]
    pub ssisr: SSISR,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - FIFO Control Register"]
    pub ssifcr: SSIFCR,
    #[doc = "0x14 - FIFO Status Register"]
    pub ssifsr: SSIFSR,
    #[doc = "0x18 - Transmit FIFO Data Register"]
    pub ssiftdr: SSIFTDR,
    #[doc = "0x1c - Receive FIFO Data Register"]
    pub ssifrdr: SSIFRDR,
    #[doc = "0x20 - TDM Mode Register"]
    pub ssitdmr: SSITDMR,
    #[doc = "0x24 - Status Control Register"]
    pub ssiscr: SSISCR,
}
#[doc = "SSICR (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssicr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssicr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssicr`]
module"]
pub type SSICR = crate::Reg<ssicr::SSICR_SPEC>;
#[doc = "Control Register"]
pub mod ssicr;
#[doc = "SSISR (rw) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssisr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssisr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssisr`]
module"]
pub type SSISR = crate::Reg<ssisr::SSISR_SPEC>;
#[doc = "Status Register"]
pub mod ssisr;
#[doc = "SSIFCR (rw) register accessor: FIFO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssifcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssifcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssifcr`]
module"]
pub type SSIFCR = crate::Reg<ssifcr::SSIFCR_SPEC>;
#[doc = "FIFO Control Register"]
pub mod ssifcr;
#[doc = "SSIFSR (rw) register accessor: FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssifsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssifsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssifsr`]
module"]
pub type SSIFSR = crate::Reg<ssifsr::SSIFSR_SPEC>;
#[doc = "FIFO Status Register"]
pub mod ssifsr;
#[doc = "SSIFTDR (w) register accessor: Transmit FIFO Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssiftdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssiftdr`]
module"]
pub type SSIFTDR = crate::Reg<ssiftdr::SSIFTDR_SPEC>;
#[doc = "Transmit FIFO Data Register"]
pub mod ssiftdr;
#[doc = "SSIFRDR (r) register accessor: Receive FIFO Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssifrdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssifrdr`]
module"]
pub type SSIFRDR = crate::Reg<ssifrdr::SSIFRDR_SPEC>;
#[doc = "Receive FIFO Data Register"]
pub mod ssifrdr;
#[doc = "SSITDMR (rw) register accessor: TDM Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssitdmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssitdmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssitdmr`]
module"]
pub type SSITDMR = crate::Reg<ssitdmr::SSITDMR_SPEC>;
#[doc = "TDM Mode Register"]
pub mod ssitdmr;
#[doc = "SSISCR (rw) register accessor: Status Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssiscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssiscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssiscr`]
module"]
pub type SSISCR = crate::Reg<ssiscr::SSISCR_SPEC>;
#[doc = "Status Control Register"]
pub mod ssiscr;
