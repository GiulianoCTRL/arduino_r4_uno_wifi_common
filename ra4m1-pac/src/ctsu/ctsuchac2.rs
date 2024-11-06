#[doc = "Register `CTSUCHAC2` reader"]
pub type R = crate::R<CTSUCHAC2_SPEC>;
#[doc = "Register `CTSUCHAC2` writer"]
pub type W = crate::W<CTSUCHAC2_SPEC>;
#[doc = "Field `CTSUCHAC2` reader - CTSU Channel Enable Control 2. 0: Not measurement target 1: Measurement target Note: CTSUCHAC2\\[0\\]
corresponds to TS16 and CTSUCHAC2\\[7\\]
corresponds to TS23."]
pub type CTSUCHAC2_R = crate::FieldReader<CTSUCHAC2_A>;
#[doc = "CTSU Channel Enable Control 2. 0: Not measurement target 1: Measurement target Note: CTSUCHAC2\\[0\\]
corresponds to TS16 and CTSUCHAC2\\[7\\]
corresponds to TS23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CTSUCHAC2_A(u8);
impl From<CTSUCHAC2_A> for u8 {
    #[inline(always)]
    fn from(val: CTSUCHAC2_A) -> Self {
        val.0 as _
    }
}
impl crate::FieldSpec for CTSUCHAC2_A {
    type Ux = u8;
}
#[doc = "Field `CTSUCHAC2` writer - CTSU Channel Enable Control 2. 0: Not measurement target 1: Measurement target Note: CTSUCHAC2\\[0\\]
corresponds to TS16 and CTSUCHAC2\\[7\\]
corresponds to TS23."]
pub type CTSUCHAC2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, CTSUCHAC2_A>;
impl R {
    #[doc = "Bits 0:7 - CTSU Channel Enable Control 2. 0: Not measurement target 1: Measurement target Note: CTSUCHAC2\\[0\\]
corresponds to TS16 and CTSUCHAC2\\[7\\]
corresponds to TS23."]
    #[inline(always)]
    pub fn ctsuchac2(&self) -> CTSUCHAC2_R {
        CTSUCHAC2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - CTSU Channel Enable Control 2. 0: Not measurement target 1: Measurement target Note: CTSUCHAC2\\[0\\]
corresponds to TS16 and CTSUCHAC2\\[7\\]
corresponds to TS23."]
    #[inline(always)]
    #[must_use]
    pub fn ctsuchac2(&mut self) -> CTSUCHAC2_W<CTSUCHAC2_SPEC, 0> {
        CTSUCHAC2_W::new(self)
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
#[doc = "CTSU Channel Enable Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsuchac2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsuchac2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTSUCHAC2_SPEC;
impl crate::RegisterSpec for CTSUCHAC2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctsuchac2::R`](R) reader structure"]
impl crate::Readable for CTSUCHAC2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctsuchac2::W`](W) writer structure"]
impl crate::Writable for CTSUCHAC2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUCHAC2 to value 0"]
impl crate::Resettable for CTSUCHAC2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
