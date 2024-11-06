#[doc = "Register `D0FIFOL` reader"]
pub type R = crate::R<D0FIFOL_SPEC>;
#[doc = "Register `D0FIFOL` writer"]
pub type W = crate::W<D0FIFOL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<D0FIFOL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
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
#[doc = "D0FIFO Port Register L\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d0fifol::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d0fifol::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D0FIFOL_SPEC;
impl crate::RegisterSpec for D0FIFOL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`d0fifol::R`](R) reader structure"]
impl crate::Readable for D0FIFOL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`d0fifol::W`](W) writer structure"]
impl crate::Writable for D0FIFOL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D0FIFOL to value 0"]
impl crate::Resettable for D0FIFOL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
