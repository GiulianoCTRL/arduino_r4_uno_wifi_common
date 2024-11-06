#[doc = "Register `ADWINLLB` reader"]
pub type R = crate::R<ADWINLLB_SPEC>;
#[doc = "Register `ADWINLLB` writer"]
pub type W = crate::W<ADWINLLB_SPEC>;
#[doc = "Field `ADWINLLB` reader - This register is used to compare A window function is used to set the lower level of the window B."]
pub type ADWINLLB_R = crate::FieldReader<u16>;
#[doc = "Field `ADWINLLB` writer - This register is used to compare A window function is used to set the lower level of the window B."]
pub type ADWINLLB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is used to compare A window function is used to set the lower level of the window B."]
    #[inline(always)]
    pub fn adwinllb(&self) -> ADWINLLB_R {
        ADWINLLB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is used to compare A window function is used to set the lower level of the window B."]
    #[inline(always)]
    #[must_use]
    pub fn adwinllb(&mut self) -> ADWINLLB_W<ADWINLLB_SPEC, 0> {
        ADWINLLB_W::new(self)
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
#[doc = "A/D Compare Function Window B Lower-Side Level Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adwinllb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adwinllb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADWINLLB_SPEC;
impl crate::RegisterSpec for ADWINLLB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adwinllb::R`](R) reader structure"]
impl crate::Readable for ADWINLLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adwinllb::W`](W) writer structure"]
impl crate::Writable for ADWINLLB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADWINLLB to value 0"]
impl crate::Resettable for ADWINLLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
