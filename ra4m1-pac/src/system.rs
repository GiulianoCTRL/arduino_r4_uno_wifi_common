#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    #[doc = "0x0c - Standby Control Register"]
    pub sbycr: SBYCR,
    _reserved1: [u8; 0x0e],
    #[doc = "0x1c - Module Stop Control Register A"]
    pub mstpcra: MSTPCRA,
    #[doc = "0x20 - System Clock Division Control Register"]
    pub sckdivcr: SCKDIVCR,
    _reserved3: [u8; 0x02],
    #[doc = "0x26 - System Clock Source Control Register"]
    pub sckscr: SCKSCR,
    _reserved4: [u8; 0x03],
    #[doc = "0x2a - PLL Control Register"]
    pub pllcr: PLLCR,
    #[doc = "0x2b - PLL Clock Control Register2"]
    pub pllccr2: PLLCCR2,
    _reserved6: [u8; 0x05],
    #[doc = "0x31 - Memory Wait Cycle Control Register"]
    pub memwait: MEMWAIT,
    #[doc = "0x32 - Main Clock Oscillator Control Register"]
    pub mosccr: MOSCCR,
    _reserved8: [u8; 0x03],
    #[doc = "0x36 - High-Speed On-Chip Oscillator Control Register"]
    pub hococr: HOCOCR,
    _reserved9: [u8; 0x01],
    #[doc = "0x38 - Middle-Speed On-Chip Oscillator Control Register"]
    pub mococr: MOCOCR,
    _reserved10: [u8; 0x03],
    #[doc = "0x3c - Oscillation Stabilization Flag Register"]
    pub oscsf: OSCSF,
    _reserved11: [u8; 0x01],
    #[doc = "0x3e - Clock Out Control Register"]
    pub ckocr: CKOCR,
    #[doc = "0x3f - Trace Clock Control Register"]
    pub trckcr: TRCKCR,
    #[doc = "0x40 - Oscillation Stop Detection Control Register"]
    pub ostdcr: OSTDCR,
    #[doc = "0x41 - Oscillation Stop Detection Status Register"]
    pub ostdsr: OSTDSR,
    _reserved15: [u8; 0x0e],
    #[doc = "0x50 - Segment LCD Source Clock Control Register"]
    pub slcdsckcr: SLCDSCKCR,
    _reserved16: [u8; 0x10],
    #[doc = "0x61 - MOCO User Trimming Control Register"]
    pub mocoutcr: MOCOUTCR,
    #[doc = "0x62 - HOCO User Trimming Control Register"]
    pub hocoutcr: HOCOUTCR,
    _reserved18: [u8; 0x2f],
    #[doc = "0x92 - Snooze Control Register"]
    pub snzcr: SNZCR,
    _reserved19: [u8; 0x01],
    #[doc = "0x94 - Snooze End Control Register"]
    pub snzedcr: SNZEDCR,
    _reserved20: [u8; 0x03],
    #[doc = "0x98 - Snooze Request Control Register"]
    pub snzreqcr: SNZREQCR,
    _reserved21: [u8; 0x02],
    #[doc = "0x9e - Flash Operation Control Register"]
    pub flstop: FLSTOP,
    _reserved22: [u8; 0x01],
    #[doc = "0xa0 - Operating Power Control Register"]
    pub opccr: OPCCR,
    _reserved23: [u8; 0x01],
    #[doc = "0xa2 - Main Clock Oscillator Wait Control Register"]
    pub moscwtcr: MOSCWTCR,
    _reserved24: [u8; 0x02],
    #[doc = "0xa5 - High-Speed On-Chip Oscillator Wait Control Register"]
    pub hocowtcr: HOCOWTCR,
    _reserved25: [u8; 0x04],
    #[doc = "0xaa - Sub Operating Power Control Register"]
    pub sopccr: SOPCCR,
    _reserved26: [u8; 0x15],
    #[doc = "0xc0 - Reset Status Register 1"]
    pub rstsr1: RSTSR1,
    _reserved27: [u8; 0x04],
    #[doc = "0xc6 - Backup Register Access Control Register"]
    pub bkracr: BKRACR,
    _reserved28: [u8; 0x09],
    #[doc = "0xd0 - USB Clock Control register"]
    pub usbckcr: USBCKCR,
    _reserved29: [u8; 0x0f],
    #[doc = "0xe0 - Voltage Monitor 1 Circuit Control Register 1"]
    pub lvd1cr1: LVDCR1,
    #[doc = "0xe1 - Voltage Monitor 1 Circuit Status Register"]
    pub lvd1sr: LVDSR,
    #[doc = "0xe2 - Voltage Monitor 2 Circuit Control Register 1"]
    pub lvd2cr1: LVDCR1,
    #[doc = "0xe3 - Voltage Monitor 2 Circuit Status Register"]
    pub lvd2sr: LVDSR,
    _reserved33: [u8; 0x031a],
    #[doc = "0x3fe - Protect Register"]
    pub prcr: PRCR,
    _reserved34: [u8; 0x0e],
    #[doc = "0x40e - System Control OCD Control Register"]
    pub syocdcr: SYOCDCR,
    _reserved35: [u8; 0x01],
    #[doc = "0x410 - Reset Status Register 0"]
    pub rstsr0: RSTSR0,
    #[doc = "0x411 - Reset Status Register 2"]
    pub rstsr2: RSTSR2,
    _reserved37: [u8; 0x01],
    #[doc = "0x413 - Main Clock Oscillator Mode Oscillation Control Register"]
    pub momcr: MOMCR,
    _reserved38: [u8; 0x03],
    #[doc = "0x417 - Voltage Monitor Circuit Control Register"]
    pub lvcmpcr: LVCMPCR,
    #[doc = "0x418 - Voltage Detection Level Select Register"]
    pub lvdlvlr: LVDLVLR,
    _reserved40: [u8; 0x01],
    #[doc = "0x41a - Voltage Monitor %s Circuit Control Register 0"]
    pub lvdcr0: [LVDCR0; 2],
    _reserved41: [u8; 0x03],
    #[doc = "0x41f - VBATT Control Register1"]
    pub vbtcr1: VBTCR1,
    _reserved42: [u8; 0x60],
    #[doc = "0x480 - Sub-Clock Oscillator Control Register"]
    pub sosccr: SOSCCR,
    #[doc = "0x481 - Sub Clock Oscillator Mode Control Register"]
    pub somcr: SOMCR,
    _reserved44: [u8; 0x0e],
    #[doc = "0x490 - Low-Speed On-Chip Oscillator Control Register"]
    pub lococr: LOCOCR,
    _reserved45: [u8; 0x01],
    #[doc = "0x492 - LOCO User Trimming Control Register"]
    pub locoutcr: LOCOUTCR,
    _reserved46: [u8; 0x1d],
    #[doc = "0x4b0 - VBATT Control Register2"]
    pub vbtcr2: VBTCR2,
    #[doc = "0x4b1 - VBATT Status Register"]
    pub vbtsr: VBTSR,
    #[doc = "0x4b2 - VBATT Comparator Control Register"]
    pub vbtcmpcr: VBTCMPCR,
    _reserved49: [u8; 0x01],
    #[doc = "0x4b4 - VBATT Pin Low Voltage Detect Interrupt Control Register"]
    pub vbtlvdicr: VBTLVDICR,
    _reserved50: [u8; 0x01],
    #[doc = "0x4b6 - VBATT Wakeup function Control Register"]
    pub vbtwctlr: VBTWCTLR,
    _reserved51: [u8; 0x01],
    #[doc = "0x4b8 - VBATT Wakeup I/O 0 Output Trigger Select Register"]
    pub vbtwch0otsr: VBTWCH0OTSR,
    #[doc = "0x4b9 - VBATT Wakeup I/O 1 Output Trigger Select Register"]
    pub vbtwch1otsr: VBTWCH1OTSR,
    #[doc = "0x4ba - VBATT Wakeup I/O 2 Output Trigger Select Register"]
    pub vbtwch2otsr: VBTWCH2OTSR,
    #[doc = "0x4bb - VBATT Input Control Register"]
    pub vbtictlr: VBTICTLR,
    #[doc = "0x4bc - VBATT Output Control Register"]
    pub vbtoctlr: VBTOCTLR,
    #[doc = "0x4bd - VBATT Wakeup Trigger source Enable Register"]
    pub vbtwter: VBTWTER,
    #[doc = "0x4be - VBATT Wakeup Trigger source Edge Register"]
    pub vbtwegr: VBTWEGR,
    #[doc = "0x4bf - VBATT Wakeup trigger source Flag Register"]
    pub vbtwfr: VBTWFR,
    _reserved59: [u8; 0x40],
    #[doc = "0x500..0x700 - VBATT Backup Register \\[%s\\]"]
    pub vbtbkr: [VBTBKR; 512],
}
impl RegisterBlock {
    #[doc = "0x41a - Voltage Monitor 1 Circuit Control Register 0"]
    #[inline(always)]
    pub fn lvd1cr0(&self) -> &LVDCR0 {
        &self.lvdcr0[0]
    }
    #[doc = "0x41b - Voltage Monitor 2 Circuit Control Register 0"]
    #[inline(always)]
    pub fn lvd2cr0(&self) -> &LVDCR0 {
        &self.lvdcr0[1]
    }
}
#[doc = "VBTCR1 (rw) register accessor: VBATT Control Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbtcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbtcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbtcr1`]
module"]
pub type VBTCR1 = crate::Reg<vbtcr1::VBTCR1_SPEC>;
#[doc = "VBATT Control Register1"]
pub mod vbtcr1;
#[doc = "VBTCR2 (rw) register accessor: VBATT Control Register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbtcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbtcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbtcr2`]
module"]
pub type VBTCR2 = crate::Reg<vbtcr2::VBTCR2_SPEC>;
#[doc = "VBATT Control Register2"]
pub mod vbtcr2;
#[doc = "VBTSR (rw) register accessor: VBATT Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbtsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbtsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbtsr`]
module"]
pub type VBTSR = crate::Reg<vbtsr::VBTSR_SPEC>;
#[doc = "VBATT Status Register"]
pub mod vbtsr;
#[doc = "VBTCMPCR (rw) register accessor: VBATT Comparator Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbtcmpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbtcmpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbtcmpcr`]
module"]
pub type VBTCMPCR = crate::Reg<vbtcmpcr::VBTCMPCR_SPEC>;
#[doc = "VBATT Comparator Control Register"]
pub mod vbtcmpcr;
#[doc = "VBTLVDICR (rw) register accessor: VBATT Pin Low Voltage Detect Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbtlvdicr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbtlvdicr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbtlvdicr`]
module"]
pub type VBTLVDICR = crate::Reg<vbtlvdicr::VBTLVDICR_SPEC>;
#[doc = "VBATT Pin Low Voltage Detect Interrupt Control Register"]
pub mod vbtlvdicr;
#[doc = "VBTWCTLR (rw) register accessor: VBATT Wakeup function Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbtwctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbtwctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbtwctlr`]
module"]
pub type VBTWCTLR = crate::Reg<vbtwctlr::VBTWCTLR_SPEC>;
#[doc = "VBATT Wakeup function Control Register"]
pub mod vbtwctlr;
#[doc = "VBTWCH0OTSR (rw) register accessor: VBATT Wakeup I/O 0 Output Trigger Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbtwch0otsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbtwch0otsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbtwch0otsr`]
module"]
pub type VBTWCH0OTSR = crate::Reg<vbtwch0otsr::VBTWCH0OTSR_SPEC>;
#[doc = "VBATT Wakeup I/O 0 Output Trigger Select Register"]
pub mod vbtwch0otsr;
#[doc = "VBTWCH1OTSR (rw) register accessor: VBATT Wakeup I/O 1 Output Trigger Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbtwch1otsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbtwch1otsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbtwch1otsr`]
module"]
pub type VBTWCH1OTSR = crate::Reg<vbtwch1otsr::VBTWCH1OTSR_SPEC>;
#[doc = "VBATT Wakeup I/O 1 Output Trigger Select Register"]
pub mod vbtwch1otsr;
#[doc = "VBTWCH2OTSR (rw) register accessor: VBATT Wakeup I/O 2 Output Trigger Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbtwch2otsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbtwch2otsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbtwch2otsr`]
module"]
pub type VBTWCH2OTSR = crate::Reg<vbtwch2otsr::VBTWCH2OTSR_SPEC>;
#[doc = "VBATT Wakeup I/O 2 Output Trigger Select Register"]
pub mod vbtwch2otsr;
#[doc = "VBTICTLR (rw) register accessor: VBATT Input Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbtictlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbtictlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbtictlr`]
module"]
pub type VBTICTLR = crate::Reg<vbtictlr::VBTICTLR_SPEC>;
#[doc = "VBATT Input Control Register"]
pub mod vbtictlr;
#[doc = "VBTOCTLR (rw) register accessor: VBATT Output Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbtoctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbtoctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbtoctlr`]
module"]
pub type VBTOCTLR = crate::Reg<vbtoctlr::VBTOCTLR_SPEC>;
#[doc = "VBATT Output Control Register"]
pub mod vbtoctlr;
#[doc = "VBTWTER (rw) register accessor: VBATT Wakeup Trigger source Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbtwter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbtwter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbtwter`]
module"]
pub type VBTWTER = crate::Reg<vbtwter::VBTWTER_SPEC>;
#[doc = "VBATT Wakeup Trigger source Enable Register"]
pub mod vbtwter;
#[doc = "VBTWEGR (rw) register accessor: VBATT Wakeup Trigger source Edge Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbtwegr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbtwegr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbtwegr`]
module"]
pub type VBTWEGR = crate::Reg<vbtwegr::VBTWEGR_SPEC>;
#[doc = "VBATT Wakeup Trigger source Edge Register"]
pub mod vbtwegr;
#[doc = "VBTWFR (rw) register accessor: VBATT Wakeup trigger source Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbtwfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbtwfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbtwfr`]
module"]
pub type VBTWFR = crate::Reg<vbtwfr::VBTWFR_SPEC>;
#[doc = "VBATT Wakeup trigger source Flag Register"]
pub mod vbtwfr;
#[doc = "VBTBKR (rw) register accessor: VBATT Backup Register \\[%s\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbtbkr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbtbkr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbtbkr`]
module"]
pub type VBTBKR = crate::Reg<vbtbkr::VBTBKR_SPEC>;
#[doc = "VBATT Backup Register \\[%s\\]"]
pub mod vbtbkr;
#[doc = "SCKDIVCR (rw) register accessor: System Clock Division Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sckdivcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sckdivcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sckdivcr`]
module"]
pub type SCKDIVCR = crate::Reg<sckdivcr::SCKDIVCR_SPEC>;
#[doc = "System Clock Division Control Register"]
pub mod sckdivcr;
#[doc = "SCKSCR (rw) register accessor: System Clock Source Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sckscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sckscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sckscr`]
module"]
pub type SCKSCR = crate::Reg<sckscr::SCKSCR_SPEC>;
#[doc = "System Clock Source Control Register"]
pub mod sckscr;
#[doc = "PLLCR (rw) register accessor: PLL Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllcr`]
module"]
pub type PLLCR = crate::Reg<pllcr::PLLCR_SPEC>;
#[doc = "PLL Control Register"]
pub mod pllcr;
#[doc = "PLLCCR2 (rw) register accessor: PLL Clock Control Register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllccr2`]
module"]
pub type PLLCCR2 = crate::Reg<pllccr2::PLLCCR2_SPEC>;
#[doc = "PLL Clock Control Register2"]
pub mod pllccr2;
#[doc = "MEMWAIT (rw) register accessor: Memory Wait Cycle Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memwait::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`memwait::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memwait`]
module"]
pub type MEMWAIT = crate::Reg<memwait::MEMWAIT_SPEC>;
#[doc = "Memory Wait Cycle Control Register"]
pub mod memwait;
#[doc = "MOSCCR (rw) register accessor: Main Clock Oscillator Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mosccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mosccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mosccr`]
module"]
pub type MOSCCR = crate::Reg<mosccr::MOSCCR_SPEC>;
#[doc = "Main Clock Oscillator Control Register"]
pub mod mosccr;
#[doc = "HOCOCR (rw) register accessor: High-Speed On-Chip Oscillator Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hococr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hococr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hococr`]
module"]
pub type HOCOCR = crate::Reg<hococr::HOCOCR_SPEC>;
#[doc = "High-Speed On-Chip Oscillator Control Register"]
pub mod hococr;
#[doc = "MOCOCR (rw) register accessor: Middle-Speed On-Chip Oscillator Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mococr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mococr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mococr`]
module"]
pub type MOCOCR = crate::Reg<mococr::MOCOCR_SPEC>;
#[doc = "Middle-Speed On-Chip Oscillator Control Register"]
pub mod mococr;
#[doc = "OSCSF (r) register accessor: Oscillation Stabilization Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oscsf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oscsf`]
module"]
pub type OSCSF = crate::Reg<oscsf::OSCSF_SPEC>;
#[doc = "Oscillation Stabilization Flag Register"]
pub mod oscsf;
#[doc = "CKOCR (rw) register accessor: Clock Out Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckocr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckocr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckocr`]
module"]
pub type CKOCR = crate::Reg<ckocr::CKOCR_SPEC>;
#[doc = "Clock Out Control Register"]
pub mod ckocr;
#[doc = "TRCKCR (rw) register accessor: Trace Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trckcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trckcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trckcr`]
module"]
pub type TRCKCR = crate::Reg<trckcr::TRCKCR_SPEC>;
#[doc = "Trace Clock Control Register"]
pub mod trckcr;
#[doc = "OSTDCR (rw) register accessor: Oscillation Stop Detection Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ostdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ostdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ostdcr`]
module"]
pub type OSTDCR = crate::Reg<ostdcr::OSTDCR_SPEC>;
#[doc = "Oscillation Stop Detection Control Register"]
pub mod ostdcr;
#[doc = "OSTDSR (rw) register accessor: Oscillation Stop Detection Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ostdsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ostdsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ostdsr`]
module"]
pub type OSTDSR = crate::Reg<ostdsr::OSTDSR_SPEC>;
#[doc = "Oscillation Stop Detection Status Register"]
pub mod ostdsr;
#[doc = "SLCDSCKCR (rw) register accessor: Segment LCD Source Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slcdsckcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slcdsckcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slcdsckcr`]
module"]
pub type SLCDSCKCR = crate::Reg<slcdsckcr::SLCDSCKCR_SPEC>;
#[doc = "Segment LCD Source Clock Control Register"]
pub mod slcdsckcr;
#[doc = "MOCOUTCR (rw) register accessor: MOCO User Trimming Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mocoutcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mocoutcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mocoutcr`]
module"]
pub type MOCOUTCR = crate::Reg<mocoutcr::MOCOUTCR_SPEC>;
#[doc = "MOCO User Trimming Control Register"]
pub mod mocoutcr;
#[doc = "HOCOUTCR (rw) register accessor: HOCO User Trimming Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hocoutcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hocoutcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hocoutcr`]
module"]
pub type HOCOUTCR = crate::Reg<hocoutcr::HOCOUTCR_SPEC>;
#[doc = "HOCO User Trimming Control Register"]
pub mod hocoutcr;
#[doc = "MOSCWTCR (rw) register accessor: Main Clock Oscillator Wait Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`moscwtcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`moscwtcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moscwtcr`]
module"]
pub type MOSCWTCR = crate::Reg<moscwtcr::MOSCWTCR_SPEC>;
#[doc = "Main Clock Oscillator Wait Control Register"]
pub mod moscwtcr;
#[doc = "HOCOWTCR (rw) register accessor: High-Speed On-Chip Oscillator Wait Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hocowtcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hocowtcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hocowtcr`]
module"]
pub type HOCOWTCR = crate::Reg<hocowtcr::HOCOWTCR_SPEC>;
#[doc = "High-Speed On-Chip Oscillator Wait Control Register"]
pub mod hocowtcr;
#[doc = "USBCKCR (rw) register accessor: USB Clock Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbckcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbckcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbckcr`]
module"]
pub type USBCKCR = crate::Reg<usbckcr::USBCKCR_SPEC>;
#[doc = "USB Clock Control register"]
pub mod usbckcr;
#[doc = "MOMCR (rw) register accessor: Main Clock Oscillator Mode Oscillation Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`momcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`momcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@momcr`]
module"]
pub type MOMCR = crate::Reg<momcr::MOMCR_SPEC>;
#[doc = "Main Clock Oscillator Mode Oscillation Control Register"]
pub mod momcr;
#[doc = "SOSCCR (rw) register accessor: Sub-Clock Oscillator Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sosccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sosccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sosccr`]
module"]
pub type SOSCCR = crate::Reg<sosccr::SOSCCR_SPEC>;
#[doc = "Sub-Clock Oscillator Control Register"]
pub mod sosccr;
#[doc = "SOMCR (rw) register accessor: Sub Clock Oscillator Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`somcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`somcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@somcr`]
module"]
pub type SOMCR = crate::Reg<somcr::SOMCR_SPEC>;
#[doc = "Sub Clock Oscillator Mode Control Register"]
pub mod somcr;
#[doc = "LOCOCR (rw) register accessor: Low-Speed On-Chip Oscillator Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lococr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lococr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lococr`]
module"]
pub type LOCOCR = crate::Reg<lococr::LOCOCR_SPEC>;
#[doc = "Low-Speed On-Chip Oscillator Control Register"]
pub mod lococr;
#[doc = "LOCOUTCR (rw) register accessor: LOCO User Trimming Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`locoutcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`locoutcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@locoutcr`]
module"]
pub type LOCOUTCR = crate::Reg<locoutcr::LOCOUTCR_SPEC>;
#[doc = "LOCO User Trimming Control Register"]
pub mod locoutcr;
#[doc = "SBYCR (rw) register accessor: Standby Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sbycr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sbycr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbycr`]
module"]
pub type SBYCR = crate::Reg<sbycr::SBYCR_SPEC>;
#[doc = "Standby Control Register"]
pub mod sbycr;
#[doc = "MSTPCRA (rw) register accessor: Module Stop Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mstpcra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mstpcra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mstpcra`]
module"]
pub type MSTPCRA = crate::Reg<mstpcra::MSTPCRA_SPEC>;
#[doc = "Module Stop Control Register A"]
pub mod mstpcra;
#[doc = "SNZCR (rw) register accessor: Snooze Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`snzcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`snzcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snzcr`]
module"]
pub type SNZCR = crate::Reg<snzcr::SNZCR_SPEC>;
#[doc = "Snooze Control Register"]
pub mod snzcr;
#[doc = "SNZEDCR (rw) register accessor: Snooze End Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`snzedcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`snzedcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snzedcr`]
module"]
pub type SNZEDCR = crate::Reg<snzedcr::SNZEDCR_SPEC>;
#[doc = "Snooze End Control Register"]
pub mod snzedcr;
#[doc = "SNZREQCR (rw) register accessor: Snooze Request Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`snzreqcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`snzreqcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snzreqcr`]
module"]
pub type SNZREQCR = crate::Reg<snzreqcr::SNZREQCR_SPEC>;
#[doc = "Snooze Request Control Register"]
pub mod snzreqcr;
#[doc = "FLSTOP (rw) register accessor: Flash Operation Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flstop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flstop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flstop`]
module"]
pub type FLSTOP = crate::Reg<flstop::FLSTOP_SPEC>;
#[doc = "Flash Operation Control Register"]
pub mod flstop;
#[doc = "OPCCR (rw) register accessor: Operating Power Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opccr`]
module"]
pub type OPCCR = crate::Reg<opccr::OPCCR_SPEC>;
#[doc = "Operating Power Control Register"]
pub mod opccr;
#[doc = "SOPCCR (rw) register accessor: Sub Operating Power Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sopccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sopccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sopccr`]
module"]
pub type SOPCCR = crate::Reg<sopccr::SOPCCR_SPEC>;
#[doc = "Sub Operating Power Control Register"]
pub mod sopccr;
#[doc = "SYOCDCR (rw) register accessor: System Control OCD Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syocdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syocdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syocdcr`]
module"]
pub type SYOCDCR = crate::Reg<syocdcr::SYOCDCR_SPEC>;
#[doc = "System Control OCD Control Register"]
pub mod syocdcr;
#[doc = "LVCMPCR (rw) register accessor: Voltage Monitor Circuit Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lvcmpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lvcmpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvcmpcr`]
module"]
pub type LVCMPCR = crate::Reg<lvcmpcr::LVCMPCR_SPEC>;
#[doc = "Voltage Monitor Circuit Control Register"]
pub mod lvcmpcr;
#[doc = "LVDLVLR (rw) register accessor: Voltage Detection Level Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lvdlvlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lvdlvlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvdlvlr`]
module"]
pub type LVDLVLR = crate::Reg<lvdlvlr::LVDLVLR_SPEC>;
#[doc = "Voltage Detection Level Select Register"]
pub mod lvdlvlr;
#[doc = "LVDCR0 (rw) register accessor: Voltage Monitor %s Circuit Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lvdcr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lvdcr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvdcr0`]
module"]
pub type LVDCR0 = crate::Reg<lvdcr0::LVDCR0_SPEC>;
#[doc = "Voltage Monitor %s Circuit Control Register 0"]
pub mod lvdcr0;
#[doc = "LVDCR1 (rw) register accessor: Voltage Monitor %s Circuit Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lvdcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lvdcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvdcr1`]
module"]
pub type LVDCR1 = crate::Reg<lvdcr1::LVDCR1_SPEC>;
#[doc = "Voltage Monitor %s Circuit Control Register 1"]
pub mod lvdcr1;
#[doc = "LVDSR (rw) register accessor: Voltage Monitor %s Circuit Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lvdsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lvdsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvdsr`]
module"]
pub type LVDSR = crate::Reg<lvdsr::LVDSR_SPEC>;
#[doc = "Voltage Monitor %s Circuit Status Register"]
pub mod lvdsr;
#[doc = "PRCR (rw) register accessor: Protect Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prcr`]
module"]
pub type PRCR = crate::Reg<prcr::PRCR_SPEC>;
#[doc = "Protect Register"]
pub mod prcr;
#[doc = "RSTSR0 (rw) register accessor: Reset Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstsr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstsr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstsr0`]
module"]
pub type RSTSR0 = crate::Reg<rstsr0::RSTSR0_SPEC>;
#[doc = "Reset Status Register 0"]
pub mod rstsr0;
#[doc = "RSTSR2 (rw) register accessor: Reset Status Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstsr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstsr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstsr2`]
module"]
pub type RSTSR2 = crate::Reg<rstsr2::RSTSR2_SPEC>;
#[doc = "Reset Status Register 2"]
pub mod rstsr2;
#[doc = "RSTSR1 (rw) register accessor: Reset Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstsr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstsr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstsr1`]
module"]
pub type RSTSR1 = crate::Reg<rstsr1::RSTSR1_SPEC>;
#[doc = "Reset Status Register 1"]
pub mod rstsr1;
#[doc = "BKRACR (rw) register accessor: Backup Register Access Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkracr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkracr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkracr`]
module"]
pub type BKRACR = crate::Reg<bkracr::BKRACR_SPEC>;
#[doc = "Backup Register Access Control Register"]
pub mod bkracr;
