#[doc = "Register `ADCMPBSR` reader"]
pub type R = crate::R<ADCMPBSR_SPEC>;
#[doc = "Register `ADCMPBSR` writer"]
pub type W = crate::W<ADCMPBSR_SPEC>;
#[doc = "Field `CMPSTB` reader - Compare window B flag. It is a status flag that shows the comparative result of CH (AN000-AN027, temperature sensor, and internal reference voltage) made the object of window B relation condition.\n\nThe field is **modified** in some way after a read operation."]
pub type CMPSTB_R = crate::BitReader<CMPSTB_A>;
#[doc = "Compare window B flag. It is a status flag that shows the comparative result of CH (AN000-AN027, temperature sensor, and internal reference voltage) made the object of window B relation condition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTB_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTB_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTB_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTB_A {
        match self.bits {
            false => CMPSTB_A::_0,
            true => CMPSTB_A::_1,
        }
    }
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTB_A::_0
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTB_A::_1
    }
}
#[doc = "Field `CMPSTB` writer - Compare window B flag. It is a status flag that shows the comparative result of CH (AN000-AN027, temperature sensor, and internal reference voltage) made the object of window B relation condition."]
pub type CMPSTB_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, CMPSTB_A>;
impl<'a, REG, const O: u8> CMPSTB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTB_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTB_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Compare window B flag. It is a status flag that shows the comparative result of CH (AN000-AN027, temperature sensor, and internal reference voltage) made the object of window B relation condition."]
    #[inline(always)]
    pub fn cmpstb(&self) -> CMPSTB_R {
        CMPSTB_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare window B flag. It is a status flag that shows the comparative result of CH (AN000-AN027, temperature sensor, and internal reference voltage) made the object of window B relation condition."]
    #[inline(always)]
    #[must_use]
    pub fn cmpstb(&mut self) -> CMPSTB_W<ADCMPBSR_SPEC, 0> {
        CMPSTB_W::new(self)
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
#[doc = "A/D Compare Function Window B Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmpbsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmpbsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCMPBSR_SPEC;
impl crate::RegisterSpec for ADCMPBSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adcmpbsr::R`](R) reader structure"]
impl crate::Readable for ADCMPBSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adcmpbsr::W`](W) writer structure"]
impl crate::Writable for ADCMPBSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPBSR to value 0"]
impl crate::Resettable for ADCMPBSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
