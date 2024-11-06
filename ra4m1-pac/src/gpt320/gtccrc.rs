#[doc = "Register `GTCCRC` reader"]
pub type R = crate::R<GTCCRC_SPEC>;
#[doc = "Register `GTCCRC` writer"]
pub type W = crate::W<GTCCRC_SPEC>;
#[doc = "Field `GTCCRC` reader - Compare Capture Register C"]
pub type GTCCRC_R = crate::FieldReader<u32>;
#[doc = "Field `GTCCRC` writer - Compare Capture Register C"]
pub type GTCCRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Compare Capture Register C"]
    #[inline(always)]
    pub fn gtccrc(&self) -> GTCCRC_R {
        GTCCRC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare Capture Register C"]
    #[inline(always)]
    #[must_use]
    pub fn gtccrc(&mut self) -> GTCCRC_W<GTCCRC_SPEC, 0> {
        GTCCRC_W::new(self)
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
#[doc = "General PWM Timer Compare Capture Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtccrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtccrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTCCRC_SPEC;
impl crate::RegisterSpec for GTCCRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtccrc::R`](R) reader structure"]
impl crate::Readable for GTCCRC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gtccrc::W`](W) writer structure"]
impl crate::Writable for GTCCRC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTCCRC to value 0xffff_ffff"]
impl crate::Resettable for GTCCRC_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
