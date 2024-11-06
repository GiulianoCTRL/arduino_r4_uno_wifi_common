#[doc = "Register `CTSUDCLKC` reader"]
pub type R = crate::R<CTSUDCLKC_SPEC>;
#[doc = "Register `CTSUDCLKC` writer"]
pub type W = crate::W<CTSUDCLKC_SPEC>;
#[doc = "Field `CTSUSSMOD` reader - CTSU Diffusion Clock Mode Select NOTE: This bit should be set to 00b."]
pub type CTSUSSMOD_R = crate::FieldReader;
#[doc = "Field `CTSUSSMOD` writer - CTSU Diffusion Clock Mode Select NOTE: This bit should be set to 00b."]
pub type CTSUSSMOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CTSUSSCNT` reader - CTSU Diffusion Clock Mode Control NOTE: This bit should be set to 11b."]
pub type CTSUSSCNT_R = crate::FieldReader;
#[doc = "Field `CTSUSSCNT` writer - CTSU Diffusion Clock Mode Control NOTE: This bit should be set to 11b."]
pub type CTSUSSCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - CTSU Diffusion Clock Mode Select NOTE: This bit should be set to 00b."]
    #[inline(always)]
    pub fn ctsussmod(&self) -> CTSUSSMOD_R {
        CTSUSSMOD_R::new(self.bits & 3)
    }
    #[doc = "Bits 4:5 - CTSU Diffusion Clock Mode Control NOTE: This bit should be set to 11b."]
    #[inline(always)]
    pub fn ctsusscnt(&self) -> CTSUSSCNT_R {
        CTSUSSCNT_R::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - CTSU Diffusion Clock Mode Select NOTE: This bit should be set to 00b."]
    #[inline(always)]
    #[must_use]
    pub fn ctsussmod(&mut self) -> CTSUSSMOD_W<CTSUDCLKC_SPEC, 0> {
        CTSUSSMOD_W::new(self)
    }
    #[doc = "Bits 4:5 - CTSU Diffusion Clock Mode Control NOTE: This bit should be set to 11b."]
    #[inline(always)]
    #[must_use]
    pub fn ctsusscnt(&mut self) -> CTSUSSCNT_W<CTSUDCLKC_SPEC, 4> {
        CTSUSSCNT_W::new(self)
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
#[doc = "CTSU High-Pass Noise Reduction Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsudclkc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsudclkc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTSUDCLKC_SPEC;
impl crate::RegisterSpec for CTSUDCLKC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctsudclkc::R`](R) reader structure"]
impl crate::Readable for CTSUDCLKC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctsudclkc::W`](W) writer structure"]
impl crate::Writable for CTSUDCLKC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUDCLKC to value 0"]
impl crate::Resettable for CTSUDCLKC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
