#[doc = "Register `DMSAR` reader"]
pub type R = crate::R<DMSAR_SPEC>;
#[doc = "Register `DMSAR` writer"]
pub type W = crate::W<DMSAR_SPEC>;
#[doc = "Field `DMSAR` reader - Specifies the transfer source start address."]
pub type DMSAR_R = crate::FieldReader<u32>;
#[doc = "Field `DMSAR` writer - Specifies the transfer source start address."]
pub type DMSAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies the transfer source start address."]
    #[inline(always)]
    pub fn dmsar(&self) -> DMSAR_R {
        DMSAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies the transfer source start address."]
    #[inline(always)]
    #[must_use]
    pub fn dmsar(&mut self) -> DMSAR_W<DMSAR_SPEC, 0> {
        DMSAR_W::new(self)
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
#[doc = "DMA Source Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmsar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmsar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMSAR_SPEC;
impl crate::RegisterSpec for DMSAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmsar::R`](R) reader structure"]
impl crate::Readable for DMSAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmsar::W`](W) writer structure"]
impl crate::Writable for DMSAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMSAR to value 0"]
impl crate::Resettable for DMSAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
