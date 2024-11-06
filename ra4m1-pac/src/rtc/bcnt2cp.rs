#[doc = "Register `BCNT2CP%s` reader"]
pub type R = crate::R<BCNT2CP_SPEC>;
#[doc = "Field `BCNT2CP` reader - BCNT2CP is a read-only register that captures the BCNT2 value when a time capture event is detected."]
pub type BCNT2CP_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - BCNT2CP is a read-only register that captures the BCNT2 value when a time capture event is detected."]
    #[inline(always)]
    pub fn bcnt2cp(&self) -> BCNT2CP_R {
        BCNT2CP_R::new(self.bits)
    }
}
#[doc = "BCNT2 Capture Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcnt2cp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BCNT2CP_SPEC;
impl crate::RegisterSpec for BCNT2CP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bcnt2cp::R`](R) reader structure"]
impl crate::Readable for BCNT2CP_SPEC {}
#[doc = "`reset()` method sets BCNT2CP%s to value 0"]
impl crate::Resettable for BCNT2CP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
