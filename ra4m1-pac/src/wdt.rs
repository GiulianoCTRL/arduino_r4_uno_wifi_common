#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - WDT Refresh Register"]
    pub wdtrr: WDTRR,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - WDT Control Register"]
    pub wdtcr: WDTCR,
    #[doc = "0x04 - WDT Status Register"]
    pub wdtsr: WDTSR,
    #[doc = "0x06 - WDT Reset Control Register"]
    pub wdtrcr: WDTRCR,
    _reserved4: [u8; 0x01],
    #[doc = "0x08 - WDT Count Stop Control Register"]
    pub wdtcstpr: WDTCSTPR,
}
#[doc = "WDTRR (rw) register accessor: WDT Refresh Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtrr`]
module"]
pub type WDTRR = crate::Reg<wdtrr::WDTRR_SPEC>;
#[doc = "WDT Refresh Register"]
pub mod wdtrr;
#[doc = "WDTCR (rw) register accessor: WDT Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtcr`]
module"]
pub type WDTCR = crate::Reg<wdtcr::WDTCR_SPEC>;
#[doc = "WDT Control Register"]
pub mod wdtcr;
#[doc = "WDTSR (rw) register accessor: WDT Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtsr`]
module"]
pub type WDTSR = crate::Reg<wdtsr::WDTSR_SPEC>;
#[doc = "WDT Status Register"]
pub mod wdtsr;
#[doc = "WDTRCR (rw) register accessor: WDT Reset Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtrcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtrcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtrcr`]
module"]
pub type WDTRCR = crate::Reg<wdtrcr::WDTRCR_SPEC>;
#[doc = "WDT Reset Control Register"]
pub mod wdtrcr;
#[doc = "WDTCSTPR (rw) register accessor: WDT Count Stop Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtcstpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtcstpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtcstpr`]
module"]
pub type WDTCSTPR = crate::Reg<wdtcstpr::WDTCSTPR_SPEC>;
#[doc = "WDT Count Stop Control Register"]
pub mod wdtcstpr;
