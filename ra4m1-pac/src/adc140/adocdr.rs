#[doc = "Register `ADOCDR` reader"]
pub type R = crate::R<ADOCDR_SPEC>;
#[doc = "Field `ADOCDR` reader - This is a 16-bit read-only register for storing the A/D result of internal reference voltage."]
pub type ADOCDR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - This is a 16-bit read-only register for storing the A/D result of internal reference voltage."]
    #[inline(always)]
    pub fn adocdr(&self) -> ADOCDR_R {
        ADOCDR_R::new(self.bits)
    }
}
#[doc = "A/D Internal Reference Voltage Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adocdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADOCDR_SPEC;
impl crate::RegisterSpec for ADOCDR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adocdr::R`](R) reader structure"]
impl crate::Readable for ADOCDR_SPEC {}
#[doc = "`reset()` method sets ADOCDR to value 0"]
impl crate::Resettable for ADOCDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
