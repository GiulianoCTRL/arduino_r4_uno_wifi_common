#[doc = "Register `ADSTRGR` reader"]
pub type R = crate::R<ADSTRGR_SPEC>;
#[doc = "Register `ADSTRGR` writer"]
pub type W = crate::W<ADSTRGR_SPEC>;
#[doc = "Field `TRSB` reader - A/D Conversion Start Trigger Select for Group B Select the A/D conversion start trigger for group B in group scan mode."]
pub type TRSB_R = crate::FieldReader;
#[doc = "Field `TRSB` writer - A/D Conversion Start Trigger Select for Group B Select the A/D conversion start trigger for group B in group scan mode."]
pub type TRSB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `TRSA` reader - A/D Conversion Start Trigger Select Select the A/D conversion start trigger in single scan mode and continuous mode. In group scan mode, the A/D conversion start trigger for group A is selected."]
pub type TRSA_R = crate::FieldReader;
#[doc = "Field `TRSA` writer - A/D Conversion Start Trigger Select Select the A/D conversion start trigger in single scan mode and continuous mode. In group scan mode, the A/D conversion start trigger for group A is selected."]
pub type TRSA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5 - A/D Conversion Start Trigger Select for Group B Select the A/D conversion start trigger for group B in group scan mode."]
    #[inline(always)]
    pub fn trsb(&self) -> TRSB_R {
        TRSB_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - A/D Conversion Start Trigger Select Select the A/D conversion start trigger in single scan mode and continuous mode. In group scan mode, the A/D conversion start trigger for group A is selected."]
    #[inline(always)]
    pub fn trsa(&self) -> TRSA_R {
        TRSA_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - A/D Conversion Start Trigger Select for Group B Select the A/D conversion start trigger for group B in group scan mode."]
    #[inline(always)]
    #[must_use]
    pub fn trsb(&mut self) -> TRSB_W<ADSTRGR_SPEC, 0> {
        TRSB_W::new(self)
    }
    #[doc = "Bits 8:13 - A/D Conversion Start Trigger Select Select the A/D conversion start trigger in single scan mode and continuous mode. In group scan mode, the A/D conversion start trigger for group A is selected."]
    #[inline(always)]
    #[must_use]
    pub fn trsa(&mut self) -> TRSA_W<ADSTRGR_SPEC, 8> {
        TRSA_W::new(self)
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
#[doc = "A/D Conversion Start Trigger Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adstrgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adstrgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADSTRGR_SPEC;
impl crate::RegisterSpec for ADSTRGR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adstrgr::R`](R) reader structure"]
impl crate::Readable for ADSTRGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adstrgr::W`](W) writer structure"]
impl crate::Writable for ADSTRGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSTRGR to value 0"]
impl crate::Resettable for ADSTRGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
