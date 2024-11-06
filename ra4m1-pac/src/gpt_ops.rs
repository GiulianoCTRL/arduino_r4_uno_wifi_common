#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Output Phase Switching Control Register"]
    pub opscr: OPSCR,
}
#[doc = "OPSCR (rw) register accessor: Output Phase Switching Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opscr`]
module"]
pub type OPSCR = crate::Reg<opscr::OPSCR_SPEC>;
#[doc = "Output Phase Switching Control Register"]
pub mod opscr;
