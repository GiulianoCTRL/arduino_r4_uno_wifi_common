#[doc = "Register `ADCMPDR1` reader"]
pub type R = crate::R<ADCMPDR1_SPEC>;
#[doc = "Register `ADCMPDR1` writer"]
pub type W = crate::W<ADCMPDR1_SPEC>;
#[doc = "Field `ADCMPDR1` reader - The ADCMPDR1 register sets the reference data when the compare window A function is used. ADCMPDR1 sets the upper-side level of window A.."]
pub type ADCMPDR1_R = crate::FieldReader<u16>;
#[doc = "Field `ADCMPDR1` writer - The ADCMPDR1 register sets the reference data when the compare window A function is used. ADCMPDR1 sets the upper-side level of window A.."]
pub type ADCMPDR1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - The ADCMPDR1 register sets the reference data when the compare window A function is used. ADCMPDR1 sets the upper-side level of window A.."]
    #[inline(always)]
    pub fn adcmpdr1(&self) -> ADCMPDR1_R {
        ADCMPDR1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - The ADCMPDR1 register sets the reference data when the compare window A function is used. ADCMPDR1 sets the upper-side level of window A.."]
    #[inline(always)]
    #[must_use]
    pub fn adcmpdr1(&mut self) -> ADCMPDR1_W<ADCMPDR1_SPEC, 0> {
        ADCMPDR1_W::new(self)
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
#[doc = "A/D Compare Function Window A Upper-Side Level Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmpdr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmpdr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCMPDR1_SPEC;
impl crate::RegisterSpec for ADCMPDR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcmpdr1::R`](R) reader structure"]
impl crate::Readable for ADCMPDR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adcmpdr1::W`](W) writer structure"]
impl crate::Writable for ADCMPDR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPDR1 to value 0"]
impl crate::Resettable for ADCMPDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
