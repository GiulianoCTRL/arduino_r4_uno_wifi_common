#[doc = "Register `ADCMPLER` reader"]
pub type R = crate::R<ADCMPLER_SPEC>;
#[doc = "Register `ADCMPLER` writer"]
pub type W = crate::W<ADCMPLER_SPEC>;
#[doc = "Field `CMPLTSA` reader - Compare Window A Temperature Sensor Output Comparison Condition Select"]
pub type CMPLTSA_R = crate::BitReader<CMPLTSA_A>;
#[doc = "Compare Window A Temperature Sensor Output Comparison Condition Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLTSA_A {
    #[doc = "0: ADCMPDR0 register value > A/D-converted value(ADCMPCR.WCMPE=0) / AD-converted value &lt; ADCMPDR0 register value or A/D-converted value > ADCMPDR1 register value(ADCMPCR.WCMPE=1)."]
    _0 = 0,
    #[doc = "1: ADCMPDR0 register value &lt; A/D-converted value(ADCMPCR.WCMPE=0) / ADCMPDR0 register value &lt; A/D-converted value &lt; ADCMPDR1 register value(ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<CMPLTSA_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLTSA_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLTSA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMPLTSA_A {
        match self.bits {
            false => CMPLTSA_A::_0,
            true => CMPLTSA_A::_1,
        }
    }
    #[doc = "ADCMPDR0 register value > A/D-converted value(ADCMPCR.WCMPE=0) / AD-converted value &lt; ADCMPDR0 register value or A/D-converted value > ADCMPDR1 register value(ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLTSA_A::_0
    }
    #[doc = "ADCMPDR0 register value &lt; A/D-converted value(ADCMPCR.WCMPE=0) / ADCMPDR0 register value &lt; A/D-converted value &lt; ADCMPDR1 register value(ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLTSA_A::_1
    }
}
#[doc = "Field `CMPLTSA` writer - Compare Window A Temperature Sensor Output Comparison Condition Select"]
pub type CMPLTSA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMPLTSA_A>;
impl<'a, REG, const O: u8> CMPLTSA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 register value > A/D-converted value(ADCMPCR.WCMPE=0) / AD-converted value &lt; ADCMPDR0 register value or A/D-converted value > ADCMPDR1 register value(ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLTSA_A::_0)
    }
    #[doc = "ADCMPDR0 register value &lt; A/D-converted value(ADCMPCR.WCMPE=0) / ADCMPDR0 register value &lt; A/D-converted value &lt; ADCMPDR1 register value(ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLTSA_A::_1)
    }
}
#[doc = "Field `CMPLOCA` reader - Compare Window A Internal Reference Voltage Comparison Condition Select"]
pub type CMPLOCA_R = crate::BitReader<CMPLOCA_A>;
#[doc = "Compare Window A Internal Reference Voltage Comparison Condition Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLOCA_A {
    #[doc = "0: ADCMPDR0 value > A/D converted value(ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or A/D converted value > ADCMPDR1 value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value &lt; A/D converted value(ADCMPCR.WCMPE=0) / ADCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value(ADCMPCR.WCMPE=1)"]
    _1 = 1,
}
impl From<CMPLOCA_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLOCA_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLOCA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMPLOCA_A {
        match self.bits {
            false => CMPLOCA_A::_0,
            true => CMPLOCA_A::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value(ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or A/D converted value > ADCMPDR1 value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLOCA_A::_0
    }
    #[doc = "ADCMPDR0 value &lt; A/D converted value(ADCMPCR.WCMPE=0) / ADCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value(ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLOCA_A::_1
    }
}
#[doc = "Field `CMPLOCA` writer - Compare Window A Internal Reference Voltage Comparison Condition Select"]
pub type CMPLOCA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMPLOCA_A>;
impl<'a, REG, const O: u8> CMPLOCA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value(ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or A/D converted value > ADCMPDR1 value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLOCA_A::_0)
    }
    #[doc = "ADCMPDR0 value &lt; A/D converted value(ADCMPCR.WCMPE=0) / ADCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value(ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLOCA_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Compare Window A Temperature Sensor Output Comparison Condition Select"]
    #[inline(always)]
    pub fn cmpltsa(&self) -> CMPLTSA_R {
        CMPLTSA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare Window A Internal Reference Voltage Comparison Condition Select"]
    #[inline(always)]
    pub fn cmploca(&self) -> CMPLOCA_R {
        CMPLOCA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare Window A Temperature Sensor Output Comparison Condition Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpltsa(&mut self) -> CMPLTSA_W<ADCMPLER_SPEC, 0> {
        CMPLTSA_W::new(self)
    }
    #[doc = "Bit 1 - Compare Window A Internal Reference Voltage Comparison Condition Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmploca(&mut self) -> CMPLOCA_W<ADCMPLER_SPEC, 1> {
        CMPLOCA_W::new(self)
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
#[doc = "A/D Compare Function Window A Extended Input Comparison Condition Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmpler::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmpler::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCMPLER_SPEC;
impl crate::RegisterSpec for ADCMPLER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adcmpler::R`](R) reader structure"]
impl crate::Readable for ADCMPLER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adcmpler::W`](W) writer structure"]
impl crate::Writable for ADCMPLER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPLER to value 0"]
impl crate::Resettable for ADCMPLER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
