#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Event Link Controller Register"]
    pub elcr: ELCR,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - Event Link Software Event Generation Register 0"]
    pub elsegr0: ELSEGR,
    _reserved2: [u8; 0x01],
    #[doc = "0x04 - Event Link Software Event Generation Register 1"]
    pub elsegr1: ELSEGR,
    _reserved3: [u8; 0x0b],
    #[doc = "0x10 - Event Link Setting Register 0"]
    pub elsr0: ELSR,
    _reserved4: [u8; 0x02],
    #[doc = "0x14 - Event Link Setting Register 1"]
    pub elsr1: ELSR,
    _reserved5: [u8; 0x02],
    #[doc = "0x18 - Event Link Setting Register 2"]
    pub elsr2: ELSR,
    _reserved6: [u8; 0x02],
    #[doc = "0x1c - Event Link Setting Register 3"]
    pub elsr3: ELSR,
    _reserved7: [u8; 0x02],
    #[doc = "0x20 - Event Link Setting Register 4"]
    pub elsr4: ELSR,
    _reserved8: [u8; 0x02],
    #[doc = "0x24 - Event Link Setting Register 5"]
    pub elsr5: ELSR,
    _reserved9: [u8; 0x02],
    #[doc = "0x28 - Event Link Setting Register 6"]
    pub elsr6: ELSR,
    _reserved10: [u8; 0x02],
    #[doc = "0x2c - Event Link Setting Register 7"]
    pub elsr7: ELSR,
    _reserved11: [u8; 0x02],
    #[doc = "0x30 - Event Link Setting Register 8"]
    pub elsr8: ELSR,
    _reserved12: [u8; 0x02],
    #[doc = "0x34 - Event Link Setting Register 9"]
    pub elsr9: ELSR,
    _reserved13: [u8; 0x0a],
    #[doc = "0x40 - Event Link Setting Register 12"]
    pub elsr12: ELSR12,
    _reserved14: [u8; 0x06],
    #[doc = "0x48 - Event Link Setting Register 14"]
    pub elsr14: ELSR14,
    _reserved15: [u8; 0x02],
    #[doc = "0x4c - Event Link Setting Register 15"]
    pub elsr15: ELSR14,
    _reserved16: [u8; 0x02],
    #[doc = "0x50 - Event Link Setting Register 16"]
    pub elsr16: ELSR14,
    _reserved17: [u8; 0x02],
    #[doc = "0x54 - Event Link Setting Register 17"]
    pub elsr17: ELSR14,
    _reserved18: [u8; 0x02],
    #[doc = "0x58 - Event Link Setting Register 18"]
    pub elsr18: ELSR14,
}
#[doc = "ELCR (rw) register accessor: Event Link Controller Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`elcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`elcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@elcr`]
module"]
pub type ELCR = crate::Reg<elcr::ELCR_SPEC>;
#[doc = "Event Link Controller Register"]
pub mod elcr;
#[doc = "ELSEGR (rw) register accessor: Event Link Software Event Generation Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`elsegr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`elsegr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@elsegr`]
module"]
pub type ELSEGR = crate::Reg<elsegr::ELSEGR_SPEC>;
#[doc = "Event Link Software Event Generation Register %s"]
pub mod elsegr;
#[doc = "ELSR (rw) register accessor: Event Link Setting Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`elsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`elsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@elsr`]
module"]
pub type ELSR = crate::Reg<elsr::ELSR_SPEC>;
#[doc = "Event Link Setting Register %s"]
pub mod elsr;
pub use elsr as elsr12;
pub use elsr as elsr14;
pub use ELSR as ELSR12;
pub use ELSR as ELSR14;
