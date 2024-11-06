#[doc = "Register `ADDR%s` reader"]
pub type R = crate::R<ADDR_SPEC>;
#[doc = "Field `ADDR` reader - The ADDR register is a 16-bit read-only registers for storing the result of A/D conversion."]
pub type ADDR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - The ADDR register is a 16-bit read-only registers for storing the result of A/D conversion."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
#[doc = "A/D Data Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR_SPEC;
impl crate::RegisterSpec for ADDR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`addr::R`](R) reader structure"]
impl crate::Readable for ADDR_SPEC {}
#[doc = "`reset()` method sets ADDR%s to value 0"]
impl crate::Resettable for ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
