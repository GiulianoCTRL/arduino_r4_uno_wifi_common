#[doc = "Register `MOCOUTCR` reader"]
pub type R = crate::R<MOCOUTCR_SPEC>;
#[doc = "Register `MOCOUTCR` writer"]
pub type W = crate::W<MOCOUTCR_SPEC>;
#[doc = "Field `MOCOUTRM` reader - MOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127 These bits are added to original MOCO trimming bits"]
pub type MOCOUTRM_R = crate::FieldReader;
#[doc = "Field `MOCOUTRM` writer - MOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127 These bits are added to original MOCO trimming bits"]
pub type MOCOUTRM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - MOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127 These bits are added to original MOCO trimming bits"]
    #[inline(always)]
    pub fn mocoutrm(&self) -> MOCOUTRM_R {
        MOCOUTRM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - MOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127 These bits are added to original MOCO trimming bits"]
    #[inline(always)]
    #[must_use]
    pub fn mocoutrm(&mut self) -> MOCOUTRM_W<MOCOUTCR_SPEC, 0> {
        MOCOUTRM_W::new(self)
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
#[doc = "MOCO User Trimming Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mocoutcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mocoutcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MOCOUTCR_SPEC;
impl crate::RegisterSpec for MOCOUTCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mocoutcr::R`](R) reader structure"]
impl crate::Readable for MOCOUTCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mocoutcr::W`](W) writer structure"]
impl crate::Writable for MOCOUTCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MOCOUTCR to value 0"]
impl crate::Resettable for MOCOUTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
