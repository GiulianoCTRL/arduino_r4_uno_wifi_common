#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Bus Master MPU Control Register A"]
    pub mmpuctla: MMPUCTLA,
    _reserved1: [u8; 0x0100],
    #[doc = "0x102 - Group A Protection of Register"]
    pub mmpupta: MMPUPTA,
    _reserved2: [u8; 0xfc],
    #[doc = "0x200 - Group A Region 0 Access Control Register"]
    pub mmpuaca0: MMPUACA,
    _reserved3: [u8; 0x02],
    #[doc = "0x204 - Group A Region 0 Start Address Register"]
    pub mmpusa0: MMPUSA,
    #[doc = "0x208 - Group A Region 0 End Address Register"]
    pub mmpuea0: MMPUEA,
    _reserved5: [u8; 0x04],
    #[doc = "0x210 - Group A Region 1 Access Control Register"]
    pub mmpuaca1: MMPUACA,
    _reserved6: [u8; 0x02],
    #[doc = "0x214 - Group A Region 1 Start Address Register"]
    pub mmpusa1: MMPUSA,
    #[doc = "0x218 - Group A Region 1 End Address Register"]
    pub mmpuea1: MMPUEA,
    _reserved8: [u8; 0x04],
    #[doc = "0x220 - Group A Region 2 Access Control Register"]
    pub mmpuaca2: MMPUACA,
    _reserved9: [u8; 0x02],
    #[doc = "0x224 - Group A Region 2 Start Address Register"]
    pub mmpusa2: MMPUSA,
    #[doc = "0x228 - Group A Region 2 End Address Register"]
    pub mmpuea2: MMPUEA,
    _reserved11: [u8; 0x04],
    #[doc = "0x230 - Group A Region 3 Access Control Register"]
    pub mmpuaca3: MMPUACA,
    _reserved12: [u8; 0x02],
    #[doc = "0x234 - Group A Region 3 Start Address Register"]
    pub mmpusa3: MMPUSA,
    #[doc = "0x238 - Group A Region 3 End Address Register"]
    pub mmpuea3: MMPUEA,
    _reserved14: [u8; 0x04],
    #[doc = "0x240 - Group A Region 4 Access Control Register"]
    pub mmpuaca4: MMPUACA,
    _reserved15: [u8; 0x02],
    #[doc = "0x244 - Group A Region 4 Start Address Register"]
    pub mmpusa4: MMPUSA,
    #[doc = "0x248 - Group A Region 4 End Address Register"]
    pub mmpuea4: MMPUEA,
    _reserved17: [u8; 0x04],
    #[doc = "0x250 - Group A Region 5 Access Control Register"]
    pub mmpuaca5: MMPUACA,
    _reserved18: [u8; 0x02],
    #[doc = "0x254 - Group A Region 5 Start Address Register"]
    pub mmpusa5: MMPUSA,
    #[doc = "0x258 - Group A Region 5 End Address Register"]
    pub mmpuea5: MMPUEA,
    _reserved20: [u8; 0x04],
    #[doc = "0x260 - Group A Region 6 Access Control Register"]
    pub mmpuaca6: MMPUACA,
    _reserved21: [u8; 0x02],
    #[doc = "0x264 - Group A Region 6 Start Address Register"]
    pub mmpusa6: MMPUSA,
    #[doc = "0x268 - Group A Region 6 End Address Register"]
    pub mmpuea6: MMPUEA,
    _reserved23: [u8; 0x04],
    #[doc = "0x270 - Group A Region 7 Access Control Register"]
    pub mmpuaca7: MMPUACA,
    _reserved24: [u8; 0x02],
    #[doc = "0x274 - Group A Region 7 Start Address Register"]
    pub mmpusa7: MMPUSA,
    #[doc = "0x278 - Group A Region 7 End Address Register"]
    pub mmpuea7: MMPUEA,
    _reserved26: [u8; 0x04],
    #[doc = "0x280 - Group A Region 8 Access Control Register"]
    pub mmpuaca8: MMPUACA,
    _reserved27: [u8; 0x02],
    #[doc = "0x284 - Group A Region 8 Start Address Register"]
    pub mmpusa8: MMPUSA,
    #[doc = "0x288 - Group A Region 8 End Address Register"]
    pub mmpuea8: MMPUEA,
    _reserved29: [u8; 0x04],
    #[doc = "0x290 - Group A Region 9 Access Control Register"]
    pub mmpuaca9: MMPUACA,
    _reserved30: [u8; 0x02],
    #[doc = "0x294 - Group A Region 9 Start Address Register"]
    pub mmpusa9: MMPUSA,
    #[doc = "0x298 - Group A Region 9 End Address Register"]
    pub mmpuea9: MMPUEA,
    _reserved32: [u8; 0x04],
    #[doc = "0x2a0 - Group A Region 10 Access Control Register"]
    pub mmpuaca10: MMPUACA,
    _reserved33: [u8; 0x02],
    #[doc = "0x2a4 - Group A Region 10 Start Address Register"]
    pub mmpusa10: MMPUSA,
    #[doc = "0x2a8 - Group A Region 10 End Address Register"]
    pub mmpuea10: MMPUEA,
    _reserved35: [u8; 0x04],
    #[doc = "0x2b0 - Group A Region 11 Access Control Register"]
    pub mmpuaca11: MMPUACA,
    _reserved36: [u8; 0x02],
    #[doc = "0x2b4 - Group A Region 11 Start Address Register"]
    pub mmpusa11: MMPUSA,
    #[doc = "0x2b8 - Group A Region 11 End Address Register"]
    pub mmpuea11: MMPUEA,
    _reserved38: [u8; 0x04],
    #[doc = "0x2c0 - Group A Region 12 Access Control Register"]
    pub mmpuaca12: MMPUACA,
    _reserved39: [u8; 0x02],
    #[doc = "0x2c4 - Group A Region 12 Start Address Register"]
    pub mmpusa12: MMPUSA,
    #[doc = "0x2c8 - Group A Region 12 End Address Register"]
    pub mmpuea12: MMPUEA,
    _reserved41: [u8; 0x04],
    #[doc = "0x2d0 - Group A Region 13 Access Control Register"]
    pub mmpuaca13: MMPUACA,
    _reserved42: [u8; 0x02],
    #[doc = "0x2d4 - Group A Region 13 Start Address Register"]
    pub mmpusa13: MMPUSA,
    #[doc = "0x2d8 - Group A Region 13 End Address Register"]
    pub mmpuea13: MMPUEA,
    _reserved44: [u8; 0x04],
    #[doc = "0x2e0 - Group A Region 14 Access Control Register"]
    pub mmpuaca14: MMPUACA,
    _reserved45: [u8; 0x02],
    #[doc = "0x2e4 - Group A Region 14 Start Address Register"]
    pub mmpusa14: MMPUSA,
    #[doc = "0x2e8 - Group A Region 14 End Address Register"]
    pub mmpuea14: MMPUEA,
    _reserved47: [u8; 0x04],
    #[doc = "0x2f0 - Group A Region 15 Access Control Register"]
    pub mmpuaca15: MMPUACA,
    _reserved48: [u8; 0x02],
    #[doc = "0x2f4 - Group A Region 15 Start Address Register"]
    pub mmpusa15: MMPUSA,
    #[doc = "0x2f8 - Group A Region 15 End Address Register"]
    pub mmpuea15: MMPUEA,
}
#[doc = "MMPUCTLA (rw) register accessor: Bus Master MPU Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmpuctla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmpuctla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmpuctla`]
module"]
pub type MMPUCTLA = crate::Reg<mmpuctla::MMPUCTLA_SPEC>;
#[doc = "Bus Master MPU Control Register A"]
pub mod mmpuctla;
#[doc = "MMPUACA (rw) register accessor: Group A Region %s Access Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmpuaca::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmpuaca::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmpuaca`]
module"]
pub type MMPUACA = crate::Reg<mmpuaca::MMPUACA_SPEC>;
#[doc = "Group A Region %s Access Control Register"]
pub mod mmpuaca;
#[doc = "MMPUSA (rw) register accessor: Group A Region %s Start Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmpusa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmpusa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmpusa`]
module"]
pub type MMPUSA = crate::Reg<mmpusa::MMPUSA_SPEC>;
#[doc = "Group A Region %s Start Address Register"]
pub mod mmpusa;
#[doc = "MMPUEA (rw) register accessor: Group A Region %s End Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmpuea::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmpuea::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmpuea`]
module"]
pub type MMPUEA = crate::Reg<mmpuea::MMPUEA_SPEC>;
#[doc = "Group A Region %s End Address Register"]
pub mod mmpuea;
#[doc = "MMPUPTA (rw) register accessor: Group A Protection of Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmpupta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmpupta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmpupta`]
module"]
pub type MMPUPTA = crate::Reg<mmpupta::MMPUPTA_SPEC>;
#[doc = "Group A Protection of Register"]
pub mod mmpupta;
