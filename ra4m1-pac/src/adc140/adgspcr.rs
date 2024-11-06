#[doc = "Register `ADGSPCR` reader"]
pub type R = crate::R<ADGSPCR_SPEC>;
#[doc = "Register `ADGSPCR` writer"]
pub type W = crate::W<ADGSPCR_SPEC>;
#[doc = "Field `PGS` reader - Group A priority control setting bit. Note: When the PGS bit is to be set to 1, the ADCSR.ADCS\\[1:0\\]
bits must be set to 01b (group scan mode). If the bits are set to any other values, proper operation is not guaranteed."]
pub type PGS_R = crate::BitReader<PGS_A>;
#[doc = "Group A priority control setting bit. Note: When the PGS bit is to be set to 1, the ADCSR.ADCS\\[1:0\\]
bits must be set to 01b (group scan mode). If the bits are set to any other values, proper operation is not guaranteed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGS_A {
    #[doc = "0: Operation is without group A priority control"]
    _0 = 0,
    #[doc = "1: Operation is with group A priority control"]
    _1 = 1,
}
impl From<PGS_A> for bool {
    #[inline(always)]
    fn from(variant: PGS_A) -> Self {
        variant as u8 != 0
    }
}
impl PGS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PGS_A {
        match self.bits {
            false => PGS_A::_0,
            true => PGS_A::_1,
        }
    }
    #[doc = "Operation is without group A priority control"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PGS_A::_0
    }
    #[doc = "Operation is with group A priority control"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PGS_A::_1
    }
}
#[doc = "Field `PGS` writer - Group A priority control setting bit. Note: When the PGS bit is to be set to 1, the ADCSR.ADCS\\[1:0\\]
bits must be set to 01b (group scan mode). If the bits are set to any other values, proper operation is not guaranteed."]
pub type PGS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PGS_A>;
impl<'a, REG, const O: u8> PGS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Operation is without group A priority control"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PGS_A::_0)
    }
    #[doc = "Operation is with group A priority control"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PGS_A::_1)
    }
}
#[doc = "Field `GBRSCN` reader - Group B Restart Setting (Enabled only when PGS = 1. Reserved when PGS = 0.)"]
pub type GBRSCN_R = crate::BitReader<GBRSCN_A>;
#[doc = "Group B Restart Setting (Enabled only when PGS = 1. Reserved when PGS = 0.)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GBRSCN_A {
    #[doc = "0: Scanning for group B is not restarted after having been discontinued due to group A priority control."]
    _0 = 0,
    #[doc = "1: Scanning for group B is restarted after having been discontinued due to group A priority control."]
    _1 = 1,
}
impl From<GBRSCN_A> for bool {
    #[inline(always)]
    fn from(variant: GBRSCN_A) -> Self {
        variant as u8 != 0
    }
}
impl GBRSCN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GBRSCN_A {
        match self.bits {
            false => GBRSCN_A::_0,
            true => GBRSCN_A::_1,
        }
    }
    #[doc = "Scanning for group B is not restarted after having been discontinued due to group A priority control."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GBRSCN_A::_0
    }
    #[doc = "Scanning for group B is restarted after having been discontinued due to group A priority control."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GBRSCN_A::_1
    }
}
#[doc = "Field `GBRSCN` writer - Group B Restart Setting (Enabled only when PGS = 1. Reserved when PGS = 0.)"]
pub type GBRSCN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, GBRSCN_A>;
impl<'a, REG, const O: u8> GBRSCN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Scanning for group B is not restarted after having been discontinued due to group A priority control."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(GBRSCN_A::_0)
    }
    #[doc = "Scanning for group B is restarted after having been discontinued due to group A priority control."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(GBRSCN_A::_1)
    }
}
#[doc = "Field `GBRP` reader - Group B Single Scan Continuous Start (Enabled only when PGS = 1. Reserved when PGS = 0.) Note: When the GBRP bit has been set to 1, single scan is performed continuously for group B regardless of the setting of the GBRSCN bit."]
pub type GBRP_R = crate::BitReader<GBRP_A>;
#[doc = "Group B Single Scan Continuous Start (Enabled only when PGS = 1. Reserved when PGS = 0.) Note: When the GBRP bit has been set to 1, single scan is performed continuously for group B regardless of the setting of the GBRSCN bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GBRP_A {
    #[doc = "0: Single scan for group B is not continuously activated."]
    _0 = 0,
    #[doc = "1: Single scan for group B is continuously activated."]
    _1 = 1,
}
impl From<GBRP_A> for bool {
    #[inline(always)]
    fn from(variant: GBRP_A) -> Self {
        variant as u8 != 0
    }
}
impl GBRP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GBRP_A {
        match self.bits {
            false => GBRP_A::_0,
            true => GBRP_A::_1,
        }
    }
    #[doc = "Single scan for group B is not continuously activated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GBRP_A::_0
    }
    #[doc = "Single scan for group B is continuously activated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GBRP_A::_1
    }
}
#[doc = "Field `GBRP` writer - Group B Single Scan Continuous Start (Enabled only when PGS = 1. Reserved when PGS = 0.) Note: When the GBRP bit has been set to 1, single scan is performed continuously for group B regardless of the setting of the GBRSCN bit."]
pub type GBRP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, GBRP_A>;
impl<'a, REG, const O: u8> GBRP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single scan for group B is not continuously activated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(GBRP_A::_0)
    }
    #[doc = "Single scan for group B is continuously activated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(GBRP_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Group A priority control setting bit. Note: When the PGS bit is to be set to 1, the ADCSR.ADCS\\[1:0\\]
