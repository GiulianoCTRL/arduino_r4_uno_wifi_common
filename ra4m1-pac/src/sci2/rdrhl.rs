#[doc = "Register `RDRHL` reader"]
pub type R = crate::R<RDRHL_SPEC>;
#[doc = "Field `RDRHL` reader - RDRHL is an 16-bit register that stores receive data."]
pub type RDRHL_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - RDRHL is an 16-bit register that stores receive data."]
    #[inline(always)]
    pub fn rdrhl(&self) -> RDRHL_R {
        RDRHL_R::new(self.bits)
    }
}
#[doc = "Receive 9-bit Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdrhl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDRHL_SPEC;
impl crate::RegisterSpec for RDRHL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rdrhl::R`](R) reader structure"]
impl crate::Readable for RDRHL_SPEC {}
#[doc = "`reset()` method sets RDRHL to value 0"]
impl crate::Resettable for RDRHL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
