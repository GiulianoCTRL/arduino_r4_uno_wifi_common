#[doc = "Register `CTSUCHAC0` reader"]
pub type R = crate::R<CTSUCHAC0_SPEC>;
#[doc = "Register `CTSUCHAC0` writer"]
pub type W = crate::W<CTSUCHAC0_SPEC>;
#[doc = "Field `CTSUCHAC0` reader - CTSU Channel Enable Control 0. 0: Not measurement target 1: Measurement target Note: CTSUCHAC0\\[0\\]
corresponds to TS00 and CTSUCHAC0\\[7\\]
corresponds to TS07. but the write value of CTSUCHAC0\\[2\\]
should be 0."]
pub type CTSUCHAC0_R = crate::FieldReader<CTSUCHAC0_A>;
#[doc = "CTSU Channel Enable Control 0. 0: Not measurement target 1: Measurement target Note: CTSUCHAC0\\[0\\]
corresponds to TS00 and CTSUCHAC0\\[7\\]
corresponds to TS07. but the write value of CTSUCHAC0\\[2\\]
should be 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CTSUCHAC0_A(u8);
impl From<CTSUCHAC0_A> for u8 {
    #[inline(always)]
    fn from(val: CTSUCHAC0_A) -> Self {
        val.0 as _
    }
}
impl crate::FieldSpec for CTSUCHAC0_A {
    type Ux = u8;
}
#[doc = "Field `CTSUCHAC0` writer - CTSU Channel Enable Control 0. 0: Not measurement target 1: Measurement target Note: CTSUCHAC0\\[0\\]
corresponds to TS00 and CTSUCHAC0\\[7\\]
corresponds to TS07. but the write value of CTSUCHAC0\\[2\\]
should be 0."]
pub type CTSUCHAC0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, CTSUCHAC0_A>;
impl R {
    #[doc = "Bits 0:7 - CTSU Channel Enable Control 0. 0: Not measurement target 1: Measurement target Note: CTSUCHAC0\\[0\\]
corresponds to TS00 and CTSUCHAC0\\[7\\]
corresponds to TS07. but the write value of CTSUCHAC0\\[2\\]
should be 0."]
    #[inline(always)]
    pub fn ctsuchac0(&self) -> CTSUCHAC0_R {
        CTSUCHAC0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - CTSU Channel Enable Control 0. 0: Not measurement target 1: Measurement target Note: CTSUCHAC0\\[0\\]
corresponds to TS00 and CTSUCHAC0\\[7\\]
corresponds to TS07. but the write value of CTSUCHAC0\\[2\\]
should be 0."]
    #[inline(always)]
    #[must_use]
    pub fn ctsuchac0(&mut self) -> CTSUCHAC0_W<CTSUCHAC0_SPEC, 0> {
        CTSUCHAC0_W::new(self)
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
#[doc = "CTSU Channel Enable Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsuchac0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsuchac0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTSUCHAC0_SPEC;
impl crate::RegisterSpec for CTSUCHAC0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctsuchac0::R`](R) reader structure"]
impl crate::Readable for CTSUCHAC0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctsuchac0::W`](W) writer structure"]
impl crate::Writable for CTSUCHAC0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUCHAC0 to value 0"]
impl crate::Resettable for CTSUCHAC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
