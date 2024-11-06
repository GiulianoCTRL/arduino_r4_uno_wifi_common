#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Configuration Control Register"]
    pub syscfg: SYSCFG,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - System Configuration Status Register 0"]
    pub syssts0: SYSSTS0,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - Device State Control Register 0"]
    pub dvstctr0: DVSTCTR0,
    _reserved3: [u8; 0x0a],
    _reserved_3_cfifo: [u8; 0x02],
    _reserved4: [u8; 0x02],
    _reserved_4_d: [u8; 0x02],
    _reserved5: [u8; 0x02],
    _reserved_5_d: [u8; 0x02],
    _reserved6: [u8; 0x02],
    #[doc = "0x20 - CFIFO Port Select Register"]
    pub cfifosel: CFIFOSEL,
    #[doc = "0x22 - CFIFO Port Control Register"]
    pub cfifoctr: CFIFOCTR,
    _reserved8: [u8; 0x04],
    #[doc = "0x28 - D0FIFO Port Select Register"]
    pub d0fifosel: D0FIFOSEL,
    #[doc = "0x2a - D0FIFO Port Control Register"]
    pub d0fifoctr: D0FIFOCTR,
    #[doc = "0x2c - D1FIFO Port Select Register"]
    pub d1fifosel: D1FIFOSEL,
    #[doc = "0x2e - D1FIFO Port Control Register"]
    pub d1fifoctr: D1FIFOCTR,
    #[doc = "0x30 - Interrupt Enable Register 0"]
    pub intenb0: INTENB0,
    #[doc = "0x32 - Interrupt Enable Register 1"]
    pub intenb1: INTENB1,
    _reserved14: [u8; 0x02],
    #[doc = "0x36 - BRDY Interrupt Enable Register"]
    pub brdyenb: BRDYENB,
    #[doc = "0x38 - NRDY Interrupt Enable Register"]
    pub nrdyenb: NRDYENB,
    #[doc = "0x3a - BEMP Interrupt Enable Register"]
    pub bempenb: BEMPENB,
    #[doc = "0x3c - SOF Output Configuration Register"]
    pub sofcfg: SOFCFG,
    _reserved18: [u8; 0x02],
    #[doc = "0x40 - Interrupt Status Register 0"]
    pub intsts0: INTSTS0,
    #[doc = "0x42 - Interrupt Status Register 1"]
    pub intsts1: INTSTS1,
    _reserved20: [u8; 0x02],
    #[doc = "0x46 - BRDY Interrupt Status Register"]
    pub brdysts: BRDYSTS,
    #[doc = "0x48 - NRDY Interrupt Status Register"]
    pub nrdysts: NRDYSTS,
    #[doc = "0x4a - BEMP Interrupt Status Register"]
    pub bempsts: BEMPSTS,
    #[doc = "0x4c - Frame Number Register"]
    pub frmnum: FRMNUM,
    _reserved24: [u8; 0x06],
    #[doc = "0x54 - USB Request Type Register"]
    pub usbreq: USBREQ,
    #[doc = "0x56 - USB Request Value Register"]
    pub usbval: USBVAL,
    #[doc = "0x58 - USB Request Index Register"]
    pub usbindx: USBINDX,
    #[doc = "0x5a - USB Request Length Register"]
    pub usbleng: USBLENG,
    #[doc = "0x5c - DCP Configuration Register"]
    pub dcpcfg: DCPCFG,
    #[doc = "0x5e - DCP Maximum Packet Size Register"]
    pub dcpmaxp: DCPMAXP,
    #[doc = "0x60 - DCP Control Register"]
    pub dcpctr: DCPCTR,
    _reserved31: [u8; 0x02],
    #[doc = "0x64 - Pipe Window Select Register"]
    pub pipesel: PIPESEL,
    _reserved32: [u8; 0x02],
    #[doc = "0x68 - Pipe Configuration Register"]
    pub pipecfg: PIPECFG,
    _reserved33: [u8; 0x02],
    #[doc = "0x6c - Pipe Maximum Packet Size Register"]
    pub pipemaxp: PIPEMAXP,
    #[doc = "0x6e - Pipe Cycle Control Register"]
    pub pipeperi: PIPEPERI,
    #[doc = "0x70..0x7a - Pipe %s Control Register"]
    pub pipectr: [PIPECTR; 5],
    #[doc = "0x7a - Pipe 6 Control Register"]
    pub pipe6ctr: PIPE6CTR,
    #[doc = "0x7c - Pipe 7 Control Register"]
    pub pipe7ctr: PIPE6CTR,
    #[doc = "0x7e - Pipe 8 Control Register"]
    pub pipe8ctr: PIPE6CTR,
    #[doc = "0x80 - Pipe 9 Control Register"]
    pub pipe9ctr: PIPE6CTR,
    _reserved40: [u8; 0x0e],
    #[doc = "0x90 - Pipe 1 Transaction Counter Enable Register"]
    pub pipe1tre: PIPETRE,
    #[doc = "0x92 - Pipe 1 Transaction Counter Register"]
    pub pipe1trn: PIPETRN,
    #[doc = "0x94 - Pipe 2 Transaction Counter Enable Register"]
    pub pipe2tre: PIPETRE,
    #[doc = "0x96 - Pipe 2 Transaction Counter Register"]
    pub pipe2trn: PIPETRN,
    #[doc = "0x98 - Pipe 3 Transaction Counter Enable Register"]
    pub pipe3tre: PIPETRE,
    #[doc = "0x9a - Pipe 3 Transaction Counter Register"]
    pub pipe3trn: PIPETRN,
    #[doc = "0x9c - Pipe 4 Transaction Counter Enable Register"]
    pub pipe4tre: PIPETRE,
    #[doc = "0x9e - Pipe 4 Transaction Counter Register"]
    pub pipe4trn: PIPETRN,
    #[doc = "0xa0 - Pipe 5 Transaction Counter Enable Register"]
    pub pipe5tre: PIPETRE,
    #[doc = "0xa2 - Pipe 5 Transaction Counter Register"]
    pub pipe5trn: PIPETRN,
    _reserved50: [u8; 0x0c],
    #[doc = "0xb0 - BC Control Register 0"]
    pub usbbcctrl0: USBBCCTRL0,
    _reserved51: [u8; 0x1a],
    #[doc = "0xcc - USB Module Control Register"]
    pub usbmc: USBMC,
    _reserved52: [u8; 0x02],
    #[doc = "0xd0..0xdc - Device Address %s Configuration Register"]
    pub devadd: [DEVADD; 6],
}
impl RegisterBlock {
    #[doc = "0x14 - CFIFO Port Register L"]
    #[inline(always)]
    pub const fn cfifol(&self) -> &CFIFOL {
        unsafe { &*(self as *const Self).cast::<u8>().add(20usize).cast() }
    }
    #[doc = "0x14 - CFIFO Port Register"]
    #[inline(always)]
    pub const fn cfifo(&self) -> &CFIFO {
        unsafe { &*(self as *const Self).cast::<u8>().add(20usize).cast() }
    }
    #[doc = "0x18 - D0FIFO Port Register L"]
    #[inline(always)]
    pub const fn d0fifol(&self) -> &D0FIFOL {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x18 - D0FIFO Port Register"]
    #[inline(always)]
    pub const fn d0fifo(&self) -> &D0FIFO {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x1c - D1FIFO Port Register L"]
    #[inline(always)]
    pub const fn d1fifol(&self) -> &D1FIFOL {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x1c - D1FIFO Port Register"]
    #[inline(always)]
    pub const fn d1fifo(&self) -> &D1FIFO {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x70 - Pipe 1 Control Register"]
    #[inline(always)]
    pub fn pipe1ctr(&self) -> &PIPECTR {
        &self.pipectr[0]
    }
    #[doc = "0x72 - Pipe 2 Control Register"]
    #[inline(always)]
    pub fn pipe2ctr(&self) -> &PIPECTR {
        &self.pipectr[1]
    }
    #[doc = "0x74 - Pipe 3 Control Register"]
    #[inline(always)]
    pub fn pipe3ctr(&self) -> &PIPECTR {
        &self.pipectr[2]
    }
    #[doc = "0x76 - Pipe 4 Control Register"]
    #[inline(always)]
    pub fn pipe4ctr(&self) -> &PIPECTR {
        &self.pipectr[3]
    }
    #[doc = "0x78 - Pipe 5 Control Register"]
    #[inline(always)]
    pub fn pipe5ctr(&self) -> &PIPECTR {
        &self.pipectr[4]
    }
}
#[doc = "SYSCFG (rw) register accessor: System Configuration Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg`]
module"]
pub type SYSCFG = crate::Reg<syscfg::SYSCFG_SPEC>;
#[doc = "System Configuration Control Register"]
pub mod syscfg;
#[doc = "SYSSTS0 (r) register accessor: System Configuration Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syssts0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syssts0`]
module"]
pub type SYSSTS0 = crate::Reg<syssts0::SYSSTS0_SPEC>;
#[doc = "System Configuration Status Register 0"]
pub mod syssts0;
#[doc = "DVSTCTR0 (rw) register accessor: Device State Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvstctr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvstctr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvstctr0`]
module"]
pub type DVSTCTR0 = crate::Reg<dvstctr0::DVSTCTR0_SPEC>;
#[doc = "Device State Control Register 0"]
pub mod dvstctr0;
#[doc = "CFIFO (rw) register accessor: CFIFO Port Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfifo`]
module"]
pub type CFIFO = crate::Reg<cfifo::CFIFO_SPEC>;
#[doc = "CFIFO Port Register"]
pub mod cfifo;
#[doc = "CFIFOL (rw) register accessor: CFIFO Port Register L\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfifol::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfifol::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfifol`]
module"]
pub type CFIFOL = crate::Reg<cfifol::CFIFOL_SPEC>;
#[doc = "CFIFO Port Register L"]
pub mod cfifol;
#[doc = "D0FIFO (rw) register accessor: D0FIFO Port Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d0fifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d0fifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d0fifo`]
module"]
pub type D0FIFO = crate::Reg<d0fifo::D0FIFO_SPEC>;
#[doc = "D0FIFO Port Register"]
pub mod d0fifo;
#[doc = "D0FIFOL (rw) register accessor: D0FIFO Port Register L\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d0fifol::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d0fifol::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d0fifol`]
module"]
pub type D0FIFOL = crate::Reg<d0fifol::D0FIFOL_SPEC>;
#[doc = "D0FIFO Port Register L"]
pub mod d0fifol;
#[doc = "D1FIFO (rw) register accessor: D1FIFO Port Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d1fifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d1fifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d1fifo`]
module"]
pub type D1FIFO = crate::Reg<d1fifo::D1FIFO_SPEC>;
#[doc = "D1FIFO Port Register"]
pub mod d1fifo;
#[doc = "D1FIFOL (rw) register accessor: D1FIFO Port Register L\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d1fifol::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d1fifol::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d1fifol`]
module"]
pub type D1FIFOL = crate::Reg<d1fifol::D1FIFOL_SPEC>;
#[doc = "D1FIFO Port Register L"]
pub mod d1fifol;
#[doc = "CFIFOSEL (rw) register accessor: CFIFO Port Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfifosel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfifosel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfifosel`]
module"]
pub type CFIFOSEL = crate::Reg<cfifosel::CFIFOSEL_SPEC>;
#[doc = "CFIFO Port Select Register"]
pub mod cfifosel;
#[doc = "CFIFOCTR (rw) register accessor: CFIFO Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfifoctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfifoctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfifoctr`]
module"]
pub type CFIFOCTR = crate::Reg<cfifoctr::CFIFOCTR_SPEC>;
#[doc = "CFIFO Port Control Register"]
pub mod cfifoctr;
#[doc = "D0FIFOSEL (rw) register accessor: D0FIFO Port Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d0fifosel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d0fifosel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d0fifosel`]
module"]
pub type D0FIFOSEL = crate::Reg<d0fifosel::D0FIFOSEL_SPEC>;
#[doc = "D0FIFO Port Select Register"]
pub mod d0fifosel;
#[doc = "D0FIFOCTR (rw) register accessor: D0FIFO Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d0fifoctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d0fifoctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d0fifoctr`]
module"]
pub type D0FIFOCTR = crate::Reg<d0fifoctr::D0FIFOCTR_SPEC>;
#[doc = "D0FIFO Port Control Register"]
pub mod d0fifoctr;
#[doc = "D1FIFOSEL (rw) register accessor: D1FIFO Port Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d1fifosel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d1fifosel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d1fifosel`]
module"]
pub type D1FIFOSEL = crate::Reg<d1fifosel::D1FIFOSEL_SPEC>;
#[doc = "D1FIFO Port Select Register"]
pub mod d1fifosel;
#[doc = "D1FIFOCTR (rw) register accessor: D1FIFO Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d1fifoctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d1fifoctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d1fifoctr`]
module"]
pub type D1FIFOCTR = crate::Reg<d1fifoctr::D1FIFOCTR_SPEC>;
#[doc = "D1FIFO Port Control Register"]
pub mod d1fifoctr;
#[doc = "INTENB0 (rw) register accessor: Interrupt Enable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenb0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenb0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenb0`]
module"]
pub type INTENB0 = crate::Reg<intenb0::INTENB0_SPEC>;
#[doc = "Interrupt Enable Register 0"]
pub mod intenb0;
#[doc = "INTENB1 (rw) register accessor: Interrupt Enable Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenb1`]
module"]
pub type INTENB1 = crate::Reg<intenb1::INTENB1_SPEC>;
#[doc = "Interrupt Enable Register 1"]
pub mod intenb1;
#[doc = "BRDYENB (rw) register accessor: BRDY Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brdyenb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brdyenb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brdyenb`]
module"]
pub type BRDYENB = crate::Reg<brdyenb::BRDYENB_SPEC>;
#[doc = "BRDY Interrupt Enable Register"]
pub mod brdyenb;
#[doc = "NRDYENB (rw) register accessor: NRDY Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nrdyenb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nrdyenb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nrdyenb`]
module"]
pub type NRDYENB = crate::Reg<nrdyenb::NRDYENB_SPEC>;
#[doc = "NRDY Interrupt Enable Register"]
pub mod nrdyenb;
#[doc = "BEMPENB (rw) register accessor: BEMP Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bempenb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bempenb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bempenb`]
module"]
pub type BEMPENB = crate::Reg<bempenb::BEMPENB_SPEC>;
#[doc = "BEMP Interrupt Enable Register"]
pub mod bempenb;
#[doc = "SOFCFG (rw) register accessor: SOF Output Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sofcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sofcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sofcfg`]
module"]
pub type SOFCFG = crate::Reg<sofcfg::SOFCFG_SPEC>;
#[doc = "SOF Output Configuration Register"]
pub mod sofcfg;
#[doc = "INTSTS0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intsts0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intsts0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsts0`]
module"]
pub type INTSTS0 = crate::Reg<intsts0::INTSTS0_SPEC>;
#[doc = "Interrupt Status Register 0"]
pub mod intsts0;
#[doc = "INTSTS1 (rw) register accessor: Interrupt Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intsts1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intsts1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsts1`]
module"]
pub type INTSTS1 = crate::Reg<intsts1::INTSTS1_SPEC>;
#[doc = "Interrupt Status Register 1"]
pub mod intsts1;
#[doc = "BRDYSTS (rw) register accessor: BRDY Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brdysts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brdysts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brdysts`]
module"]
pub type BRDYSTS = crate::Reg<brdysts::BRDYSTS_SPEC>;
#[doc = "BRDY Interrupt Status Register"]
pub mod brdysts;
#[doc = "NRDYSTS (rw) register accessor: NRDY Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nrdysts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nrdysts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nrdysts`]
module"]
pub type NRDYSTS = crate::Reg<nrdysts::NRDYSTS_SPEC>;
#[doc = "NRDY Interrupt Status Register"]
pub mod nrdysts;
#[doc = "BEMPSTS (rw) register accessor: BEMP Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bempsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bempsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bempsts`]
module"]
pub type BEMPSTS = crate::Reg<bempsts::BEMPSTS_SPEC>;
#[doc = "BEMP Interrupt Status Register"]
pub mod bempsts;
#[doc = "FRMNUM (rw) register accessor: Frame Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frmnum::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frmnum::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frmnum`]
module"]
pub type FRMNUM = crate::Reg<frmnum::FRMNUM_SPEC>;
#[doc = "Frame Number Register"]
pub mod frmnum;
#[doc = "USBREQ (rw) register accessor: USB Request Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbreq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbreq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbreq`]
module"]
pub type USBREQ = crate::Reg<usbreq::USBREQ_SPEC>;
#[doc = "USB Request Type Register"]
pub mod usbreq;
#[doc = "USBVAL (rw) register accessor: USB Request Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbval::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbval::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbval`]
module"]
pub type USBVAL = crate::Reg<usbval::USBVAL_SPEC>;
#[doc = "USB Request Value Register"]
pub mod usbval;
#[doc = "USBINDX (rw) register accessor: USB Request Index Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbindx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbindx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbindx`]
module"]
pub type USBINDX = crate::Reg<usbindx::USBINDX_SPEC>;
#[doc = "USB Request Index Register"]
pub mod usbindx;
#[doc = "USBLENG (rw) register accessor: USB Request Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbleng::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbleng::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbleng`]
module"]
pub type USBLENG = crate::Reg<usbleng::USBLENG_SPEC>;
#[doc = "USB Request Length Register"]
pub mod usbleng;
#[doc = "DCPCFG (rw) register accessor: DCP Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcpcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcpcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcpcfg`]
module"]
pub type DCPCFG = crate::Reg<dcpcfg::DCPCFG_SPEC>;
#[doc = "DCP Configuration Register"]
pub mod dcpcfg;
#[doc = "DCPMAXP (rw) register accessor: DCP Maximum Packet Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcpmaxp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcpmaxp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcpmaxp`]
module"]
pub type DCPMAXP = crate::Reg<dcpmaxp::DCPMAXP_SPEC>;
#[doc = "DCP Maximum Packet Size Register"]
pub mod dcpmaxp;
#[doc = "DCPCTR (rw) register accessor: DCP Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcpctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcpctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcpctr`]
module"]
pub type DCPCTR = crate::Reg<dcpctr::DCPCTR_SPEC>;
#[doc = "DCP Control Register"]
pub mod dcpctr;
#[doc = "PIPESEL (rw) register accessor: Pipe Window Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pipesel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pipesel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pipesel`]
module"]
pub type PIPESEL = crate::Reg<pipesel::PIPESEL_SPEC>;
#[doc = "Pipe Window Select Register"]
pub mod pipesel;
#[doc = "PIPECFG (rw) register accessor: Pipe Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pipecfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pipecfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pipecfg`]
module"]
pub type PIPECFG = crate::Reg<pipecfg::PIPECFG_SPEC>;
#[doc = "Pipe Configuration Register"]
pub mod pipecfg;
#[doc = "PIPEMAXP (rw) register accessor: Pipe Maximum Packet Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pipemaxp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pipemaxp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pipemaxp`]
module"]
pub type PIPEMAXP = crate::Reg<pipemaxp::PIPEMAXP_SPEC>;
#[doc = "Pipe Maximum Packet Size Register"]
pub mod pipemaxp;
#[doc = "PIPEPERI (rw) register accessor: Pipe Cycle Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pipeperi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pipeperi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pipeperi`]
module"]
pub type PIPEPERI = crate::Reg<pipeperi::PIPEPERI_SPEC>;
#[doc = "Pipe Cycle Control Register"]
pub mod pipeperi;
#[doc = "PIPECTR (rw) register accessor: Pipe %s Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pipectr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pipectr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pipectr`]
module"]
pub type PIPECTR = crate::Reg<pipectr::PIPECTR_SPEC>;
#[doc = "Pipe %s Control Register"]
pub mod pipectr;
pub use pipectr as pipe6ctr;
pub use PIPECTR as PIPE6CTR;
#[doc = "PIPETRE (rw) register accessor: Pipe %s Transaction Counter Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pipetre::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pipetre::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pipetre`]
module"]
pub type PIPETRE = crate::Reg<pipetre::PIPETRE_SPEC>;
#[doc = "Pipe %s Transaction Counter Enable Register"]
pub mod pipetre;
#[doc = "PIPETRN (rw) register accessor: Pipe %s Transaction Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pipetrn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pipetrn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pipetrn`]
module"]
pub type PIPETRN = crate::Reg<pipetrn::PIPETRN_SPEC>;
#[doc = "Pipe %s Transaction Counter Register"]
pub mod pipetrn;
#[doc = "DEVADD (rw) register accessor: Device Address %s Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devadd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devadd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devadd`]
module"]
pub type DEVADD = crate::Reg<devadd::DEVADD_SPEC>;
#[doc = "Device Address %s Configuration Register"]
pub mod devadd;
#[doc = "USBMC (rw) register accessor: USB Module Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbmc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbmc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbmc`]
module"]
pub type USBMC = crate::Reg<usbmc::USBMC_SPEC>;
#[doc = "USB Module Control Register"]
pub mod usbmc;
#[doc = "USBBCCTRL0 (rw) register accessor: BC Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbbcctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbbcctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbbcctrl0`]
module"]
pub type USBBCCTRL0 = crate::Reg<usbbcctrl0::USBBCCTRL0_SPEC>;
#[doc = "BC Control Register 0"]
pub mod usbbcctrl0;
