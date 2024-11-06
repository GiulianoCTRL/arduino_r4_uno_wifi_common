#[doc = "Register `TDR` reader"]
pub type R = crate::R<TDR_SPEC>;
#[doc = "Register `TDR` writer"]
pub type W = crate::W<TDR_SPEC>;
#[doc = "Field `TDR` reader - TDR is an 8-bit register that stores transmit data."]
pub type TDR_R = crate::FieldReader;
#[doc = "Field `TDR` writer - TDR is an 8-bit register that stores transmit data."]
pub type TDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - TDR is an 8-bit register that stores transmit data."]
    #[inline(always)]
    pub fn tdr(&self) -> TDR_R {
        TDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - TDR is an 8-bit register that stores transmit data."]
    #[inline(always)]
    #[must_use]
    pub fn tdr(&mut self) -> TDR_W<TDR_SPEC, 0> {
        TDR_W::new(self)
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
#[doc = "Transmit Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TDR_SPEC;
impl crate::RegisterSpec for TDR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tdr::R`](R) reader structure"]
impl crate::Readable for TDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tdr::W`](W) writer structure"]
impl crate::Writable for TDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TDR to value 0xff"]
impl crate::Resettable for TDR_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
