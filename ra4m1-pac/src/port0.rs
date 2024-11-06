#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_pdr: [u8; 0x04],
    _reserved_1_pidr: [u8; 0x04],
    _reserved_2_porr: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - Output data register"]
    #[inline(always)]
    pub const fn podr(&self) -> &PODR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - Port Control Register 1"]
    #[inline(always)]
    pub const fn pcntr1(&self) -> &PCNTR1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x02 - Data direction register"]
    #[inline(always)]
    pub const fn pdr(&self) -> &PDR {
        unsafe { &*(self as *const Self).cast::<u8>().add(2usize).cast() }
    }
    #[doc = "0x04 - Port Control Register 2"]
    #[inline(always)]
    pub const fn pcntr2(&self) -> &PCNTR2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x06 - Input data register"]
    #[inline(always)]
    pub const fn pidr(&self) -> &PIDR {
        unsafe { &*(self as *const Self).cast::<u8>().add(6usize).cast() }
    }
    #[doc = "0x08 - Output reset register"]
    #[inline(always)]
    pub const fn porr(&self) -> &PORR {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x08 - Port Control Register 3"]
    #[inline(always)]
    pub const fn pcntr3(&self) -> &PCNTR3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x0a - Output set register"]
    #[inline(always)]
    pub const fn posr(&self) -> &POSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(10usize).cast() }
    }
}
#[doc = "PCNTR1 (rw) register accessor: Port Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcntr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcntr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcntr1`]
module"]
pub type PCNTR1 = crate::Reg<pcntr1::PCNTR1_SPEC>;
#[doc = "Port Control Register 1"]
pub mod pcntr1;
#[doc = "PODR (rw) register accessor: Output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`podr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`podr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@podr`]
module"]
pub type PODR = crate::Reg<podr::PODR_SPEC>;
#[doc = "Output data register"]
pub mod podr;
#[doc = "PDR (rw) register accessor: Data direction register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdr`]
module"]
pub type PDR = crate::Reg<pdr::PDR_SPEC>;
#[doc = "Data direction register"]
pub mod pdr;
#[doc = "PCNTR2 (r) register accessor: Port Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcntr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcntr2`]
module"]
pub type PCNTR2 = crate::Reg<pcntr2::PCNTR2_SPEC>;
#[doc = "Port Control Register 2"]
pub mod pcntr2;
#[doc = "PIDR (r) register accessor: Input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr`]
module"]
pub type PIDR = crate::Reg<pidr::PIDR_SPEC>;
#[doc = "Input data register"]
pub mod pidr;
#[doc = "PCNTR3 (w) register accessor: Port Control Register 3\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcntr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcntr3`]
module"]
pub type PCNTR3 = crate::Reg<pcntr3::PCNTR3_SPEC>;
#[doc = "Port Control Register 3"]
pub mod pcntr3;
#[doc = "PORR (w) register accessor: Output reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`porr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@porr`]
module"]
pub type PORR = crate::Reg<porr::PORR_SPEC>;
#[doc = "Output reset register"]
pub mod porr;
#[doc = "POSR (w) register accessor: Output set register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`posr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@posr`]
module"]
pub type POSR = crate::Reg<posr::POSR_SPEC>;
#[doc = "Output set register"]
pub mod posr;
