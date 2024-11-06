#[doc = "Register `OSCSF` reader"]
pub type R = crate::R<OSCSF_SPEC>;
#[doc = "Field `HOCOSF` reader - HOCO Clock Oscillation Stabilization Flag NOTE: The HOCOSF bit value after a reset is 1 when the OFS1.HOCOEN bit is 0. It is 0 when the OFS1.HOCOEN bit is 1."]
pub type HOCOSF_R = crate::BitReader<HOCOSF_A>;
#[doc = "HOCO Clock Oscillation Stabilization Flag NOTE: The HOCOSF bit value after a reset is 1 when the OFS1.HOCOEN bit is 0. It is 0 when the OFS1.HOCOEN bit is 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HOCOSF_A {
    #[doc = "0: The HOCO clock is stopped or oscillation of the HOCO clock has not yet become stable."]
    _0 = 0,
    #[doc = "1: Oscillation of the HOCO clock is stable so the clock is available for use as the system clock."]
    _1 = 1,
}
impl From<HOCOSF_A> for bool {
    #[inline(always)]
    fn from(variant: HOCOSF_A) -> Self {
        variant as u8 != 0
    }
}
impl HOCOSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HOCOSF_A {
        match self.bits {
            false => HOCOSF_A::_0,
            true => HOCOSF_A::_1,
        }
    }
    #[doc = "The HOCO clock is stopped or oscillation of the HOCO clock has not yet become stable."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HOCOSF_A::_0
    }
    #[doc = "Oscillation of the HOCO clock is stable so the clock is available for use as the system clock."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HOCOSF_A::_1
    }
}
#[doc = "Field `MOSCSF` reader - Main Clock Oscillation Stabilization Flag"]
pub type MOSCSF_R = crate::BitReader<MOSCSF_A>;
#[doc = "Main Clock Oscillation Stabilization Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOSCSF_A {
    #[doc = "0: MOSTP = 1 (stopping the main clock oscillator) or oscillation of the main clock has not yet become stable."]
    _0 = 0,
    #[doc = "1: Oscillation of the main clock is stable so the clock is available for use as the system clock."]
    _1 = 1,
}
impl From<MOSCSF_A> for bool {
    #[inline(always)]
    fn from(variant: MOSCSF_A) -> Self {
        variant as u8 != 0
    }
}
impl MOSCSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MOSCSF_A {
        match self.bits {
            false => MOSCSF_A::_0,
            true => MOSCSF_A::_1,
        }
    }
    #[doc = "MOSTP = 1 (stopping the main clock oscillator) or oscillation of the main clock has not yet become stable."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MOSCSF_A::_0
    }
    #[doc = "Oscillation of the main clock is stable so the clock is available for use as the system clock."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MOSCSF_A::_1
    }
}
#[doc = "Field `PLLSF` reader - PLL Clock Oscillation Stabilization Flag"]
pub type PLLSF_R = crate::BitReader<PLLSF_A>;
#[doc = "PLL Clock Oscillation Stabilization Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSF_A {
    #[doc = "0: The PLL clock is stopped or oscillation of the PLL clock has not yet become stable."]
    _0 = 0,
    #[doc = "1: Oscillation of the PLL clock is stable so the clock is available for use as the system clock."]
    _1 = 1,
}
impl From<PLLSF_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSF_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLSF_A {
        match self.bits {
            false => PLLSF_A::_0,
            true => PLLSF_A::_1,
        }
    }
    #[doc = "The PLL clock is stopped or oscillation of the PLL clock has not yet become stable."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLLSF_A::_0
    }
    #[doc = "Oscillation of the PLL clock is stable so the clock is available for use as the system clock."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLLSF_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - HOCO Clock Oscillation Stabilization Flag NOTE: The HOCOSF bit value after a reset is 1 when the OFS1.HOCOEN bit is 0. It is 0 when the OFS1.HOCOEN bit is 1."]
    #[inline(always)]
    pub fn hocosf(&self) -> HOCOSF_R {
        HOCOSF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Main Clock Oscillation Stabilization Flag"]
    #[inline(always)]
    pub fn moscsf(&self) -> MOSCSF_R {
        MOSCSF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL Clock Oscillation Stabilization Flag"]
    #[inline(always)]
    pub fn pllsf(&self) -> PLLSF_R {
        PLLSF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Oscillation Stabilization Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oscsf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSCSF_SPEC;
impl crate::RegisterSpec for OSCSF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`oscsf::R`](R) reader structure"]
impl crate::Readable for OSCSF_SPEC {}
#[doc = "`reset()` method sets OSCSF to value 0"]
impl crate::Resettable for OSCSF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
