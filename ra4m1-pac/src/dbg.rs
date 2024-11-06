#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Debug Status Register"]
    pub dbgstr: DBGSTR,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Debug Stop Control Register"]
    pub dbgstopcr: DBGSTOPCR,
    _reserved2: [u8; 0x0c],
    #[doc = "0x20 - Trace Control Register"]
    pub tracectr: TRACECTR,
}
#[doc = "DBGSTR (r) register accessor: Debug Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgstr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgstr`]
module"]
pub type DBGSTR = crate::Reg<dbgstr::DBGSTR_SPEC>;
#[doc = "Debug Status Register"]
pub mod dbgstr;
#[doc = "DBGSTOPCR (rw) register accessor: Debug Stop Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgstopcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgstopcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgstopcr`]
module"]
pub type DBGSTOPCR = crate::Reg<dbgstopcr::DBGSTOPCR_SPEC>;
#[doc = "Debug Stop Control Register"]
pub mod dbgstopcr;
#[doc = "TRACECTR (rw) register accessor: Trace Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tracectr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tracectr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tracectr`]
module"]
pub type TRACECTR = crate::Reg<tracectr::TRACECTR_SPEC>;
#[doc = "Trace Control Register"]
pub mod tracectr;
