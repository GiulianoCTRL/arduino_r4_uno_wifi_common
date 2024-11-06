#[doc = "Register `GTDVU` reader"]
pub type R = crate::R<GTDVU_SPEC>;
#[doc = "Register `GTDVU` writer"]
pub type W = crate::W<GTDVU_SPEC>;
#[doc = "Field `GTDVU` reader - Dead Time Value Register U"]
pub type GTDVU_R = crate::FieldReader<u32>;
#[doc = "Field `GTDVU` writer - Dead Time Value Register U"]
pub type GTDVU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Dead Time Value Register U"]
    #[inline(always)]
    pub fn gtdvu(&self) -> GTDVU_R {
        GTDVU_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Dead Time Value Register U"]
    #[inline(always)]
    #[must_use]
    pub fn gtdvu(&mut self) -> GTDVU_W<GTDVU_SPEC, 0> {
        GTDVU_W::new(self)
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
#[doc = "General PWM Timer Dead Time Value Register U\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtdvu::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtdvu::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTDVU_SPEC;
impl crate::RegisterSpec for GTDVU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtdvu::R`](R) reader structure"]
impl crate::Readable for GTDVU_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gtdvu::W`](W) writer structure"]
impl crate::Writable for GTDVU_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTDVU to value 0xffff"]
impl crate::Resettable for GTDVU_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
