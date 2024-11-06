#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - D/A Data Register 0"]
    pub dadr0: DADR0,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - D/A Control Register"]
    pub dacr: DACR,
    #[doc = "0x05 - DADR0 Format Select Register"]
    pub dadpr: DADPR,
    #[doc = "0x06 - D/A-A/D Synchronous Start Control Register"]
    pub daadscr: DAADSCR,
    #[doc = "0x07 - D/A VREF Control Register"]
    pub davrefcr: DAVREFCR,
}
#[doc = "DADR0 (rw) register accessor: D/A Data Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dadr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dadr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dadr0`]
module"]
pub type DADR0 = crate::Reg<dadr0::DADR0_SPEC>;
#[doc = "D/A Data Register 0"]
pub mod dadr0;
#[doc = "DACR (rw) register accessor: D/A Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacr`]
module"]
pub type DACR = crate::Reg<dacr::DACR_SPEC>;
#[doc = "D/A Control Register"]
pub mod dacr;
#[doc = "DADPR (rw) register accessor: DADR0 Format Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dadpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dadpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dadpr`]
module"]
pub type DADPR = crate::Reg<dadpr::DADPR_SPEC>;
#[doc = "DADR0 Format Select Register"]
pub mod dadpr;
#[doc = "DAADSCR (rw) register accessor: D/A-A/D Synchronous Start Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daadscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daadscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daadscr`]
module"]
pub type DAADSCR = crate::Reg<daadscr::DAADSCR_SPEC>;
#[doc = "D/A-A/D Synchronous Start Control Register"]
pub mod daadscr;
#[doc = "DAVREFCR (rw) register accessor: D/A VREF Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`davrefcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`davrefcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@davrefcr`]
module"]
pub type DAVREFCR = crate::Reg<davrefcr::DAVREFCR_SPEC>;
#[doc = "D/A VREF Control Register"]
pub mod davrefcr;
