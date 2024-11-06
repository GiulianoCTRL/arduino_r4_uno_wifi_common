#[doc = "Register `HOCOUTCR` reader"]
pub type R = crate::R<HOCOUTCR_SPEC>;
#[doc = "Register `HOCOUTCR` writer"]
pub type W = crate::W<HOCOUTCR_SPEC>;
#[doc = "Field `HOCOUTRM` reader - HOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127 These bits are added to original HOCO trimming bits"]
pub type HOCOUTRM_R = crate::FieldReader;
#[doc = "Field `HOCOUTRM` writer - HOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127 These bits are added to original HOCO trimming bits"]
pub type HOCOUTRM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - HOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127 These bits are added to original HOCO trimming bits"]
    #[inline(always)]
    pub fn hocoutrm(&self) -> HOCOUTRM_R {
        HOCOUTRM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - HOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127 These bits are added to original HOCO trimming bits"]
    #[inline(always)]
    #[must_use]
    pub fn hocoutrm(&mut self) -> HOCOUTRM_W<HOCOUTCR_SPEC, 0> {
        HOCOUTRM_W::new(self)
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
#[doc = "HOCO User Trimming Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hocoutcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hocoutcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOCOUTCR_SPEC;
impl crate::RegisterSpec for HOCOUTCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hocoutcr::R`](R) reader structure"]
impl crate::Readable for HOCOUTCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hocoutcr::W`](W) writer structure"]
impl crate::Writable for HOCOUTCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOCOUTCR to value 0"]
impl crate::Resettable for HOCOUTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
