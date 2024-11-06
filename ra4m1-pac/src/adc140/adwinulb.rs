#[doc = "Register `ADWINULB` reader"]
pub type R = crate::R<ADWINULB_SPEC>;
#[doc = "Register `ADWINULB` writer"]
pub type W = crate::W<ADWINULB_SPEC>;
#[doc = "Field `ADWINULB` reader - This register is used to compare A window function is used to set the higher level of the window B."]
pub type ADWINULB_R = crate::FieldReader<u16>;
#[doc = "Field `ADWINULB` writer - This register is used to compare A window function is used to set the higher level of the window B."]
pub type ADWINULB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is used to compare A window function is used to set the higher level of the window B."]
    #[inline(always)]
    pub fn adwinulb(&self) -> ADWINULB_R {
        ADWINULB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is used to compare A window function is used to set the higher level of the window B."]
    #[inline(always)]
    #[must_use]
    pub fn adwinulb(&mut self) -> ADWINULB_W<ADWINULB_SPEC, 0> {
        ADWINULB_W::new(self)
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
#[doc = "A/D Compare Function Window B Upper-Side Level Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adwinulb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adwinulb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADWINULB_SPEC;
impl crate::RegisterSpec for ADWINULB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adwinulb::R`](R) reader structure"]
impl crate::Readable for ADWINULB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adwinulb::W`](W) writer structure"]
impl crate::Writable for ADWINULB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADWINULB to value 0"]
impl crate::Resettable for ADWINULB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
