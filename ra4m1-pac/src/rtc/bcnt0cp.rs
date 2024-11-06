#[doc = "Register `BCNT0CP%s` reader"]
pub type R = crate::R<BCNT0CP_SPEC>;
#[doc = "Field `BCNT0CP` reader - BCNT0CP is a read-only register that captures the BCNT0 value when a time capture event is detected."]
pub type BCNT0CP_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - BCNT0CP is a read-only register that captures the BCNT0 value when a time capture event is detected."]
    #[inline(always)]
    pub fn bcnt0cp(&self) -> BCNT0CP_R {
        BCNT0CP_R::new(self.bits)
    }
}
#[doc = "BCNT0 Capture Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcnt0cp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BCNT0CP_SPEC;
impl crate::RegisterSpec for BCNT0CP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bcnt0cp::R`](R) reader structure"]
impl crate::Readable for BCNT0CP_SPEC {}
#[doc = "`reset()` method sets BCNT0CP%s to value 0"]
impl crate::Resettable for BCNT0CP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
