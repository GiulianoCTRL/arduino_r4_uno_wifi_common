#[doc = "Register `WDTRR` reader"]
pub type R = crate::R<WDTRR_SPEC>;
#[doc = "Register `WDTRR` writer"]
pub type W = crate::W<WDTRR_SPEC>;
#[doc = "Field `WDTRR` reader - WDTRR is an 8-bit register that refreshes the down-counter of the WDT."]
pub type WDTRR_R = crate::FieldReader;
#[doc = "Field `WDTRR` writer - WDTRR is an 8-bit register that refreshes the down-counter of the WDT."]
pub type WDTRR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - WDTRR is an 8-bit register that refreshes the down-counter of the WDT."]
    #[inline(always)]
    pub fn wdtrr(&self) -> WDTRR_R {
        WDTRR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - WDTRR is an 8-bit register that refreshes the down-counter of the WDT."]
    #[inline(always)]
    #[must_use]
    pub fn wdtrr(&mut self) -> WDTRR_W<WDTRR_SPEC, 0> {
        WDTRR_W::new(self)
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
#[doc = "WDT Refresh Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDTRR_SPEC;
impl crate::RegisterSpec for WDTRR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wdtrr::R`](R) reader structure"]
impl crate::Readable for WDTRR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdtrr::W`](W) writer structure"]
impl crate::Writable for WDTRR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTRR to value 0xff"]
impl crate::Resettable for WDTRR_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
