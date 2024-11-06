#[doc = "Register `DACS%s` reader"]
pub type R = crate::R<DACS_SPEC>;
#[doc = "Register `DACS%s` writer"]
pub type W = crate::W<DACS_SPEC>;
#[doc = "Field `DACS` reader - DACS D/A conversion store data note: When 8-bit D/A Converter output is selected as the reference input for the ACMPLP in the COMPSEL1 register, and ACMPLP operation is enabled (COMPMDR.CnENB = 1), changing the DACS\\[7:0\\]
bits for the channel in use is prohibited."]
pub type DACS_R = crate::FieldReader;
#[doc = "Field `DACS` writer - DACS D/A conversion store data note: When 8-bit D/A Converter output is selected as the reference input for the ACMPLP in the COMPSEL1 register, and ACMPLP operation is enabled (COMPMDR.CnENB = 1), changing the DACS\\[7:0\\]
bits for the channel in use is prohibited."]
pub type DACS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DACS D/A conversion store data note: When 8-bit D/A Converter output is selected as the reference input for the ACMPLP in the COMPSEL1 register, and ACMPLP operation is enabled (COMPMDR.CnENB = 1), changing the DACS\\[7:0\\]
bits for the channel in use is prohibited."]
    #[inline(always)]
    pub fn dacs(&self) -> DACS_R {
        DACS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - DACS D/A conversion store data note: When 8-bit D/A Converter output is selected as the reference input for the ACMPLP in the COMPSEL1 register, and ACMPLP operation is enabled (COMPMDR.CnENB = 1), changing the DACS\\[7:0\\]
bits for the channel in use is prohibited."]
    #[inline(always)]
    #[must_use]
    pub fn dacs(&mut self) -> DACS_W<DACS_SPEC, 0> {
        DACS_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "D/A Conversion Value Setting Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DACS_SPEC;
impl crate::RegisterSpec for DACS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dacs::R`](R) reader structure"]
impl crate::Readable for DACS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dacs::W`](W) writer structure"]
impl crate::Writable for DACS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DACS%s to value 0"]
impl crate::Resettable for DACS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
