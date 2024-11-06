#[doc = "Register `CTSUSST` reader"]
pub type R = crate::R<CTSUSST_SPEC>;
#[doc = "Register `CTSUSST` writer"]
pub type W = crate::W<CTSUSST_SPEC>;
#[doc = "Field `CTSUSST` reader - CTSU Sensor Stabilization Wait Control NOTE: The value of these bits should be fixed to 00010000b."]
pub type CTSUSST_R = crate::FieldReader;
#[doc = "Field `CTSUSST` writer - CTSU Sensor Stabilization Wait Control NOTE: The value of these bits should be fixed to 00010000b."]
pub type CTSUSST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - CTSU Sensor Stabilization Wait Control NOTE: The value of these bits should be fixed to 00010000b."]
    #[inline(always)]
    pub fn ctsusst(&self) -> CTSUSST_R {
        CTSUSST_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - CTSU Sensor Stabilization Wait Control NOTE: The value of these bits should be fixed to 00010000b."]
    #[inline(always)]
    #[must_use]
    pub fn ctsusst(&mut self) -> CTSUSST_W<CTSUSST_SPEC, 0> {
        CTSUSST_W::new(self)
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
#[doc = "CTSU Sensor Stabilization Wait Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsusst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsusst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTSUSST_SPEC;
impl crate::RegisterSpec for CTSUSST_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctsusst::R`](R) reader structure"]
impl crate::Readable for CTSUSST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctsusst::W`](W) writer structure"]
impl crate::Writable for CTSUSST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUSST to value 0"]
impl crate::Resettable for CTSUSST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
