#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CAC Control Register 0"]
    pub cacr0: CACR0,
    #[doc = "0x01 - CAC Control Register 1"]
    pub cacr1: CACR1,
    #[doc = "0x02 - CAC Control Register 2"]
    pub cacr2: CACR2,
    #[doc = "0x03 - CAC Interrupt Control Register"]
    pub caicr: CAICR,
    #[doc = "0x04 - CAC Status Register"]
    pub castr: CASTR,
    _reserved5: [u8; 0x01],
    #[doc = "0x06 - CAC Upper-Limit Value Setting Register"]
    pub caulvr: CAULVR,
    #[doc = "0x08 - CAC Lower-Limit Value Setting Register"]
    pub callvr: CALLVR,
    #[doc = "0x0a - CAC Counter Buffer Register"]
    pub cacntbr: CACNTBR,
}
#[doc = "CACR0 (rw) register accessor: CAC Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cacr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cacr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cacr0`]
module"]
pub type CACR0 = crate::Reg<cacr0::CACR0_SPEC>;
#[doc = "CAC Control Register 0"]
pub mod cacr0;
#[doc = "CACR1 (rw) register accessor: CAC Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cacr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cacr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cacr1`]
module"]
pub type CACR1 = crate::Reg<cacr1::CACR1_SPEC>;
#[doc = "CAC Control Register 1"]
pub mod cacr1;
#[doc = "CACR2 (rw) register accessor: CAC Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cacr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cacr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cacr2`]
module"]
pub type CACR2 = crate::Reg<cacr2::CACR2_SPEC>;
#[doc = "CAC Control Register 2"]
pub mod cacr2;
#[doc = "CAICR (rw) register accessor: CAC Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`caicr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`caicr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@caicr`]
module"]
pub type CAICR = crate::Reg<caicr::CAICR_SPEC>;
#[doc = "CAC Interrupt Control Register"]
pub mod caicr;
#[doc = "CASTR (r) register accessor: CAC Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`castr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@castr`]
module"]
pub type CASTR = crate::Reg<castr::CASTR_SPEC>;
#[doc = "CAC Status Register"]
pub mod castr;
#[doc = "CAULVR (rw) register accessor: CAC Upper-Limit Value Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`caulvr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`caulvr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@caulvr`]
module"]
pub type CAULVR = crate::Reg<caulvr::CAULVR_SPEC>;
#[doc = "CAC Upper-Limit Value Setting Register"]
pub mod caulvr;
#[doc = "CALLVR (rw) register accessor: CAC Lower-Limit Value Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`callvr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`callvr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@callvr`]
module"]
pub type CALLVR = crate::Reg<callvr::CALLVR_SPEC>;
#[doc = "CAC Lower-Limit Value Setting Register"]
pub mod callvr;
#[doc = "CACNTBR (r) register accessor: CAC Counter Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cacntbr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cacntbr`]
module"]
pub type CACNTBR = crate::Reg<cacntbr::CACNTBR_SPEC>;
#[doc = "CAC Counter Buffer Register"]
pub mod cacntbr;
