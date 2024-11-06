#[doc = "Register `ICDRR` reader"]
pub type R = crate::R<ICDRR_SPEC>;
#[doc = "Field `ICDRR` reader - 8-bit register that stores the received data"]
pub type ICDRR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 8-bit register that stores the received data"]
    #[inline(always)]
    pub fn icdrr(&self) -> ICDRR_R {
        ICDRR_R::new(self.bits)
    }
}
#[doc = "I2C Bus Receive Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icdrr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICDRR_SPEC;
impl crate::RegisterSpec for ICDRR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`icdrr::R`](R) reader structure"]
impl crate::Readable for ICDRR_SPEC {}
#[doc = "`reset()` method sets ICDRR to value 0"]
impl crate::Resettable for ICDRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
