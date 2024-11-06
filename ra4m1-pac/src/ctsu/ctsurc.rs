#[doc = "Register `CTSURC` reader"]
pub type R = crate::R<CTSURC_SPEC>;
#[doc = "Field `CTSURC` reader - CTSU Reference Counter These bits indicate the measurement result of the reference ICO. These bits indicate FFFFh when an overflow occurs."]
pub type CTSURC_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - CTSU Reference Counter These bits indicate the measurement result of the reference ICO. These bits indicate FFFFh when an overflow occurs."]
    #[inline(always)]
    pub fn ctsurc(&self) -> CTSURC_R {
        CTSURC_R::new(self.bits)
    }
}
#[doc = "CTSU Reference Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsurc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTSURC_SPEC;
impl crate::RegisterSpec for CTSURC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctsurc::R`](R) reader structure"]
impl crate::Readable for CTSURC_SPEC {}
#[doc = "`reset()` method sets CTSURC to value 0"]
impl crate::Resettable for CTSURC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
