#[doc = "Register `DMOFR` reader"]
pub type R = crate::R<DMOFR_SPEC>;
#[doc = "Register `DMOFR` writer"]
pub type W = crate::W<DMOFR_SPEC>;
#[doc = "Field `DMOFR` reader - Specifies the offset when offset addition is selected as the address update mode for transfer source or destination."]
pub type DMOFR_R = crate::FieldReader<u32>;
#[doc = "Field `DMOFR` writer - Specifies the offset when offset addition is selected as the address update mode for transfer source or destination."]
pub type DMOFR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies the offset when offset addition is selected as the address update mode for transfer source or destination."]
    #[inline(always)]
    pub fn dmofr(&self) -> DMOFR_R {
        DMOFR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies the offset when offset addition is selected as the address update mode for transfer source or destination."]
    #[inline(always)]
    #[must_use]
    pub fn dmofr(&mut self) -> DMOFR_W<DMOFR_SPEC, 0> {
        DMOFR_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA Offset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmofr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmofr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMOFR_SPEC;
impl crate::RegisterSpec for DMOFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmofr::R`](R) reader structure"]
impl crate::Readable for DMOFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmofr::W`](W) writer structure"]
impl crate::Writable for DMOFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMOFR to value 0"]
impl crate::Resettable for DMOFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
