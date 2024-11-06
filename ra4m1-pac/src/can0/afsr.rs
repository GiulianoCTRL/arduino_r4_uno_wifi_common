#[doc = "Register `AFSR` reader"]
pub type R = crate::R<AFSR_SPEC>;
#[doc = "Register `AFSR` writer"]
pub type W = crate::W<AFSR_SPEC>;
#[doc = "Field `AFSR` reader - After the standard ID of a received message is written, the value converted for data table search can be read."]
pub type AFSR_R = crate::FieldReader<u16>;
#[doc = "Field `AFSR` writer - After the standard ID of a received message is written, the value converted for data table search can be read."]
pub type AFSR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - After the standard ID of a received message is written, the value converted for data table search can be read."]
    #[inline(always)]
    pub fn afsr(&self) -> AFSR_R {
        AFSR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - After the standard ID of a received message is written, the value converted for data table search can be read."]
    #[inline(always)]
    #[must_use]
    pub fn afsr(&mut self) -> AFSR_W<AFSR_SPEC, 0> {
        AFSR_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Acceptance Filter Support Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AFSR_SPEC;
impl crate::RegisterSpec for AFSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`afsr::R`](R) reader structure"]
impl crate::Readable for AFSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`afsr::W`](W) writer structure"]
impl crate::Writable for AFSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFSR to value 0"]
impl crate::Resettable for AFSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
