#[doc = "Register `CTSUCHAC4` reader"]
pub type R = crate::R<CTSUCHAC4_SPEC>;
#[doc = "Register `CTSUCHAC4` writer"]
pub type W = crate::W<CTSUCHAC4_SPEC>;
#[doc = "Field `CTSUCHAC4` reader - CTSU Channel Enable Control 4. 0: Not measurement target 1: Measurement target Note: CTSUCHAC4\\[0\\]
corresponds to TS32 and CTSUCHAC4\\[3\\]
corresponds to TS35. but the write value of CTSUCHAC0\\[4\\],CTSUCHAC4\\[5\\],CTSUCHAC4\\[6\\],CTSUCHAC4\\[7\\]
should be 0."]
pub type CTSUCHAC4_R = crate::FieldReader<CTSUCHAC4_A>;
#[doc = "CTSU Channel Enable Control 4. 0: Not measurement target 1: Measurement target Note: CTSUCHAC4\\[0\\]
corresponds to TS32 and CTSUCHAC4\\[3\\]
corresponds to TS35. but the write value of CTSUCHAC0\\[4\\],CTSUCHAC4\\[5\\],CTSUCHAC4\\[6\\],CTSUCHAC4\\[7\\]
should be 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CTSUCHAC4_A(u8);
impl From<CTSUCHAC4_A> for u8 {
    #[inline(always)]
    fn from(val: CTSUCHAC4_A) -> Self {
        val.0 as _
    }
}
impl crate::FieldSpec for CTSUCHAC4_A {
    type Ux = u8;
}
#[doc = "Field `CTSUCHAC4` writer - CTSU Channel Enable Control 4. 0: Not measurement target 1: Measurement target Note: CTSUCHAC4\\[0\\]
corresponds to TS32 and CTSUCHAC4\\[3\\]
corresponds to TS35. but the write value of CTSUCHAC0\\[4\\],CTSUCHAC4\\[5\\],CTSUCHAC4\\[6\\],CTSUCHAC4\\[7\\]
should be 0."]
pub type CTSUCHAC4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, CTSUCHAC4_A>;
impl R {
    #[doc = "Bits 0:3 - CTSU Channel Enable Control 4. 0: Not measurement target 1: Measurement target Note: CTSUCHAC4\\[0\\]
corresponds to TS32 and CTSUCHAC4\\[3\\]
corresponds to TS35. but the write value of CTSUCHAC0\\[4\\],CTSUCHAC4\\[5\\],CTSUCHAC4\\[6\\],CTSUCHAC4\\[7\\]
should be 0."]
    #[inline(always)]
    pub fn ctsuchac4(&self) -> CTSUCHAC4_R {
        CTSUCHAC4_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - CTSU Channel Enable Control 4. 0: Not measurement target 1: Measurement target Note: CTSUCHAC4\\[0\\]
corresponds to TS32 and CTSUCHAC4\\[3\\]
corresponds to TS35. but the write value of CTSUCHAC0\\[4\\],CTSUCHAC4\\[5\\],CTSUCHAC4\\[6\\],CTSUCHAC4\\[7\\]
should be 0."]
    #[inline(always)]
    #[must_use]
    pub fn ctsuchac4(&mut self) -> CTSUCHAC4_W<CTSUCHAC4_SPEC, 0> {
        CTSUCHAC4_W::new(self)
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
#[doc = "CTSU Channel Enable Control Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsuchac4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsuchac4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTSUCHAC4_SPEC;
impl crate::RegisterSpec for CTSUCHAC4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctsuchac4::R`](R) reader structure"]
impl crate::Readable for CTSUCHAC4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctsuchac4::W`](W) writer structure"]
impl crate::Writable for CTSUCHAC4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUCHAC4 to value 0"]
impl crate::Resettable for CTSUCHAC4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
