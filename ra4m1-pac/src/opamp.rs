#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "0x08 - Operational amplifier mode control register"]
    pub ampmc: AMPMC,
    #[doc = "0x09 - Operational amplifier trigger mode control register"]
    pub amptrm: AMPTRM,
    #[doc = "0x0a - Operational Amplifier Activation Trigger Select Register"]
    pub amptrs: AMPTRS,
    #[doc = "0x0b - Operational amplifier control register"]
    pub ampc: AMPC,
    #[doc = "0x0c - Operational amplifier monitor register"]
    pub ampmon: AMPMON,
}
#[doc = "AMPMC (rw) register accessor: Operational amplifier mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ampmc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ampmc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ampmc`]
module"]
pub type AMPMC = crate::Reg<ampmc::AMPMC_SPEC>;
#[doc = "Operational amplifier mode control register"]
pub mod ampmc;
#[doc = "AMPTRM (rw) register accessor: Operational amplifier trigger mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`amptrm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`amptrm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amptrm`]
module"]
pub type AMPTRM = crate::Reg<amptrm::AMPTRM_SPEC>;
#[doc = "Operational amplifier trigger mode control register"]
pub mod amptrm;
#[doc = "AMPTRS (rw) register accessor: Operational Amplifier Activation Trigger Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`amptrs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`amptrs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amptrs`]
module"]
pub type AMPTRS = crate::Reg<amptrs::AMPTRS_SPEC>;
#[doc = "Operational Amplifier Activation Trigger Select Register"]
pub mod amptrs;
#[doc = "AMPC (rw) register accessor: Operational amplifier control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ampc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ampc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ampc`]
module"]
pub type AMPC = crate::Reg<ampc::AMPC_SPEC>;
#[doc = "Operational amplifier control register"]
pub mod ampc;
#[doc = "AMPMON (r) register accessor: Operational amplifier monitor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ampmon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ampmon`]
module"]
pub type AMPMON = crate::Reg<ampmon::AMPMON_SPEC>;
#[doc = "Operational amplifier monitor register"]
pub mod ampmon;
