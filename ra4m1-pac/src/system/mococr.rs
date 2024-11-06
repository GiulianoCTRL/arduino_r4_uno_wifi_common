#[doc = "Register `MOCOCR` reader"]
pub type R = crate::R<MOCOCR_SPEC>;
#[doc = "Register `MOCOCR` writer"]
pub type W = crate::W<MOCOCR_SPEC>;
#[doc = "Field `MCSTP` reader - MOCO Stop"]
pub type MCSTP_R = crate::BitReader<MCSTP_A>;
#[doc = "MOCO Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCSTP_A {
    #[doc = "0: MOCO is operating."]
    _0 = 0,
    #[doc = "1: MOCO is stopped."]
    _1 = 1,
}
impl From<MCSTP_A> for bool {
    #[inline(always)]
    fn from(variant: MCSTP_A) -> Self {
        variant as u8 != 0
    }
}
impl MCSTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MCSTP_A {
        match self.bits {
            false => MCSTP_A::_0,
            true => MCSTP_A::_1,
        }
    }
    #[doc = "MOCO is operating."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MCSTP_A::_0
    }
    #[doc = "MOCO is stopped."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MCSTP_A::_1
    }
}
#[doc = "Field `MCSTP` writer - MOCO Stop"]
pub type MCSTP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MCSTP_A>;
impl<'a, REG, const O: u8> MCSTP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MOCO is operating."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MCSTP_A::_0)
    }
    #[doc = "MOCO is stopped."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MCSTP_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - MOCO Stop"]
    #[inline(always)]
    pub fn mcstp(&self) -> MCSTP_R {
        MCSTP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MOCO Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mcstp(&mut self) -> MCSTP_W<MOCOCR_SPEC, 0> {
        MCSTP_W::new(self)
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
#[doc = "Middle-Speed On-Chip Oscillator Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mococr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mococr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MOCOCR_SPEC;
impl crate::RegisterSpec for MOCOCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mococr::R`](R) reader structure"]
impl crate::Readable for MOCOCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mococr::W`](W) writer structure"]
impl crate::Writable for MOCOCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MOCOCR to value 0"]
impl crate::Resettable for MOCOCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
