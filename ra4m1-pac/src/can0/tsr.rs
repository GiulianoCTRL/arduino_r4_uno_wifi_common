#[doc = "Register `TSR` reader"]
pub type R = crate::R<TSR_SPEC>;
#[doc = "Field `TSR` reader - Free-running counter value for the time stamp function"]
pub type TSR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Free-running counter value for the time stamp function"]
    #[inline(always)]
    pub fn tsr(&self) -> TSR_R {
        TSR_R::new(self.bits)
    }
}
#[doc = "Time Stamp Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSR_SPEC;
impl crate::RegisterSpec for TSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tsr::R`](R) reader structure"]
impl crate::Readable for TSR_SPEC {}
#[doc = "`reset()` method sets TSR to value 0"]
impl crate::Resettable for TSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
