#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C Bus Control Register 1"]
    pub iccr1: ICCR1,
    #[doc = "0x01 - I2C Bus Control Register 2"]
    pub iccr2: ICCR2,
    #[doc = "0x02 - I2C Bus Mode Register 1"]
    pub icmr1: ICMR1,
    #[doc = "0x03 - I2C Bus Mode Register 2"]
    pub icmr2: ICMR2,
    #[doc = "0x04 - I2C Bus Mode Register 3"]
    pub icmr3: ICMR3,
    #[doc = "0x05 - I2C Bus Function Enable Register"]
    pub icfer: ICFER,
    #[doc = "0x06 - I2C Bus Status Enable Register"]
    pub icser: ICSER,
    #[doc = "0x07 - I2C Bus Interrupt Enable Register"]
    pub icier: ICIER,
    #[doc = "0x08 - I2C Bus Status Register 1"]
    pub icsr1: ICSR1,
    #[doc = "0x09 - I2C Bus Status Register 2"]
    pub icsr2: ICSR2,
    #[doc = "0x0a - Slave Address Register L0"]
    pub sarl0: SARL,
    #[doc = "0x0b - Slave Address Register U0"]
    pub saru0: SARU,
    #[doc = "0x0c - Slave Address Register L1"]
    pub sarl1: SARL,
    #[doc = "0x0d - Slave Address Register U1"]
    pub saru1: SARU,
    #[doc = "0x0e - Slave Address Register L2"]
    pub sarl2: SARL,
    #[doc = "0x0f - Slave Address Register U2"]
    pub saru2: SARU,
    #[doc = "0x10 - I2C Bus Bit Rate Low-Level Register"]
    pub icbrl: ICBRL,
    #[doc = "0x11 - I2C Bus Bit Rate High-Level Register"]
    pub icbrh: ICBRH,
    #[doc = "0x12 - I2C Bus Transmit Data Register"]
    pub icdrt: ICDRT,
    #[doc = "0x13 - I2C Bus Receive Data Register"]
    pub icdrr: ICDRR,
}
#[doc = "ICCR1 (rw) register accessor: I2C Bus Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iccr1`]
module"]
pub type ICCR1 = crate::Reg<iccr1::ICCR1_SPEC>;
#[doc = "I2C Bus Control Register 1"]
pub mod iccr1;
#[doc = "ICCR2 (rw) register accessor: I2C Bus Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iccr2`]
module"]
pub type ICCR2 = crate::Reg<iccr2::ICCR2_SPEC>;
#[doc = "I2C Bus Control Register 2"]
pub mod iccr2;
#[doc = "ICMR1 (rw) register accessor: I2C Bus Mode Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icmr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icmr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icmr1`]
module"]
pub type ICMR1 = crate::Reg<icmr1::ICMR1_SPEC>;
#[doc = "I2C Bus Mode Register 1"]
pub mod icmr1;
#[doc = "ICMR2 (rw) register accessor: I2C Bus Mode Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icmr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icmr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icmr2`]
module"]
pub type ICMR2 = crate::Reg<icmr2::ICMR2_SPEC>;
#[doc = "I2C Bus Mode Register 2"]
pub mod icmr2;
#[doc = "ICMR3 (rw) register accessor: I2C Bus Mode Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icmr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icmr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icmr3`]
module"]
pub type ICMR3 = crate::Reg<icmr3::ICMR3_SPEC>;
#[doc = "I2C Bus Mode Register 3"]
pub mod icmr3;
#[doc = "ICFER (rw) register accessor: I2C Bus Function Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icfer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icfer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icfer`]
module"]
pub type ICFER = crate::Reg<icfer::ICFER_SPEC>;
#[doc = "I2C Bus Function Enable Register"]
pub mod icfer;
#[doc = "ICSER (rw) register accessor: I2C Bus Status Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icser::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icser::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icser`]
module"]
pub type ICSER = crate::Reg<icser::ICSER_SPEC>;
#[doc = "I2C Bus Status Enable Register"]
pub mod icser;
#[doc = "ICIER (rw) register accessor: I2C Bus Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icier`]
module"]
pub type ICIER = crate::Reg<icier::ICIER_SPEC>;
#[doc = "I2C Bus Interrupt Enable Register"]
pub mod icier;
#[doc = "ICSR1 (rw) register accessor: I2C Bus Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icsr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icsr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icsr1`]
module"]
pub type ICSR1 = crate::Reg<icsr1::ICSR1_SPEC>;
#[doc = "I2C Bus Status Register 1"]
pub mod icsr1;
#[doc = "ICSR2 (rw) register accessor: I2C Bus Status Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icsr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icsr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icsr2`]
module"]
pub type ICSR2 = crate::Reg<icsr2::ICSR2_SPEC>;
#[doc = "I2C Bus Status Register 2"]
pub mod icsr2;
#[doc = "SARL (rw) register accessor: Slave Address Register L%s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sarl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sarl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sarl`]
module"]
pub type SARL = crate::Reg<sarl::SARL_SPEC>;
#[doc = "Slave Address Register L%s"]
pub mod sarl;
#[doc = "SARU (rw) register accessor: Slave Address Register U%s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saru::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saru::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saru`]
module"]
pub type SARU = crate::Reg<saru::SARU_SPEC>;
#[doc = "Slave Address Register U%s"]
pub mod saru;
#[doc = "ICBRL (rw) register accessor: I2C Bus Bit Rate Low-Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icbrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icbrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icbrl`]
module"]
pub type ICBRL = crate::Reg<icbrl::ICBRL_SPEC>;
#[doc = "I2C Bus Bit Rate Low-Level Register"]
pub mod icbrl;
#[doc = "ICBRH (rw) register accessor: I2C Bus Bit Rate High-Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icbrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icbrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icbrh`]
module"]
pub type ICBRH = crate::Reg<icbrh::ICBRH_SPEC>;
#[doc = "I2C Bus Bit Rate High-Level Register"]
pub mod icbrh;
#[doc = "ICDRT (rw) register accessor: I2C Bus Transmit Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icdrt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icdrt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icdrt`]
module"]
pub type ICDRT = crate::Reg<icdrt::ICDRT_SPEC>;
#[doc = "I2C Bus Transmit Data Register"]
pub mod icdrt;
#[doc = "ICDRR (r) register accessor: I2C Bus Receive Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icdrr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icdrr`]
module"]
pub type ICDRR = crate::Reg<icdrr::ICDRR_SPEC>;
#[doc = "I2C Bus Receive Data Register"]
pub mod icdrr;
