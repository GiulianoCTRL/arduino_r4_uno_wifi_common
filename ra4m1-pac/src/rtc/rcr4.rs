#[doc = "Register `RCR4` reader"]
pub type R = crate::R<RCR4_SPEC>;
#[doc = "Register `RCR4` writer"]
pub type W = crate::W<RCR4_SPEC>;
#[doc = "Field `RCKSEL` reader - Count Source Select"]
pub type RCKSEL_R = crate::BitReader<RCKSEL_A>;
#[doc = "Count Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCKSEL_A {
    #[doc = "0: Sub-clock oscillator is selected."]
    _0 = 0,
    #[doc = "1: LOCO clock oscillator is selected."]
    _1 = 1,
}
impl From<RCKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: RCKSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl RCKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RCKSEL_A {
        match self.bits {
            false => RCKSEL_A::_0,
            true => RCKSEL_A::_1,
        }
    }
    #[doc = "Sub-clock oscillator is selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCKSEL_A::_0
    }
    #[doc = "LOCO clock oscillator is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCKSEL_A::_1
    }
}
#[doc = "Field `RCKSEL` writer - Count Source Select"]
pub type RCKSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RCKSEL_A>;
impl<'a, REG, const O: u8> RCKSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sub-clock oscillator is selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RCKSEL_A::_0)
    }
    #[doc = "LOCO clock oscillator is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RCKSEL_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Count Source Select"]
    #[inline(always)]
    pub fn rcksel(&self) -> RCKSEL_R {
        RCKSEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Count Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn rcksel(&mut self) -> RCKSEL_W<RCR4_SPEC, 0> {
        RCKSEL_W::new(self)
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
#[doc = "RTC Control Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCR4_SPEC;
impl crate::RegisterSpec for RCR4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rcr4::R`](R) reader structure"]
impl crate::Readable for RCR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rcr4::W`](W) writer structure"]
impl crate::Writable for RCR4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCR4 to value 0"]
impl crate::Resettable for RCR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
