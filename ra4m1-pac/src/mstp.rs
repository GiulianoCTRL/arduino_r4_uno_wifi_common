#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Stop Control Register B"]
    pub mstpcrb: MSTPCRB,
    #[doc = "0x04 - Module Stop Control Register C"]
    pub mstpcrc: MSTPCRC,
    #[doc = "0x08 - Module Stop Control Register D"]
    pub mstpcrd: MSTPCRD,
}
#[doc = "MSTPCRB (rw) register accessor: Module Stop Control Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mstpcrb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mstpcrb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mstpcrb`]
module"]
pub type MSTPCRB = crate::Reg<mstpcrb::MSTPCRB_SPEC>;
#[doc = "Module Stop Control Register B"]
pub mod mstpcrb;
#[doc = "MSTPCRC (rw) register accessor: Module Stop Control Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mstpcrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mstpcrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mstpcrc`]
module"]
pub type MSTPCRC = crate::Reg<mstpcrc::MSTPCRC_SPEC>;
#[doc = "Module Stop Control Register C"]
pub mod mstpcrc;
#[doc = "MSTPCRD (rw) register accessor: Module Stop Control Register D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mstpcrd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mstpcrd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mstpcrd`]
module"]
pub type MSTPCRD = crate::Reg<mstpcrd::MSTPCRD_SPEC>;
#[doc = "Module Stop Control Register D"]
pub mod mstpcrd;
