#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DTC Control Register"]
    pub dtccr: DTCCR,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - DTC Vector Base Register"]
    pub dtcvbr: DTCVBR,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - DTC Module Start Register"]
    pub dtcst: DTCST,
    _reserved3: [u8; 0x01],
    #[doc = "0x0e - DTC Status Register"]
    pub dtcsts: DTCSTS,
}
#[doc = "DTCCR (rw) register accessor: DTC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtccr`]
module"]
pub type DTCCR = crate::Reg<dtccr::DTCCR_SPEC>;
#[doc = "DTC Control Register"]
pub mod dtccr;
#[doc = "DTCVBR (rw) register accessor: DTC Vector Base Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtcvbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtcvbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtcvbr`]
module"]
pub type DTCVBR = crate::Reg<dtcvbr::DTCVBR_SPEC>;
#[doc = "DTC Vector Base Register"]
pub mod dtcvbr;
#[doc = "DTCST (rw) register accessor: DTC Module Start Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtcst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtcst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtcst`]
module"]
pub type DTCST = crate::Reg<dtcst::DTCST_SPEC>;
#[doc = "DTC Module Start Register"]
pub mod dtcst;
#[doc = "DTCSTS (r) register accessor: DTC Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtcsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtcsts`]
module"]
pub type DTCSTS = crate::Reg<dtcsts::DTCSTS_SPEC>;
#[doc = "DTC Status Register"]
pub mod dtcsts;
