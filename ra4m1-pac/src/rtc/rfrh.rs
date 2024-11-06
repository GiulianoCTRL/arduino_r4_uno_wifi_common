#[doc = "Register `RFRH` reader"]
pub type R = crate::R<RFRH_SPEC>;
#[doc = "Register `RFRH` writer"]
pub type W = crate::W<RFRH_SPEC>;
#[doc = "Field `RFC16` reader - Frequency Comparison Value (b16) To generate the operating clock from the LOCOclock, this bit sets the comparison value of the 128-Hz clock cycle."]
pub type RFC16_R = crate::BitReader;
#[doc = "Field `RFC16` writer - Frequency Comparison Value (b16) To generate the operating clock from the LOCOclock, this bit sets the comparison value of the 128-Hz clock cycle."]
pub type RFC16_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Frequency Comparison Value (b16) To generate the operating clock from the LOCOclock, this bit sets the comparison value of the 128-Hz clock cycle."]
    #[inline(always)]
    pub fn rfc16(&self) -> RFC16_R {
        RFC16_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frequency Comparison Value (b16) To generate the operating clock from the LOCOclock, this bit sets the comparison value of the 128-Hz clock cycle."]
    #[inline(always)]
    #[must_use]
    pub fn rfc16(&mut self) -> RFC16_W<RFRH_SPEC, 0> {
        RFC16_W::new(self)
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
#[doc = "Frequency Register H\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFRH_SPEC;
impl crate::RegisterSpec for RFRH_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rfrh::R`](R) reader structure"]
impl crate::Readable for RFRH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rfrh::W`](W) writer structure"]
impl crate::Writable for RFRH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFRH to value 0"]
impl crate::Resettable for RFRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
