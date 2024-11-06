#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Source Address Register"]
    pub dmsar: DMSAR,
    #[doc = "0x04 - DMA Destination Address Register"]
    pub dmdar: DMDAR,
    #[doc = "0x08 - DMA Transfer Count Register"]
    pub dmcra: DMCRA,
    #[doc = "0x0c - DMA Block Transfer Count Register"]
    pub dmcrb: DMCRB,
    _reserved4: [u8; 0x02],
    #[doc = "0x10 - DMA Transfer Mode Register"]
    pub dmtmd: DMTMD,
    _reserved5: [u8; 0x01],
    #[doc = "0x13 - DMA Interrupt Setting Register"]
    pub dmint: DMINT,
    #[doc = "0x14 - DMA Address Mode Register"]
    pub dmamd: DMAMD,
    _reserved7: [u8; 0x02],
    #[doc = "0x18 - DMA Offset Register"]
    pub dmofr: DMOFR,
    #[doc = "0x1c - DMA Transfer Enable Register"]
    pub dmcnt: DMCNT,
    #[doc = "0x1d - DMA Software Start Register"]
    pub dmreq: DMREQ,
    #[doc = "0x1e - DMA Status Register"]
    pub dmsts: DMSTS,
}
#[doc = "DMSAR (rw) register accessor: DMA Source Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmsar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmsar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmsar`]
module"]
pub type DMSAR = crate::Reg<dmsar::DMSAR_SPEC>;
#[doc = "DMA Source Address Register"]
pub mod dmsar;
#[doc = "DMDAR (rw) register accessor: DMA Destination Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmdar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmdar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmdar`]
module"]
pub type DMDAR = crate::Reg<dmdar::DMDAR_SPEC>;
#[doc = "DMA Destination Address Register"]
pub mod dmdar;
#[doc = "DMCRA (rw) register accessor: DMA Transfer Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmcra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmcra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmcra`]
module"]
pub type DMCRA = crate::Reg<dmcra::DMCRA_SPEC>;
#[doc = "DMA Transfer Count Register"]
pub mod dmcra;
#[doc = "DMCRB (rw) register accessor: DMA Block Transfer Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmcrb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmcrb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmcrb`]
module"]
pub type DMCRB = crate::Reg<dmcrb::DMCRB_SPEC>;
#[doc = "DMA Block Transfer Count Register"]
pub mod dmcrb;
#[doc = "DMTMD (rw) register accessor: DMA Transfer Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmtmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmtmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmtmd`]
module"]
pub type DMTMD = crate::Reg<dmtmd::DMTMD_SPEC>;
#[doc = "DMA Transfer Mode Register"]
pub mod dmtmd;
#[doc = "DMINT (rw) register accessor: DMA Interrupt Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmint`]
module"]
pub type DMINT = crate::Reg<dmint::DMINT_SPEC>;
#[doc = "DMA Interrupt Setting Register"]
pub mod dmint;
#[doc = "DMAMD (rw) register accessor: DMA Address Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamd`]
module"]
pub type DMAMD = crate::Reg<dmamd::DMAMD_SPEC>;
#[doc = "DMA Address Mode Register"]
pub mod dmamd;
#[doc = "DMOFR (rw) register accessor: DMA Offset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmofr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmofr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmofr`]
module"]
pub type DMOFR = crate::Reg<dmofr::DMOFR_SPEC>;
#[doc = "DMA Offset Register"]
pub mod dmofr;
#[doc = "DMCNT (rw) register accessor: DMA Transfer Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmcnt`]
module"]
pub type DMCNT = crate::Reg<dmcnt::DMCNT_SPEC>;
#[doc = "DMA Transfer Enable Register"]
pub mod dmcnt;
#[doc = "DMREQ (rw) register accessor: DMA Software Start Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmreq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmreq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmreq`]
module"]
pub type DMREQ = crate::Reg<dmreq::DMREQ_SPEC>;
#[doc = "DMA Software Start Register"]
pub mod dmreq;
#[doc = "DMSTS (rw) register accessor: DMA Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmsts`]
module"]
pub type DMSTS = crate::Reg<dmsts::DMSTS_SPEC>;
#[doc = "DMA Status Register"]
pub mod dmsts;
