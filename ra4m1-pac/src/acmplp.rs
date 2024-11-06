#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ACMPLP Mode Setting Register"]
    pub compmdr: COMPMDR,
    #[doc = "0x01 - ACMPLP Filter Control Register"]
    pub compfir: COMPFIR,
    #[doc = "0x02 - ACMPLP Output Control Register"]
    pub compocr: COMPOCR,
    _reserved3: [u8; 0x01],
    #[doc = "0x04 - Comparator Input Select Register"]
    pub compsel0: COMPSEL0,
    #[doc = "0x05 - Comparator Reference Voltage Select Register"]
    pub compsel1: COMPSEL1,
}
#[doc = "COMPMDR (rw) register accessor: ACMPLP Mode Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`compmdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`compmdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compmdr`]
module"]
pub type COMPMDR = crate::Reg<compmdr::COMPMDR_SPEC>;
#[doc = "ACMPLP Mode Setting Register"]
pub mod compmdr;
#[doc = "COMPFIR (rw) register accessor: ACMPLP Filter Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`compfir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`compfir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compfir`]
module"]
pub type COMPFIR = crate::Reg<compfir::COMPFIR_SPEC>;
#[doc = "ACMPLP Filter Control Register"]
pub mod compfir;
#[doc = "COMPOCR (rw) register accessor: ACMPLP Output Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`compocr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`compocr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compocr`]
module"]
pub type COMPOCR = crate::Reg<compocr::COMPOCR_SPEC>;
#[doc = "ACMPLP Output Control Register"]
pub mod compocr;
#[doc = "COMPSEL0 (rw) register accessor: Comparator Input Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`compsel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`compsel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compsel0`]
module"]
pub type COMPSEL0 = crate::Reg<compsel0::COMPSEL0_SPEC>;
#[doc = "Comparator Input Select Register"]
pub mod compsel0;
#[doc = "COMPSEL1 (rw) register accessor: Comparator Reference Voltage Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`compsel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`compsel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compsel1`]
module"]
pub type COMPSEL1 = crate::Reg<compsel1::COMPSEL1_SPEC>;
#[doc = "Comparator Reference Voltage Select Register"]
pub mod compsel1;
