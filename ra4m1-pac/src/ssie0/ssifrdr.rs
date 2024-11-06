#[doc = "Register `SSIFRDR` reader"]
pub type R = crate::R<SSIFRDR_SPEC>;
#[doc = "Field `SSIFRDR` reader - Receive FIFO data."]
pub type SSIFRDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Receive FIFO data."]
    #[inline(always)]
    pub fn ssifrdr(&self) -> SSIFRDR_R {
        SSIFRDR_R::new(self.bits)
    }
}
#[doc = "Receive FIFO Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssifrdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSIFRDR_SPEC;
impl crate::RegisterSpec for SSIFRDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssifrdr::R`](R) reader structure"]
impl crate::Readable for SSIFRDR_SPEC {}
#[doc = "`reset()` method sets SSIFRDR to value 0"]
impl crate::Resettable for SSIFRDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
