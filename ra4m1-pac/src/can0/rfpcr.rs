#[doc = "Register `RFPCR` writer"]
pub type W = crate::W<RFPCR_SPEC>;
#[doc = "Field `RFPCR` writer - The CPU-side pointer for the receive FIFO is incremented by writing FFh to RFPCR."]
pub type RFPCR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl W {
    #[doc = "Bits 0:7 - The CPU-side pointer for the receive FIFO is incremented by writing FFh to RFPCR."]
    #[inline(always)]
    #[must_use]
    pub fn rfpcr(&mut self) -> RFPCR_W<RFPCR_SPEC, 0> {
        RFPCR_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Receive FIFO Pointer Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfpcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFPCR_SPEC;
impl crate::RegisterSpec for RFPCR_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`rfpcr::W`](W) writer structure"]
impl crate::Writable for RFPCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFPCR to value 0"]
impl crate::Resettable for RFPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
