#[doc = "Register `GTPBR` reader"]
pub type R = crate::R<GTPBR_SPEC>;
#[doc = "Register `GTPBR` writer"]
pub type W = crate::W<GTPBR_SPEC>;
#[doc = "Field `GTPBR` reader - Cycle Setting Buffer Register"]
pub type GTPBR_R = crate::FieldReader<u32>;
#[doc = "Field `GTPBR` writer - Cycle Setting Buffer Register"]
pub type GTPBR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Cycle Setting Buffer Register"]
    #[inline(always)]
    pub fn gtpbr(&self) -> GTPBR_R {
        GTPBR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cycle Setting Buffer Register"]
    #[inline(always)]
    #[must_use]
    pub fn gtpbr(&mut self) -> GTPBR_W<GTPBR_SPEC, 0> {
        GTPBR_W::new(self)
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
#[doc = "General PWM Timer Cycle Setting Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtpbr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtpbr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTPBR_SPEC;
impl crate::RegisterSpec for GTPBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtpbr::R`](R) reader structure"]
impl crate::Readable for GTPBR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gtpbr::W`](W) writer structure"]
impl crate::Writable for GTPBR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTPBR to value 0xffff"]
impl crate::Resettable for GTPBR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
