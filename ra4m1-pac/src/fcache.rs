#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    #[doc = "0x100 - Flash Cache Enable Register"]
    pub fcachee: FCACHEE,
    _reserved1: [u8; 0x02],
    #[doc = "0x104 - Flash Cache Invalidate Register"]
    pub fcacheiv: FCACHEIV,
    _reserved2: [u8; 0x16],
    #[doc = "0x11c - Flash Wait Cycle Register"]
    pub flwt: FLWT,
}
#[doc = "FCACHEE (rw) register accessor: Flash Cache Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcachee::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcachee::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcachee`]
module"]
pub type FCACHEE = crate::Reg<fcachee::FCACHEE_SPEC>;
#[doc = "Flash Cache Enable Register"]
pub mod fcachee;
#[doc = "FCACHEIV (rw) register accessor: Flash Cache Invalidate Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcacheiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcacheiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcacheiv`]
module"]
pub type FCACHEIV = crate::Reg<fcacheiv::FCACHEIV_SPEC>;
#[doc = "Flash Cache Invalidate Register"]
pub mod fcacheiv;
#[doc = "FLWT (rw) register accessor: Flash Wait Cycle Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flwt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flwt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flwt`]
module"]
pub type FLWT = crate::Reg<flwt::FLWT_SPEC>;
#[doc = "Flash Wait Cycle Register"]
pub mod flwt;
