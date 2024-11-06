#[doc = "Register `PLLCR` reader"]
pub type R = crate::R<PLLCR_SPEC>;
#[doc = "Register `PLLCR` writer"]
pub type W = crate::W<PLLCR_SPEC>;
#[doc = "Field `PLLSTP` reader - PLL Stop Control"]
pub type PLLSTP_R = crate::BitReader<PLLSTP_A>;
#[doc = "PLL Stop Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSTP_A {
    #[doc = "0: PLL is operating."]
    _0 = 0,
    #[doc = "1: PLL is stopped."]
    _1 = 1,
}
impl From<PLLSTP_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSTP_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLSTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLSTP_A {
        match self.bits {
            false => PLLSTP_A::_0,
            true => PLLSTP_A::_1,
        }
    }
    #[doc = "PLL is operating."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLLSTP_A::_0
    }
    #[doc = "PLL is stopped."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLLSTP_A::_1
    }
}
#[doc = "Field `PLLSTP` writer - PLL Stop Control"]
pub type PLLSTP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PLLSTP_A>;
impl<'a, REG, const O: u8> PLLSTP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL is operating."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSTP_A::_0)
    }
    #[doc = "PLL is stopped."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSTP_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - PLL Stop Control"]
    #[inline(always)]
    pub fn pllstp(&self) -> PLLSTP_R {
        PLLSTP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL Stop Control"]
    #[inline(always)]
    #[must_use]
    pub fn pllstp(&mut self) -> PLLSTP_W<PLLCR_SPEC, 0> {
        PLLSTP_W::new(self)
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
#[doc = "PLL Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLCR_SPEC;
impl crate::RegisterSpec for PLLCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pllcr::R`](R) reader structure"]
impl crate::Readable for PLLCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pllcr::W`](W) writer structure"]
impl crate::Writable for PLLCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLCR to value 0x01"]
impl crate::Resettable for PLLCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
