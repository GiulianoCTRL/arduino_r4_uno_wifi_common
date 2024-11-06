#[doc = "Register `DODSR` reader"]
pub type R = crate::R<DODSR_SPEC>;
#[doc = "Register `DODSR` writer"]
pub type W = crate::W<DODSR_SPEC>;
#[doc = "Field `DODSR` reader - This register stores 16-bit data for use as a reference in data comparison mode. This register also stores the results of operations in data addition and data subtraction modes."]
pub type DODSR_R = crate::FieldReader<u16>;
#[doc = "Field `DODSR` writer - This register stores 16-bit data for use as a reference in data comparison mode. This register also stores the results of operations in data addition and data subtraction modes."]
pub type DODSR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - This register stores 16-bit data for use as a reference in data comparison mode. This register also stores the results of operations in data addition and data subtraction modes."]
    #[inline(always)]
    pub fn dodsr(&self) -> DODSR_R {
        DODSR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register stores 16-bit data for use as a reference in data comparison mode. This register also stores the results of operations in data addition and data subtraction modes."]
    #[inline(always)]
    #[must_use]
    pub fn dodsr(&mut self) -> DODSR_W<DODSR_SPEC, 0> {
        DODSR_W::new(self)
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
#[doc = "DOC Data Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dodsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dodsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DODSR_SPEC;
impl crate::RegisterSpec for DODSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dodsr::R`](R) reader structure"]
impl crate::Readable for DODSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dodsr::W`](W) writer structure"]
impl crate::Writable for DODSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DODSR to value 0"]
impl crate::Resettable for DODSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
