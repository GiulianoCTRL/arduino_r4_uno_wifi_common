#[doc = "Register `SBYCR` reader"]
pub type R = crate::R<SBYCR_SPEC>;
#[doc = "Register `SBYCR` writer"]
pub type W = crate::W<SBYCR_SPEC>;
#[doc = "Field `SSBY` reader - Software Standby"]
pub type SSBY_R = crate::BitReader<SSBY_A>;
#[doc = "Software Standby\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSBY_A {
    #[doc = "0: Sleep mode"]
    _0 = 0,
    #[doc = "1: Software Standby mode"]
    _1 = 1,
}
impl From<SSBY_A> for bool {
    #[inline(always)]
    fn from(variant: SSBY_A) -> Self {
        variant as u8 != 0
    }
}
impl SSBY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSBY_A {
        match self.bits {
            false => SSBY_A::_0,
            true => SSBY_A::_1,
        }
    }
    #[doc = "Sleep mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSBY_A::_0
    }
    #[doc = "Software Standby mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSBY_A::_1
    }
}
#[doc = "Field `SSBY` writer - Software Standby"]
pub type SSBY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SSBY_A>;
impl<'a, REG, const O: u8> SSBY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sleep mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSBY_A::_0)
    }
    #[doc = "Software Standby mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSBY_A::_1)
    }
}
impl R {
    #[doc = "Bit 15 - Software Standby"]
    #[inline(always)]
    pub fn ssby(&self) -> SSBY_R {
        SSBY_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Software Standby"]
    #[inline(always)]
    #[must_use]
    pub fn ssby(&mut self) -> SSBY_W<SBYCR_SPEC, 15> {
        SSBY_W::new(self)
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
#[doc = "Standby Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sbycr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sbycr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SBYCR_SPEC;
impl crate::RegisterSpec for SBYCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sbycr::R`](R) reader structure"]
impl crate::Readable for SBYCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sbycr::W`](W) writer structure"]
impl crate::Writable for SBYCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SBYCR to value 0x4000"]
impl crate::Resettable for SBYCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000;
}
