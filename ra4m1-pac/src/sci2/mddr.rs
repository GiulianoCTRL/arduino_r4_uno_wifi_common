#[doc = "Register `MDDR` reader"]
pub type R = crate::R<MDDR_SPEC>;
#[doc = "Register `MDDR` writer"]
pub type W = crate::W<MDDR_SPEC>;
#[doc = "Field `MDDR` reader - MDDR corrects the bit rate adjusted by the BRR register."]
pub type MDDR_R = crate::FieldReader;
#[doc = "Field `MDDR` writer - MDDR corrects the bit rate adjusted by the BRR register."]
pub type MDDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - MDDR corrects the bit rate adjusted by the BRR register."]
    #[inline(always)]
    pub fn mddr(&self) -> MDDR_R {
        MDDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - MDDR corrects the bit rate adjusted by the BRR register."]
    #[inline(always)]
    #[must_use]
    pub fn mddr(&mut self) -> MDDR_W<MDDR_SPEC, 0> {
        MDDR_W::new(self)
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
#[doc = "Modulation Duty Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDDR_SPEC;
impl crate::RegisterSpec for MDDR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mddr::R`](R) reader structure"]
impl crate::Readable for MDDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mddr::W`](W) writer structure"]
impl crate::Writable for MDDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDDR to value 0xff"]
impl crate::Resettable for MDDR_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
