#[doc = "Register `ADHVREFCNT` reader"]
pub type R = crate::R<ADHVREFCNT_SPEC>;
#[doc = "Register `ADHVREFCNT` writer"]
pub type W = crate::W<ADHVREFCNT_SPEC>;
#[doc = "Field `HVSEL` reader - High-Potential Reference Voltage Select"]
pub type HVSEL_R = crate::FieldReader<HVSEL_A>;
#[doc = "High-Potential Reference Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HVSEL_A {
    #[doc = "0: AVCC0 is selected as the high-potential reference voltage"]
    _00 = 0,
    #[doc = "1: VREFH0 is selected as the high-potential reference voltage"]
    _01 = 1,
    #[doc = "2: Internal reference voltage is selected as the high-potential reference voltage"]
    _10 = 2,
    #[doc = "3: Internal node discharge. No reference voltage pin is selected."]
    _11 = 3,
}
impl From<HVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: HVSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HVSEL_A {
    type Ux = u8;
}
impl HVSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HVSEL_A {
        match self.bits {
            0 => HVSEL_A::_00,
            1 => HVSEL_A::_01,
            2 => HVSEL_A::_10,
            3 => HVSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "AVCC0 is selected as the high-potential reference voltage"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == HVSEL_A::_00
    }
    #[doc = "VREFH0 is selected as the high-potential reference voltage"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == HVSEL_A::_01
    }
    #[doc = "Internal reference voltage is selected as the high-potential reference voltage"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == HVSEL_A::_10
    }
    #[doc = "Internal node discharge. No reference voltage pin is selected."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == HVSEL_A::_11
    }
}
#[doc = "Field `HVSEL` writer - High-Potential Reference Voltage Select"]
pub type HVSEL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, HVSEL_A>;
impl<'a, REG, const O: u8> HVSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AVCC0 is selected as the high-potential reference voltage"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(HVSEL_A::_00)
    }
    #[doc = "VREFH0 is selected as the high-potential reference voltage"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(HVSEL_A::_01)
    }
    #[doc = "Internal reference voltage is selected as the high-potential reference voltage"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(HVSEL_A::_10)
    }
    #[doc = "Internal node discharge. No reference voltage pin is selected."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(HVSEL_A::_11)
    }
}
#[doc = "Field `LVSEL` reader - Low-Potential Reference Voltage Select"]
pub type LVSEL_R = crate::FieldReader<LVSEL_A>;
#[doc = "Low-Potential Reference Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LVSEL_A {
    #[doc = "0: AVSS0 is selected as the low-potential reference voltage"]
    _0 = 0,
    #[doc = "1: VREFL0 is selected as the low-potential reference voltage."]
    _1 = 1,
}
impl From<LVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LVSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LVSEL_A {
    type Ux = u8;
}
impl LVSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LVSEL_A> {
        match self.bits {
            0 => Some(LVSEL_A::_0),
            1 => Some(LVSEL_A::_1),
            _ => None,
        }
    }
    #[doc = "AVSS0 is selected as the low-potential reference voltage"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVSEL_A::_0
    }
    #[doc = "VREFL0 is selected as the low-potential reference voltage."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVSEL_A::_1
    }
}
#[doc = "Field `LVSEL` writer - Low-Potential Reference Voltage Select"]
pub type LVSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, LVSEL_A>;
impl<'a, REG, const O: u8> LVSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AVSS0 is selected as the low-potential reference voltage"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LVSEL_A::_0)
    }
    #[doc = "VREFL0 is selected as the low-potential reference voltage."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LVSEL_A::_1)
    }
}
#[doc = "Field `ADSLP` reader - Sleep"]
pub type ADSLP_R = crate::BitReader<ADSLP_A>;
#[doc = "Sleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSLP_A {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: Standby state."]
    _1 = 1,
}
impl From<ADSLP_A> for bool {
    #[inline(always)]
    fn from(variant: ADSLP_A) -> Self {
        variant as u8 != 0
    }
}
impl ADSLP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADSLP_A {
        match self.bits {
            false => ADSLP_A::_0,
            true => ADSLP_A::_1,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADSLP_A::_0
    }
    #[doc = "Standby state."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADSLP_A::_1
    }
}
#[doc = "Field `ADSLP` writer - Sleep"]
pub type ADSLP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADSLP_A>;
impl<'a, REG, const O: u8> ADSLP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADSLP_A::_0)
    }
    #[doc = "Standby state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADSLP_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - High-Potential Reference Voltage Select"]
    #[inline(always)]
    pub fn hvsel(&self) -> HVSEL_R {
        HVSEL_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:4 - Low-Potential Reference Voltage Select"]
    #[inline(always)]
    pub fn lvsel(&self) -> LVSEL_R {
        LVSEL_R::new((self.bits >> 2) & 7)
    }
    #[doc = "Bit 7 - Sleep"]
    #[inline(always)]
    pub fn adslp(&self) -> ADSLP_R {
        ADSLP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - High-Potential Reference Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn hvsel(&mut self) -> HVSEL_W<ADHVREFCNT_SPEC, 0> {
        HVSEL_W::new(self)
    }
    #[doc = "Bits 2:4 - Low-Potential Reference Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn lvsel(&mut self) -> LVSEL_W<ADHVREFCNT_SPEC, 2> {
        LVSEL_W::new(self)
    }
    #[doc = "Bit 7 - Sleep"]
    #[inline(always)]
    #[must_use]
    pub fn adslp(&mut self) -> ADSLP_W<ADHVREFCNT_SPEC, 7> {
        ADSLP_W::new(self)
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
#[doc = "A/D High-Potential/Low-Potential Reference Voltage Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adhvrefcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adhvrefcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADHVREFCNT_SPEC;
impl crate::RegisterSpec for ADHVREFCNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adhvrefcnt::R`](R) reader structure"]
impl crate::Readable for ADHVREFCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adhvrefcnt::W`](W) writer structure"]
impl crate::Writable for ADHVREFCNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADHVREFCNT to value 0"]
impl crate::Resettable for ADHVREFCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
