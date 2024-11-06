#[doc = "Register `FRDRL` reader"]
pub type R = crate::R<FRDRL_SPEC>;
#[doc = "Field `RDATL` reader - Serial receive data (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected) NOTE: When reading both of FRDRH register and FRDRL register, please read by an order of the FRDRH register and the FRDRL register."]
pub type RDATL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Serial receive data (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected) NOTE: When reading both of FRDRH register and FRDRL register, please read by an order of the FRDRH register and the FRDRL register."]
    #[inline(always)]
    pub fn rdatl(&self) -> RDATL_R {
        RDATL_R::new(self.bits)
    }
}
#[doc = "Receive FIFO Data Register L\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frdrl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRDRL_SPEC;
impl crate::RegisterSpec for FRDRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`frdrl::R`](R) reader structure"]
impl crate::Readable for FRDRL_SPEC {}
#[doc = "`reset()` method sets FRDRL to value 0"]
impl crate::Resettable for FRDRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
