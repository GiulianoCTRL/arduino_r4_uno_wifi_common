#[doc = "Register `FDR` reader"]
pub type R = crate::R<FDR_SPEC>;
#[doc = "Field `R` reader - Receive FIFO Data Count Indicate the quantity of receive data stored in FRDRH and FRDRL (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, while FCR.FM=1)"]
pub type R_R = crate::FieldReader;
#[doc = "Field `T` reader - Transmit FIFO Data Count Indicate the quantity of non-transmit data stored in FTDRH and FTDRL (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, while FCR.FM=1)"]
pub type T_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Receive FIFO Data Count Indicate the quantity of receive data stored in FRDRH and FRDRL (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, while FCR.FM=1)"]
    #[inline(always)]
    pub fn r(&self) -> R_R {
        R_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Transmit FIFO Data Count Indicate the quantity of non-transmit data stored in FTDRH and FTDRL (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, while FCR.FM=1)"]
    #[inline(always)]
    pub fn t(&self) -> T_R {
        T_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
#[doc = "FIFO Data Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDR_SPEC;
impl crate::RegisterSpec for FDR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fdr::R`](R) reader structure"]
impl crate::Readable for FDR_SPEC {}
#[doc = "`reset()` method sets FDR to value 0"]
impl crate::Resettable for FDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
