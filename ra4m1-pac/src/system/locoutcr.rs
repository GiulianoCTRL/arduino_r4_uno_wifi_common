#[doc = "Register `LOCOUTCR` reader"]
pub type R = crate::R<LOCOUTCR_SPEC>;
#[doc = "Register `LOCOUTCR` writer"]
pub type W = crate::W<LOCOUTCR_SPEC>;
#[doc = "Field `LOCOUTRM` reader - LOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127 These bits are added to original LOCO trimming bits"]
pub type LOCOUTRM_R = crate::FieldReader;
#[doc = "Field `LOCOUTRM` writer - LOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127 These bits are added to original LOCO trimming bits"]
pub type LOCOUTRM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - LOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127 These bits are added to original LOCO trimming bits"]
    #[inline(always)]
    pub fn locoutrm(&self) -> LOCOUTRM_R {
        LOCOUTRM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - LOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127 These bits are added to original LOCO trimming bits"]
    #[inline(always)]
    #[must_use]
    pub fn locoutrm(&mut self) -> LOCOUTRM_W<LOCOUTCR_SPEC, 0> {
        LOCOUTRM_W::new(self)
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
#[doc = "LOCO User Trimming Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`locoutcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`locoutcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOCOUTCR_SPEC;
impl crate::RegisterSpec for LOCOUTCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`locoutcr::R`](R) reader structure"]
impl crate::Readable for LOCOUTCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`locoutcr::W`](W) writer structure"]
impl crate::Writable for LOCOUTCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOCOUTCR to value 0"]
impl crate::Resettable for LOCOUTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
