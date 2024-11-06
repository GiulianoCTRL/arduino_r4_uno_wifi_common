#[doc = "Register `CRCDOR_HA` reader"]
pub type R = crate::R<CRCDOR_HA_SPEC>;
#[doc = "Register `CRCDOR_HA` writer"]
pub type W = crate::W<CRCDOR_HA_SPEC>;
#[doc = "Field `CRCDOR_HA` reader - Calculation output Data (Case of CRC-16 or CRC-CCITT )"]
pub type CRCDOR_HA_R = crate::FieldReader<u16>;
#[doc = "Field `CRCDOR_HA` writer - Calculation output Data (Case of CRC-16 or CRC-CCITT )"]
pub type CRCDOR_HA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Calculation output Data (Case of CRC-16 or CRC-CCITT )"]
    #[inline(always)]
    pub fn crcdor_ha(&self) -> CRCDOR_HA_R {
        CRCDOR_HA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Calculation output Data (Case of CRC-16 or CRC-CCITT )"]
    #[inline(always)]
    #[must_use]
    pub fn crcdor_ha(&mut self) -> CRCDOR_HA_W<CRCDOR_HA_SPEC, 0> {
        CRCDOR_HA_W::new(self)
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
#[doc = "CRC Data Output Register (halfword access)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcdor_ha::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcdor_ha::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRCDOR_HA_SPEC;
impl crate::RegisterSpec for CRCDOR_HA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crcdor_ha::R`](R) reader structure"]
impl crate::Readable for CRCDOR_HA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crcdor_ha::W`](W) writer structure"]
impl crate::Writable for CRCDOR_HA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCDOR_HA to value 0"]
impl crate::Resettable for CRCDOR_HA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
