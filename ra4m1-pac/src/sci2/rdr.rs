#[doc = "Register `RDR` reader"]
pub type R = crate::R<RDR_SPEC>;
#[doc = "Field `RDR` reader - RDR is an 8-bit register that stores receive data."]
pub type RDR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RDR is an 8-bit register that stores receive data."]
    #[inline(always)]
    pub fn rdr(&self) -> RDR_R {
        RDR_R::new(self.bits)
    }
}
#[doc = "Receive Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDR_SPEC;
impl crate::RegisterSpec for RDR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rdr::R`](R) reader structure"]
impl crate::Readable for RDR_SPEC {}
#[doc = "`reset()` method sets RDR to value 0"]
impl crate::Resettable for RDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
