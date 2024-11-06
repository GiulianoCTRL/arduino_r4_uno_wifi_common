#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IWDT Refresh Register"]
    pub iwdtrr: IWDTRR,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - IWDT Status Register"]
    pub iwdtsr: IWDTSR,
}
#[doc = "IWDTRR (rw) register accessor: IWDT Refresh Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdtrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwdtrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdtrr`]
module"]
pub type IWDTRR = crate::Reg<iwdtrr::IWDTRR_SPEC>;
#[doc = "IWDT Refresh Register"]
pub mod iwdtrr;
#[doc = "IWDTSR (rw) register accessor: IWDT Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdtsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwdtsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdtsr`]
module"]
pub type IWDTSR = crate::Reg<iwdtsr::IWDTSR_SPEC>;
#[doc = "IWDT Status Register"]
pub mod iwdtsr;
