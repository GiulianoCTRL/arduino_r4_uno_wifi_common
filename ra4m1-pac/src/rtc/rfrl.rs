#[doc = "Register `RFRL` reader"]
pub type R = crate::R<RFRL_SPEC>;
#[doc = "Register `RFRL` writer"]
pub type W = crate::W<RFRL_SPEC>;
#[doc = "Field `RFC` reader - Frequency Comparison Value(b15-b0) To generate the operating clock from the main clock, this bit sets the comparison value of the 128-Hz clock cycle."]
pub type RFC_R = crate::FieldReader<u16>;
#[doc = "Field `RFC` writer - Frequency Comparison Value(b15-b0) To generate the operating clock from the main clock, this bit sets the comparison value of the 128-Hz clock cycle."]
pub type RFC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Frequency Comparison Value(b15-b0) To generate the operating clock from the main clock, this bit sets the comparison value of the 128-Hz clock cycle."]
    #[inline(always)]
    pub fn rfc(&self) -> RFC_R {
        RFC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Frequency Comparison Value(b15-b0) To generate the operating clock from the main clock, this bit sets the comparison value of the 128-Hz clock cycle."]
    #[inline(always)]
    #[must_use]
    pub fn rfc(&mut self) -> RFC_W<RFRL_SPEC, 0> {
        RFC_W::new(self)
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
#[doc = "Frequency Register L\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFRL_SPEC;
impl crate::RegisterSpec for RFRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rfrl::R`](R) reader structure"]
impl crate::Readable for RFRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rfrl::W`](W) writer structure"]
impl crate::Writable for RFRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFRL to value 0"]
impl crate::Resettable for RFRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
