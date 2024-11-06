#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - General PWM Timer Write-Protection Register"]
    pub gtwp: GTWP,
    #[doc = "0x04 - General PWM Timer Software Start Register"]
    pub gtstr: GTSTR,
    #[doc = "0x08 - General PWM Timer Software Stop Register"]
    pub gtstp: GTSTP,
    #[doc = "0x0c - General PWM Timer Software Clear Register"]
    pub gtclr: GTCLR,
    #[doc = "0x10 - General PWM Timer Start Source Select Register"]
    pub gtssr: GTSSR,
    #[doc = "0x14 - General PWM Timer Stop Source Select Register"]
    pub gtpsr: GTPSR,
    #[doc = "0x18 - General PWM Timer Clear Source Select Register"]
    pub gtcsr: GTCSR,
    #[doc = "0x1c - General PWM Timer Up Count Source Select Register"]
    pub gtupsr: GTUPSR,
    #[doc = "0x20 - General PWM Timer Down Count Source Select Register"]
    pub gtdnsr: GTDNSR,
    #[doc = "0x24 - General PWM Timer Input Capture Source Select Register A"]
    pub gticasr: GTICASR,
    #[doc = "0x28 - General PWM Timer Input Capture Source Select Register B"]
    pub gticbsr: GTICBSR,
    #[doc = "0x2c - General PWM Timer Control Register"]
    pub gtcr: GTCR,
    #[doc = "0x30 - General PWM Timer Count Direction and Duty Setting Register"]
    pub gtuddtyc: GTUDDTYC,
    #[doc = "0x34 - General PWM Timer I/O Control Register"]
    pub gtior: GTIOR,
    #[doc = "0x38 - General PWM Timer Interrupt Output Setting Register"]
    pub gtintad: GTINTAD,
    #[doc = "0x3c - General PWM Timer Status Register"]
    pub gtst: GTST,
    #[doc = "0x40 - General PWM Timer Buffer Enable Register"]
    pub gtber: GTBER,
    _reserved17: [u8; 0x04],
    #[doc = "0x48 - General PWM Timer Counter"]
    pub gtcnt: GTCNT,
    #[doc = "0x4c - General PWM Timer Compare Capture Register A"]
    pub gtccra: GTCCRA,
    #[doc = "0x50 - General PWM Timer Compare Capture Register B"]
    pub gtccrb: GTCCRB,
    #[doc = "0x54 - General PWM Timer Compare Capture Register C"]
    pub gtccrc: GTCCRC,
    #[doc = "0x58 - General PWM Timer Compare Capture Register E"]
    pub gtccre: GTCCRE,
    #[doc = "0x5c - General PWM Timer Compare Capture Register D"]
    pub gtccrd: GTCCRD,
    #[doc = "0x60 - General PWM Timer Compare Capture Register F"]
    pub gtccrf: GTCCRF,
    #[doc = "0x64 - General PWM Timer Cycle Setting Register"]
    pub gtpr: GTPR,
    #[doc = "0x68 - General PWM Timer Cycle Setting Buffer Register"]
    pub gtpbr: GTPBR,
    _reserved26: [u8; 0x1c],
    #[doc = "0x88 - General PWM Timer Dead Time Control Register"]
    pub gtdtcr: GTDTCR,
    #[doc = "0x8c - General PWM Timer Dead Time Value Register U"]
    pub gtdvu: GTDVU,
}
#[doc = "GTWP (rw) register accessor: General PWM Timer Write-Protection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtwp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtwp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtwp`]
module"]
pub type GTWP = crate::Reg<gtwp::GTWP_SPEC>;
#[doc = "General PWM Timer Write-Protection Register"]
pub mod gtwp;
#[doc = "GTSTR (rw) register accessor: General PWM Timer Software Start Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtstr`]
module"]
pub type GTSTR = crate::Reg<gtstr::GTSTR_SPEC>;
#[doc = "General PWM Timer Software Start Register"]
pub mod gtstr;
#[doc = "GTSTP (rw) register accessor: General PWM Timer Software Stop Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtstp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtstp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtstp`]
module"]
pub type GTSTP = crate::Reg<gtstp::GTSTP_SPEC>;
#[doc = "General PWM Timer Software Stop Register"]
pub mod gtstp;
#[doc = "GTCLR (w) register accessor: General PWM Timer Software Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtclr`]
module"]
pub type GTCLR = crate::Reg<gtclr::GTCLR_SPEC>;
#[doc = "General PWM Timer Software Clear Register"]
pub mod gtclr;
#[doc = "GTSSR (rw) register accessor: General PWM Timer Start Source Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtssr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtssr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtssr`]
module"]
pub type GTSSR = crate::Reg<gtssr::GTSSR_SPEC>;
#[doc = "General PWM Timer Start Source Select Register"]
pub mod gtssr;
#[doc = "GTPSR (rw) register accessor: General PWM Timer Stop Source Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtpsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtpsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtpsr`]
module"]
pub type GTPSR = crate::Reg<gtpsr::GTPSR_SPEC>;
#[doc = "General PWM Timer Stop Source Select Register"]
pub mod gtpsr;
#[doc = "GTCSR (rw) register accessor: General PWM Timer Clear Source Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtcsr`]
module"]
pub type GTCSR = crate::Reg<gtcsr::GTCSR_SPEC>;
#[doc = "General PWM Timer Clear Source Select Register"]
pub mod gtcsr;
#[doc = "GTUPSR (rw) register accessor: General PWM Timer Up Count Source Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtupsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtupsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtupsr`]
module"]
pub type GTUPSR = crate::Reg<gtupsr::GTUPSR_SPEC>;
#[doc = "General PWM Timer Up Count Source Select Register"]
pub mod gtupsr;
#[doc = "GTDNSR (rw) register accessor: General PWM Timer Down Count Source Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtdnsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtdnsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtdnsr`]
module"]
pub type GTDNSR = crate::Reg<gtdnsr::GTDNSR_SPEC>;
#[doc = "General PWM Timer Down Count Source Select Register"]
pub mod gtdnsr;
#[doc = "GTICASR (rw) register accessor: General PWM Timer Input Capture Source Select Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gticasr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gticasr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gticasr`]
module"]
pub type GTICASR = crate::Reg<gticasr::GTICASR_SPEC>;
#[doc = "General PWM Timer Input Capture Source Select Register A"]
pub mod gticasr;
#[doc = "GTICBSR (rw) register accessor: General PWM Timer Input Capture Source Select Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gticbsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gticbsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gticbsr`]
module"]
pub type GTICBSR = crate::Reg<gticbsr::GTICBSR_SPEC>;
#[doc = "General PWM Timer Input Capture Source Select Register B"]
pub mod gticbsr;
#[doc = "GTCR (rw) register accessor: General PWM Timer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtcr`]
module"]
pub type GTCR = crate::Reg<gtcr::GTCR_SPEC>;
#[doc = "General PWM Timer Control Register"]
pub mod gtcr;
#[doc = "GTUDDTYC (rw) register accessor: General PWM Timer Count Direction and Duty Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtuddtyc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtuddtyc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtuddtyc`]
module"]
pub type GTUDDTYC = crate::Reg<gtuddtyc::GTUDDTYC_SPEC>;
#[doc = "General PWM Timer Count Direction and Duty Setting Register"]
pub mod gtuddtyc;
#[doc = "GTIOR (rw) register accessor: General PWM Timer I/O Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtior::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtior::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtior`]
module"]
pub type GTIOR = crate::Reg<gtior::GTIOR_SPEC>;
#[doc = "General PWM Timer I/O Control Register"]
pub mod gtior;
#[doc = "GTINTAD (rw) register accessor: General PWM Timer Interrupt Output Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtintad::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtintad::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtintad`]
module"]
pub type GTINTAD = crate::Reg<gtintad::GTINTAD_SPEC>;
#[doc = "General PWM Timer Interrupt Output Setting Register"]
pub mod gtintad;
#[doc = "GTST (rw) register accessor: General PWM Timer Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtst`]
module"]
pub type GTST = crate::Reg<gtst::GTST_SPEC>;
#[doc = "General PWM Timer Status Register"]
pub mod gtst;
#[doc = "GTBER (rw) register accessor: General PWM Timer Buffer Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtber::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtber::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtber`]
module"]
pub type GTBER = crate::Reg<gtber::GTBER_SPEC>;
#[doc = "General PWM Timer Buffer Enable Register"]
pub mod gtber;
#[doc = "GTCNT (rw) register accessor: General PWM Timer Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtcnt`]
module"]
pub type GTCNT = crate::Reg<gtcnt::GTCNT_SPEC>;
#[doc = "General PWM Timer Counter"]
pub mod gtcnt;
#[doc = "GTCCRA (rw) register accessor: General PWM Timer Compare Capture Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtccra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtccra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtccra`]
module"]
pub type GTCCRA = crate::Reg<gtccra::GTCCRA_SPEC>;
#[doc = "General PWM Timer Compare Capture Register A"]
pub mod gtccra;
#[doc = "GTCCRB (rw) register accessor: General PWM Timer Compare Capture Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtccrb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtccrb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtccrb`]
module"]
pub type GTCCRB = crate::Reg<gtccrb::GTCCRB_SPEC>;
#[doc = "General PWM Timer Compare Capture Register B"]
pub mod gtccrb;
#[doc = "GTCCRC (rw) register accessor: General PWM Timer Compare Capture Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtccrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtccrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtccrc`]
module"]
pub type GTCCRC = crate::Reg<gtccrc::GTCCRC_SPEC>;
#[doc = "General PWM Timer Compare Capture Register C"]
pub mod gtccrc;
#[doc = "GTCCRE (rw) register accessor: General PWM Timer Compare Capture Register E\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtccre::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtccre::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtccre`]
module"]
pub type GTCCRE = crate::Reg<gtccre::GTCCRE_SPEC>;
#[doc = "General PWM Timer Compare Capture Register E"]
pub mod gtccre;
#[doc = "GTCCRD (rw) register accessor: General PWM Timer Compare Capture Register D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtccrd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtccrd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtccrd`]
module"]
pub type GTCCRD = crate::Reg<gtccrd::GTCCRD_SPEC>;
#[doc = "General PWM Timer Compare Capture Register D"]
pub mod gtccrd;
#[doc = "GTCCRF (rw) register accessor: General PWM Timer Compare Capture Register F\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtccrf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtccrf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtccrf`]
module"]
pub type GTCCRF = crate::Reg<gtccrf::GTCCRF_SPEC>;
#[doc = "General PWM Timer Compare Capture Register F"]
pub mod gtccrf;
#[doc = "GTPR (rw) register accessor: General PWM Timer Cycle Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtpr`]
module"]
pub type GTPR = crate::Reg<gtpr::GTPR_SPEC>;
#[doc = "General PWM Timer Cycle Setting Register"]
pub mod gtpr;
#[doc = "GTPBR (rw) register accessor: General PWM Timer Cycle Setting Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtpbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtpbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtpbr`]
module"]
pub type GTPBR = crate::Reg<gtpbr::GTPBR_SPEC>;
#[doc = "General PWM Timer Cycle Setting Buffer Register"]
pub mod gtpbr;
#[doc = "GTDTCR (rw) register accessor: General PWM Timer Dead Time Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtdtcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtdtcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtdtcr`]
module"]
pub type GTDTCR = crate::Reg<gtdtcr::GTDTCR_SPEC>;
#[doc = "General PWM Timer Dead Time Control Register"]
pub mod gtdtcr;
#[doc = "GTDVU (rw) register accessor: General PWM Timer Dead Time Value Register U\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtdvu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtdvu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtdvu`]
module"]
pub type GTDVU = crate::Reg<gtdvu::GTDVU_SPEC>;
#[doc = "General PWM Timer Dead Time Value Register U"]
pub mod gtdvu;
