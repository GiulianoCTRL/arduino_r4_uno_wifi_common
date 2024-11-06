#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - A/D Control Register"]
    pub adcsr: ADCSR,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - A/D Channel Select Register A0"]
    pub adansa0: ADANSA0,
    #[doc = "0x06 - A/D Channel Select Register A1"]
    pub adansa1: ADANSA1,
    #[doc = "0x08 - A/D-Converted Value Addition/Average Channel Select Register 0"]
    pub adads0: ADADS0,
    #[doc = "0x0a - A/D-Converted Value Addition/Average Channel Select Register 1"]
    pub adads1: ADADS1,
    #[doc = "0x0c - A/D-Converted Value Addition/Average Count Select Register"]
    pub adadc: ADADC,
    _reserved6: [u8; 0x01],
    #[doc = "0x0e - A/D Control Extended Register"]
    pub adcer: ADCER,
    #[doc = "0x10 - A/D Conversion Start Trigger Select Register"]
    pub adstrgr: ADSTRGR,
    #[doc = "0x12 - A/D Conversion Extended Input Control Register"]
    pub adexicr: ADEXICR,
    #[doc = "0x14 - A/D Channel Select Register B0"]
    pub adansb0: ADANSB0,
    #[doc = "0x16 - A/D Channel Select Register B1"]
    pub adansb1: ADANSB1,
    #[doc = "0x18 - A/D Data Duplication Register"]
    pub addbldr: ADDBLDR,
    #[doc = "0x1a - A/D Temperature Sensor Data Register"]
    pub adtsdr: ADTSDR,
    #[doc = "0x1c - A/D Internal Reference Voltage Data Register"]
    pub adocdr: ADOCDR,
    #[doc = "0x1e - A/D Self-Diagnosis Data Register"]
    pub adrd: ADRD,
    #[doc = "0x20..0x3e - A/D Data Register %s"]
    pub addr: [ADDR; 15],
    _reserved16: [u8; 0x02],
    #[doc = "0x40 - A/D Data Register 16"]
    pub addr16: ADDR16,
    #[doc = "0x42 - A/D Data Register 17"]
    pub addr17: ADDR16,
    #[doc = "0x44 - A/D Data Register 18"]
    pub addr18: ADDR16,
    #[doc = "0x46 - A/D Data Register 19"]
    pub addr19: ADDR16,
    #[doc = "0x48 - A/D Data Register 20"]
    pub addr20: ADDR16,
    #[doc = "0x4a - A/D Data Register 21"]
    pub addr21: ADDR16,
    #[doc = "0x4c - A/D Data Register 22"]
    pub addr22: ADDR16,
    #[doc = "0x4e - A/D Data Register 23"]
    pub addr23: ADDR16,
    #[doc = "0x50 - A/D Data Register 24"]
    pub addr24: ADDR16,
    #[doc = "0x52 - A/D Data Register 25"]
    pub addr25: ADDR16,
    _reserved26: [u8; 0x26],
    #[doc = "0x7a - A/D Disconnection Detection Control Register"]
    pub addiscr: ADDISCR,
    _reserved27: [u8; 0x05],
    #[doc = "0x80 - A/D Group Scan Priority Control Register"]
    pub adgspcr: ADGSPCR,
    _reserved28: [u8; 0x02],
    #[doc = "0x84 - A/D Data Duplexing Register A"]
    pub addbldra: ADDBLDRA,
    #[doc = "0x86 - A/D Data Duplexing Register B"]
    pub addbldrb: ADDBLDRB,
    _reserved30: [u8; 0x02],
    #[doc = "0x8a - A/D High-Potential/Low-Potential Reference Voltage Control Register"]
    pub adhvrefcnt: ADHVREFCNT,
    _reserved31: [u8; 0x01],
    #[doc = "0x8c - A/D Compare Function Window A/B Status Monitor Register"]
    pub adwinmon: ADWINMON,
    _reserved32: [u8; 0x03],
    #[doc = "0x90 - A/D Compare Function Control Register"]
    pub adcmpcr: ADCMPCR,
    #[doc = "0x92 - A/D Compare Function Window A Extended Input Select Register"]
    pub adcmpanser: ADCMPANSER,
    #[doc = "0x93 - A/D Compare Function Window A Extended Input Comparison Condition Setting Register"]
    pub adcmpler: ADCMPLER,
    #[doc = "0x94 - A/D Compare Function Window A Channel Select Register 0"]
    pub adcmpansr0: ADCMPANSR0,
    #[doc = "0x96 - A/D Compare Function Window A Channel Select Register 1"]
    pub adcmpansr1: ADCMPANSR1,
    #[doc = "0x98 - A/D Compare Function Window A Comparison Condition Setting Register 0"]
    pub adcmplr0: ADCMPLR0,
    #[doc = "0x9a - A/D Compare Function Window A Comparison Condition Setting Register 1"]
    pub adcmplr1: ADCMPLR1,
    #[doc = "0x9c - A/D Compare Function Window A Lower-Side Level Setting Register"]
    pub adcmpdr0: ADCMPDR0,
    #[doc = "0x9e - A/D Compare Function Window A Upper-Side Level Setting Register"]
    pub adcmpdr1: ADCMPDR1,
    #[doc = "0xa0 - A/D Compare Function Window A Channel Status Register 0"]
    pub adcmpsr0: ADCMPSR0,
    #[doc = "0xa2 - A/D Compare Function Window A Channel Status Register 1"]
    pub adcmpsr1: ADCMPSR1,
    #[doc = "0xa4 - A/D Compare Function Window A Extended Input Channel Status Register"]
    pub adcmpser: ADCMPSER,
    _reserved44: [u8; 0x01],
    #[doc = "0xa6 - A/D Compare Function Window B Channel Selection Register"]
    pub adcmpbnsr: ADCMPBNSR,
    _reserved45: [u8; 0x01],
    #[doc = "0xa8 - A/D Compare Function Window B Lower-Side Level Setting Register"]
    pub adwinllb: ADWINLLB,
    #[doc = "0xaa - A/D Compare Function Window B Upper-Side Level Setting Register"]
    pub adwinulb: ADWINULB,
    #[doc = "0xac - A/D Compare Function Window B Status Register"]
    pub adcmpbsr: ADCMPBSR,
    _reserved48: [u8; 0x30],
    #[doc = "0xdd - A/D Sampling State Register L"]
    pub adsstrl: ADSSTRL,
    #[doc = "0xde - A/D Sampling State Register T"]
    pub adsstrt: ADSSTRT,
    #[doc = "0xdf - A/D Sampling State Register O"]
    pub adsstro: ADSSTRO,
    #[doc = "0xe0..0xef - A/D Sampling State Register %s"]
    pub adsstr: [ADSSTR; 15],
}
#[doc = "ADCSR (rw) register accessor: A/D Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsr`]
module"]
pub type ADCSR = crate::Reg<adcsr::ADCSR_SPEC>;
#[doc = "A/D Control Register"]
pub mod adcsr;
#[doc = "ADANSA0 (rw) register accessor: A/D Channel Select Register A0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adansa0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adansa0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adansa0`]
module"]
pub type ADANSA0 = crate::Reg<adansa0::ADANSA0_SPEC>;
#[doc = "A/D Channel Select Register A0"]
pub mod adansa0;
#[doc = "ADANSA1 (rw) register accessor: A/D Channel Select Register A1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adansa1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adansa1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adansa1`]
module"]
pub type ADANSA1 = crate::Reg<adansa1::ADANSA1_SPEC>;
#[doc = "A/D Channel Select Register A1"]
pub mod adansa1;
#[doc = "ADADS0 (rw) register accessor: A/D-Converted Value Addition/Average Channel Select Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adads0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adads0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adads0`]
module"]
pub type ADADS0 = crate::Reg<adads0::ADADS0_SPEC>;
#[doc = "A/D-Converted Value Addition/Average Channel Select Register 0"]
pub mod adads0;
#[doc = "ADADS1 (rw) register accessor: A/D-Converted Value Addition/Average Channel Select Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adads1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adads1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adads1`]
module"]
pub type ADADS1 = crate::Reg<adads1::ADADS1_SPEC>;
#[doc = "A/D-Converted Value Addition/Average Channel Select Register 1"]
pub mod adads1;
#[doc = "ADADC (rw) register accessor: A/D-Converted Value Addition/Average Count Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adadc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adadc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adadc`]
module"]
pub type ADADC = crate::Reg<adadc::ADADC_SPEC>;
#[doc = "A/D-Converted Value Addition/Average Count Select Register"]
pub mod adadc;
#[doc = "ADCER (rw) register accessor: A/D Control Extended Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcer`]
module"]
pub type ADCER = crate::Reg<adcer::ADCER_SPEC>;
#[doc = "A/D Control Extended Register"]
pub mod adcer;
#[doc = "ADSTRGR (rw) register accessor: A/D Conversion Start Trigger Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adstrgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adstrgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adstrgr`]
module"]
pub type ADSTRGR = crate::Reg<adstrgr::ADSTRGR_SPEC>;
#[doc = "A/D Conversion Start Trigger Select Register"]
pub mod adstrgr;
#[doc = "ADEXICR (rw) register accessor: A/D Conversion Extended Input Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adexicr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adexicr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adexicr`]
module"]
pub type ADEXICR = crate::Reg<adexicr::ADEXICR_SPEC>;
#[doc = "A/D Conversion Extended Input Control Register"]
pub mod adexicr;
#[doc = "ADANSB0 (rw) register accessor: A/D Channel Select Register B0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adansb0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adansb0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adansb0`]
module"]
pub type ADANSB0 = crate::Reg<adansb0::ADANSB0_SPEC>;
#[doc = "A/D Channel Select Register B0"]
pub mod adansb0;
#[doc = "ADANSB1 (rw) register accessor: A/D Channel Select Register B1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adansb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adansb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adansb1`]
module"]
pub type ADANSB1 = crate::Reg<adansb1::ADANSB1_SPEC>;
#[doc = "A/D Channel Select Register B1"]
pub mod adansb1;
#[doc = "ADDBLDR (r) register accessor: A/D Data Duplication Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addbldr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addbldr`]
module"]
pub type ADDBLDR = crate::Reg<addbldr::ADDBLDR_SPEC>;
#[doc = "A/D Data Duplication Register"]
pub mod addbldr;
#[doc = "ADTSDR (r) register accessor: A/D Temperature Sensor Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adtsdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adtsdr`]
module"]
pub type ADTSDR = crate::Reg<adtsdr::ADTSDR_SPEC>;
#[doc = "A/D Temperature Sensor Data Register"]
pub mod adtsdr;
#[doc = "ADOCDR (r) register accessor: A/D Internal Reference Voltage Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adocdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adocdr`]
module"]
pub type ADOCDR = crate::Reg<adocdr::ADOCDR_SPEC>;
#[doc = "A/D Internal Reference Voltage Data Register"]
pub mod adocdr;
#[doc = "ADRD (r) register accessor: A/D Self-Diagnosis Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adrd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adrd`]
module"]
pub type ADRD = crate::Reg<adrd::ADRD_SPEC>;
#[doc = "A/D Self-Diagnosis Data Register"]
pub mod adrd;
#[doc = "ADDR (r) register accessor: A/D Data Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "A/D Data Register %s"]
pub mod addr;
pub use addr as addr16;
pub use ADDR as ADDR16;
#[doc = "ADDISCR (rw) register accessor: A/D Disconnection Detection Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addiscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addiscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addiscr`]
module"]
pub type ADDISCR = crate::Reg<addiscr::ADDISCR_SPEC>;
#[doc = "A/D Disconnection Detection Control Register"]
pub mod addiscr;
#[doc = "ADGSPCR (rw) register accessor: A/D Group Scan Priority Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adgspcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adgspcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adgspcr`]
module"]
pub type ADGSPCR = crate::Reg<adgspcr::ADGSPCR_SPEC>;
#[doc = "A/D Group Scan Priority Control Register"]
pub mod adgspcr;
#[doc = "ADDBLDRA (r) register accessor: A/D Data Duplexing Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addbldra::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addbldra`]
module"]
pub type ADDBLDRA = crate::Reg<addbldra::ADDBLDRA_SPEC>;
#[doc = "A/D Data Duplexing Register A"]
pub mod addbldra;
#[doc = "ADDBLDRB (r) register accessor: A/D Data Duplexing Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addbldrb::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addbldrb`]
module"]
pub type ADDBLDRB = crate::Reg<addbldrb::ADDBLDRB_SPEC>;
#[doc = "A/D Data Duplexing Register B"]
pub mod addbldrb;
#[doc = "ADHVREFCNT (rw) register accessor: A/D High-Potential/Low-Potential Reference Voltage Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adhvrefcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adhvrefcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adhvrefcnt`]
module"]
pub type ADHVREFCNT = crate::Reg<adhvrefcnt::ADHVREFCNT_SPEC>;
#[doc = "A/D High-Potential/Low-Potential Reference Voltage Control Register"]
pub mod adhvrefcnt;
#[doc = "ADWINMON (r) register accessor: A/D Compare Function Window A/B Status Monitor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adwinmon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adwinmon`]
module"]
pub type ADWINMON = crate::Reg<adwinmon::ADWINMON_SPEC>;
#[doc = "A/D Compare Function Window A/B Status Monitor Register"]
pub mod adwinmon;
#[doc = "ADCMPCR (rw) register accessor: A/D Compare Function Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmpcr`]
module"]
pub type ADCMPCR = crate::Reg<adcmpcr::ADCMPCR_SPEC>;
#[doc = "A/D Compare Function Control Register"]
pub mod adcmpcr;
#[doc = "ADCMPANSER (rw) register accessor: A/D Compare Function Window A Extended Input Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmpanser::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmpanser::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmpanser`]
module"]
pub type ADCMPANSER = crate::Reg<adcmpanser::ADCMPANSER_SPEC>;
#[doc = "A/D Compare Function Window A Extended Input Select Register"]
pub mod adcmpanser;
#[doc = "ADCMPLER (rw) register accessor: A/D Compare Function Window A Extended Input Comparison Condition Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmpler::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmpler::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmpler`]
module"]
pub type ADCMPLER = crate::Reg<adcmpler::ADCMPLER_SPEC>;
#[doc = "A/D Compare Function Window A Extended Input Comparison Condition Setting Register"]
pub mod adcmpler;
#[doc = "ADCMPANSR0 (rw) register accessor: A/D Compare Function Window A Channel Select Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmpansr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmpansr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmpansr0`]
module"]
pub type ADCMPANSR0 = crate::Reg<adcmpansr0::ADCMPANSR0_SPEC>;
#[doc = "A/D Compare Function Window A Channel Select Register 0"]
pub mod adcmpansr0;
#[doc = "ADCMPANSR1 (rw) register accessor: A/D Compare Function Window A Channel Select Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmpansr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmpansr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmpansr1`]
module"]
pub type ADCMPANSR1 = crate::Reg<adcmpansr1::ADCMPANSR1_SPEC>;
#[doc = "A/D Compare Function Window A Channel Select Register 1"]
pub mod adcmpansr1;
#[doc = "ADCMPLR0 (rw) register accessor: A/D Compare Function Window A Comparison Condition Setting Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmplr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmplr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmplr0`]
module"]
pub type ADCMPLR0 = crate::Reg<adcmplr0::ADCMPLR0_SPEC>;
#[doc = "A/D Compare Function Window A Comparison Condition Setting Register 0"]
pub mod adcmplr0;
#[doc = "ADCMPLR1 (rw) register accessor: A/D Compare Function Window A Comparison Condition Setting Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmplr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmplr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmplr1`]
module"]
pub type ADCMPLR1 = crate::Reg<adcmplr1::ADCMPLR1_SPEC>;
#[doc = "A/D Compare Function Window A Comparison Condition Setting Register 1"]
pub mod adcmplr1;
#[doc = "ADCMPDR0 (rw) register accessor: A/D Compare Function Window A Lower-Side Level Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmpdr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmpdr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmpdr0`]
module"]
pub type ADCMPDR0 = crate::Reg<adcmpdr0::ADCMPDR0_SPEC>;
#[doc = "A/D Compare Function Window A Lower-Side Level Setting Register"]
pub mod adcmpdr0;
#[doc = "ADCMPDR1 (rw) register accessor: A/D Compare Function Window A Upper-Side Level Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmpdr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmpdr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmpdr1`]
module"]
pub type ADCMPDR1 = crate::Reg<adcmpdr1::ADCMPDR1_SPEC>;
#[doc = "A/D Compare Function Window A Upper-Side Level Setting Register"]
pub mod adcmpdr1;
#[doc = "ADCMPSR0 (rw) register accessor: A/D Compare Function Window A Channel Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmpsr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmpsr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmpsr0`]
module"]
pub type ADCMPSR0 = crate::Reg<adcmpsr0::ADCMPSR0_SPEC>;
#[doc = "A/D Compare Function Window A Channel Status Register 0"]
pub mod adcmpsr0;
#[doc = "ADCMPSR1 (rw) register accessor: A/D Compare Function Window A Channel Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmpsr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmpsr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmpsr1`]
module"]
pub type ADCMPSR1 = crate::Reg<adcmpsr1::ADCMPSR1_SPEC>;
#[doc = "A/D Compare Function Window A Channel Status Register 1"]
pub mod adcmpsr1;
#[doc = "ADCMPSER (rw) register accessor: A/D Compare Function Window A Extended Input Channel Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmpser::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmpser::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmpser`]
module"]
pub type ADCMPSER = crate::Reg<adcmpser::ADCMPSER_SPEC>;
#[doc = "A/D Compare Function Window A Extended Input Channel Status Register"]
pub mod adcmpser;
#[doc = "ADCMPBNSR (rw) register accessor: A/D Compare Function Window B Channel Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmpbnsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmpbnsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmpbnsr`]
module"]
pub type ADCMPBNSR = crate::Reg<adcmpbnsr::ADCMPBNSR_SPEC>;
#[doc = "A/D Compare Function Window B Channel Selection Register"]
pub mod adcmpbnsr;
#[doc = "ADWINLLB (rw) register accessor: A/D Compare Function Window B Lower-Side Level Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adwinllb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adwinllb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adwinllb`]
module"]
pub type ADWINLLB = crate::Reg<adwinllb::ADWINLLB_SPEC>;
#[doc = "A/D Compare Function Window B Lower-Side Level Setting Register"]
pub mod adwinllb;
#[doc = "ADWINULB (rw) register accessor: A/D Compare Function Window B Upper-Side Level Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adwinulb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adwinulb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adwinulb`]
module"]
pub type ADWINULB = crate::Reg<adwinulb::ADWINULB_SPEC>;
#[doc = "A/D Compare Function Window B Upper-Side Level Setting Register"]
pub mod adwinulb;
#[doc = "ADCMPBSR (rw) register accessor: A/D Compare Function Window B Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmpbsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmpbsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmpbsr`]
module"]
pub type ADCMPBSR = crate::Reg<adcmpbsr::ADCMPBSR_SPEC>;
#[doc = "A/D Compare Function Window B Status Register"]
pub mod adcmpbsr;
#[doc = "ADSSTRL (rw) register accessor: A/D Sampling State Register L\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adsstrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adsstrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adsstrl`]
module"]
pub type ADSSTRL = crate::Reg<adsstrl::ADSSTRL_SPEC>;
#[doc = "A/D Sampling State Register L"]
pub mod adsstrl;
#[doc = "ADSSTRT (rw) register accessor: A/D Sampling State Register T\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adsstrt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adsstrt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adsstrt`]
module"]
pub type ADSSTRT = crate::Reg<adsstrt::ADSSTRT_SPEC>;
#[doc = "A/D Sampling State Register T"]
pub mod adsstrt;
#[doc = "ADSSTRO (rw) register accessor: A/D Sampling State Register O\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adsstro::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adsstro::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adsstro`]
module"]
pub type ADSSTRO = crate::Reg<adsstro::ADSSTRO_SPEC>;
#[doc = "A/D Sampling State Register O"]
pub mod adsstro;
#[doc = "ADSSTR (rw) register accessor: A/D Sampling State Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adsstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adsstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adsstr`]
module"]
pub type ADSSTR = crate::Reg<adsstr::ADSSTR_SPEC>;
#[doc = "A/D Sampling State Register %s"]
pub mod adsstr;
