#[doc = "Register `BCNT2AR` reader"]
pub type R = crate::R<BCNT2AR_SPEC>;
#[doc = "Register `BCNT2AR` writer"]
pub type W = crate::W<BCNT2AR_SPEC>;
#[doc = "Field `BCNT2AR` reader - The BCNT2AR counter is a readable/writable 32-bit binary counter b23 to b16."]
pub type BCNT2AR_R = crate::FieldReader;
#[doc = "Field `BCNT2AR` writer - The BCNT2AR counter is a readable/writable 32-bit binary counter b23 to b16."]
pub type BCNT2AR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - The BCNT2AR counter is a readable/writable 32-bit binary counter b23 to b16."]
    #[inline(always)]
    pub fn bcnt2ar(&self) -> BCNT2AR_R {
        BCNT2AR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - The BCNT2AR counter is a readable/writable 32-bit binary counter b23 to b16."]
    #[inline(always)]
    #[must_use]
    pub fn bcnt2ar(&mut self) -> BCNT2AR_W<BCNT2AR_SPEC, 0> {
        BCNT2AR_W::new(self)
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
#[doc = "Binary Counter 2 Alarm Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcnt2ar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcnt2ar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BCNT2AR_SPEC;
impl crate::RegisterSpec for BCNT2AR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bcnt2ar::R`](R) reader structure"]
impl crate::Readable for BCNT2AR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bcnt2ar::W`](W) writer structure"]
impl crate::Writable for BCNT2AR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCNT2AR to value 0"]
impl crate::Resettable for BCNT2AR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
