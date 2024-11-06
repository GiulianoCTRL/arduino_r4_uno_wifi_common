#[doc = "Register `CDR` reader"]
pub type R = crate::R<CDR_SPEC>;
#[doc = "Register `CDR` writer"]
pub type W = crate::W<CDR_SPEC>;
#[doc = "Field `CMPD` reader - Compare Match Data Compare data pattern for address match wake-up function"]
pub type CMPD_R = crate::FieldReader<u16>;
#[doc = "Field `CMPD` writer - Compare Match Data Compare data pattern for address match wake-up function"]
pub type CMPD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
impl R {
    #[doc = "Bits 0:8 - Compare Match Data Compare data pattern for address match wake-up function"]
    #[inline(always)]
    pub fn cmpd(&self) -> CMPD_R {
        CMPD_R::new(self.bits & 0x01ff)
    }
}
impl W {
    #[doc = "Bits 0:8 - Compare Match Data Compare data pattern for address match wake-up function"]
    #[inline(always)]
    #[must_use]
    pub fn cmpd(&mut self) -> CMPD_W<CDR_SPEC, 0> {
        CMPD_W::new(self)
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
#[doc = "Compare Match Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CDR_SPEC;
impl crate::RegisterSpec for CDR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cdr::R`](R) reader structure"]
impl crate::Readable for CDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cdr::W`](W) writer structure"]
impl crate::Writable for CDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CDR to value 0"]
impl crate::Resettable for CDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
