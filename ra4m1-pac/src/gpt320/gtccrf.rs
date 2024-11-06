#[doc = "Register `GTCCRF` reader"]
pub type R = crate::R<GTCCRF_SPEC>;
#[doc = "Register `GTCCRF` writer"]
pub type W = crate::W<GTCCRF_SPEC>;
#[doc = "Field `GTCCRF` reader - Compare Capture Register F"]
pub type GTCCRF_R = crate::FieldReader<u32>;
#[doc = "Field `GTCCRF` writer - Compare Capture Register F"]
pub type GTCCRF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Compare Capture Register F"]
    #[inline(always)]
    pub fn gtccrf(&self) -> GTCCRF_R {
        GTCCRF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare Capture Register F"]
    #[inline(always)]
    #[must_use]
    pub fn gtccrf(&mut self) -> GTCCRF_W<GTCCRF_SPEC, 0> {
        GTCCRF_W::new(self)
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
#[doc = "General PWM Timer Compare Capture Register F\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtccrf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtccrf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTCCRF_SPEC;
impl crate::RegisterSpec for GTCCRF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtccrf::R`](R) reader structure"]
impl crate::Readable for GTCCRF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gtccrf::W`](W) writer structure"]
impl crate::Writable for GTCCRF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTCCRF to value 0xffff_ffff"]
impl crate::Resettable for GTCCRF_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
