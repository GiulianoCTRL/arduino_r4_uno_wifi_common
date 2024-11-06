#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LCD Mode Register 0"]
    pub lcdm0: LCDM0,
    #[doc = "0x01 - LCD Mode Register 1"]
    pub lcdm1: LCDM1,
    #[doc = "0x02 - LCD Clock Control Register 0"]
    pub lcdc0: LCDC0,
    #[doc = "0x03 - LCD Boost Level Control Register"]
    pub vlcd: VLCD,
    _reserved4: [u8; 0xfc],
    #[doc = "0x100..0x126 - LCD Display Data Register %s"]
    pub seg: [SEG; 38],
}
#[doc = "LCDM0 (rw) register accessor: LCD Mode Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcdm0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcdm0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm0`]
module"]
pub type LCDM0 = crate::Reg<lcdm0::LCDM0_SPEC>;
#[doc = "LCD Mode Register 0"]
pub mod lcdm0;
#[doc = "LCDM1 (rw) register accessor: LCD Mode Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcdm1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcdm1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm1`]
module"]
pub type LCDM1 = crate::Reg<lcdm1::LCDM1_SPEC>;
#[doc = "LCD Mode Register 1"]
pub mod lcdm1;
#[doc = "LCDC0 (rw) register accessor: LCD Clock Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcdc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcdc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdc0`]
module"]
pub type LCDC0 = crate::Reg<lcdc0::LCDC0_SPEC>;
#[doc = "LCD Clock Control Register 0"]
pub mod lcdc0;
#[doc = "VLCD (rw) register accessor: LCD Boost Level Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vlcd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vlcd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vlcd`]
module"]
pub type VLCD = crate::Reg<vlcd::VLCD_SPEC>;
#[doc = "LCD Boost Level Control Register"]
pub mod vlcd;
#[doc = "SEG (rw) register accessor: LCD Display Data Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seg`]
module"]
pub type SEG = crate::Reg<seg::SEG_SPEC>;
#[doc = "LCD Display Data Register %s"]
pub mod seg;
