#[doc = "Register `AGTCMB` reader"]
pub type R = crate::R<AGTCMB_SPEC>;
#[doc = "Register `AGTCMB` writer"]
pub type W = crate::W<AGTCMB_SPEC>;
#[doc = "Field `AGTCMB` reader - AGT Compare Match B data is stored. NOTE : When 1 is written to the TSTOP bit in the AGTCR register, set to FFFFH"]
pub type AGTCMB_R = crate::FieldReader<u16>;
#[doc = "Field `AGTCMB` writer - AGT Compare Match B data is stored. NOTE : When 1 is written to the TSTOP bit in the AGTCR register, set to FFFFH"]
pub type AGTCMB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - AGT Compare Match B data is stored. NOTE : When 1 is written to the TSTOP bit in the AGTCR register, set to FFFFH"]
    #[inline(always)]
    pub fn agtcmb(&self) -> AGTCMB_R {
        AGTCMB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - AGT Compare Match B data is stored. NOTE : When 1 is written to the TSTOP bit in the AGTCR register, set to FFFFH"]
    #[inline(always)]
    #[must_use]
    pub fn agtcmb(&mut self) -> AGTCMB_W<AGTCMB_SPEC, 0> {
        AGTCMB_W::new(self)
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
#[doc = "AGT Compare Match B Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`agtcmb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`agtcmb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AGTCMB_SPEC;
impl crate::RegisterSpec for AGTCMB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`agtcmb::R`](R) reader structure"]
impl crate::Readable for AGTCMB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`agtcmb::W`](W) writer structure"]
impl crate::Writable for AGTCMB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AGTCMB to value 0xffff"]
impl crate::Resettable for AGTCMB_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
