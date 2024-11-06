#[doc = "Register `CALLVR` reader"]
pub type R = crate::R<CALLVR_SPEC>;
#[doc = "Register `CALLVR` writer"]
pub type W = crate::W<CALLVR_SPEC>;
#[doc = "Field `CALLVR` reader - CALLVR is a 16-bit readable/writable register that stores the lower-limit value of the frequency."]
pub type CALLVR_R = crate::FieldReader<u16>;
#[doc = "Field `CALLVR` writer - CALLVR is a 16-bit readable/writable register that stores the lower-limit value of the frequency."]
pub type CALLVR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - CALLVR is a 16-bit readable/writable register that stores the lower-limit value of the frequency."]
    #[inline(always)]
    pub fn callvr(&self) -> CALLVR_R {
        CALLVR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CALLVR is a 16-bit readable/writable register that stores the lower-limit value of the frequency."]
    #[inline(always)]
    #[must_use]
    pub fn callvr(&mut self) -> CALLVR_W<CALLVR_SPEC, 0> {
        CALLVR_W::new(self)
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
#[doc = "CAC Lower-Limit Value Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`callvr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`callvr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALLVR_SPEC;
impl crate::RegisterSpec for CALLVR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`callvr::R`](R) reader structure"]
impl crate::Readable for CALLVR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`callvr::W`](W) writer structure"]
impl crate::Writable for CALLVR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CALLVR to value 0"]
impl crate::Resettable for CALLVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
