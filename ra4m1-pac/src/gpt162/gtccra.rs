#[doc = "Register `GTCCRA` reader"]
pub type R = crate::R<GTCCRA_SPEC>;
#[doc = "Register `GTCCRA` writer"]
pub type W = crate::W<GTCCRA_SPEC>;
#[doc = "Field `GTCCRA` reader - Compare Capture Register A"]
pub type GTCCRA_R = crate::FieldReader<u32>;
#[doc = "Field `GTCCRA` writer - Compare Capture Register A"]
pub type GTCCRA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Compare Capture Register A"]
    #[inline(always)]
    pub fn gtccra(&self) -> GTCCRA_R {
        GTCCRA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare Capture Register A"]
    #[inline(always)]
    #[must_use]
    pub fn gtccra(&mut self) -> GTCCRA_W<GTCCRA_SPEC, 0> {
        GTCCRA_W::new(self)
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
#[doc = "General PWM Timer Compare Capture Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtccra::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtccra::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTCCRA_SPEC;
impl crate::RegisterSpec for GTCCRA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtccra::R`](R) reader structure"]
impl crate::Readable for GTCCRA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gtccra::W`](W) writer structure"]
impl crate::Writable for GTCCRA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTCCRA to value 0xffff"]
impl crate::Resettable for GTCCRA_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
