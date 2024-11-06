#[doc = "Register `KRCTL` reader"]
pub type R = crate::R<KRCTL_SPEC>;
#[doc = "Register `KRCTL` writer"]
pub type W = crate::W<KRCTL_SPEC>;
#[doc = "Field `KREG` reader - Detection Edge Selection (KRF0 to KRF7)"]
pub type KREG_R = crate::BitReader<KREG_A>;
#[doc = "Detection Edge Selection (KRF0 to KRF7)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KREG_A {
    #[doc = "0: Falling edge"]
    _0 = 0,
    #[doc = "1: Rising edge"]
    _1 = 1,
}
impl From<KREG_A> for bool {
    #[inline(always)]
    fn from(variant: KREG_A) -> Self {
        variant as u8 != 0
    }
}
impl KREG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> KREG_A {
        match self.bits {
            false => KREG_A::_0,
            true => KREG_A::_1,
        }
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KREG_A::_0
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KREG_A::_1
    }
}
#[doc = "Field `KREG` writer - Detection Edge Selection (KRF0 to KRF7)"]
pub type KREG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, KREG_A>;
impl<'a, REG, const O: u8> KREG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(KREG_A::_0)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(KREG_A::_1)
    }
}
#[doc = "Field `KRMD` reader - Usage of Key Interrupt Flags(KR0 to KR7)"]
pub type KRMD_R = crate::BitReader<KRMD_A>;
#[doc = "Usage of Key Interrupt Flags(KR0 to KR7)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KRMD_A {
    #[doc = "0: Do not use key interrupt flags"]
    _0 = 0,
    #[doc = "1: Use key interrupt flags."]
    _1 = 1,
}
impl From<KRMD_A> for bool {
    #[inline(always)]
    fn from(variant: KRMD_A) -> Self {
        variant as u8 != 0
    }
}
impl KRMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> KRMD_A {
        match self.bits {
            false => KRMD_A::_0,
            true => KRMD_A::_1,
        }
    }
    #[doc = "Do not use key interrupt flags"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KRMD_A::_0
    }
    #[doc = "Use key interrupt flags."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KRMD_A::_1
    }
}
#[doc = "Field `KRMD` writer - Usage of Key Interrupt Flags(KR0 to KR7)"]
pub type KRMD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, KRMD_A>;
impl<'a, REG, const O: u8> KRMD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not use key interrupt flags"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(KRMD_A::_0)
    }
    #[doc = "Use key interrupt flags."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(KRMD_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Detection Edge Selection (KRF0 to KRF7)"]
    #[inline(always)]
    pub fn kreg(&self) -> KREG_R {
        KREG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Usage of Key Interrupt Flags(KR0 to KR7)"]
    #[inline(always)]
    pub fn krmd(&self) -> KRMD_R {
        KRMD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Detection Edge Selection (KRF0 to KRF7)"]
    #[inline(always)]
    #[must_use]
    pub fn kreg(&mut self) -> KREG_W<KRCTL_SPEC, 0> {
        KREG_W::new(self)
    }
    #[doc = "Bit 7 - Usage of Key Interrupt Flags(KR0 to KR7)"]
    #[inline(always)]
    #[must_use]
    pub fn krmd(&mut self) -> KRMD_W<KRCTL_SPEC, 7> {
        KRMD_W::new(self)
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
#[doc = "KEY Return Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`krctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`krctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KRCTL_SPEC;
impl crate::RegisterSpec for KRCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`krctl::R`](R) reader structure"]
impl crate::Readable for KRCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`krctl::W`](W) writer structure"]
impl crate::Writable for KRCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KRCTL to value 0"]
impl crate::Resettable for KRCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
