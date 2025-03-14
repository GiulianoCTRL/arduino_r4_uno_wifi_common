#[doc = "Register `GTCCRD` reader"]
pub type R = crate::R<GTCCRD_SPEC>;
#[doc = "Register `GTCCRD` writer"]
pub type W = crate::W<GTCCRD_SPEC>;
#[doc = "Field `GTCCRD` reader - Compare Capture Register D"]
pub type GTCCRD_R = crate::FieldReader<u32>;
#[doc = "Field `GTCCRD` writer - Compare Capture Register D"]
pub type GTCCRD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Compare Capture Register D"]
    #[inline(always)]
    pub fn gtccrd(&self) -> GTCCRD_R {
        GTCCRD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare Capture Register D"]
    #[inline(always)]
    #[must_use]
    pub fn gtccrd(&mut self) -> GTCCRD_W<GTCCRD_SPEC, 0> {
        GTCCRD_W::new(self)
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
#[doc = "General PWM Timer Compare Capture Register D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtccrd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtccrd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTCCRD_SPEC;
impl crate::RegisterSpec for GTCCRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtccrd::R`](R) reader structure"]
impl crate::Readable for GTCCRD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gtccrd::W`](W) writer structure"]
impl crate::Writable for GTCCRD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTCCRD to value 0xffff_ffff"]
impl crate::Resettable for GTCCRD_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
