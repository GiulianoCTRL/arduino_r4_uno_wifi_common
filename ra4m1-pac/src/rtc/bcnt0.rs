#[doc = "Register `BCNT0` reader"]
pub type R = crate::R<BCNT0_SPEC>;
#[doc = "Register `BCNT0` writer"]
pub type W = crate::W<BCNT0_SPEC>;
#[doc = "Field `BCNT0` reader - The BCNT0 counter is a readable/writable 32-bit binary counter b7 to b0."]
pub type BCNT0_R = crate::FieldReader;
#[doc = "Field `BCNT0` writer - The BCNT0 counter is a readable/writable 32-bit binary counter b7 to b0."]
pub type BCNT0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - The BCNT0 counter is a readable/writable 32-bit binary counter b7 to b0."]
    #[inline(always)]
    pub fn bcnt0(&self) -> BCNT0_R {
        BCNT0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - The BCNT0 counter is a readable/writable 32-bit binary counter b7 to b0."]
    #[inline(always)]
    #[must_use]
    pub fn bcnt0(&mut self) -> BCNT0_W<BCNT0_SPEC, 0> {
        BCNT0_W::new(self)
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
#[doc = "Binary Counter 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcnt0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcnt0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BCNT0_SPEC;
impl crate::RegisterSpec for BCNT0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bcnt0::R`](R) reader structure"]
impl crate::Readable for BCNT0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bcnt0::W`](W) writer structure"]
impl crate::Writable for BCNT0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCNT0 to value 0"]
impl crate::Resettable for BCNT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
