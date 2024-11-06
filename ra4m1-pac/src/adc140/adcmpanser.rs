#[doc = "Register `ADCMPANSER` reader"]
pub type R = crate::R<ADCMPANSER_SPEC>;
#[doc = "Register `ADCMPANSER` writer"]
pub type W = crate::W<ADCMPANSER_SPEC>;
#[doc = "Field `CMPTSA` reader - Temperature sensor output Compare selection bit."]
pub type CMPTSA_R = crate::BitReader<CMPTSA_A>;
#[doc = "Temperature sensor output Compare selection bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPTSA_A {
    #[doc = "0: Excludes the temperature sensor output from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes the temperature sensor output in the compare window A target range."]
    _1 = 1,
}
impl From<CMPTSA_A> for bool {
    #[inline(always)]
    fn from(variant: CMPTSA_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPTSA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMPTSA_A {
        match self.bits {
            false => CMPTSA_A::_0,
            true => CMPTSA_A::_1,
        }
    }
    #[doc = "Excludes the temperature sensor output from the compare window A target range."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPTSA_A::_0
    }
    #[doc = "Includes the temperature sensor output in the compare window A target range."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPTSA_A::_1
    }
}
#[doc = "Field `CMPTSA` writer - Temperature sensor output Compare selection bit."]
pub type CMPTSA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMPTSA_A>;
impl<'a, REG, const O: u8> CMPTSA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Excludes the temperature sensor output from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPTSA_A::_0)
    }
    #[doc = "Includes the temperature sensor output in the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPTSA_A::_1)
    }
}
#[doc = "Field `CMPOCA` reader - Internal reference voltage Compare selection bit."]
pub type CMPOCA_R = crate::BitReader<CMPOCA_A>;
#[doc = "Internal reference voltage Compare selection bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPOCA_A {
    #[doc = "0: Excludes the internal reference voltage from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes the internal reference voltage in the compare window A target range."]
    _1 = 1,
}
impl From<CMPOCA_A> for bool {
    #[inline(always)]
    fn from(variant: CMPOCA_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPOCA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMPOCA_A {
        match self.bits {
            false => CMPOCA_A::_0,
            true => CMPOCA_A::_1,
        }
    }
    #[doc = "Excludes the internal reference voltage from the compare window A target range."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPOCA_A::_0
    }
    #[doc = "Includes the internal reference voltage in the compare window A target range."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPOCA_A::_1
    }
}
#[doc = "Field `CMPOCA` writer - Internal reference voltage Compare selection bit."]
pub type CMPOCA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMPOCA_A>;
impl<'a, REG, const O: u8> CMPOCA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Excludes the internal reference voltage from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPOCA_A::_0)
    }
    #[doc = "Includes the internal reference voltage in the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPOCA_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Temperature sensor output Compare selection bit."]
    #[inline(always)]
    pub fn cmptsa(&self) -> CMPTSA_R {
        CMPTSA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal reference voltage Compare selection bit."]
    #[inline(always)]
    pub fn cmpoca(&self) -> CMPOCA_R {
        CMPOCA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Temperature sensor output Compare selection bit."]
    #[inline(always)]
    #[must_use]
    pub fn cmptsa(&mut self) -> CMPTSA_W<ADCMPANSER_SPEC, 0> {
        CMPTSA_W::new(self)
    }
    #[doc = "Bit 1 - Internal reference voltage Compare selection bit."]
    #[inline(always)]
    #[must_use]
    pub fn cmpoca(&mut self) -> CMPOCA_W<ADCMPANSER_SPEC, 1> {
        CMPOCA_W::new(self)
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
#[doc = "A/D Compare Function Window A Extended Input Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmpanser::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmpanser::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCMPANSER_SPEC;
impl crate::RegisterSpec for ADCMPANSER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adcmpanser::R`](R) reader structure"]
impl crate::Readable for ADCMPANSER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adcmpanser::W`](W) writer structure"]
impl crate::Writable for ADCMPANSER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPANSER to value 0"]
impl crate::Resettable for ADCMPANSER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
