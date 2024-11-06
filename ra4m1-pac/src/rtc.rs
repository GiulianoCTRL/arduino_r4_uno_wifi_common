#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - 64-Hz Counter"]
    pub r64cnt: R64CNT,
    _reserved1: [u8; 0x01],
    _reserved_1_bcnt0: [u8; 0x01],
    _reserved2: [u8; 0x01],
    _reserved_2_bcnt1: [u8; 0x01],
    _reserved3: [u8; 0x01],
    _reserved_3_bcnt2: [u8; 0x01],
    _reserved4: [u8; 0x01],
    _reserved_4_bcnt3: [u8; 0x01],
    _reserved5: [u8; 0x01],
    #[doc = "0x0a - Day Counter"]
    pub rdaycnt: RDAYCNT,
    _reserved6: [u8; 0x01],
    #[doc = "0x0c - Month Counter"]
    pub rmoncnt: RMONCNT,
    _reserved7: [u8; 0x01],
    #[doc = "0x0e - Year Counter"]
    pub ryrcnt: RYRCNT,
    _reserved_8_rsecar: [u8; 0x01],
    _reserved9: [u8; 0x01],
    _reserved_9_rminar: [u8; 0x01],
    _reserved10: [u8; 0x01],
    _reserved_10_rhrar: [u8; 0x01],
    _reserved11: [u8; 0x01],
    _reserved_11_rwkar: [u8; 0x01],
    _reserved12: [u8; 0x01],
    _reserved_12_rdayar: [u8; 0x01],
    _reserved13: [u8; 0x01],
    _reserved_13_rmonar: [u8; 0x01],
    _reserved14: [u8; 0x01],
    _reserved_14_ryrar: [u8; 0x02],
    _reserved_15_ryraren: [u8; 0x01],
    _reserved16: [u8; 0x03],
    #[doc = "0x22 - RTC Control Register 1"]
    pub rcr1: RCR1,
    _reserved17: [u8; 0x01],
    #[doc = "0x24 - RTC Control Register 2"]
    pub rcr2: RCR2,
    _reserved18: [u8; 0x03],
    #[doc = "0x28 - RTC Control Register 4"]
    pub rcr4: RCR4,
    _reserved19: [u8; 0x01],
    #[doc = "0x2a - Frequency Register H"]
    pub rfrh: RFRH,
    #[doc = "0x2c - Frequency Register L"]
    pub rfrl: RFRL,
    #[doc = "0x2e - Time Error Adjustment Register"]
    pub radj: RADJ,
    _reserved22: [u8; 0x11],
    #[doc = "0x40 - Time Capture Control Register 0"]
    pub rtccr0: RTCCR,
    _reserved23: [u8; 0x01],
    #[doc = "0x42 - Time Capture Control Register 1"]
    pub rtccr1: RTCCR,
    _reserved24: [u8; 0x01],
    #[doc = "0x44 - Time Capture Control Register 2"]
    pub rtccr2: RTCCR,
    _reserved25: [u8; 0x0d],
    _reserved_25_rseccp0: [u8; 0x01],
    _reserved26: [u8; 0x01],
    _reserved_26_rmincp0: [u8; 0x01],
    _reserved27: [u8; 0x01],
    _reserved_27_rhrcp0: [u8; 0x01],
    _reserved28: [u8; 0x03],
    _reserved_28_rdaycp0: [u8; 0x01],
    _reserved29: [u8; 0x01],
    #[doc = "0x5c - Month Capture Register 0"]
    pub rmoncp0: RMONCP,
    _reserved30: [u8; 0x05],
    _reserved_30_rseccp1: [u8; 0x01],
    _reserved31: [u8; 0x01],
    _reserved_31_rmincp1: [u8; 0x01],
    _reserved32: [u8; 0x01],
    _reserved_32_rhrcp1: [u8; 0x01],
    _reserved33: [u8; 0x03],
    _reserved_33_rdaycp1: [u8; 0x01],
    _reserved34: [u8; 0x01],
    #[doc = "0x6c - Month Capture Register 1"]
    pub rmoncp1: RMONCP,
    _reserved35: [u8; 0x05],
    _reserved_35_rseccp2: [u8; 0x01],
    _reserved36: [u8; 0x01],
    _reserved_36_rmincp2: [u8; 0x01],
    _reserved37: [u8; 0x01],
    _reserved_37_rhrcp2: [u8; 0x01],
    _reserved38: [u8; 0x03],
    _reserved_38_rdaycp2: [u8; 0x01],
    _reserved39: [u8; 0x01],
    #[doc = "0x7c - Month Capture Register 2"]
    pub rmoncp2: RMONCP,
}
impl RegisterBlock {
    #[doc = "0x02 - Binary Counter 0"]
    #[inline(always)]
    pub const fn bcnt0(&self) -> &BCNT0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2usize).cast() }
    }
    #[doc = "0x02 - Second Counter"]
    #[inline(always)]
    pub const fn rseccnt(&self) -> &RSECCNT {
        unsafe { &*(self as *const Self).cast::<u8>().add(2usize).cast() }
    }
    #[doc = "0x04 - Binary Counter 1"]
    #[inline(always)]
    pub const fn bcnt1(&self) -> &BCNT1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - Minute Counter"]
    #[inline(always)]
    pub const fn rmincnt(&self) -> &RMINCNT {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x06 - Binary Counter 2"]
    #[inline(always)]
    pub const fn bcnt2(&self) -> &BCNT2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(6usize).cast() }
    }
    #[doc = "0x06 - Hour Counter"]
    #[inline(always)]
    pub const fn rhrcnt(&self) -> &RHRCNT {
        unsafe { &*(self as *const Self).cast::<u8>().add(6usize).cast() }
    }
    #[doc = "0x08 - Binary Counter 3"]
    #[inline(always)]
    pub const fn bcnt3(&self) -> &BCNT3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x08 - Day-of-Week Counter"]
    #[inline(always)]
    pub const fn rwkcnt(&self) -> &RWKCNT {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x10 - Binary Counter 0 Alarm Register"]
    #[inline(always)]
    pub const fn bcnt0ar(&self) -> &BCNT0AR {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x10 - Second Alarm Register"]
    #[inline(always)]
    pub const fn rsecar(&self) -> &RSECAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x12 - Binary Counter 1 Alarm Register"]
    #[inline(always)]
    pub const fn bcnt1ar(&self) -> &BCNT1AR {
        unsafe { &*(self as *const Self).cast::<u8>().add(18usize).cast() }
    }
    #[doc = "0x12 - Minute Alarm Register"]
    #[inline(always)]
    pub const fn rminar(&self) -> &RMINAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(18usize).cast() }
    }
    #[doc = "0x14 - Binary Counter 2 Alarm Register"]
    #[inline(always)]
    pub const fn bcnt2ar(&self) -> &BCNT2AR {
        unsafe { &*(self as *const Self).cast::<u8>().add(20usize).cast() }
    }
    #[doc = "0x14 - Hour Alarm Register"]
    #[inline(always)]
    pub const fn rhrar(&self) -> &RHRAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(20usize).cast() }
    }
    #[doc = "0x16 - Binary Counter 3 Alarm Register"]
    #[inline(always)]
    pub const fn bcnt3ar(&self) -> &BCNT3AR {
        unsafe { &*(self as *const Self).cast::<u8>().add(22usize).cast() }
    }
    #[doc = "0x16 - Day-of-Week Alarm Register"]
    #[inline(always)]
    pub const fn rwkar(&self) -> &RWKAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(22usize).cast() }
    }
    #[doc = "0x18 - Binary Counter 0 Alarm Enable Register"]
    #[inline(always)]
    pub const fn bcnt0aer(&self) -> &BCNT0AER {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x18 - Date Alarm Register"]
    #[inline(always)]
    pub const fn rdayar(&self) -> &RDAYAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x1a - Binary Counter 1 Alarm Enable Register"]
    #[inline(always)]
    pub const fn bcnt1aer(&self) -> &BCNT1AER {
        unsafe { &*(self as *const Self).cast::<u8>().add(26usize).cast() }
    }
    #[doc = "0x1a - Month Alarm Register"]
    #[inline(always)]
    pub const fn rmonar(&self) -> &RMONAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(26usize).cast() }
    }
    #[doc = "0x1c - Binary Counter 2 Alarm Enable Register"]
    #[inline(always)]
    pub const fn bcnt2aer(&self) -> &BCNT2AER {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x1c - Year Alarm Register"]
    #[inline(always)]
    pub const fn ryrar(&self) -> &RYRAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x1e - Binary Counter 3 Alarm Enable Register"]
    #[inline(always)]
    pub const fn bcnt3aer(&self) -> &BCNT3AER {
        unsafe { &*(self as *const Self).cast::<u8>().add(30usize).cast() }
    }
    #[doc = "0x1e - Year Alarm Enable Register"]
    #[inline(always)]
    pub const fn ryraren(&self) -> &RYRAREN {
        unsafe { &*(self as *const Self).cast::<u8>().add(30usize).cast() }
    }
    #[doc = "0x52 - BCNT0 Capture Register 0"]
    #[inline(always)]
    pub const fn bcnt0cp0(&self) -> &BCNT0CP {
        unsafe { &*(self as *const Self).cast::<u8>().add(82usize).cast() }
    }
    #[doc = "0x52 - Second Capture Register 0"]
    #[inline(always)]
    pub const fn rseccp0(&self) -> &RSECCP {
        unsafe { &*(self as *const Self).cast::<u8>().add(82usize).cast() }
    }
    #[doc = "0x54 - BCNT1 Capture Register 0"]
    #[inline(always)]
    pub const fn bcnt1cp0(&self) -> &BCNT1CP {
        unsafe { &*(self as *const Self).cast::<u8>().add(84usize).cast() }
    }
    #[doc = "0x54 - Minute Capture Register 0"]
    #[inline(always)]
    pub const fn rmincp0(&self) -> &RMINCP {
        unsafe { &*(self as *const Self).cast::<u8>().add(84usize).cast() }
    }
    #[doc = "0x56 - BCNT2 Capture Register 0"]
    #[inline(always)]
    pub const fn bcnt2cp0(&self) -> &BCNT2CP {
        unsafe { &*(self as *const Self).cast::<u8>().add(86usize).cast() }
    }
    #[doc = "0x56 - Hour Capture Register 0"]
    #[inline(always)]
    pub const fn rhrcp0(&self) -> &RHRCP {
        unsafe { &*(self as *const Self).cast::<u8>().add(86usize).cast() }
    }
    #[doc = "0x5a - BCNT3 Capture Register 0"]
    #[inline(always)]
    pub const fn bcnt3cp0(&self) -> &BCNT3CP {
        unsafe { &*(self as *const Self).cast::<u8>().add(90usize).cast() }
    }
    #[doc = "0x5a - Date Capture Register 0"]
    #[inline(always)]
    pub const fn rdaycp0(&self) -> &RDAYCP {
        unsafe { &*(self as *const Self).cast::<u8>().add(90usize).cast() }
    }
    #[doc = "0x62 - BCNT0 Capture Register 1"]
    #[inline(always)]
    pub const fn bcnt0cp1(&self) -> &BCNT0CP {
        unsafe { &*(self as *const Self).cast::<u8>().add(98usize).cast() }
    }
    #[doc = "0x62 - Second Capture Register 1"]
    #[inline(always)]
    pub const fn rseccp1(&self) -> &RSECCP {
        unsafe { &*(self as *const Self).cast::<u8>().add(98usize).cast() }
    }
    #[doc = "0x64 - BCNT1 Capture Register 1"]
    #[inline(always)]
    pub const fn bcnt1cp1(&self) -> &BCNT1CP {
        unsafe { &*(self as *const Self).cast::<u8>().add(100usize).cast() }
    }
    #[doc = "0x64 - Minute Capture Register 1"]
    #[inline(always)]
    pub const fn rmincp1(&self) -> &RMINCP {
        unsafe { &*(self as *const Self).cast::<u8>().add(100usize).cast() }
    }
    #[doc = "0x66 - BCNT2 Capture Register 1"]
    #[inline(always)]
    pub const fn bcnt2cp1(&self) -> &BCNT2CP {
        unsafe { &*(self as *const Self).cast::<u8>().add(102usize).cast() }
    }
    #[doc = "0x66 - Hour Capture Register 1"]
    #[inline(always)]
    pub const fn rhrcp1(&self) -> &RHRCP {
        unsafe { &*(self as *const Self).cast::<u8>().add(102usize).cast() }
    }
    #[doc = "0x6a - BCNT3 Capture Register 1"]
    #[inline(always)]
    pub const fn bcnt3cp1(&self) -> &BCNT3CP {
        unsafe { &*(self as *const Self).cast::<u8>().add(106usize).cast() }
    }
    #[doc = "0x6a - Date Capture Register 1"]
    #[inline(always)]
    pub const fn rdaycp1(&self) -> &RDAYCP {
        unsafe { &*(self as *const Self).cast::<u8>().add(106usize).cast() }
    }
    #[doc = "0x72 - BCNT0 Capture Register 2"]
    #[inline(always)]
    pub const fn bcnt0cp2(&self) -> &BCNT0CP {
        unsafe { &*(self as *const Self).cast::<u8>().add(114usize).cast() }
    }
    #[doc = "0x72 - Second Capture Register 2"]
    #[inline(always)]
    pub const fn rseccp2(&self) -> &RSECCP {
        unsafe { &*(self as *const Self).cast::<u8>().add(114usize).cast() }
    }
    #[doc = "0x74 - BCNT1 Capture Register 2"]
    #[inline(always)]
    pub const fn bcnt1cp2(&self) -> &BCNT1CP {
        unsafe { &*(self as *const Self).cast::<u8>().add(116usize).cast() }
    }
    #[doc = "0x74 - Minute Capture Register 2"]
    #[inline(always)]
    pub const fn rmincp2(&self) -> &RMINCP {
        unsafe { &*(self as *const Self).cast::<u8>().add(116usize).cast() }
    }
    #[doc = "0x76 - BCNT2 Capture Register 2"]
    #[inline(always)]
    pub const fn bcnt2cp2(&self) -> &BCNT2CP {
        unsafe { &*(self as *const Self).cast::<u8>().add(118usize).cast() }
    }
    #[doc = "0x76 - Hour Capture Register 2"]
    #[inline(always)]
    pub const fn rhrcp2(&self) -> &RHRCP {
        unsafe { &*(self as *const Self).cast::<u8>().add(118usize).cast() }
    }
    #[doc = "0x7a - BCNT3 Capture Register 2"]
    #[inline(always)]
    pub const fn bcnt3cp2(&self) -> &BCNT3CP {
        unsafe { &*(self as *const Self).cast::<u8>().add(122usize).cast() }
    }
    #[doc = "0x7a - Date Capture Register 2"]
    #[inline(always)]
    pub const fn rdaycp2(&self) -> &RDAYCP {
        unsafe { &*(self as *const Self).cast::<u8>().add(122usize).cast() }
    }
}
#[doc = "R64CNT (r) register accessor: 64-Hz Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r64cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r64cnt`]
module"]
pub type R64CNT = crate::Reg<r64cnt::R64CNT_SPEC>;
#[doc = "64-Hz Counter"]
pub mod r64cnt;
#[doc = "RSECCNT (rw) register accessor: Second Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rseccnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rseccnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rseccnt`]
module"]
pub type RSECCNT = crate::Reg<rseccnt::RSECCNT_SPEC>;
#[doc = "Second Counter"]
pub mod rseccnt;
#[doc = "BCNT0 (rw) register accessor: Binary Counter 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcnt0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcnt0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt0`]
module"]
pub type BCNT0 = crate::Reg<bcnt0::BCNT0_SPEC>;
#[doc = "Binary Counter 0"]
pub mod bcnt0;
#[doc = "RMINCNT (rw) register accessor: Minute Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rmincnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rmincnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmincnt`]
module"]
pub type RMINCNT = crate::Reg<rmincnt::RMINCNT_SPEC>;
#[doc = "Minute Counter"]
pub mod rmincnt;
#[doc = "BCNT1 (rw) register accessor: Binary Counter 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcnt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcnt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt1`]
module"]
pub type BCNT1 = crate::Reg<bcnt1::BCNT1_SPEC>;
#[doc = "Binary Counter 1"]
pub mod bcnt1;
#[doc = "RHRCNT (rw) register accessor: Hour Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rhrcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rhrcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rhrcnt`]
module"]
pub type RHRCNT = crate::Reg<rhrcnt::RHRCNT_SPEC>;
#[doc = "Hour Counter"]
pub mod rhrcnt;
#[doc = "BCNT2 (rw) register accessor: Binary Counter 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcnt2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcnt2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt2`]
module"]
pub type BCNT2 = crate::Reg<bcnt2::BCNT2_SPEC>;
#[doc = "Binary Counter 2"]
pub mod bcnt2;
#[doc = "RWKCNT (rw) register accessor: Day-of-Week Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rwkcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rwkcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rwkcnt`]
module"]
pub type RWKCNT = crate::Reg<rwkcnt::RWKCNT_SPEC>;
#[doc = "Day-of-Week Counter"]
pub mod rwkcnt;
#[doc = "BCNT3 (rw) register accessor: Binary Counter 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcnt3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcnt3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt3`]
module"]
pub type BCNT3 = crate::Reg<bcnt3::BCNT3_SPEC>;
#[doc = "Binary Counter 3"]
pub mod bcnt3;
#[doc = "RDAYCNT (rw) register accessor: Day Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdaycnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdaycnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdaycnt`]
module"]
pub type RDAYCNT = crate::Reg<rdaycnt::RDAYCNT_SPEC>;
#[doc = "Day Counter"]
pub mod rdaycnt;
#[doc = "RMONCNT (rw) register accessor: Month Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rmoncnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rmoncnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmoncnt`]
module"]
pub type RMONCNT = crate::Reg<rmoncnt::RMONCNT_SPEC>;
#[doc = "Month Counter"]
pub mod rmoncnt;
#[doc = "RYRCNT (rw) register accessor: Year Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ryrcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ryrcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ryrcnt`]
module"]
pub type RYRCNT = crate::Reg<ryrcnt::RYRCNT_SPEC>;
#[doc = "Year Counter"]
pub mod ryrcnt;
#[doc = "RSECAR (rw) register accessor: Second Alarm Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsecar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsecar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsecar`]
module"]
pub type RSECAR = crate::Reg<rsecar::RSECAR_SPEC>;
#[doc = "Second Alarm Register"]
pub mod rsecar;
#[doc = "BCNT0AR (rw) register accessor: Binary Counter 0 Alarm Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcnt0ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcnt0ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt0ar`]
module"]
pub type BCNT0AR = crate::Reg<bcnt0ar::BCNT0AR_SPEC>;
#[doc = "Binary Counter 0 Alarm Register"]
pub mod bcnt0ar;
#[doc = "RMINAR (rw) register accessor: Minute Alarm Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rminar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rminar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rminar`]
module"]
pub type RMINAR = crate::Reg<rminar::RMINAR_SPEC>;
#[doc = "Minute Alarm Register"]
pub mod rminar;
#[doc = "BCNT1AR (rw) register accessor: Binary Counter 1 Alarm Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcnt1ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcnt1ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt1ar`]
module"]
pub type BCNT1AR = crate::Reg<bcnt1ar::BCNT1AR_SPEC>;
#[doc = "Binary Counter 1 Alarm Register"]
pub mod bcnt1ar;
#[doc = "RHRAR (rw) register accessor: Hour Alarm Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rhrar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rhrar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rhrar`]
module"]
pub type RHRAR = crate::Reg<rhrar::RHRAR_SPEC>;
#[doc = "Hour Alarm Register"]
pub mod rhrar;
#[doc = "BCNT2AR (rw) register accessor: Binary Counter 2 Alarm Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcnt2ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcnt2ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt2ar`]
module"]
pub type BCNT2AR = crate::Reg<bcnt2ar::BCNT2AR_SPEC>;
#[doc = "Binary Counter 2 Alarm Register"]
pub mod bcnt2ar;
#[doc = "RWKAR (rw) register accessor: Day-of-Week Alarm Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rwkar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rwkar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rwkar`]
module"]
pub type RWKAR = crate::Reg<rwkar::RWKAR_SPEC>;
#[doc = "Day-of-Week Alarm Register"]
pub mod rwkar;
#[doc = "BCNT3AR (rw) register accessor: Binary Counter 3 Alarm Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcnt3ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcnt3ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt3ar`]
module"]
pub type BCNT3AR = crate::Reg<bcnt3ar::BCNT3AR_SPEC>;
#[doc = "Binary Counter 3 Alarm Register"]
pub mod bcnt3ar;
#[doc = "RDAYAR (rw) register accessor: Date Alarm Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdayar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdayar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdayar`]
module"]
pub type RDAYAR = crate::Reg<rdayar::RDAYAR_SPEC>;
#[doc = "Date Alarm Register"]
pub mod rdayar;
#[doc = "BCNT0AER (rw) register accessor: Binary Counter 0 Alarm Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcnt0aer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcnt0aer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt0aer`]
module"]
pub type BCNT0AER = crate::Reg<bcnt0aer::BCNT0AER_SPEC>;
#[doc = "Binary Counter 0 Alarm Enable Register"]
pub mod bcnt0aer;
#[doc = "RMONAR (rw) register accessor: Month Alarm Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rmonar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rmonar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmonar`]
module"]
pub type RMONAR = crate::Reg<rmonar::RMONAR_SPEC>;
#[doc = "Month Alarm Register"]
pub mod rmonar;
#[doc = "BCNT1AER (rw) register accessor: Binary Counter 1 Alarm Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcnt1aer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcnt1aer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt1aer`]
module"]
pub type BCNT1AER = crate::Reg<bcnt1aer::BCNT1AER_SPEC>;
#[doc = "Binary Counter 1 Alarm Enable Register"]
pub mod bcnt1aer;
#[doc = "RYRAR (rw) register accessor: Year Alarm Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ryrar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ryrar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ryrar`]
module"]
pub type RYRAR = crate::Reg<ryrar::RYRAR_SPEC>;
#[doc = "Year Alarm Register"]
pub mod ryrar;
#[doc = "BCNT2AER (rw) register accessor: Binary Counter 2 Alarm Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcnt2aer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcnt2aer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt2aer`]
module"]
pub type BCNT2AER = crate::Reg<bcnt2aer::BCNT2AER_SPEC>;
#[doc = "Binary Counter 2 Alarm Enable Register"]
pub mod bcnt2aer;
#[doc = "RYRAREN (rw) register accessor: Year Alarm Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ryraren::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ryraren::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ryraren`]
module"]
pub type RYRAREN = crate::Reg<ryraren::RYRAREN_SPEC>;
#[doc = "Year Alarm Enable Register"]
pub mod ryraren;
#[doc = "BCNT3AER (rw) register accessor: Binary Counter 3 Alarm Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcnt3aer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcnt3aer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt3aer`]
module"]
pub type BCNT3AER = crate::Reg<bcnt3aer::BCNT3AER_SPEC>;
#[doc = "Binary Counter 3 Alarm Enable Register"]
pub mod bcnt3aer;
#[doc = "RCR1 (rw) register accessor: RTC Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr1`]
module"]
pub type RCR1 = crate::Reg<rcr1::RCR1_SPEC>;
#[doc = "RTC Control Register 1"]
pub mod rcr1;
#[doc = "RCR2 (rw) register accessor: RTC Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr2`]
module"]
pub type RCR2 = crate::Reg<rcr2::RCR2_SPEC>;
#[doc = "RTC Control Register 2"]
pub mod rcr2;
#[doc = "RCR4 (rw) register accessor: RTC Control Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr4`]
module"]
pub type RCR4 = crate::Reg<rcr4::RCR4_SPEC>;
#[doc = "RTC Control Register 4"]
pub mod rcr4;
#[doc = "RFRH (rw) register accessor: Frequency Register H\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfrh`]
module"]
pub type RFRH = crate::Reg<rfrh::RFRH_SPEC>;
#[doc = "Frequency Register H"]
pub mod rfrh;
#[doc = "RFRL (rw) register accessor: Frequency Register L\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfrl`]
module"]
pub type RFRL = crate::Reg<rfrl::RFRL_SPEC>;
#[doc = "Frequency Register L"]
pub mod rfrl;
#[doc = "RADJ (rw) register accessor: Time Error Adjustment Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`radj::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`radj::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@radj`]
module"]
pub type RADJ = crate::Reg<radj::RADJ_SPEC>;
#[doc = "Time Error Adjustment Register"]
pub mod radj;
#[doc = "RTCCR (rw) register accessor: Time Capture Control Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccr`]
module"]
pub type RTCCR = crate::Reg<rtccr::RTCCR_SPEC>;
#[doc = "Time Capture Control Register %s"]
pub mod rtccr;
#[doc = "RSECCP (r) register accessor: Second Capture Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rseccp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rseccp`]
module"]
pub type RSECCP = crate::Reg<rseccp::RSECCP_SPEC>;
#[doc = "Second Capture Register %s"]
pub mod rseccp;
#[doc = "BCNT0CP (r) register accessor: BCNT0 Capture Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcnt0cp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt0cp`]
module"]
pub type BCNT0CP = crate::Reg<bcnt0cp::BCNT0CP_SPEC>;
#[doc = "BCNT0 Capture Register %s"]
pub mod bcnt0cp;
#[doc = "RMINCP (r) register accessor: Minute Capture Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rmincp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmincp`]
module"]
pub type RMINCP = crate::Reg<rmincp::RMINCP_SPEC>;
#[doc = "Minute Capture Register %s"]
pub mod rmincp;
#[doc = "BCNT1CP (r) register accessor: BCNT1 Capture Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcnt1cp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt1cp`]
module"]
pub type BCNT1CP = crate::Reg<bcnt1cp::BCNT1CP_SPEC>;
#[doc = "BCNT1 Capture Register %s"]
pub mod bcnt1cp;
#[doc = "RHRCP (r) register accessor: Hour Capture Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rhrcp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rhrcp`]
module"]
pub type RHRCP = crate::Reg<rhrcp::RHRCP_SPEC>;
#[doc = "Hour Capture Register %s"]
pub mod rhrcp;
#[doc = "BCNT2CP (r) register accessor: BCNT2 Capture Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcnt2cp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt2cp`]
module"]
pub type BCNT2CP = crate::Reg<bcnt2cp::BCNT2CP_SPEC>;
#[doc = "BCNT2 Capture Register %s"]
pub mod bcnt2cp;
#[doc = "RDAYCP (r) register accessor: Date Capture Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdaycp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdaycp`]
module"]
pub type RDAYCP = crate::Reg<rdaycp::RDAYCP_SPEC>;
#[doc = "Date Capture Register %s"]
pub mod rdaycp;
#[doc = "BCNT3CP (r) register accessor: BCNT3 Capture Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcnt3cp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt3cp`]
module"]
pub type BCNT3CP = crate::Reg<bcnt3cp::BCNT3CP_SPEC>;
#[doc = "BCNT3 Capture Register %s"]
pub mod bcnt3cp;
#[doc = "RMONCP (r) register accessor: Month Capture Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rmoncp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmoncp`]
module"]
pub type RMONCP = crate::Reg<rmoncp::RMONCP_SPEC>;
#[doc = "Month Capture Register %s"]
pub mod rmoncp;
