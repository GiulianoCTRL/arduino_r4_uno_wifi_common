#[doc = "Register `CTSUMCH1` reader"]
pub type R = crate::R<CTSUMCH1_SPEC>;
#[doc = "Register `CTSUMCH1` writer"]
pub type W = crate::W<CTSUMCH1_SPEC>;
#[doc = "Field `CTSUMCH1` reader - CTSU Measurement Channel 1 Note1: If the value of CTSUMCH1 was set to b'111111, the measurement is stopped."]
pub type CTSUMCH1_R = crate::FieldReader<CTSUMCH1_A>;
#[doc = "CTSU Measurement Channel 1 Note1: If the value of CTSUMCH1 was set to b'111111, the measurement is stopped.\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CTSUMCH1_A(u8);
impl From<CTSUMCH1_A> for u8 {
    #[inline(always)]
    fn from(val: CTSUMCH1_A) -> Self {
        val.0 as _
    }
}
impl crate::FieldSpec for CTSUMCH1_A {
    type Ux = u8;
}
impl R {
    #[doc = "Bits 0:5 - CTSU Measurement Channel 1 Note1: If the value of CTSUMCH1 was set to b'111111, the measurement is stopped."]
    #[inline(always)]
    pub fn ctsumch1(&self) -> CTSUMCH1_R {
        CTSUMCH1_R::new(self.bits & 0x3f)
    }
}
impl W {
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
#[doc = "CTSU Measurement Channel Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsumch1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsumch1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTSUMCH1_SPEC;
impl crate::RegisterSpec for CTSUMCH1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctsumch1::R`](R) reader structure"]
impl crate::Readable for CTSUMCH1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctsumch1::W`](W) writer structure"]
impl crate::Writable for CTSUMCH1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUMCH1 to value 0x3f"]
impl crate::Resettable for CTSUMCH1_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f;
}
