#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CTSU Control Register 0"]
    pub ctsucr0: CTSUCR0,
    #[doc = "0x01 - CTSU Control Register 1"]
    pub ctsucr1: CTSUCR1,
    #[doc = "0x02 - CTSU Synchronous Noise Reduction Setting Register"]
    pub ctsusdprs: CTSUSDPRS,
    #[doc = "0x03 - CTSU Sensor Stabilization Wait Control Register"]
    pub ctsusst: CTSUSST,
    #[doc = "0x04 - CTSU Measurement Channel Register 0"]
    pub ctsumch0: CTSUMCH0,
    #[doc = "0x05 - CTSU Measurement Channel Register 1"]
    pub ctsumch1: CTSUMCH1,
    #[doc = "0x06 - CTSU Channel Enable Control Register 0"]
    pub ctsuchac0: CTSUCHAC0,
    #[doc = "0x07 - CTSU Channel Enable Control Register 1"]
    pub ctsuchac1: CTSUCHAC1,
    #[doc = "0x08 - CTSU Channel Enable Control Register 2"]
    pub ctsuchac2: CTSUCHAC2,
    #[doc = "0x09 - CTSU Channel Enable Control Register 3"]
    pub ctsuchac3: CTSUCHAC3,
    #[doc = "0x0a - CTSU Channel Enable Control Register 4"]
    pub ctsuchac4: CTSUCHAC4,
    #[doc = "0x0b - CTSU Channel Transmit/Receive Control Register 0"]
    pub ctsuchtrc0: CTSUCHTRC0,
    #[doc = "0x0c - CTSU Channel Transmit/Receive Control Register 1"]
    pub ctsuchtrc1: CTSUCHTRC1,
    #[doc = "0x0d - CTSU Channel Transmit/Receive Control Register 3"]
    pub ctsuchtrc2: CTSUCHTRC2,
    #[doc = "0x0e - CTSU Channel Transmit/Receive Control Register 3"]
    pub ctsuchtrc3: CTSUCHTRC3,
    #[doc = "0x0f - CTSU Channel Transmit/Receive Control Register 4"]
    pub ctsuchtrc4: CTSUCHTRC4,
    #[doc = "0x10 - CTSU High-Pass Noise Reduction Control Register"]
    pub ctsudclkc: CTSUDCLKC,
    #[doc = "0x11 - CTSU Status Register"]
    pub ctsust: CTSUST,
    #[doc = "0x12 - CTSU High-Pass Noise Reduction Spectrum Diffusion Control Register"]
    pub ctsussc: CTSUSSC,
    #[doc = "0x14 - CTSU Sensor Offset Register 0"]
    pub ctsuso0: CTSUSO0,
    #[doc = "0x16 - CTSU Sensor Offset Register 1"]
    pub ctsuso1: CTSUSO1,
    #[doc = "0x18 - CTSU Sensor Counter"]
    pub ctsusc: CTSUSC,
    #[doc = "0x1a - CTSU Reference Counter"]
    pub ctsurc: CTSURC,
    #[doc = "0x1c - CTSU Error Status Register"]
    pub ctsuerrs: CTSUERRS,
}
#[doc = "CTSUCR0 (rw) register accessor: CTSU Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsucr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsucr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctsucr0`]
module"]
pub type CTSUCR0 = crate::Reg<ctsucr0::CTSUCR0_SPEC>;
#[doc = "CTSU Control Register 0"]
pub mod ctsucr0;
#[doc = "CTSUCR1 (rw) register accessor: CTSU Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsucr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsucr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctsucr1`]
module"]
pub type CTSUCR1 = crate::Reg<ctsucr1::CTSUCR1_SPEC>;
#[doc = "CTSU Control Register 1"]
pub mod ctsucr1;
#[doc = "CTSUSDPRS (rw) register accessor: CTSU Synchronous Noise Reduction Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsusdprs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsusdprs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctsusdprs`]
module"]
pub type CTSUSDPRS = crate::Reg<ctsusdprs::CTSUSDPRS_SPEC>;
#[doc = "CTSU Synchronous Noise Reduction Setting Register"]
pub mod ctsusdprs;
#[doc = "CTSUSST (rw) register accessor: CTSU Sensor Stabilization Wait Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsusst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsusst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctsusst`]
module"]
pub type CTSUSST = crate::Reg<ctsusst::CTSUSST_SPEC>;
#[doc = "CTSU Sensor Stabilization Wait Control Register"]
pub mod ctsusst;
#[doc = "CTSUMCH0 (rw) register accessor: CTSU Measurement Channel Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsumch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsumch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctsumch0`]
module"]
pub type CTSUMCH0 = crate::Reg<ctsumch0::CTSUMCH0_SPEC>;
#[doc = "CTSU Measurement Channel Register 0"]
pub mod ctsumch0;
#[doc = "CTSUMCH1 (rw) register accessor: CTSU Measurement Channel Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsumch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsumch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctsumch1`]
module"]
pub type CTSUMCH1 = crate::Reg<ctsumch1::CTSUMCH1_SPEC>;
#[doc = "CTSU Measurement Channel Register 1"]
pub mod ctsumch1;
#[doc = "CTSUCHAC0 (rw) register accessor: CTSU Channel Enable Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsuchac0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsuchac0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctsuchac0`]
module"]
pub type CTSUCHAC0 = crate::Reg<ctsuchac0::CTSUCHAC0_SPEC>;
#[doc = "CTSU Channel Enable Control Register 0"]
pub mod ctsuchac0;
#[doc = "CTSUCHAC1 (rw) register accessor: CTSU Channel Enable Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsuchac1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsuchac1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctsuchac1`]
module"]
pub type CTSUCHAC1 = crate::Reg<ctsuchac1::CTSUCHAC1_SPEC>;
#[doc = "CTSU Channel Enable Control Register 1"]
pub mod ctsuchac1;
#[doc = "CTSUCHAC2 (rw) register accessor: CTSU Channel Enable Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsuchac2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsuchac2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctsuchac2`]
module"]
pub type CTSUCHAC2 = crate::Reg<ctsuchac2::CTSUCHAC2_SPEC>;
#[doc = "CTSU Channel Enable Control Register 2"]
pub mod ctsuchac2;
#[doc = "CTSUCHAC3 (rw) register accessor: CTSU Channel Enable Control Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsuchac3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsuchac3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctsuchac3`]
module"]
pub type CTSUCHAC3 = crate::Reg<ctsuchac3::CTSUCHAC3_SPEC>;
#[doc = "CTSU Channel Enable Control Register 3"]
pub mod ctsuchac3;
#[doc = "CTSUCHAC4 (rw) register accessor: CTSU Channel Enable Control Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsuchac4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsuchac4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctsuchac4`]
module"]
pub type CTSUCHAC4 = crate::Reg<ctsuchac4::CTSUCHAC4_SPEC>;
#[doc = "CTSU Channel Enable Control Register 4"]
pub mod ctsuchac4;
#[doc = "CTSUCHTRC0 (rw) register accessor: CTSU Channel Transmit/Receive Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsuchtrc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsuchtrc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctsuchtrc0`]
module"]
pub type CTSUCHTRC0 = crate::Reg<ctsuchtrc0::CTSUCHTRC0_SPEC>;
#[doc = "CTSU Channel Transmit/Receive Control Register 0"]
pub mod ctsuchtrc0;
#[doc = "CTSUCHTRC1 (rw) register accessor: CTSU Channel Transmit/Receive Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsuchtrc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsuchtrc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctsuchtrc1`]
module"]
pub type CTSUCHTRC1 = crate::Reg<ctsuchtrc1::CTSUCHTRC1_SPEC>;
#[doc = "CTSU Channel Transmit/Receive Control Register 1"]
pub mod ctsuchtrc1;
#[doc = "CTSUCHTRC2 (rw) register accessor: CTSU Channel Transmit/Receive Control Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsuchtrc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsuchtrc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctsuchtrc2`]
module"]
pub type CTSUCHTRC2 = crate::Reg<ctsuchtrc2::CTSUCHTRC2_SPEC>;
#[doc = "CTSU Channel Transmit/Receive Control Register 3"]
pub mod ctsuchtrc2;
#[doc = "CTSUCHTRC3 (rw) register accessor: CTSU Channel Transmit/Receive Control Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsuchtrc3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsuchtrc3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctsuchtrc3`]
module"]
pub type CTSUCHTRC3 = crate::Reg<ctsuchtrc3::CTSUCHTRC3_SPEC>;
#[doc = "CTSU Channel Transmit/Receive Control Register 3"]
pub mod ctsuchtrc3;
#[doc = "CTSUCHTRC4 (rw) register accessor: CTSU Channel Transmit/Receive Control Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsuchtrc4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsuchtrc4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctsuchtrc4`]
module"]
pub type CTSUCHTRC4 = crate::Reg<ctsuchtrc4::CTSUCHTRC4_SPEC>;
#[doc = "CTSU Channel Transmit/Receive Control Register 4"]
pub mod ctsuchtrc4;
#[doc = "CTSUDCLKC (rw) register accessor: CTSU High-Pass Noise Reduction Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsudclkc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsudclkc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctsudclkc`]
module"]
pub type CTSUDCLKC = crate::Reg<ctsudclkc::CTSUDCLKC_SPEC>;
#[doc = "CTSU High-Pass Noise Reduction Control Register"]
pub mod ctsudclkc;
#[doc = "CTSUST (rw) register accessor: CTSU Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsust::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsust::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctsust`]
module"]
pub type CTSUST = crate::Reg<ctsust::CTSUST_SPEC>;
#[doc = "CTSU Status Register"]
pub mod ctsust;
#[doc = "CTSUSSC (rw) register accessor: CTSU High-Pass Noise Reduction Spectrum Diffusion Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsussc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsussc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctsussc`]
module"]
pub type CTSUSSC = crate::Reg<ctsussc::CTSUSSC_SPEC>;
#[doc = "CTSU High-Pass Noise Reduction Spectrum Diffusion Control Register"]
pub mod ctsussc;
#[doc = "CTSUSO0 (rw) register accessor: CTSU Sensor Offset Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsuso0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsuso0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctsuso0`]
module"]
pub type CTSUSO0 = crate::Reg<ctsuso0::CTSUSO0_SPEC>;
#[doc = "CTSU Sensor Offset Register 0"]
pub mod ctsuso0;
#[doc = "CTSUSO1 (rw) register accessor: CTSU Sensor Offset Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsuso1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsuso1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctsuso1`]
module"]
pub type CTSUSO1 = crate::Reg<ctsuso1::CTSUSO1_SPEC>;
#[doc = "CTSU Sensor Offset Register 1"]
pub mod ctsuso1;
#[doc = "CTSUSC (r) register accessor: CTSU Sensor Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsusc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctsusc`]
module"]
pub type CTSUSC = crate::Reg<ctsusc::CTSUSC_SPEC>;
#[doc = "CTSU Sensor Counter"]
pub mod ctsusc;
#[doc = "CTSURC (r) register accessor: CTSU Reference Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsurc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctsurc`]
module"]
pub type CTSURC = crate::Reg<ctsurc::CTSURC_SPEC>;
#[doc = "CTSU Reference Counter"]
pub mod ctsurc;
#[doc = "CTSUERRS (r) register accessor: CTSU Error Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsuerrs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctsuerrs`]
module"]
pub type CTSUERRS = crate::Reg<ctsuerrs::CTSUERRS_SPEC>;
#[doc = "CTSU Error Status Register"]
pub mod ctsuerrs;
