#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - KEY Return Control Register"]
    pub krctl: KRCTL,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - KEY Return Flag Register"]
    pub krf: KRF,
    _reserved2: [u8; 0x03],
    #[doc = "0x08 - KEY Return Mode Register"]
    pub krm: KRM,
}
#[doc = "KRCTL (rw) register accessor: KEY Return Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`krctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`krctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@krctl`]
module"]
pub type KRCTL = crate::Reg<krctl::KRCTL_SPEC>;
#[doc = "KEY Return Control Register"]
pub mod krctl;
#[doc = "KRF (rw) register accessor: KEY Return Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`krf::R`]. WARN: The register is **modified** in some way after a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`krf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@krf`]
module"]
pub type KRF = crate::Reg<krf::KRF_SPEC>;
#[doc = "KEY Return Flag Register"]
pub mod krf;
#[doc = "KRM (rw) register accessor: KEY Return Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`krm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`krm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@krm`]
module"]
pub type KRM = crate::Reg<krm::KRM_SPEC>;
#[doc = "KEY Return Mode Register"]
pub mod krm;
