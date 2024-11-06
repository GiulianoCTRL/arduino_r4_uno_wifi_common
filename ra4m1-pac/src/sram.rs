#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM Parity Error Operation After Detection Register"]
    pub parioad: PARIOAD,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - SRAM Protection Register"]
    pub sramprcr: SRAMPRCR,
    _reserved2: [u8; 0xbb],
    #[doc = "0xc0 - ECC Operating Mode Control Register"]
    pub eccmode: ECCMODE,
    #[doc = "0xc1 - ECC 2-Bit Error Status Register"]
    pub ecc2sts: ECC2STS,
    #[doc = "0xc2 - ECC 1-Bit Error Information Update Enable Register"]
    pub ecc1stsen: ECC1STSEN,
    #[doc = "0xc3 - ECC 1-Bit Error Status Register"]
    pub ecc1sts: ECC1STS,
    #[doc = "0xc4 - ECC Protection Register"]
    pub eccprcr: ECCPRCR,
    _reserved7: [u8; 0x0b],
    #[doc = "0xd0 - ECC Protection Register 2"]
    pub eccprcr2: ECCPRCR2,
    _reserved8: [u8; 0x03],
    #[doc = "0xd4 - ECC Test Control Register"]
    pub eccetst: ECCETST,
    _reserved9: [u8; 0x03],
    #[doc = "0xd8 - SRAM ECC Error Operation After Detection Register"]
    pub eccoad: ECCOAD,
}
#[doc = "PARIOAD (rw) register accessor: SRAM Parity Error Operation After Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`parioad::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`parioad::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@parioad`]
module"]
pub type PARIOAD = crate::Reg<parioad::PARIOAD_SPEC>;
#[doc = "SRAM Parity Error Operation After Detection Register"]
pub mod parioad;
#[doc = "SRAMPRCR (rw) register accessor: SRAM Protection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sramprcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sramprcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sramprcr`]
module"]
pub type SRAMPRCR = crate::Reg<sramprcr::SRAMPRCR_SPEC>;
#[doc = "SRAM Protection Register"]
pub mod sramprcr;
#[doc = "ECCMODE (rw) register accessor: ECC Operating Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccmode`]
module"]
pub type ECCMODE = crate::Reg<eccmode::ECCMODE_SPEC>;
#[doc = "ECC Operating Mode Control Register"]
pub mod eccmode;
#[doc = "ECC2STS (rw) register accessor: ECC 2-Bit Error Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc2sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc2sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc2sts`]
module"]
pub type ECC2STS = crate::Reg<ecc2sts::ECC2STS_SPEC>;
#[doc = "ECC 2-Bit Error Status Register"]
pub mod ecc2sts;
#[doc = "ECC1STSEN (rw) register accessor: ECC 1-Bit Error Information Update Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc1stsen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc1stsen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc1stsen`]
module"]
pub type ECC1STSEN = crate::Reg<ecc1stsen::ECC1STSEN_SPEC>;
#[doc = "ECC 1-Bit Error Information Update Enable Register"]
pub mod ecc1stsen;
#[doc = "ECC1STS (rw) register accessor: ECC 1-Bit Error Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc1sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc1sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc1sts`]
module"]
pub type ECC1STS = crate::Reg<ecc1sts::ECC1STS_SPEC>;
#[doc = "ECC 1-Bit Error Status Register"]
pub mod ecc1sts;
#[doc = "ECCPRCR (rw) register accessor: ECC Protection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccprcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccprcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccprcr`]
module"]
pub type ECCPRCR = crate::Reg<eccprcr::ECCPRCR_SPEC>;
#[doc = "ECC Protection Register"]
pub mod eccprcr;
#[doc = "ECCPRCR2 (rw) register accessor: ECC Protection Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccprcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccprcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccprcr2`]
module"]
pub type ECCPRCR2 = crate::Reg<eccprcr2::ECCPRCR2_SPEC>;
#[doc = "ECC Protection Register 2"]
pub mod eccprcr2;
#[doc = "ECCETST (rw) register accessor: ECC Test Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccetst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccetst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccetst`]
module"]
pub type ECCETST = crate::Reg<eccetst::ECCETST_SPEC>;
#[doc = "ECC Test Control Register"]
pub mod eccetst;
#[doc = "ECCOAD (rw) register accessor: SRAM ECC Error Operation After Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccoad::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccoad::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccoad`]
module"]
pub type ECCOAD = crate::Reg<eccoad::ECCOAD_SPEC>;
#[doc = "SRAM ECC Error Operation After Detection Register"]
pub mod eccoad;
