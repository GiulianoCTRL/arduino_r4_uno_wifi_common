#[doc = "Register `MOMCR` reader"]
pub type R = crate::R<MOMCR_SPEC>;
#[doc = "Register `MOMCR` writer"]
pub type W = crate::W<MOMCR_SPEC>;
#[doc = "Field `MODRV1` reader - Main Clock Oscillator Drive Capability 1 Switching"]
pub type MODRV1_R = crate::BitReader<MODRV1_A>;
#[doc = "Main Clock Oscillator Drive Capability 1 Switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODRV1_A {
    #[doc = "0: 10 MHz to 20 MHz"]
    _0 = 0,
    #[doc = "1: 1 MHz to 10 MHz."]
    _1 = 1,
}
impl From<MODRV1_A> for bool {
    #[inline(always)]
    fn from(variant: MODRV1_A) -> Self {
        variant as u8 != 0
    }
}
impl MODRV1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODRV1_A {
        match self.bits {
            false => MODRV1_A::_0,
            true => MODRV1_A::_1,
        }
    }
    #[doc = "10 MHz to 20 MHz"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MODRV1_A::_0
    }
    #[doc = "1 MHz to 10 MHz."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MODRV1_A::_1
    }
}
#[doc = "Field `MODRV1` writer - Main Clock Oscillator Drive Capability 1 Switching"]
pub type MODRV1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MODRV1_A>;
impl<'a, REG, const O: u8> MODRV1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "10 MHz to 20 MHz"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MODRV1_A::_0)
    }
    #[doc = "1 MHz to 10 MHz."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MODRV1_A::_1)
    }
}
#[doc = "Field `MOSEL` reader - Main Clock Oscillator Switching"]
pub type MOSEL_R = crate::BitReader<MOSEL_A>;
#[doc = "Main Clock Oscillator Switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOSEL_A {
    #[doc = "0: Resonator"]
    _0 = 0,
    #[doc = "1: External clock input"]
    _1 = 1,
}
impl From<MOSEL_A> for bool {
    #[inline(always)]
    fn from(variant: MOSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl MOSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MOSEL_A {
        match self.bits {
            false => MOSEL_A::_0,
            true => MOSEL_A::_1,
        }
    }
    #[doc = "Resonator"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MOSEL_A::_0
    }
    #[doc = "External clock input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MOSEL_A::_1
    }
}
#[doc = "Field `MOSEL` writer - Main Clock Oscillator Switching"]
pub type MOSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MOSEL_A>;
impl<'a, REG, const O: u8> MOSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resonator"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MOSEL_A::_0)
    }
    #[doc = "External clock input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MOSEL_A::_1)
    }
}
impl R {
    #[doc = "Bit 3 - Main Clock Oscillator Drive Capability 1 Switching"]
    #[inline(always)]
    pub fn modrv1(&self) -> MODRV1_R {
        MODRV1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Main Clock Oscillator Switching"]
    #[inline(always)]
    pub fn mosel(&self) -> MOSEL_R {
        MOSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Main Clock Oscillator Drive Capability 1 Switching"]
    #[inline(always)]
    #[must_use]
    pub fn modrv1(&mut self) -> MODRV1_W<MOMCR_SPEC, 3> {
        MODRV1_W::new(self)
    }
    #[doc = "Bit 6 - Main Clock Oscillator Switching"]
    #[inline(always)]
    #[must_use]
    pub fn mosel(&mut self) -> MOSEL_W<MOMCR_SPEC, 6> {
        MOSEL_W::new(self)
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
#[doc = "Main Clock Oscillator Mode Oscillation Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`momcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`momcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MOMCR_SPEC;
impl crate::RegisterSpec for MOMCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`momcr::R`](R) reader structure"]
impl crate::Readable for MOMCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`momcr::W`](W) writer structure"]
impl crate::Writable for MOMCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MOMCR to value 0"]
impl crate::Resettable for MOMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
