#[doc = "Register `TECR` reader"]
pub type R = crate::R<TECR_SPEC>;
#[doc = "Field `TECR` reader - Transmit error count function TECR increments or decrements the counter value according to the error status of the CAN module during transmission."]
pub type TECR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Transmit error count function TECR increments or decrements the counter value according to the error status of the CAN module during transmission."]
    #[inline(always)]
    pub fn tecr(&self) -> TECR_R {
        TECR_R::new(self.bits)
    }
}
#[doc = "Transmit Error Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tecr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TECR_SPEC;
impl crate::RegisterSpec for TECR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tecr::R`](R) reader structure"]
impl crate::Readable for TECR_SPEC {}
#[doc = "`reset()` method sets TECR to value 0"]
impl crate::Resettable for TECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
