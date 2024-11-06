#[doc = "Register `CAULVR` reader"]
pub type R = crate::R<CAULVR_SPEC>;
#[doc = "Register `CAULVR` writer"]
pub type W = crate::W<CAULVR_SPEC>;
#[doc = "Field `CAULVR` reader - CAULVR is a 16-bit readable/writable register that stores the upper-limit value of the frequency."]
pub type CAULVR_R = crate::FieldReader<u16>;
#[doc = "Field `CAULVR` writer - CAULVR is a 16-bit readable/writable register that stores the upper-limit value of the frequency."]
pub type CAULVR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - CAULVR is a 16-bit readable/writable register that stores the upper-limit value of the frequency."]
    #[inline(always)]
    pub fn caulvr(&self) -> CAULVR_R {
        CAULVR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CAULVR is a 16-bit readable/writable register that stores the upper-limit value of the frequency."]
    #[inline(always)]
    #[must_use]
    pub fn caulvr(&mut self) -> CAULVR_W<CAULVR_SPEC, 0> {
        CAULVR_W::new(self)
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
#[doc = "CAC Upper-Limit Value Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`caulvr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`caulvr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAULVR_SPEC;
impl crate::RegisterSpec for CAULVR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`caulvr::R`](R) reader structure"]
impl crate::Readable for CAULVR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`caulvr::W`](W) writer structure"]
impl crate::Writable for CAULVR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAULVR to value 0"]
impl crate::Resettable for CAULVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
