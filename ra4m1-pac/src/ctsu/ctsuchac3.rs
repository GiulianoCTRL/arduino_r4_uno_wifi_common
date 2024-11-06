#[doc = "Register `CTSUCHAC3` reader"]
pub type R = crate::R<CTSUCHAC3_SPEC>;
#[doc = "Register `CTSUCHAC3` writer"]
pub type W = crate::W<CTSUCHAC3_SPEC>;
#[doc = "Field `CTSUCHAC3` reader - CTSU Channel Enable Control 3. 0: Not measurement target 1: Measurement target Note: CTSUCHAC3\\[0\\]
corresponds to TS24 and CTSUCHAC3\\[7\\]
corresponds to TS31."]
pub type CTSUCHAC3_R = crate::FieldReader<CTSUCHAC3_A>;
#[doc = "CTSU Channel Enable Control 3. 0: Not measurement target 1: Measurement target Note: CTSUCHAC3\\[0\\]
corresponds to TS24 and CTSUCHAC3\\[7\\]
corresponds to TS31.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CTSUCHAC3_A(u8);
impl From<CTSUCHAC3_A> for u8 {
    #[inline(always)]
    fn from(val: CTSUCHAC3_A) -> Self {
        val.0 as _
    }
}
impl crate::FieldSpec for CTSUCHAC3_A {
    type Ux = u8;
}
#[doc = "Field `CTSUCHAC3` writer - CTSU Channel Enable Control 3. 0: Not measurement target 1: Measurement target Note: CTSUCHAC3\\[0\\]
corresponds to TS24 and CTSUCHAC3\\[7\\]
corresponds to TS31."]
pub type CTSUCHAC3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, CTSUCHAC3_A>;
impl R {
    #[doc = "Bits 0:7 - CTSU Channel Enable Control 3. 0: Not measurement target 1: Measurement target Note: CTSUCHAC3\\[0\\]
corresponds to TS24 and CTSUCHAC3\\[7\\]
corresponds to TS31."]
    #[inline(always)]
    pub fn ctsuchac3(&self) -> CTSUCHAC3_R {
        CTSUCHAC3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - CTSU Channel Enable Control 3. 0: Not measurement target 1: Measurement target Note: CTSUCHAC3\\[0\\]
corresponds to TS24 and CTSUCHAC3\\[7\\]
corresponds to TS31."]
    #[inline(always)]
    #[must_use]
    pub fn ctsuchac3(&mut self) -> CTSUCHAC3_W<CTSUCHAC3_SPEC, 0> {
        CTSUCHAC3_W::new(self)
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
#[doc = "CTSU Channel Enable Control Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsuchac3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsuchac3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTSUCHAC3_SPEC;
impl crate::RegisterSpec for CTSUCHAC3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctsuchac3::R`](R) reader structure"]
impl crate::Readable for CTSUCHAC3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctsuchac3::W`](W) writer structure"]
impl crate::Writable for CTSUCHAC3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUCHAC3 to value 0"]
impl crate::Resettable for CTSUCHAC3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
