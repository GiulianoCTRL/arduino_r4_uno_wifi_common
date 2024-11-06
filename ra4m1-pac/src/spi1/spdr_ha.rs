#[doc = "Register `SPDR_HA` reader"]
pub type R = crate::R<SPDR_HA_SPEC>;
#[doc = "Register `SPDR_HA` writer"]
pub type W = crate::W<SPDR_HA_SPEC>;
#[doc = "Field `SPDR_HA` reader - SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI. When accessing in halfword (SPDCR.SPLW=0), access SPDR_HA."]
pub type SPDR_HA_R = crate::FieldReader<u16>;
#[doc = "Field `SPDR_HA` writer - SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI. When accessing in halfword (SPDCR.SPLW=0), access SPDR_HA."]
pub type SPDR_HA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI. When accessing in halfword (SPDCR.SPLW=0), access SPDR_HA."]
    #[inline(always)]
    pub fn spdr_ha(&self) -> SPDR_HA_R {
        SPDR_HA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI. When accessing in halfword (SPDCR.SPLW=0), access SPDR_HA."]
    #[inline(always)]
    #[must_use]
    pub fn spdr_ha(&mut self) -> SPDR_HA_W<SPDR_HA_SPEC, 0> {
        SPDR_HA_W::new(self)
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
#[doc = "SPI Data Register ( halfword access )\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdr_ha::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdr_ha::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPDR_HA_SPEC;
impl crate::RegisterSpec for SPDR_HA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`spdr_ha::R`](R) reader structure"]
impl crate::Readable for SPDR_HA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spdr_ha::W`](W) writer structure"]
impl crate::Writable for SPDR_HA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPDR_HA to value 0"]
impl crate::Resettable for SPDR_HA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
