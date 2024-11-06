#[doc = "Register `ADTSDR` reader"]
pub type R = crate::R<ADTSDR_SPEC>;
#[doc = "Field `ADTSDR` reader - This is a 16-bit read-only register for storing the A/D conversion result of temperature sensor output."]
pub type ADTSDR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - This is a 16-bit read-only register for storing the A/D conversion result of temperature sensor output."]
    #[inline(always)]
    pub fn adtsdr(&self) -> ADTSDR_R {
        ADTSDR_R::new(self.bits)
    }
}
#[doc = "A/D Temperature Sensor Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adtsdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADTSDR_SPEC;
impl crate::RegisterSpec for ADTSDR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adtsdr::R`](R) reader structure"]
impl crate::Readable for ADTSDR_SPEC {}
#[doc = "`reset()` method sets ADTSDR to value 0"]
impl crate::Resettable for ADTSDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
