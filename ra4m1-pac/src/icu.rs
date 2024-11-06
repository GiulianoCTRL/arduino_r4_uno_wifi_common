#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x0d - IRQ Control Register %s"]
    pub irqcr: [IRQCR; 13],
    _reserved1: [u8; 0x01],
    #[doc = "0x0e - IRQ Control Register 14"]
    pub irqcr14: IRQCR14,
    #[doc = "0x0f - IRQ Control Register 15"]
    pub irqcr15: IRQCR14,
    _reserved3: [u8; 0xf0],
    #[doc = "0x100 - NMI Pin Interrupt Control Register"]
    pub nmicr: NMICR,
    _reserved4: [u8; 0x1f],
    #[doc = "0x120 - Non-Maskable Interrupt Enable Register"]
    pub nmier: NMIER,
    _reserved5: [u8; 0x0e],
    #[doc = "0x130 - Non-Maskable Interrupt Status Clear Register"]
    pub nmiclr: NMICLR,
    _reserved6: [u8; 0x0e],
    #[doc = "0x140 - Non-Maskable Interrupt Status Register"]
    pub nmisr: NMISR,
    _reserved7: [u8; 0x5e],
    #[doc = "0x1a0 - Wake Up Interrupt Enable Register"]
    pub wupen: WUPEN,
    _reserved8: [u8; 0x5c],
    #[doc = "0x200 - SYS Event Link Setting Register"]
    pub selsr0: SELSR0,
    _reserved9: [u8; 0x7e],
    #[doc = "0x280 - DMAC Event Link Setting Register 0"]
    pub delsr0: DELSR,
    _reserved10: [u8; 0x02],
    #[doc = "0x284 - DMAC Event Link Setting Register 1"]
    pub delsr1: DELSR,
    _reserved11: [u8; 0x02],
    #[doc = "0x288 - DMAC Event Link Setting Register 2"]
    pub delsr2: DELSR,
    _reserved12: [u8; 0x02],
    #[doc = "0x28c - DMAC Event Link Setting Register 3"]
    pub delsr3: DELSR,
    _reserved13: [u8; 0x72],
    #[doc = "0x300..0x380 - ICU Event Link Setting Register %s"]
    pub ielsr: [IELSR; 32],
}
#[doc = "IRQCR (rw) register accessor: IRQ Control Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqcr`]
module"]
pub type IRQCR = crate::Reg<irqcr::IRQCR_SPEC>;
#[doc = "IRQ Control Register %s"]
pub mod irqcr;
pub use irqcr as irqcr14;
pub use IRQCR as IRQCR14;
#[doc = "NMISR (r) register accessor: Non-Maskable Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nmisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmisr`]
module"]
pub type NMISR = crate::Reg<nmisr::NMISR_SPEC>;
#[doc = "Non-Maskable Interrupt Status Register"]
pub mod nmisr;
#[doc = "NMIER (rw) register accessor: Non-Maskable Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nmier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nmier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmier`]
module"]
pub type NMIER = crate::Reg<nmier::NMIER_SPEC>;
#[doc = "Non-Maskable Interrupt Enable Register"]
pub mod nmier;
#[doc = "NMICLR (rw) register accessor: Non-Maskable Interrupt Status Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nmiclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nmiclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmiclr`]
module"]
pub type NMICLR = crate::Reg<nmiclr::NMICLR_SPEC>;
#[doc = "Non-Maskable Interrupt Status Clear Register"]
pub mod nmiclr;
#[doc = "NMICR (rw) register accessor: NMI Pin Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nmicr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nmicr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmicr`]
module"]
pub type NMICR = crate::Reg<nmicr::NMICR_SPEC>;
#[doc = "NMI Pin Interrupt Control Register"]
pub mod nmicr;
#[doc = "IELSR (rw) register accessor: ICU Event Link Setting Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ielsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ielsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ielsr`]
module"]
pub type IELSR = crate::Reg<ielsr::IELSR_SPEC>;
#[doc = "ICU Event Link Setting Register %s"]
pub mod ielsr;
#[doc = "DELSR (rw) register accessor: DMAC Event Link Setting Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`delsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`delsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@delsr`]
module"]
pub type DELSR = crate::Reg<delsr::DELSR_SPEC>;
#[doc = "DMAC Event Link Setting Register %s"]
pub mod delsr;
#[doc = "SELSR0 (rw) register accessor: SYS Event Link Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`selsr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`selsr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@selsr0`]
module"]
pub type SELSR0 = crate::Reg<selsr0::SELSR0_SPEC>;
#[doc = "SYS Event Link Setting Register"]
pub mod selsr0;
#[doc = "WUPEN (rw) register accessor: Wake Up Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wupen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wupen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wupen`]
module"]
pub type WUPEN = crate::Reg<wupen::WUPEN_SPEC>;
#[doc = "Wake Up Interrupt Enable Register"]
pub mod wupen;
