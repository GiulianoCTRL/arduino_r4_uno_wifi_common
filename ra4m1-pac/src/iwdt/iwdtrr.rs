#[doc = "Register `IWDTRR` reader"]
pub type R = crate::R<IWDTRR_SPEC>;
#[doc = "Register `IWDTRR` writer"]
pub type W = crate::W<IWDTRR_SPEC>;
#[doc = "Field `IWDTRR` reader - The counter is refreshed by writing 0x00 and then writing 0xFF to this register."]
pub type IWDTRR_R = crate::FieldReader;
#[doc = "Field `IWDTRR` writer - The counter is refreshed by writing 0x00 and then writing 0xFF to this register."]
pub type IWDTRR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - The counter is refreshed by writing 0x00 and then writing 0xFF to this register."]
    #[inline(always)]
    pub fn iwdtrr(&self) -> IWDTRR_R {
        IWDTRR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - The counter is refreshed by writing 0x00 and then writing 0xFF to this register."]
    #[inline(always)]
    #[must_use]
    pub fn iwdtrr(&mut self) -> IWDTRR_W<IWDTRR_SPEC, 0> {
        IWDTRR_W::new(self)
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
#[doc = "IWDT Refresh Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdtrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwdtrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IWDTRR_SPEC;
impl crate::RegisterSpec for IWDTRR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`iwdtrr::R`](R) reader structure"]
impl crate::Readable for IWDTRR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iwdtrr::W`](W) writer structure"]
impl crate::Writable for IWDTRR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IWDTRR to value 0xff"]
impl crate::Resettable for IWDTRR_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
