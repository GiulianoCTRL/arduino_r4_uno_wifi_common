#[doc = "Register `BCNT1AR` reader"]
pub type R = crate::R<BCNT1AR_SPEC>;
#[doc = "Register `BCNT1AR` writer"]
pub type W = crate::W<BCNT1AR_SPEC>;
#[doc = "Field `BCNT1AR` reader - he BCNT1AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b15 to b8."]
pub type BCNT1AR_R = crate::FieldReader;
#[doc = "Field `BCNT1AR` writer - he BCNT1AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b15 to b8."]
pub type BCNT1AR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - he BCNT1AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b15 to b8."]
    #[inline(always)]
    pub fn bcnt1ar(&self) -> BCNT1AR_R {
        BCNT1AR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - he BCNT1AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b15 to b8."]
    #[inline(always)]
    #[must_use]
    pub fn bcnt1ar(&mut self) -> BCNT1AR_W<BCNT1AR_SPEC, 0> {
        BCNT1AR_W::new(self)
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
#[doc = "Binary Counter 1 Alarm Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcnt1ar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcnt1ar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BCNT1AR_SPEC;
impl crate::RegisterSpec for BCNT1AR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bcnt1ar::R`](R) reader structure"]
impl crate::Readable for BCNT1AR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bcnt1ar::W`](W) writer structure"]
impl crate::Writable for BCNT1AR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCNT1AR to value 0"]
impl crate::Resettable for BCNT1AR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
