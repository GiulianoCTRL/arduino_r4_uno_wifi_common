#[doc = "Register `CTSUSO0` reader"]
pub type R = crate::R<CTSUSO0_SPEC>;
#[doc = "Register `CTSUSO0` writer"]
pub type W = crate::W<CTSUSO0_SPEC>;
#[doc = "Field `CTSUSO` reader - CTSU Sensor Offset Adjustment Current offset amount is CTSUSO ( 0 to 1023 )"]
pub type CTSUSO_R = crate::FieldReader<u16>;
#[doc = "Field `CTSUSO` writer - CTSU Sensor Offset Adjustment Current offset amount is CTSUSO ( 0 to 1023 )"]
pub type CTSUSO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `CTSUSNUM` reader - CTSU Measurement Count Setting"]
pub type CTSUSNUM_R = crate::FieldReader;
#[doc = "Field `CTSUSNUM` writer - CTSU Measurement Count Setting"]
pub type CTSUSNUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:9 - CTSU Sensor Offset Adjustment Current offset amount is CTSUSO ( 0 to 1023 )"]
    #[inline(always)]
    pub fn ctsuso(&self) -> CTSUSO_R {
        CTSUSO_R::new(self.bits & 0x03ff)
    }
    #[doc = "Bits 10:15 - CTSU Measurement Count Setting"]
    #[inline(always)]
    pub fn ctsusnum(&self) -> CTSUSNUM_R {
        CTSUSNUM_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - CTSU Sensor Offset Adjustment Current offset amount is CTSUSO ( 0 to 1023 )"]
    #[inline(always)]
    #[must_use]
    pub fn ctsuso(&mut self) -> CTSUSO_W<CTSUSO0_SPEC, 0> {
        CTSUSO_W::new(self)
    }
    #[doc = "Bits 10:15 - CTSU Measurement Count Setting"]
    #[inline(always)]
    #[must_use]
    pub fn ctsusnum(&mut self) -> CTSUSNUM_W<CTSUSO0_SPEC, 10> {
        CTSUSNUM_W::new(self)
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
#[doc = "CTSU Sensor Offset Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsuso0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsuso0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTSUSO0_SPEC;
impl crate::RegisterSpec for CTSUSO0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctsuso0::R`](R) reader structure"]
impl crate::Readable for CTSUSO0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctsuso0::W`](W) writer structure"]
impl crate::Writable for CTSUSO0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUSO0 to value 0"]
impl crate::Resettable for CTSUSO0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
