#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Stack Pointer Monitor Operation After Detection Register"]
    pub mspmpuoad: MSPMPUOAD,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - Stack Pointer Monitor Access Control Register"]
    pub mspmpuctl: MSPMPUCTL,
    #[doc = "0x06 - Stack Pointer Monitor Protection Register"]
    pub mspmpupt: MSPMPUPT,
    #[doc = "0x08 - Main Stack Pointer (MSP) Monitor Start Address Register"]
    pub mspmpusa: MSPMPUSA,
    #[doc = "0x0c - Main Stack Pointer (MSP) Monitor End Address Register"]
    pub mspmpuea: MSPMPUEA,
    #[doc = "0x10 - Stack Pointer Monitor Operation After Detection Register"]
    pub pspmpuoad: PSPMPUOAD,
    _reserved6: [u8; 0x02],
    #[doc = "0x14 - Stack Pointer Monitor Access Control Register"]
    pub pspmpuctl: PSPMPUCTL,
    #[doc = "0x16 - Stack Pointer Monitor Protection Register"]
    pub pspmpupt: PSPMPUPT,
    #[doc = "0x18 - Process Stack Pointer (PSP) Monitor Start Address Register"]
    pub pspmpusa: PSPMPUSA,
    #[doc = "0x1c - Process Stack Pointer (PSP) Monitor End Address Register"]
    pub pspmpuea: PSPMPUEA,
}
#[doc = "MSPMPUOAD (rw) register accessor: Stack Pointer Monitor Operation After Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mspmpuoad::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mspmpuoad::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mspmpuoad`]
module"]
pub type MSPMPUOAD = crate::Reg<mspmpuoad::MSPMPUOAD_SPEC>;
#[doc = "Stack Pointer Monitor Operation After Detection Register"]
pub mod mspmpuoad;
#[doc = "MSPMPUCTL (rw) register accessor: Stack Pointer Monitor Access Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mspmpuctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mspmpuctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mspmpuctl`]
module"]
pub type MSPMPUCTL = crate::Reg<mspmpuctl::MSPMPUCTL_SPEC>;
#[doc = "Stack Pointer Monitor Access Control Register"]
pub mod mspmpuctl;
#[doc = "MSPMPUPT (rw) register accessor: Stack Pointer Monitor Protection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mspmpupt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mspmpupt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mspmpupt`]
module"]
pub type MSPMPUPT = crate::Reg<mspmpupt::MSPMPUPT_SPEC>;
#[doc = "Stack Pointer Monitor Protection Register"]
pub mod mspmpupt;
#[doc = "MSPMPUSA (rw) register accessor: Main Stack Pointer (MSP) Monitor Start Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mspmpusa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mspmpusa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mspmpusa`]
module"]
pub type MSPMPUSA = crate::Reg<mspmpusa::MSPMPUSA_SPEC>;
#[doc = "Main Stack Pointer (MSP) Monitor Start Address Register"]
pub mod mspmpusa;
#[doc = "MSPMPUEA (rw) register accessor: Main Stack Pointer (MSP) Monitor End Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mspmpuea::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mspmpuea::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mspmpuea`]
module"]
pub type MSPMPUEA = crate::Reg<mspmpuea::MSPMPUEA_SPEC>;
#[doc = "Main Stack Pointer (MSP) Monitor End Address Register"]
pub mod mspmpuea;
#[doc = "PSPMPUOAD (rw) register accessor: Stack Pointer Monitor Operation After Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pspmpuoad::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pspmpuoad::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pspmpuoad`]
module"]
pub type PSPMPUOAD = crate::Reg<pspmpuoad::PSPMPUOAD_SPEC>;
#[doc = "Stack Pointer Monitor Operation After Detection Register"]
pub mod pspmpuoad;
#[doc = "PSPMPUCTL (rw) register accessor: Stack Pointer Monitor Access Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pspmpuctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pspmpuctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pspmpuctl`]
module"]
pub type PSPMPUCTL = crate::Reg<pspmpuctl::PSPMPUCTL_SPEC>;
#[doc = "Stack Pointer Monitor Access Control Register"]
pub mod pspmpuctl;
#[doc = "PSPMPUPT (rw) register accessor: Stack Pointer Monitor Protection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pspmpupt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pspmpupt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pspmpupt`]
module"]
pub type PSPMPUPT = crate::Reg<pspmpupt::PSPMPUPT_SPEC>;
#[doc = "Stack Pointer Monitor Protection Register"]
pub mod pspmpupt;
#[doc = "PSPMPUSA (rw) register accessor: Process Stack Pointer (PSP) Monitor Start Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pspmpusa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pspmpusa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pspmpusa`]
module"]
pub type PSPMPUSA = crate::Reg<pspmpusa::PSPMPUSA_SPEC>;
#[doc = "Process Stack Pointer (PSP) Monitor Start Address Register"]
pub mod pspmpusa;
#[doc = "PSPMPUEA (rw) register accessor: Process Stack Pointer (PSP) Monitor End Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pspmpuea::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pspmpuea::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pspmpuea`]
module"]
pub type PSPMPUEA = crate::Reg<pspmpuea::PSPMPUEA_SPEC>;
#[doc = "Process Stack Pointer (PSP) Monitor End Address Register"]
pub mod pspmpuea;
