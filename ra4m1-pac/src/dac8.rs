#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - D/A Conversion Value Setting Register %s"]
    pub dacs: [DACS; 2],
    _reserved1: [u8; 0x01],
    #[doc = "0x03 - D/A Converter Mode Register"]
    pub dam: DAM,
}
#[doc = "DACS (rw) register accessor: D/A Conversion Value Setting Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacs`]
module"]
pub type DACS = crate::Reg<dacs::DACS_SPEC>;
#[doc = "D/A Conversion Value Setting Register %s"]
pub mod dacs;
#[doc = "DAM (rw) register accessor: D/A Converter Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dam::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dam::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dam`]
module"]
pub type DAM = crate::Reg<dam::DAM_SPEC>;
#[doc = "D/A Converter Mode Register"]
pub mod dam;
