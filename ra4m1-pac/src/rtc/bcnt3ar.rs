#[doc = "Register `BCNT3AR` reader"]
pub type R = crate::R<BCNT3AR_SPEC>;
#[doc = "Register `BCNT3AR` writer"]
pub type W = crate::W<BCNT3AR_SPEC>;
#[doc = "Field `BCNT3AR` reader - The BCNT3AR counter is a readable/writable 32-bit binary counter b31 to b24."]
pub type BCNT3AR_R = crate::FieldReader;
#[doc = "Field `BCNT3AR` writer - The BCNT3AR counter is a readable/writable 32-bit binary counter b31 to b24."]
pub type BCNT3AR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - The BCNT3AR counter is a readable/writable 32-bit binary counter b31 to b24."]
    #[inline(always)]
    pub fn bcnt3ar(&self) -> BCNT3AR_R {
        BCNT3AR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - The BCNT3AR counter is a readable/writable 32-bit binary counter b31 to b24."]
    #[inline(always)]
    #[must_use]
    pub fn bcnt3ar(&mut self) -> BCNT3AR_W<BCNT3AR_SPEC, 0> {
        BCNT3AR_W::new(self)
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
#[doc = "Binary Counter 3 Alarm Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcnt3ar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcnt3ar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BCNT3AR_SPEC;
impl crate::RegisterSpec for BCNT3AR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bcnt3ar::R`](R) reader structure"]
impl crate::Readable for BCNT3AR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bcnt3ar::W`](W) writer structure"]
impl crate::Writable for BCNT3AR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCNT3AR to value 0"]
impl crate::Resettable for BCNT3AR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
