#[doc = "Register `ADCMPSER` reader"]
pub type R = crate::R<ADCMPSER_SPEC>;
#[doc = "Register `ADCMPSER` writer"]
pub type W = crate::W<ADCMPSER_SPEC>;
#[doc = "Field `CMPSTTSA` reader - Compare Window A Temperature Sensor Output Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time.\n\nThe field is **modified** in some way after a read operation."]
pub type CMPSTTSA_R = crate::BitReader<CMPSTTSA_A>;
#[doc = "Compare Window A Temperature Sensor Output Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTTSA_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTTSA_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTTSA_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTTSA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTTSA_A {
        match self.bits {
            false => CMPSTTSA_A::_0,
            true => CMPSTTSA_A::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTTSA_A::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTTSA_A::_1
    }
}
#[doc = "Field `CMPSTTSA` writer - Compare Window A Temperature Sensor Output Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time."]
pub type CMPSTTSA_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, CMPSTTSA_A>;
impl<'a, REG, const O: u8> CMPSTTSA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTTSA_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTTSA_A::_1)
    }
}
#[doc = "Field `CMPSTOCA` reader - Compare Window A Internal Reference Voltage Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time.\n\nThe field is **modified** in some way after a read operation."]
pub type CMPSTOCA_R = crate::BitReader<CMPSTOCA_A>;
#[doc = "Compare Window A Internal Reference Voltage Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTOCA_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTOCA_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTOCA_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTOCA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTOCA_A {
        match self.bits {
            false => CMPSTOCA_A::_0,
            true => CMPSTOCA_A::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTOCA_A::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTOCA_A::_1
    }
}
#[doc = "Field `CMPSTOCA` writer - Compare Window A Internal Reference Voltage Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time."]
pub type CMPSTOCA_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, CMPSTOCA_A>;
impl<'a, REG, const O: u8> CMPSTOCA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTOCA_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTOCA_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Compare Window A Temperature Sensor Output Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time."]
    #[inline(always)]
    pub fn cmpsttsa(&self) -> CMPSTTSA_R {
        CMPSTTSA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare Window A Internal Reference Voltage Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time."]
    #[inline(always)]
    pub fn cmpstoca(&self) -> CMPSTOCA_R {
        CMPSTOCA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare Window A Temperature Sensor Output Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time."]
    #[inline(always)]
    #[must_use]
    pub fn cmpsttsa(&mut self) -> CMPSTTSA_W<ADCMPSER_SPEC, 0> {
        CMPSTTSA_W::new(self)
    }
    #[doc = "Bit 1 - Compare Window A Internal Reference Voltage Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time."]
    #[inline(always)]
    #[must_use]
    pub fn cmpstoca(&mut self) -> CMPSTOCA_W<ADCMPSER_SPEC, 1> {
        CMPSTOCA_W::new(self)
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
#[doc = "A/D Compare Function Window A Extended Input Channel Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmpser::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmpser::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCMPSER_SPEC;
impl crate::RegisterSpec for ADCMPSER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adcmpser::R`](R) reader structure"]
impl crate::Readable for ADCMPSER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adcmpser::W`](W) writer structure"]
impl crate::Writable for ADCMPSER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x03;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPSER to value 0"]
impl crate::Resettable for ADCMPSER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
