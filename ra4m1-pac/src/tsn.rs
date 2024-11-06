#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0228],
    #[doc = "0x228 - Temperature Sensor Calibration Data Register L"]
    pub tscdrl: TSCDRL,
    #[doc = "0x229 - Temperature Sensor Calibration Data Register H"]
    pub tscdrh: TSCDRH,
}
#[doc = "TSCDRH (r) register accessor: Temperature Sensor Calibration Data Register H\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscdrh::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscdrh`]
module"]
pub type TSCDRH = crate::Reg<tscdrh::TSCDRH_SPEC>;
#[doc = "Temperature Sensor Calibration Data Register H"]
pub mod tscdrh;
#[doc = "TSCDRL (r) register accessor: Temperature Sensor Calibration Data Register L\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscdrl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscdrl`]
module"]
pub type TSCDRL = crate::Reg<tscdrl::TSCDRL_SPEC>;
#[doc = "Temperature Sensor Calibration Data Register L"]
pub mod tscdrl;
