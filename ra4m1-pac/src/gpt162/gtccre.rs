#[doc = "Register `GTCCRE` reader"]
pub type R = crate::R<GTCCRE_SPEC>;
#[doc = "Register `GTCCRE` writer"]
pub type W = crate::W<GTCCRE_SPEC>;
#[doc = "Field `GTCCRE` reader - Compare Capture Register E"]
pub type GTCCRE_R = crate::FieldReader<u32>;
#[doc = "Field `GTCCRE` writer - Compare Capture Register E"]
pub type GTCCRE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Compare Capture Register E"]
    #[inline(always)]
    pub fn gtccre(&self) -> GTCCRE_R {
        GTCCRE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare Capture Register E"]
    #[inline(always)]
    #[must_use]
    pub fn gtccre(&mut self) -> GTCCRE_W<GTCCRE_SPEC, 0> {
        GTCCRE_W::new(self)
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
#[doc = "General PWM Timer Compare Capture Register E\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtccre::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtccre::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTCCRE_SPEC;
impl crate::RegisterSpec for GTCCRE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtccre::R`](R) reader structure"]
impl crate::Readable for GTCCRE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gtccre::W`](W) writer structure"]
impl crate::Writable for GTCCRE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTCCRE to value 0xffff"]
impl crate::Resettable for GTCCRE_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
