#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - POEG Group A Setting Register"]
    pub poegga: POEGG,
    _reserved1: [u8; 0xfc],
    #[doc = "0x100 - POEG Group B Setting Register"]
    pub poeggb: POEGG,
}
#[doc = "POEGG (rw) register accessor: POEG Group %s Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`poegg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`poegg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@poegg`]
module"]
pub type POEGG = crate::Reg<poegg::POEGG_SPEC>;
#[doc = "POEG Group %s Setting Register"]
pub mod poegg;
