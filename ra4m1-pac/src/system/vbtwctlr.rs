#[doc = "Register `VBTWCTLR` reader"]
pub type R = crate::R<VBTWCTLR_SPEC>;
#[doc = "Register `VBTWCTLR` writer"]
pub type W = crate::W<VBTWCTLR_SPEC>;
#[doc = "Field `VWEN` reader - VBATT wakeup enable"]
pub type VWEN_R = crate::BitReader<VWEN_A>;
#[doc = "VBATT wakeup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VWEN_A {
    #[doc = "0: Disable Wakeup function"]
    _0 = 0,
    #[doc = "1: Enable Wakeup function"]
    _1 = 1,
}
impl From<VWEN_A> for bool {
    #[inline(always)]
    fn from(variant: VWEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VWEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VWEN_A {
        match self.bits {
            false => VWEN_A::_0,
            true => VWEN_A::_1,
        }
    }
    #[doc = "Disable Wakeup function"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VWEN_A::_0
    }
    #[doc = "Enable Wakeup function"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VWEN_A::_1
    }
}
#[doc = "Field `VWEN` writer - VBATT wakeup enable"]
pub type VWEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VWEN_A>;
impl<'a, REG, const O: u8> VWEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Wakeup function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VWEN_A::_0)
    }
    #[doc = "Enable Wakeup function"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VWEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - VBATT wakeup enable"]
    #[inline(always)]
    pub fn vwen(&self) -> VWEN_R {
        VWEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBATT wakeup enable"]
    #[inline(always)]
    #[must_use]
    pub fn vwen(&mut self) -> VWEN_W<VBTWCTLR_SPEC, 0> {
        VWEN_W::new(self)
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
#[doc = "VBATT Wakeup function Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbtwctlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbtwctlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VBTWCTLR_SPEC;
impl crate::RegisterSpec for VBTWCTLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vbtwctlr::R`](R) reader structure"]
impl crate::Readable for VBTWCTLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vbtwctlr::W`](W) writer structure"]
impl crate::Writable for VBTWCTLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VBTWCTLR to value 0"]
impl crate::Resettable for VBTWCTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
