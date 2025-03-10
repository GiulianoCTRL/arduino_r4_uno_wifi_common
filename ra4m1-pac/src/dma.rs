#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMAC Module Activation Register"]
    pub dmast: DMAST,
}
#[doc = "DMAST (rw) register accessor: DMAC Module Activation Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmast::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmast::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmast`]
module"]
pub type DMAST = crate::Reg<dmast::DMAST_SPEC>;
#[doc = "DMAC Module Activation Register"]
pub mod dmast;
