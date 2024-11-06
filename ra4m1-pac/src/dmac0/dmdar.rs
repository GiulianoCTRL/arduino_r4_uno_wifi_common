#[doc = "Register `DMDAR` reader"]
pub type R = crate::R<DMDAR_SPEC>;
#[doc = "Register `DMDAR` writer"]
pub type W = crate::W<DMDAR_SPEC>;
#[doc = "Field `DMDAR` reader - Specifies the transfer destination start address."]
pub type DMDAR_R = crate::FieldReader<u32>;
#[doc = "Field `DMDAR` writer - Specifies the transfer destination start address."]
pub type DMDAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies the transfer destination start address."]
    #[inline(always)]
    pub fn dmdar(&self) -> DMDAR_R {
        DMDAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies the transfer destination start address."]
    #[inline(always)]
    #[must_use]
    pub fn dmdar(&mut self) -> DMDAR_W<DMDAR_SPEC, 0> {
        DMDAR_W::new(self)
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
#[doc = "DMA Destination Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmdar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmdar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMDAR_SPEC;
impl crate::RegisterSpec for DMDAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmdar::R`](R) reader structure"]
impl crate::Readable for DMDAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmdar::W`](W) writer structure"]
impl crate::Writable for DMDAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMDAR to value 0"]
impl crate::Resettable for DMDAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
