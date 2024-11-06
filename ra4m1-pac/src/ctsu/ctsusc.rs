#[doc = "Register `CTSUSC` reader"]
pub type R = crate::R<CTSUSC_SPEC>;
#[doc = "Field `CTSUSC` reader - CTSU Sensor Counter These bits indicate the measurement result of the CTSU. These bits indicate FFFFh when an overflow occurs."]
pub type CTSUSC_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - CTSU Sensor Counter These bits indicate the measurement result of the CTSU. These bits indicate FFFFh when an overflow occurs."]
    #[inline(always)]
    pub fn ctsusc(&self) -> CTSUSC_R {
        CTSUSC_R::new(self.bits)
    }
}
#[doc = "CTSU Sensor Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsusc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTSUSC_SPEC;
impl crate::RegisterSpec for CTSUSC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctsusc::R`](R) reader structure"]
impl crate::Readable for CTSUSC_SPEC {}
#[doc = "`reset()` method sets CTSUSC to value 0"]
impl crate::Resettable for CTSUSC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
