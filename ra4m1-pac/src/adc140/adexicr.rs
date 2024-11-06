#[doc = "Register `ADEXICR` reader"]
pub type R = crate::R<ADEXICR_SPEC>;
#[doc = "Register `ADEXICR` writer"]
pub type W = crate::W<ADEXICR_SPEC>;
#[doc = "Field `TSSAD` reader - Temperature Sensor Output A/D converted Value Addition/Average Mode Select"]
pub type TSSAD_R = crate::BitReader<TSSAD_A>;
#[doc = "Temperature Sensor Output A/D converted Value Addition/Average Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSSAD_A {
    #[doc = "0: Temperature sensor output A/D-converted value addition/average mode is not selected."]
    _0 = 0,
    #[doc = "1: Temperature sensor output A/D-converted value addition/average mode is selected."]
    _1 = 1,
}
impl From<TSSAD_A> for bool {
    #[inline(always)]
    fn from(variant: TSSAD_A) -> Self {
        variant as u8 != 0
    }
}
impl TSSAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSSAD_A {
        match self.bits {
            false => TSSAD_A::_0,
            true => TSSAD_A::_1,
        }
    }
    #[doc = "Temperature sensor output A/D-converted value addition/average mode is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSSAD_A::_0
    }
    #[doc = "Temperature sensor output A/D-converted value addition/average mode is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSSAD_A::_1
    }
}
#[doc = "Field `TSSAD` writer - Temperature Sensor Output A/D converted Value Addition/Average Mode Select"]
pub type TSSAD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TSSAD_A>;
impl<'a, REG, const O: u8> TSSAD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Temperature sensor output A/D-converted value addition/average mode is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TSSAD_A::_0)
    }
    #[doc = "Temperature sensor output A/D-converted value addition/average mode is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TSSAD_A::_1)
    }
}
#[doc = "Field `OCSAD` reader - Internal Reference Voltage A/D converted Value Addition/Average Mode Select"]
pub type OCSAD_R = crate::BitReader<OCSAD_A>;
#[doc = "Internal Reference Voltage A/D converted Value Addition/Average Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCSAD_A {
    #[doc = "0: Internal reference voltage A/D-converted value addition/average mode is not selected."]
    _0 = 0,
    #[doc = "1: Internal reference voltage A/D-converted value addition/average mode is selected."]
    _1 = 1,
}
impl From<OCSAD_A> for bool {
    #[inline(always)]
    fn from(variant: OCSAD_A) -> Self {
        variant as u8 != 0
    }
}
impl OCSAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OCSAD_A {
        match self.bits {
            false => OCSAD_A::_0,
            true => OCSAD_A::_1,
        }
    }
    #[doc = "Internal reference voltage A/D-converted value addition/average mode is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OCSAD_A::_0
    }
    #[doc = "Internal reference voltage A/D-converted value addition/average mode is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OCSAD_A::_1
    }
}
#[doc = "Field `OCSAD` writer - Internal Reference Voltage A/D converted Value Addition/Average Mode Select"]
pub type OCSAD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OCSAD_A>;
impl<'a, REG, const O: u8> OCSAD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal reference voltage A/D-converted value addition/average mode is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OCSAD_A::_0)
    }
    #[doc = "Internal reference voltage A/D-converted value addition/average mode is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OCSAD_A::_1)
    }
}
#[doc = "Field `TSSA` reader - Temperature Sensor Output A/D Conversion Select"]
pub type TSSA_R = crate::BitReader<TSSA_A>;
#[doc = "Temperature Sensor Output A/D Conversion Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSSA_A {
    #[doc = "0: The temperature sensor output is not selected."]
    _0 = 0,
    #[doc = "1: The temperature sensor output is selected."]
    _1 = 1,
}
impl From<TSSA_A> for bool {
    #[inline(always)]
    fn from(variant: TSSA_A) -> Self {
        variant as u8 != 0
    }
}
impl TSSA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSSA_A {
        match self.bits {
            false => TSSA_A::_0,
            true => TSSA_A::_1,
        }
    }
    #[doc = "The temperature sensor output is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSSA_A::_0
    }
    #[doc = "The temperature sensor output is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSSA_A::_1
    }
}
#[doc = "Field `TSSA` writer - Temperature Sensor Output A/D Conversion Select"]
pub type TSSA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TSSA_A>;
impl<'a, REG, const O: u8> TSSA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The temperature sensor output is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TSSA_A::_0)
    }
    #[doc = "The temperature sensor output is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TSSA_A::_1)
    }
}
#[doc = "Field `OCSA` reader - Internal Reference Voltage A/D Conversion Select"]
pub type OCSA_R = crate::BitReader<OCSA_A>;
#[doc = "Internal Reference Voltage A/D Conversion Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCSA_A {
    #[doc = "0: The internal reference voltage is not selected."]
    _0 = 0,
    #[doc = "1: The internal reference voltage is selected for group A in single scan mode, continuous scan mode, or group scan mode."]
    _1 = 1,
}
impl From<OCSA_A> for bool {
    #[inline(always)]
    fn from(variant: OCSA_A) -> Self {
        variant as u8 != 0
    }
}
impl OCSA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OCSA_A {
        match self.bits {
            false => OCSA_A::_0,
            true => OCSA_A::_1,
        }
    }
    #[doc = "The internal reference voltage is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OCSA_A::_0
    }
    #[doc = "The internal reference voltage is selected for group A in single scan mode, continuous scan mode, or group scan mode."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OCSA_A::_1
    }
}
#[doc = "Field `OCSA` writer - Internal Reference Voltage A/D Conversion Select"]
pub type OCSA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OCSA_A>;
impl<'a, REG, const O: u8> OCSA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The internal reference voltage is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OCSA_A::_0)
    }
    #[doc = "The internal reference voltage is selected for group A in single scan mode, continuous scan mode, or group scan mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OCSA_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Temperature Sensor Output A/D converted Value Addition/Average Mode Select"]
    #[inline(always)]
    pub fn tssad(&self) -> TSSAD_R {
        TSSAD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal Reference Voltage A/D converted Value Addition/Average Mode Select"]
    #[inline(always)]
    pub fn ocsad(&self) -> OCSAD_R {
        OCSAD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Temperature Sensor Output A/D Conversion Select"]
    #[inline(always)]
    pub fn tssa(&self) -> TSSA_R {
        TSSA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Internal Reference Voltage A/D Conversion Select"]
    #[inline(always)]
    pub fn ocsa(&self) -> OCSA_R {
        OCSA_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Temperature Sensor Output A/D converted Value Addition/Average Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn tssad(&mut self) -> TSSAD_W<ADEXICR_SPEC, 0> {
        TSSAD_W::new(self)
    }
    #[doc = "Bit 1 - Internal Reference Voltage A/D converted Value Addition/Average Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn ocsad(&mut self) -> OCSAD_W<ADEXICR_SPEC, 1> {
        OCSAD_W::new(self)
    }
    #[doc = "Bit 8 - Temperature Sensor Output A/D Conversion Select"]
    #[inline(always)]
    #[must_use]
    pub fn tssa(&mut self) -> TSSA_W<ADEXICR_SPEC, 8> {
        TSSA_W::new(self)
    }
    #[doc = "Bit 9 - Internal Reference Voltage A/D Conversion Select"]
    #[inline(always)]
    #[must_use]
    pub fn ocsa(&mut self) -> OCSA_W<ADEXICR_SPEC, 9> {
        OCSA_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "A/D Conversion Extended Input Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adexicr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adexicr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADEXICR_SPEC;
impl crate::RegisterSpec for ADEXICR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adexicr::R`](R) reader structure"]
impl crate::Readable for ADEXICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adexicr::W`](W) writer structure"]
impl crate::Writable for ADEXICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADEXICR to value 0"]
impl crate::Resettable for ADEXICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
