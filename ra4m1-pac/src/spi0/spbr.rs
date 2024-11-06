#[doc = "Register `SPBR` reader"]
pub type R = crate::R<SPBR_SPEC>;
#[doc = "Register `SPBR` writer"]
pub type W = crate::W<SPBR_SPEC>;
#[doc = "Field `SPR` reader - SPBR sets the bit rate in master mode."]
pub type SPR_R = crate::FieldReader;
#[doc = "Field `SPR` writer - SPBR sets the bit rate in master mode."]
pub type SPR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - SPBR sets the bit rate in master mode."]
    #[inline(always)]
    pub fn spr(&self) -> SPR_R {
        SPR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - SPBR sets the bit rate in master mode."]
    #[inline(always)]
    #[must_use]
    pub fn spr(&mut self) -> SPR_W<SPBR_SPEC, 0> {
        SPR_W::new(self)
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
#[doc = "SPI Bit Rate Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spbr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spbr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPBR_SPEC;
impl crate::RegisterSpec for SPBR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spbr::R`](R) reader structure"]
impl crate::Readable for SPBR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spbr::W`](W) writer structure"]
impl crate::Writable for SPBR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPBR to value 0xff"]
impl crate::Resettable for SPBR_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
