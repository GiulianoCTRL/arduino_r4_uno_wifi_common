#[doc = "Register `OSTDCR` reader"]
pub type R = crate::R<OSTDCR_SPEC>;
#[doc = "Register `OSTDCR` writer"]
pub type W = crate::W<OSTDCR_SPEC>;
#[doc = "Field `OSTDIE` reader - Oscillation Stop Detection Interrupt Enable"]
pub type OSTDIE_R = crate::BitReader<OSTDIE_A>;
#[doc = "Oscillation Stop Detection Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSTDIE_A {
    #[doc = "0: The oscillation stop detection interrupt is disabled. Oscillation stop detection is not notified to the POEG."]
    _0 = 0,
    #[doc = "1: The oscillation stop detection interrupt is enabled. Oscillation stop detection is notified to the POEG."]
    _1 = 1,
}
impl From<OSTDIE_A> for bool {
    #[inline(always)]
    fn from(variant: OSTDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl OSTDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSTDIE_A {
        match self.bits {
            false => OSTDIE_A::_0,
            true => OSTDIE_A::_1,
        }
    }
    #[doc = "The oscillation stop detection interrupt is disabled. Oscillation stop detection is not notified to the POEG."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OSTDIE_A::_0
    }
    #[doc = "The oscillation stop detection interrupt is enabled. Oscillation stop detection is notified to the POEG."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OSTDIE_A::_1
    }
}
#[doc = "Field `OSTDIE` writer - Oscillation Stop Detection Interrupt Enable"]
pub type OSTDIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OSTDIE_A>;
impl<'a, REG, const O: u8> OSTDIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The oscillation stop detection interrupt is disabled. Oscillation stop detection is not notified to the POEG."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OSTDIE_A::_0)
    }
    #[doc = "The oscillation stop detection interrupt is enabled. Oscillation stop detection is notified to the POEG."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OSTDIE_A::_1)
    }
}
#[doc = "Field `OSTDE` reader - Oscillation Stop Detection Function Enable"]
pub type OSTDE_R = crate::BitReader<OSTDE_A>;
#[doc = "Oscillation Stop Detection Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSTDE_A {
    #[doc = "0: Oscillation stop detection function is disabled."]
    _0 = 0,
    #[doc = "1: Oscillation stop detection function is enabled."]
    _1 = 1,
}
impl From<OSTDE_A> for bool {
    #[inline(always)]
    fn from(variant: OSTDE_A) -> Self {
        variant as u8 != 0
    }
}
impl OSTDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSTDE_A {
        match self.bits {
            false => OSTDE_A::_0,
            true => OSTDE_A::_1,
        }
    }
    #[doc = "Oscillation stop detection function is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OSTDE_A::_0
    }
    #[doc = "Oscillation stop detection function is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OSTDE_A::_1
    }
}
#[doc = "Field `OSTDE` writer - Oscillation Stop Detection Function Enable"]
pub type OSTDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OSTDE_A>;
impl<'a, REG, const O: u8> OSTDE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Oscillation stop detection function is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OSTDE_A::_0)
    }
    #[doc = "Oscillation stop detection function is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OSTDE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Oscillation Stop Detection Interrupt Enable"]
    #[inline(always)]
    pub fn ostdie(&self) -> OSTDIE_R {
        OSTDIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Oscillation Stop Detection Function Enable"]
    #[inline(always)]
    pub fn ostde(&self) -> OSTDE_R {
        OSTDE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Oscillation Stop Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ostdie(&mut self) -> OSTDIE_W<OSTDCR_SPEC, 0> {
        OSTDIE_W::new(self)
    }
    #[doc = "Bit 7 - Oscillation Stop Detection Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ostde(&mut self) -> OSTDE_W<OSTDCR_SPEC, 7> {
        OSTDE_W::new(self)
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
#[doc = "Oscillation Stop Detection Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ostdcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ostdcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSTDCR_SPEC;
impl crate::RegisterSpec for OSTDCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ostdcr::R`](R) reader structure"]
impl crate::Readable for OSTDCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ostdcr::W`](W) writer structure"]
impl crate::Writable for OSTDCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSTDCR to value 0"]
impl crate::Resettable for OSTDCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
