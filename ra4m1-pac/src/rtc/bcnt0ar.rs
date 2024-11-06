#[doc = "Register `BCNT0AR` reader"]
pub type R = crate::R<BCNT0AR_SPEC>;
#[doc = "Register `BCNT0AR` writer"]
pub type W = crate::W<BCNT0AR_SPEC>;
#[doc = "Field `BCNT0AR` reader - he BCNT0AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b7 to b0."]
pub type BCNT0AR_R = crate::FieldReader;
#[doc = "Field `BCNT0AR` writer - he BCNT0AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b7 to b0."]
pub type BCNT0AR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - he BCNT0AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b7 to b0."]
    #[inline(always)]
    pub fn bcnt0ar(&self) -> BCNT0AR_R {
        BCNT0AR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - he BCNT0AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b7 to b0."]
    #[inline(always)]
    #[must_use]
    pub fn bcnt0ar(&mut self) -> BCNT0AR_W<BCNT0AR_SPEC, 0> {
        BCNT0AR_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Binary Counter 0 Alarm Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcnt0ar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcnt0ar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BCNT0AR_SPEC;
impl crate::RegisterSpec for BCNT0AR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bcnt0ar::R`](R) reader structure"]
impl crate::Readable for BCNT0AR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bcnt0ar::W`](W) writer structure"]
impl crate::Writable for BCNT0AR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCNT0AR to value 0"]
impl crate::Resettable for BCNT0AR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
