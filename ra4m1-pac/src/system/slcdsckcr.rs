#[doc = "Register `SLCDSCKCR` reader"]
pub type R = crate::R<SLCDSCKCR_SPEC>;
#[doc = "Register `SLCDSCKCR` writer"]
pub type W = crate::W<SLCDSCKCR_SPEC>;
#[doc = "Field `LCDSCKSEL` reader - LCD Source Clock (LCDSRCCLK) Select"]
pub type LCDSCKSEL_R = crate::FieldReader<LCDSCKSEL_A>;
#[doc = "LCD Source Clock (LCDSRCCLK) Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LCDSCKSEL_A {
    #[doc = "0: LOCO"]
    _000 = 0,
    #[doc = "1: SOSC"]
    _001 = 1,
    #[doc = "2: MOSC"]
    _010 = 2,
    #[doc = "4: HOCO"]
    _100 = 4,
}
impl From<LCDSCKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LCDSCKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LCDSCKSEL_A {
    type Ux = u8;
}
impl LCDSCKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LCDSCKSEL_A> {
        match self.bits {
            0 => Some(LCDSCKSEL_A::_000),
            1 => Some(LCDSCKSEL_A::_001),
            2 => Some(LCDSCKSEL_A::_010),
            4 => Some(LCDSCKSEL_A::_100),
            _ => None,
        }
    }
    #[doc = "LOCO"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == LCDSCKSEL_A::_000
    }
    #[doc = "SOSC"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == LCDSCKSEL_A::_001
    }
    #[doc = "MOSC"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == LCDSCKSEL_A::_010
    }
    #[doc = "HOCO"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == LCDSCKSEL_A::_100
    }
}
#[doc = "Field `LCDSCKSEL` writer - LCD Source Clock (LCDSRCCLK) Select"]
pub type LCDSCKSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, LCDSCKSEL_A>;
impl<'a, REG, const O: u8> LCDSCKSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LOCO"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(LCDSCKSEL_A::_000)
    }
    #[doc = "SOSC"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(LCDSCKSEL_A::_001)
    }
    #[doc = "MOSC"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(LCDSCKSEL_A::_010)
    }
    #[doc = "HOCO"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(LCDSCKSEL_A::_100)
    }
}
#[doc = "Field `LCDSCKEN` reader - LCD Source Clock Out Enable"]
pub type LCDSCKEN_R = crate::BitReader<LCDSCKEN_A>;
#[doc = "LCD Source Clock Out Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDSCKEN_A {
    #[doc = "0: LCD source clock out disabled"]
    _0 = 0,
    #[doc = "1: LCD source clock out enabled."]
    _1 = 1,
}
impl From<LCDSCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: LCDSCKEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDSCKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCDSCKEN_A {
        match self.bits {
            false => LCDSCKEN_A::_0,
            true => LCDSCKEN_A::_1,
        }
    }
    #[doc = "LCD source clock out disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LCDSCKEN_A::_0
    }
    #[doc = "LCD source clock out enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LCDSCKEN_A::_1
    }
}
#[doc = "Field `LCDSCKEN` writer - LCD Source Clock Out Enable"]
pub type LCDSCKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LCDSCKEN_A>;
impl<'a, REG, const O: u8> LCDSCKEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD source clock out disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LCDSCKEN_A::_0)
    }
    #[doc = "LCD source clock out enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LCDSCKEN_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - LCD Source Clock (LCDSRCCLK) Select"]
    #[inline(always)]
    pub fn lcdscksel(&self) -> LCDSCKSEL_R {
        LCDSCKSEL_R::new(self.bits & 7)
    }
    #[doc = "Bit 7 - LCD Source Clock Out Enable"]
    #[inline(always)]
    pub fn lcdscken(&self) -> LCDSCKEN_R {
        LCDSCKEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - LCD Source Clock (LCDSRCCLK) Select"]
    #[inline(always)]
    #[must_use]
    pub fn lcdscksel(&mut self) -> LCDSCKSEL_W<SLCDSCKCR_SPEC, 0> {
        LCDSCKSEL_W::new(self)
    }
    #[doc = "Bit 7 - LCD Source Clock Out Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lcdscken(&mut self) -> LCDSCKEN_W<SLCDSCKCR_SPEC, 7> {
        LCDSCKEN_W::new(self)
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
#[doc = "Segment LCD Source Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slcdsckcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slcdsckcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLCDSCKCR_SPEC;
impl crate::RegisterSpec for SLCDSCKCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`slcdsckcr::R`](R) reader structure"]
impl crate::Readable for SLCDSCKCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slcdsckcr::W`](W) writer structure"]
impl crate::Writable for SLCDSCKCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLCDSCKCR to value 0"]
impl crate::Resettable for SLCDSCKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
