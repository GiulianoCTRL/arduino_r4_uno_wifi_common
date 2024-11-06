#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1000],
    #[doc = "0x1000 - Master Bus Control Register M4I"]
    pub busmcntm4i: BUSMCNT,
    _reserved1: [u8; 0x02],
    #[doc = "0x1004 - Master Bus Control Register M4D"]
    pub busmcntm4d: BUSMCNT,
    _reserved2: [u8; 0x02],
    #[doc = "0x1008 - Master Bus Control Register SYS"]
    pub busmcntsys: BUSMCNT,
    _reserved3: [u8; 0x02],
    #[doc = "0x100c - Master Bus Control Register DMA"]
    pub busmcntdma: BUSMCNT,
    _reserved4: [u8; 0xf2],
    #[doc = "0x1100 - Slave Bus Control Register FLI"]
    pub busscntfli: BUSSCNTFLI,
    _reserved5: [u8; 0x06],
    #[doc = "0x1108 - Slave Bus Control Register MBIU"]
    pub busscntmbiu: BUSSCNT,
    _reserved6: [u8; 0x02],
    #[doc = "0x110c - Slave Bus Control Register RAM0"]
    pub busscntram0: BUSSCNT,
    _reserved7: [u8; 0x06],
    #[doc = "0x1114 - Slave Bus Control Register P0B"]
    pub busscntp0b: BUSSCNTP0B,
    _reserved8: [u8; 0x02],
    #[doc = "0x1118 - Slave Bus Control Register P2B"]
    pub busscntp2b: BUSSCNTP0B,
    _reserved9: [u8; 0x02],
    #[doc = "0x111c - Slave Bus Control Register P3B"]
    pub busscntp3b: BUSSCNTP0B,
    _reserved10: [u8; 0x02],
    #[doc = "0x1120 - Slave Bus Control Register P4B"]
    pub busscntp4b: BUSSCNTP0B,
    _reserved11: [u8; 0x06],
    #[doc = "0x1128 - Slave Bus Control Register P6B"]
    pub busscntp6b: BUSSCNTP6B,
    _reserved12: [u8; 0x06],
    #[doc = "0x1130 - Slave Bus Control Register FBU"]
    pub busscntfbu: BUSSCNTFBU,
    _reserved13: [u8; 0x06ce],
    #[doc = "0x1800 - Bus Error Address Register 1"]
    pub bus1erradd: BUSERRADD,
    #[doc = "0x1804 - Bus Error Status Register 1"]
    pub bus1errstat: BUSERRSTAT,
    _reserved15: [u8; 0x0b],
    #[doc = "0x1810 - Bus Error Address Register 2"]
    pub bus2erradd: BUSERRADD,
    #[doc = "0x1814 - Bus Error Status Register 2"]
    pub bus2errstat: BUSERRSTAT,
    _reserved17: [u8; 0x0b],
    #[doc = "0x1820 - Bus Error Address Register 3"]
    pub bus3erradd: BUSERRADD,
    #[doc = "0x1824 - Bus Error Status Register 3"]
    pub bus3errstat: BUSERRSTAT,
    _reserved19: [u8; 0x0b],
    #[doc = "0x1830 - Bus Error Address Register 4"]
    pub bus4erradd: BUSERRADD,
    #[doc = "0x1834 - Bus Error Status Register 4"]
    pub bus4errstat: BUSERRSTAT,
}
#[doc = "BUSMCNT (rw) register accessor: Master Bus Control Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busmcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`busmcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busmcnt`]
module"]
pub type BUSMCNT = crate::Reg<busmcnt::BUSMCNT_SPEC>;
#[doc = "Master Bus Control Register %s"]
pub mod busmcnt;
#[doc = "BUSSCNTFLI (rw) register accessor: Slave Bus Control Register FLI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busscntfli::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`busscntfli::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busscntfli`]
module"]
pub type BUSSCNTFLI = crate::Reg<busscntfli::BUSSCNTFLI_SPEC>;
#[doc = "Slave Bus Control Register FLI"]
pub mod busscntfli;
#[doc = "BUSSCNT (rw) register accessor: Slave Bus Control Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busscnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`busscnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busscnt`]
module"]
pub type BUSSCNT = crate::Reg<busscnt::BUSSCNT_SPEC>;
#[doc = "Slave Bus Control Register %s"]
pub mod busscnt;
pub use busscnt as busscntp0b;
pub use BUSSCNT as BUSSCNTP0B;
#[doc = "BUSSCNTP6B (rw) register accessor: Slave Bus Control Register P6B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busscntp6b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`busscntp6b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busscntp6b`]
module"]
pub type BUSSCNTP6B = crate::Reg<busscntp6b::BUSSCNTP6B_SPEC>;
#[doc = "Slave Bus Control Register P6B"]
pub mod busscntp6b;
#[doc = "BUSSCNTFBU (rw) register accessor: Slave Bus Control Register FBU\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busscntfbu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`busscntfbu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busscntfbu`]
module"]
pub type BUSSCNTFBU = crate::Reg<busscntfbu::BUSSCNTFBU_SPEC>;
#[doc = "Slave Bus Control Register FBU"]
pub mod busscntfbu;
#[doc = "BUSERRADD (r) register accessor: Bus Error Address Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buserradd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buserradd`]
module"]
pub type BUSERRADD = crate::Reg<buserradd::BUSERRADD_SPEC>;
#[doc = "Bus Error Address Register %s"]
pub mod buserradd;
#[doc = "BUSERRSTAT (r) register accessor: Bus Error Status Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buserrstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buserrstat`]
module"]
pub type BUSERRSTAT = crate::Reg<buserrstat::BUSERRSTAT_SPEC>;
#[doc = "Bus Error Status Register %s"]
pub mod buserrstat;
