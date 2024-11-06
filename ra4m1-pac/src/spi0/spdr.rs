#[doc = "Register `SPDR` reader"]
pub type R = crate::R<SPDR_SPEC>;
#[doc = "Register `SPDR` writer"]
pub type W = crate::W<SPDR_SPEC>;
#[doc = "Field `SPDR` reader - SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI. When accessing in word (SPDCR.SPLW=1), access SPDR."]
pub type SPDR_R = crate::FieldReader<u32>;
#[doc = "Field `SPDR` writer - SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI. When accessing in word (SPDCR.SPLW=1), access SPDR."]
pub type SPDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI. When accessing in word (SPDCR.SPLW=1), access SPDR."]
    #[inline(always)]
    pub fn spdr(&self) -> SPDR_R {
        SPDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI. When accessing in word (SPDCR.SPLW=1), access SPDR."]
    #[inline(always)]
    #[must_use]
    pub fn spdr(&mut self) -> SPDR_W<SPDR_SPEC, 0> {
        SPDR_W::new(self)
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
#[doc = "SPI Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPDR_SPEC;
impl crate::RegisterSpec for SPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdr::R`](R) reader structure"]
impl crate::Readable for SPDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spdr::W`](W) writer structure"]
impl crate::Writable for SPDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPDR to value 0"]
impl crate::Resettable for SPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
