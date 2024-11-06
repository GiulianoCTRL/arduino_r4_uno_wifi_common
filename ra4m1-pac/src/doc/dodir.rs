#[doc = "Register `DODIR` reader"]
pub type R = crate::R<DODIR_SPEC>;
#[doc = "Register `DODIR` writer"]
pub type W = crate::W<DODIR_SPEC>;
#[doc = "Field `DODIR` reader - 16-bit read-write register in which 16-bit data for use in the operations are stored."]
pub type DODIR_R = crate::FieldReader<u16>;
#[doc = "Field `DODIR` writer - 16-bit read-write register in which 16-bit data for use in the operations are stored."]
pub type DODIR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - 16-bit read-write register in which 16-bit data for use in the operations are stored."]
    #[inline(always)]
    pub fn dodir(&self) -> DODIR_R {
        DODIR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 16-bit read-write register in which 16-bit data for use in the operations are stored."]
    #[inline(always)]
    #[must_use]
    pub fn dodir(&mut self) -> DODIR_W<DODIR_SPEC, 0> {
        DODIR_W::new(self)
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
#[doc = "DOC Data Input Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dodir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dodir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DODIR_SPEC;
impl crate::RegisterSpec for DODIR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dodir::R`](R) reader structure"]
impl crate::Readable for DODIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dodir::W`](W) writer structure"]
impl crate::Writable for DODIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DODIR to value 0"]
impl crate::Resettable for DODIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
