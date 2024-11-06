#[doc = "Register `USBCKCR` reader"]
pub type R = crate::R<USBCKCR_SPEC>;
#[doc = "Register `USBCKCR` writer"]
pub type W = crate::W<USBCKCR_SPEC>;
#[doc = "Field `USBCLKSEL` reader - USB Clock Source Select"]
pub type USBCLKSEL_R = crate::BitReader<USBCLKSEL_A>;
#[doc = "USB Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBCLKSEL_A {
    #[doc = "0: PLL(Value after reset)"]
    _0 = 0,
    #[doc = "1: HOCO"]
    _1 = 1,
}
impl From<USBCLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: USBCLKSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl USBCLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USBCLKSEL_A {
        match self.bits {
            false => USBCLKSEL_A::_0,
            true => USBCLKSEL_A::_1,
        }
    }
    #[doc = "PLL(Value after reset)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBCLKSEL_A::_0
    }
    #[doc = "HOCO"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBCLKSEL_A::_1
    }
}
#[doc = "Field `USBCLKSEL` writer - USB Clock Source Select"]
pub type USBCLKSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USBCLKSEL_A>;
impl<'a, REG, const O: u8> USBCLKSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL(Value after reset)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USBCLKSEL_A::_0)
    }
    #[doc = "HOCO"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USBCLKSEL_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - USB Clock Source Select"]
    #[inline(always)]
    pub fn usbclksel(&self) -> USBCLKSEL_R {
        USBCLKSEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn usbclksel(&mut self) -> USBCLKSEL_W<USBCKCR_SPEC, 0> {
        USBCLKSEL_W::new(self)
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
#[doc = "USB Clock Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbckcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbckcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBCKCR_SPEC;
impl crate::RegisterSpec for USBCKCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usbckcr::R`](R) reader structure"]
impl crate::Readable for USBCKCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbckcr::W`](W) writer structure"]
impl crate::Writable for USBCKCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBCKCR to value 0"]
impl crate::Resettable for USBCKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
