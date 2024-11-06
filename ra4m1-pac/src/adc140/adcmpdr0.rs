#[doc = "Register `ADCMPDR0` reader"]
pub type R = crate::R<ADCMPDR0_SPEC>;
#[doc = "Register `ADCMPDR0` writer"]
pub type W = crate::W<ADCMPDR0_SPEC>;
#[doc = "Field `ADCMPDR0` reader - The ADCMPDR0 register sets the reference data when the compare window A function is used. ADCMPDR0 sets the lower-side level of window A."]
pub type ADCMPDR0_R = crate::FieldReader<u16>;
#[doc = "Field `ADCMPDR0` writer - The ADCMPDR0 register sets the reference data when the compare window A function is used. ADCMPDR0 sets the lower-side level of window A."]
pub type ADCMPDR0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - The ADCMPDR0 register sets the reference data when the compare window A function is used. ADCMPDR0 sets the lower-side level of window A."]
    #[inline(always)]
    pub fn adcmpdr0(&self) -> ADCMPDR0_R {
        ADCMPDR0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - The ADCMPDR0 register sets the reference data when the compare window A function is used. ADCMPDR0 sets the lower-side level of window A."]
    #[inline(always)]
    #[must_use]
    pub fn adcmpdr0(&mut self) -> ADCMPDR0_W<ADCMPDR0_SPEC, 0> {
        ADCMPDR0_W::new(self)
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
#[doc = "A/D Compare Function Window A Lower-Side Level Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmpdr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmpdr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCMPDR0_SPEC;
impl crate::RegisterSpec for ADCMPDR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcmpdr0::R`](R) reader structure"]
impl crate::Readable for ADCMPDR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adcmpdr0::W`](W) writer structure"]
impl crate::Writable for ADCMPDR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPDR0 to value 0"]
impl crate::Resettable for ADCMPDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
