#[doc = "Register `AGTCMA` reader"]
pub type R = crate::R<AGTCMA_SPEC>;
#[doc = "Register `AGTCMA` writer"]
pub type W = crate::W<AGTCMA_SPEC>;
#[doc = "Field `AGTCMA` reader - AGT Compare Match A data is stored. NOTE : When 1 is written to the TSTOP bit in the AGTCRn register, set to FFFFH"]
pub type AGTCMA_R = crate::FieldReader<u16>;
#[doc = "Field `AGTCMA` writer - AGT Compare Match A data is stored. NOTE : When 1 is written to the TSTOP bit in the AGTCRn register, set to FFFFH"]
pub type AGTCMA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - AGT Compare Match A data is stored. NOTE : When 1 is written to the TSTOP bit in the AGTCRn register, set to FFFFH"]
    #[inline(always)]
    pub fn agtcma(&self) -> AGTCMA_R {
        AGTCMA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - AGT Compare Match A data is stored. NOTE : When 1 is written to the TSTOP bit in the AGTCRn register, set to FFFFH"]
    #[inline(always)]
    #[must_use]
    pub fn agtcma(&mut self) -> AGTCMA_W<AGTCMA_SPEC, 0> {
        AGTCMA_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AGT Compare Match A Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`agtcma::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`agtcma::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AGTCMA_SPEC;
impl crate::RegisterSpec for AGTCMA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`agtcma::R`](R) reader structure"]
impl crate::Readable for AGTCMA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`agtcma::W`](W) writer structure"]
impl crate::Writable for AGTCMA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AGTCMA to value 0xffff"]
impl crate::Resettable for AGTCMA_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
