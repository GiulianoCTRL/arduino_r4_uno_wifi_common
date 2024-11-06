#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DOC Control Register"]
    pub docr: DOCR,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - DOC Data Input Register"]
    pub dodir: DODIR,
    #[doc = "0x04 - DOC Data Setting Register"]
    pub dodsr: DODSR,
}
#[doc = "DOCR (rw) register accessor: DOC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`docr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`docr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@docr`]
module"]
pub type DOCR = crate::Reg<docr::DOCR_SPEC>;
#[doc = "DOC Control Register"]
pub mod docr;
#[doc = "DODIR (rw) register accessor: DOC Data Input Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dodir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dodir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dodir`]
module"]
pub type DODIR = crate::Reg<dodir::DODIR_SPEC>;
#[doc = "DOC Data Input Register"]
pub mod dodir;
#[doc = "DODSR (rw) register accessor: DOC Data Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dodsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dodsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dodsr`]
module"]
pub type DODSR = crate::Reg<dodsr::DODSR_SPEC>;
#[doc = "DOC Data Setting Register"]
pub mod dodsr;
