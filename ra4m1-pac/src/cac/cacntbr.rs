#[doc = "Register `CACNTBR` reader"]
pub type R = crate::R<CACNTBR_SPEC>;
#[doc = "Field `CACNTBR` reader - CACNTBR is a 16-bit read-only register that retains the counter value at the time a valid reference signal edge is input"]
pub type CACNTBR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - CACNTBR is a 16-bit read-only register that retains the counter value at the time a valid reference signal edge is input"]
    #[inline(always)]
    pub fn cacntbr(&self) -> CACNTBR_R {
        CACNTBR_R::new(self.bits)
    }
}
#[doc = "CAC Counter Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cacntbr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACNTBR_SPEC;
impl crate::RegisterSpec for CACNTBR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cacntbr::R`](R) reader structure"]
impl crate::Readable for CACNTBR_SPEC {}
#[doc = "`reset()` method sets CACNTBR to value 0"]
impl crate::Resettable for CACNTBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