bits must be set to 01b (group scan mode). If the bits are set to any other values, proper operation is not guaranteed."]
    #[inline(always)]
    pub fn pgs(&self) -> PGS_R {
        PGS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Group B Restart Setting (Enabled only when PGS = 1. Reserved when PGS = 0.)"]
    #[inline(always)]
    pub fn gbrscn(&self) -> GBRSCN_R {
        GBRSCN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 15 - Group B Single Scan Continuous Start (Enabled only when PGS = 1. Reserved when PGS = 0.) Note: When the GBRP bit has been set to 1, single scan is performed continuously for group B regardless of the setting of the GBRSCN bit."]
    #[inline(always)]
    pub fn gbrp(&self) -> GBRP_R {
        GBRP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Group A priority control setting bit. Note: When the PGS bit is to be set to 1, the ADCSR.ADCS\\[1:0\\]
bits must be set to 01b (group scan mode). If the bits are set to any other values, proper operation is not guaranteed."]
    #[inline(always)]
    #[must_use]
    pub fn pgs(&mut self) -> PGS_W<ADGSPCR_SPEC, 0> {
        PGS_W::new(self)
    }
    #[doc = "Bit 1 - Group B Restart Setting (Enabled only when PGS = 1. Reserved when PGS = 0.)"]
    #[inline(always)]
    #[must_use]
    pub fn gbrscn(&mut self) -> GBRSCN_W<ADGSPCR_SPEC, 1> {
        GBRSCN_W::new(self)
    }
    #[doc = "Bit 15 - Group B Single Scan Continuous Start (Enabled only when PGS = 1. Reserved when PGS = 0.) Note: When the GBRP bit has been set to 1, single scan is performed continuously for group B regardless of the setting of the GBRSCN bit."]
    #[inline(always)]
    #[must_use]
    pub fn gbrp(&mut self) -> GBRP_W<ADGSPCR_SPEC, 15> {
        GBRP_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "A/D Group Scan Priority Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adgspcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adgspcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADGSPCR_SPEC;
impl crate::RegisterSpec for ADGSPCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adgspcr::R`](R) reader structure"]
impl crate::Readable for ADGSPCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adgspcr::W`](W) writer structure"]
impl crate::Writable for ADGSPCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADGSPCR to value 0"]
impl crate::Resettable for ADGSPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
