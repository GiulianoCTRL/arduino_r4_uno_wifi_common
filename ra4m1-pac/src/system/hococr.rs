#[doc = "Register `HOCOCR` reader"]
pub type R = crate::R<HOCOCR_SPEC>;
#[doc = "Register `HOCOCR` writer"]
pub type W = crate::W<HOCOCR_SPEC>;
#[doc = "Field `HCSTP` reader - HOCO Stop"]
pub type HCSTP_R = crate::BitReader<HCSTP_A>;
#[doc = "HOCO Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HCSTP_A {
    #[doc = "0: HOCO is operating."]
    _0 = 0,
    #[doc = "1: HOCO is stopped."]
    _1 = 1,
}
impl From<HCSTP_A> for bool {
    #[inline(always)]
    fn from(variant: HCSTP_A) -> Self {
        variant as u8 != 0
    }
}
impl HCSTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HCSTP_A {
        match self.bits {
            false => HCSTP_A::_0,
            true => HCSTP_A::_1,
        }
    }
    #[doc = "HOCO is operating."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HCSTP_A::_0
    }
    #[doc = "HOCO is stopped."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HCSTP_A::_1
    }
}
#[doc = "Field `HCSTP` writer - HOCO Stop"]
pub type HCSTP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HCSTP_A>;
impl<'a, REG, const O: u8> HCSTP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HOCO is operating."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(HCSTP_A::_0)
    }
    #[doc = "HOCO is stopped."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(HCSTP_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - HOCO Stop"]
    #[inline(always)]
    pub fn hcstp(&self) -> HCSTP_R {
        HCSTP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HOCO Stop"]
    #[inline(always)]
    #[must_use]
    pub fn hcstp(&mut self) -> HCSTP_W<HOCOCR_SPEC, 0> {
        HCSTP_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "High-Speed On-Chip Oscillator Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hococr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hococr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOCOCR_SPEC;
impl crate::RegisterSpec for HOCOCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hococr::R`](R) reader structure"]
impl crate::Readable for HOCOCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hococr::W`](W) writer structure"]
impl crate::Writable for HOCOCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOCOCR to value 0"]
impl crate::Resettable for HOCOCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
