#[doc = "Register `DTCVBR` reader"]
pub type R = crate::R<DTCVBR_SPEC>;
#[doc = "Register `DTCVBR` writer"]
pub type W = crate::W<DTCVBR_SPEC>;
#[doc = "Field `DTCVBR` reader - DTC Vector Base Address. Note: A value cannot be set in the lower-order 10 bits. These bits are fixed to 0."]
pub type DTCVBR_R = crate::FieldReader<u32>;
#[doc = "Field `DTCVBR` writer - DTC Vector Base Address. Note: A value cannot be set in the lower-order 10 bits. These bits are fixed to 0."]
pub type DTCVBR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - DTC Vector Base Address. Note: A value cannot be set in the lower-order 10 bits. These bits are fixed to 0."]
    #[inline(always)]
    pub fn dtcvbr(&self) -> DTCVBR_R {
        DTCVBR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DTC Vector Base Address. Note: A value cannot be set in the lower-order 10 bits. These bits are fixed to 0."]
    #[inline(always)]
    #[must_use]
    pub fn dtcvbr(&mut self) -> DTCVBR_W<DTCVBR_SPEC, 0> {
        DTCVBR_W::new(self)
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
#[doc = "DTC Vector Base Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtcvbr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtcvbr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTCVBR_SPEC;
impl crate::RegisterSpec for DTCVBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtcvbr::R`](R) reader structure"]
impl crate::Readable for DTCVBR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dtcvbr::W`](W) writer structure"]
impl crate::Writable for DTCVBR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTCVBR to value 0"]
impl crate::Resettable for DTCVBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
