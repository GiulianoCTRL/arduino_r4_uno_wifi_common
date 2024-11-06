#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_smr: [u8; 0x01],
    #[doc = "0x01 - Bit Rate Register"]
    pub brr: BRR,
    _reserved_2_scr: [u8; 0x01],
    #[doc = "0x03 - Transmit Data Register"]
    pub tdr: TDR,
    _reserved_4_ssr: [u8; 0x01],
    #[doc = "0x05 - Receive Data Register"]
    pub rdr: RDR,
    #[doc = "0x06 - Smart Card Mode Register"]
    pub scmr: SCMR,
    #[doc = "0x07 - Serial Extended Mode Register"]
    pub semr: SEMR,
    #[doc = "0x08 - Noise Filter Setting Register"]
    pub snfr: SNFR,
    #[doc = "0x09 - I2C Mode Register 1"]
    pub simr1: SIMR1,
    #[doc = "0x0a - I2C Mode Register 2"]
    pub simr2: SIMR2,
    #[doc = "0x0b - I2C Mode Register 3"]
    pub simr3: SIMR3,
    #[doc = "0x0c - I2C Status Register"]
    pub sisr: SISR,
    #[doc = "0x0d - SPI Mode Register"]
    pub spmr: SPMR,
    _reserved_14_ftdrh: [u8; 0x02],
    _reserved_15_frdrh: [u8; 0x02],
    #[doc = "0x12 - Modulation Duty Register"]
    pub mddr: MDDR,
    #[doc = "0x13 - Data Compare Match Control Register"]
    pub dccr: DCCR,
    #[doc = "0x14 - FIFO Control Register"]
    pub fcr: FCR,
    #[doc = "0x16 - FIFO Data Count Register"]
    pub fdr: FDR,
    #[doc = "0x18 - Line Status Register"]
    pub lsr: LSR,
    #[doc = "0x1a - Compare Match Data Register"]
    pub cdr: CDR,
    #[doc = "0x1c - Serial Port Register"]
    pub sptr: SPTR,
}
impl RegisterBlock {
    #[doc = "0x00 - Serial mode register (SCMR.SMIF = 1)"]
    #[inline(always)]
    pub const fn smr_smci(&self) -> &SMR_SMCI {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - Serial Mode Register (SCMR.SMIF = 0)"]
    #[inline(always)]
    pub const fn smr(&self) -> &SMR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x02 - Serial Control Register (SCMR.SMIF =1)"]
    #[inline(always)]
    pub const fn scr_smci(&self) -> &SCR_SMCI {
        unsafe { &*(self as *const Self).cast::<u8>().add(2usize).cast() }
    }
    #[doc = "0x02 - Serial Control Register (SCMR.SMIF = 0)"]
    #[inline(always)]
    pub const fn scr(&self) -> &SCR {
        unsafe { &*(self as *const Self).cast::<u8>().add(2usize).cast() }
    }
    #[doc = "0x04 - Serial Status Register(SCMR.SMIF = 1)"]
    #[inline(always)]
    pub const fn ssr_smci(&self) -> &SSR_SMCI {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - Serial Status Register(SCMR.SMIF = 0 and FCR.FM=1)"]
    #[inline(always)]
    pub const fn ssr_fifo(&self) -> &SSR_FIFO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - Serial Status Register(SCMR.SMIF = 0 and FCR.FM=0)"]
    #[inline(always)]
    pub const fn ssr(&self) -> &SSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x0e - Transmit FIFO Data Register H"]
    #[inline(always)]
    pub const fn ftdrh(&self) -> &FTDRH {
        unsafe { &*(self as *const Self).cast::<u8>().add(14usize).cast() }
    }
    #[doc = "0x0e - Transmit FIFO Data Register HL"]
    #[inline(always)]
    pub const fn ftdrhl(&self) -> &FTDRHL {
        unsafe { &*(self as *const Self).cast::<u8>().add(14usize).cast() }
    }
    #[doc = "0x0e - Transmit 9-bit Data Register"]
    #[inline(always)]
    pub const fn tdrhl(&self) -> &TDRHL {
        unsafe { &*(self as *const Self).cast::<u8>().add(14usize).cast() }
    }
    #[doc = "0x0f - Transmit FIFO Data Register L"]
    #[inline(always)]
    pub const fn ftdrl(&self) -> &FTDRL {
        unsafe { &*(self as *const Self).cast::<u8>().add(15usize).cast() }
    }
    #[doc = "0x10 - Receive FIFO Data Register H"]
    #[inline(always)]
    pub const fn frdrh(&self) -> &FRDRH {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x10 - Receive FIFO Data Register HL"]
    #[inline(always)]
    pub const fn frdrhl(&self) -> &FRDRHL {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x10 - Receive 9-bit Data Register"]
    #[inline(always)]
    pub const fn rdrhl(&self) -> &RDRHL {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x11 - Receive FIFO Data Register L"]
    #[inline(always)]
    pub const fn frdrl(&self) -> &FRDRL {
        unsafe { &*(self as *const Self).cast::<u8>().add(17usize).cast() }
    }
}
#[doc = "SMR (rw) register accessor: Serial Mode Register (SCMR.SMIF = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smr`]
module"]
pub type SMR = crate::Reg<smr::SMR_SPEC>;
#[doc = "Serial Mode Register (SCMR.SMIF = 0)"]
pub mod smr;
#[doc = "SMR_SMCI (rw) register accessor: Serial mode register (SCMR.SMIF = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smr_smci::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smr_smci::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smr_smci`]
module"]
pub type SMR_SMCI = crate::Reg<smr_smci::SMR_SMCI_SPEC>;
#[doc = "Serial mode register (SCMR.SMIF = 1)"]
pub mod smr_smci;
#[doc = "BRR (rw) register accessor: Bit Rate Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brr`]
module"]
pub type BRR = crate::Reg<brr::BRR_SPEC>;
#[doc = "Bit Rate Register"]
pub mod brr;
#[doc = "SCR (rw) register accessor: Serial Control Register (SCMR.SMIF = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`]
module"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "Serial Control Register (SCMR.SMIF = 0)"]
pub mod scr;
#[doc = "SCR_SMCI (rw) register accessor: Serial Control Register (SCMR.SMIF =1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr_smci::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr_smci::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr_smci`]
module"]
pub type SCR_SMCI = crate::Reg<scr_smci::SCR_SMCI_SPEC>;
#[doc = "Serial Control Register (SCMR.SMIF =1)"]
pub mod scr_smci;
#[doc = "TDR (rw) register accessor: Transmit Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr`]
module"]
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
#[doc = "Transmit Data Register"]
pub mod tdr;
#[doc = "SSR (rw) register accessor: Serial Status Register(SCMR.SMIF = 0 and FCR.FM=0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssr`]
module"]
pub type SSR = crate::Reg<ssr::SSR_SPEC>;
#[doc = "Serial Status Register(SCMR.SMIF = 0 and FCR.FM=0)"]
pub mod ssr;
#[doc = "SSR_FIFO (rw) register accessor: Serial Status Register(SCMR.SMIF = 0 and FCR.FM=1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssr_fifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssr_fifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssr_fifo`]
module"]
pub type SSR_FIFO = crate::Reg<ssr_fifo::SSR_FIFO_SPEC>;
#[doc = "Serial Status Register(SCMR.SMIF = 0 and FCR.FM=1)"]
pub mod ssr_fifo;
#[doc = "SSR_SMCI (rw) register accessor: Serial Status Register(SCMR.SMIF = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssr_smci::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssr_smci::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssr_smci`]
module"]
pub type SSR_SMCI = crate::Reg<ssr_smci::SSR_SMCI_SPEC>;
#[doc = "Serial Status Register(SCMR.SMIF = 1)"]
pub mod ssr_smci;
#[doc = "RDR (r) register accessor: Receive Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdr`]
module"]
pub type RDR = crate::Reg<rdr::RDR_SPEC>;
#[doc = "Receive Data Register"]
pub mod rdr;
#[doc = "SCMR (rw) register accessor: Smart Card Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmr`]
module"]
pub type SCMR = crate::Reg<scmr::SCMR_SPEC>;
#[doc = "Smart Card Mode Register"]
pub mod scmr;
#[doc = "SEMR (rw) register accessor: Serial Extended Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`semr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`semr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@semr`]
module"]
pub type SEMR = crate::Reg<semr::SEMR_SPEC>;
#[doc = "Serial Extended Mode Register"]
pub mod semr;
#[doc = "SNFR (rw) register accessor: Noise Filter Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`snfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`snfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snfr`]
module"]
pub type SNFR = crate::Reg<snfr::SNFR_SPEC>;
#[doc = "Noise Filter Setting Register"]
pub mod snfr;
#[doc = "SIMR1 (rw) register accessor: I2C Mode Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`simr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`simr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@simr1`]
module"]
pub type SIMR1 = crate::Reg<simr1::SIMR1_SPEC>;
#[doc = "I2C Mode Register 1"]
pub mod simr1;
#[doc = "SIMR2 (rw) register accessor: I2C Mode Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`simr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`simr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@simr2`]
module"]
pub type SIMR2 = crate::Reg<simr2::SIMR2_SPEC>;
#[doc = "I2C Mode Register 2"]
pub mod simr2;
#[doc = "SIMR3 (rw) register accessor: I2C Mode Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`simr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`simr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@simr3`]
module"]
pub type SIMR3 = crate::Reg<simr3::SIMR3_SPEC>;
#[doc = "I2C Mode Register 3"]
pub mod simr3;
#[doc = "SISR (r) register accessor: I2C Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sisr`]
module"]
pub type SISR = crate::Reg<sisr::SISR_SPEC>;
#[doc = "I2C Status Register"]
pub mod sisr;
#[doc = "SPMR (rw) register accessor: SPI Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spmr`]
module"]
pub type SPMR = crate::Reg<spmr::SPMR_SPEC>;
#[doc = "SPI Mode Register"]
pub mod spmr;
#[doc = "TDRHL (rw) register accessor: Transmit 9-bit Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdrhl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdrhl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdrhl`]
module"]
pub type TDRHL = crate::Reg<tdrhl::TDRHL_SPEC>;
#[doc = "Transmit 9-bit Data Register"]
pub mod tdrhl;
#[doc = "FTDRHL (w) register accessor: Transmit FIFO Data Register HL\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftdrhl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftdrhl`]
module"]
pub type FTDRHL = crate::Reg<ftdrhl::FTDRHL_SPEC>;
#[doc = "Transmit FIFO Data Register HL"]
pub mod ftdrhl;
#[doc = "FTDRH (w) register accessor: Transmit FIFO Data Register H\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftdrh::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftdrh`]
module"]
pub type FTDRH = crate::Reg<ftdrh::FTDRH_SPEC>;
#[doc = "Transmit FIFO Data Register H"]
pub mod ftdrh;
#[doc = "FTDRL (w) register accessor: Transmit FIFO Data Register L\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftdrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftdrl`]
module"]
pub type FTDRL = crate::Reg<ftdrl::FTDRL_SPEC>;
#[doc = "Transmit FIFO Data Register L"]
pub mod ftdrl;
#[doc = "RDRHL (r) register accessor: Receive 9-bit Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdrhl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdrhl`]
module"]
pub type RDRHL = crate::Reg<rdrhl::RDRHL_SPEC>;
#[doc = "Receive 9-bit Data Register"]
pub mod rdrhl;
#[doc = "FRDRHL (r) register accessor: Receive FIFO Data Register HL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frdrhl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frdrhl`]
module"]
pub type FRDRHL = crate::Reg<frdrhl::FRDRHL_SPEC>;
#[doc = "Receive FIFO Data Register HL"]
pub mod frdrhl;
#[doc = "FRDRH (r) register accessor: Receive FIFO Data Register H\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frdrh::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frdrh`]
module"]
pub type FRDRH = crate::Reg<frdrh::FRDRH_SPEC>;
#[doc = "Receive FIFO Data Register H"]
pub mod frdrh;
#[doc = "FRDRL (r) register accessor: Receive FIFO Data Register L\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frdrl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frdrl`]
module"]
pub type FRDRL = crate::Reg<frdrl::FRDRL_SPEC>;
#[doc = "Receive FIFO Data Register L"]
pub mod frdrl;
#[doc = "MDDR (rw) register accessor: Modulation Duty Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mddr`]
module"]
pub type MDDR = crate::Reg<mddr::MDDR_SPEC>;
#[doc = "Modulation Duty Register"]
pub mod mddr;
#[doc = "DCCR (rw) register accessor: Data Compare Match Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dccr`]
module"]
pub type DCCR = crate::Reg<dccr::DCCR_SPEC>;
#[doc = "Data Compare Match Control Register"]
pub mod dccr;
#[doc = "FCR (rw) register accessor: FIFO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`]
module"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "FIFO Control Register"]
pub mod fcr;
#[doc = "FDR (r) register accessor: FIFO Data Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdr`]
module"]
pub type FDR = crate::Reg<fdr::FDR_SPEC>;
#[doc = "FIFO Data Count Register"]
pub mod fdr;
#[doc = "LSR (r) register accessor: Line Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsr`]
module"]
pub type LSR = crate::Reg<lsr::LSR_SPEC>;
#[doc = "Line Status Register"]
pub mod lsr;
#[doc = "CDR (rw) register accessor: Compare Match Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr`]
module"]
pub type CDR = crate::Reg<cdr::CDR_SPEC>;
#[doc = "Compare Match Data Register"]
pub mod cdr;
#[doc = "SPTR (rw) register accessor: Serial Port Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sptr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sptr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sptr`]
module"]
pub type SPTR = crate::Reg<sptr::SPTR_SPEC>;
#[doc = "Serial Port Register"]
pub mod sptr;
