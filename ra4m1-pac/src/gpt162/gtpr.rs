#[doc = "Register `GTPR` reader"]
pub type R = crate::R<GTPR_SPEC>;
#[doc = "Register `GTPR` writer"]
pub type W = crate::W<GTPR_SPEC>;
#[doc = "Field `GTPR` reader - Cycle Setting Register"]
pub type GTPR_R = crate::FieldReader<u32>;
#[doc = "Field `GTPR` writer - Cycle Setting Register"]
pub type GTPR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Cycle Setting Register"]
    #[inline(always)]
    pub fn gtpr(&self) -> GTPR_R {
        GTPR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cycle Setting Register"]
    #[inline(always)]
    #[must_use]
    pub fn gtpr(&mut self) -> GTPR_W<GTPR_SPEC, 0> {
        GTPR_W::new(self)
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
#[doc = "General PWM Timer Cycle Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTPR_SPEC;
impl crate::RegisterSpec for GTPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtpr::R`](R) reader structure"]
impl crate::Readable for GTPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gtpr::W`](W) writer structure"]
impl crate::Writable for GTPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTPR to value 0xffff"]
impl crate::Resettable for GTPR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
