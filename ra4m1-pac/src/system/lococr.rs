#[doc = "Register `LOCOCR` reader"]
pub type R = crate::R<LOCOCR_SPEC>;
#[doc = "Register `LOCOCR` writer"]
pub type W = crate::W<LOCOCR_SPEC>;
#[doc = "Field `LCSTP` reader - LOCO Stop"]
pub type LCSTP_R = crate::BitReader<LCSTP_A>;
#[doc = "LOCO Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCSTP_A {
    #[doc = "0: LOCO is operating."]
    _0 = 0,
    #[doc = "1: LOCO is stopped."]
    _1 = 1,
}
impl From<LCSTP_A> for bool {
    #[inline(always)]
    fn from(variant: LCSTP_A) -> Self {
        variant as u8 != 0
    }
}
impl LCSTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCSTP_A {
        match self.bits {
            false => LCSTP_A::_0,
            true => LCSTP_A::_1,
        }
    }
    #[doc = "LOCO is operating."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LCSTP_A::_0
    }
    #[doc = "LOCO is stopped."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LCSTP_A::_1
    }
}
#[doc = "Field `LCSTP` writer - LOCO Stop"]
pub type LCSTP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LCSTP_A>;
impl<'a, REG, const O: u8> LCSTP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LOCO is operating."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LCSTP_A::_0)
    }
    #[doc = "LOCO is stopped."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LCSTP_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - LOCO Stop"]
    #[inline(always)]
    pub fn lcstp(&self) -> LCSTP_R {
        LCSTP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LOCO Stop"]
    #[inline(always)]
    #[must_use]
    pub fn lcstp(&mut self) -> LCSTP_W<LOCOCR_SPEC, 0> {
        LCSTP_W::new(self)
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
#[doc = "Low-Speed On-Chip Oscillator Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lococr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lococr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOCOCR_SPEC;
impl crate::RegisterSpec for LOCOCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lococr::R`](R) reader structure"]
impl crate::Readable for LOCOCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lococr::W`](W) writer structure"]
impl crate::Writable for LOCOCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOCOCR to value 0"]
impl crate::Resettable for LOCOCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
