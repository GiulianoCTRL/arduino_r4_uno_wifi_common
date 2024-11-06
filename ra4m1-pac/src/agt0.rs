#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AGT Counter Register"]
    pub agt: AGT,
    #[doc = "0x02 - AGT Compare Match A Register"]
    pub agtcma: AGTCMA,
    #[doc = "0x04 - AGT Compare Match B Register"]
    pub agtcmb: AGTCMB,
    _reserved3: [u8; 0x02],
    #[doc = "0x08 - AGT Control Register"]
    pub agtcr: AGTCR,
    #[doc = "0x09 - AGT Mode Register 1"]
    pub agtmr1: AGTMR1,
    #[doc = "0x0a - AGT Mode Register 2"]
    pub agtmr2: AGTMR2,
    _reserved6: [u8; 0x01],
    #[doc = "0x0c - AGT I/O Control Register"]
    pub agtioc: AGTIOC,
    #[doc = "0x0d - AGT Event Pin Select Register"]
    pub agtisr: AGTISR,
    #[doc = "0x0e - AGT Compare Match Function Select Register"]
    pub agtcmsr: AGTCMSR,
    #[doc = "0x0f - AGT Pin Select Register"]
    pub agtiosel: AGTIOSEL,
}
#[doc = "AGT (rw) register accessor: AGT Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`agt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`agt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agt`]
module"]
pub type AGT = crate::Reg<agt::AGT_SPEC>;
#[doc = "AGT Counter Register"]
pub mod agt;
#[doc = "AGTCMA (rw) register accessor: AGT Compare Match A Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`agtcma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`agtcma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agtcma`]
module"]
pub type AGTCMA = crate::Reg<agtcma::AGTCMA_SPEC>;
#[doc = "AGT Compare Match A Register"]
pub mod agtcma;
#[doc = "AGTCMB (rw) register accessor: AGT Compare Match B Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`agtcmb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`agtcmb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agtcmb`]
module"]
pub type AGTCMB = crate::Reg<agtcmb::AGTCMB_SPEC>;
#[doc = "AGT Compare Match B Register"]
pub mod agtcmb;
#[doc = "AGTCR (rw) register accessor: AGT Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`agtcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`agtcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agtcr`]
module"]
pub type AGTCR = crate::Reg<agtcr::AGTCR_SPEC>;
#[doc = "AGT Control Register"]
pub mod agtcr;
#[doc = "AGTMR1 (rw) register accessor: AGT Mode Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`agtmr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`agtmr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agtmr1`]
module"]
pub type AGTMR1 = crate::Reg<agtmr1::AGTMR1_SPEC>;
#[doc = "AGT Mode Register 1"]
pub mod agtmr1;
#[doc = "AGTMR2 (rw) register accessor: AGT Mode Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`agtmr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`agtmr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agtmr2`]
module"]
pub type AGTMR2 = crate::Reg<agtmr2::AGTMR2_SPEC>;
#[doc = "AGT Mode Register 2"]
pub mod agtmr2;
#[doc = "AGTIOC (rw) register accessor: AGT I/O Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`agtioc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`agtioc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agtioc`]
module"]
pub type AGTIOC = crate::Reg<agtioc::AGTIOC_SPEC>;
#[doc = "AGT I/O Control Register"]
pub mod agtioc;
#[doc = "AGTISR (rw) register accessor: AGT Event Pin Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`agtisr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`agtisr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agtisr`]
module"]
pub type AGTISR = crate::Reg<agtisr::AGTISR_SPEC>;
#[doc = "AGT Event Pin Select Register"]
pub mod agtisr;
#[doc = "AGTCMSR (rw) register accessor: AGT Compare Match Function Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`agtcmsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`agtcmsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agtcmsr`]
module"]
pub type AGTCMSR = crate::Reg<agtcmsr::AGTCMSR_SPEC>;
#[doc = "AGT Compare Match Function Select Register"]
pub mod agtcmsr;
#[doc = "AGTIOSEL (rw) register accessor: AGT Pin Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`agtiosel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`agtiosel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agtiosel`]
module"]
pub type AGTIOSEL = crate::Reg<agtiosel::AGTIOSEL_SPEC>;
#[doc = "AGT Pin Select Register"]
pub mod agtiosel;
