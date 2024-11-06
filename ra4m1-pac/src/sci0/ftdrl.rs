#[doc = "Register `FTDRL` writer"]
pub type W = crate::W<FTDRL_SPEC>;
#[doc = "Field `TDATL` writer - Serial transmit data(b7-b0) (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
pub type TDATL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl W {
    #[doc = "Bits 0:7 - Serial transmit data(b7-b0) (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
    #[inline(always)]
    #[must_use]
    pub fn tdatl(&mut self) -> TDATL_W<FTDRL_SPEC, 0> {
        TDATL_W::new(self)
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
#[doc = "Transmit FIFO Data Register L\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftdrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FTDRL_SPEC;
impl crate::RegisterSpec for FTDRL_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`ftdrl::W`](W) writer structure"]
impl crate::Writable for FTDRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FTDRL to value 0xff"]
impl crate::Resettable for FTDRL_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
