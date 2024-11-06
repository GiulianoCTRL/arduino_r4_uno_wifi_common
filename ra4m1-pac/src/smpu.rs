#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Slave MPU Control Register"]
    pub smpuctl: SMPUCTL,
    _reserved1: [u8; 0x0e],
    #[doc = "0x10 - Access Control Register for MBIU"]
    pub smpumbiu: SMPUMBIU,
    _reserved2: [u8; 0x02],
    #[doc = "0x14 - Access Control Register for FBIU"]
    pub smpufbiu: SMPUFBIU,
    _reserved3: [u8; 0x02],
    #[doc = "0x18 - Access Control Register for SRAM0"]
    pub smpusram0: SMPUSRAM0,
    _reserved4: [u8; 0x06],
    #[doc = "0x20 - Access Control Register for P0BIU"]
    pub smpup0biu: SMPUPBIU,
    _reserved5: [u8; 0x02],
    #[doc = "0x24 - Access Control Register for P2BIU"]
    pub smpup2biu: SMPUPBIU,
    _reserved6: [u8; 0x02],
    #[doc = "0x28 - Access Control Register for P6BIU"]
    pub smpup6biu: SMPUPBIU,
}
#[doc = "SMPUCTL (rw) register accessor: Slave MPU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpuctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpuctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smpuctl`]
module"]
pub type SMPUCTL = crate::Reg<smpuctl::SMPUCTL_SPEC>;
#[doc = "Slave MPU Control Register"]
pub mod smpuctl;
#[doc = "SMPUMBIU (rw) register accessor: Access Control Register for MBIU\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpumbiu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpumbiu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smpumbiu`]
module"]
pub type SMPUMBIU = crate::Reg<smpumbiu::SMPUMBIU_SPEC>;
#[doc = "Access Control Register for MBIU"]
pub mod smpumbiu;
#[doc = "SMPUFBIU (rw) register accessor: Access Control Register for FBIU\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpufbiu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpufbiu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smpufbiu`]
module"]
pub type SMPUFBIU = crate::Reg<smpufbiu::SMPUFBIU_SPEC>;
#[doc = "Access Control Register for FBIU"]
pub mod smpufbiu;
#[doc = "SMPUSRAM0 (rw) register accessor: Access Control Register for SRAM0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpusram0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpusram0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smpusram0`]
module"]
pub type SMPUSRAM0 = crate::Reg<smpusram0::SMPUSRAM0_SPEC>;
#[doc = "Access Control Register for SRAM0"]
pub mod smpusram0;
#[doc = "SMPUPBIU (rw) register accessor: Access Control Register for P%sBIU\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpupbiu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpupbiu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smpupbiu`]
module"]
pub type SMPUPBIU = crate::Reg<smpupbiu::SMPUPBIU_SPEC>;
#[doc = "Access Control Register for P%sBIU"]
pub mod smpupbiu;
