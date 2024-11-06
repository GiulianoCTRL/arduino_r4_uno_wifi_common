#[doc = "Register `GTCNT` reader"]
pub type R = crate::R<GTCNT_SPEC>;
#[doc = "Register `GTCNT` writer"]
pub type W = crate::W<GTCNT_SPEC>;
#[doc = "Field `GTCNT` reader - Counter"]
pub type GTCNT_R = crate::FieldReader<u32>;
#[doc = "Field `GTCNT` writer - Counter"]
pub type GTCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter"]
    #[inline(always)]
    pub fn gtcnt(&self) -> GTCNT_R {
        GTCNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter"]
    #[inline(always)]
    #[must_use]
    pub fn gtcnt(&mut self) -> GTCNT_W<GTCNT_SPEC, 0> {
        GTCNT_W::new(self)
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
#[doc = "General PWM Timer Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTCNT_SPEC;
impl crate::RegisterSpec for GTCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtcnt::R`](R) reader structure"]
impl crate::Readable for GTCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gtcnt::W`](W) writer structure"]
impl crate::Writable for GTCNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTCNT to value 0"]
impl crate::Resettable for GTCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
