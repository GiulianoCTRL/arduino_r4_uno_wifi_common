#[doc = "Register `CTSUCHAC1` reader"]
pub type R = crate::R<CTSUCHAC1_SPEC>;
#[doc = "Register `CTSUCHAC1` writer"]
pub type W = crate::W<CTSUCHAC1_SPEC>;
#[doc = "Field `CTSUCHAC1` reader - CTSU Channel Enable Control 1. 0: Not measurement target 1: Measurement target Note: CTSUCHAC1\\[0\\]
corresponds to TS08 and CTSUCHAC1\\[7\\]
corresponds to TS15."]
pub type CTSUCHAC1_R = crate::FieldReader<CTSUCHAC1_A>;
#[doc = "CTSU Channel Enable Control 1. 0: Not measurement target 1: Measurement target Note: CTSUCHAC1\\[0\\]
corresponds to TS08 and CTSUCHAC1\\[7\\]
corresponds to TS15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CTSUCHAC1_A(u8);
impl From<CTSUCHAC1_A> for u8 {
    #[inline(always)]
    fn from(val: CTSUCHAC1_A) -> Self {
        val.0 as _
    }
}
impl crate::FieldSpec for CTSUCHAC1_A {
    type Ux = u8;
}
#[doc = "Field `CTSUCHAC1` writer - CTSU Channel Enable Control 1. 0: Not measurement target 1: Measurement target Note: CTSUCHAC1\\[0\\]
corresponds to TS08 and CTSUCHAC1\\[7\\]
corresponds to TS15."]
pub type CTSUCHAC1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, CTSUCHAC1_A>;
impl R {
    #[doc = "Bits 0:7 - CTSU Channel Enable Control 1. 0: Not measurement target 1: Measurement target Note: CTSUCHAC1\\[0\\]
corresponds to TS08 and CTSUCHAC1\\[7\\]
corresponds to TS15."]
    #[inline(always)]
    pub fn ctsuchac1(&self) -> CTSUCHAC1_R {
        CTSUCHAC1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - CTSU Channel Enable Control 1. 0: Not measurement target 1: Measurement target Note: CTSUCHAC1\\[0\\]
corresponds to TS08 and CTSUCHAC1\\[7\\]
corresponds to TS15."]
    #[inline(always)]
    #[must_use]
    pub fn ctsuchac1(&mut self) -> CTSUCHAC1_W<CTSUCHAC1_SPEC, 0> {
        CTSUCHAC1_W::new(self)
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
#[doc = "CTSU Channel Enable Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsuchac1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsuchac1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTSUCHAC1_SPEC;
impl crate::RegisterSpec for CTSUCHAC1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctsuchac1::R`](R) reader structure"]
impl crate::Readable for CTSUCHAC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctsuchac1::W`](W) writer structure"]
impl crate::Writable for CTSUCHAC1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUCHAC1 to value 0"]
impl crate::Resettable for CTSUCHAC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
