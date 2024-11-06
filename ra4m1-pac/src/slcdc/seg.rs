#[doc = "Register `SEG%s` reader"]
pub type R = crate::R<SEG_SPEC>;
#[doc = "Register `SEG%s` writer"]
pub type W = crate::W<SEG_SPEC>;
#[doc = "Field `SEG` reader - LCD Display Data"]
pub type SEG_R = crate::FieldReader;
#[doc = "Field `SEG` writer - LCD Display Data"]
pub type SEG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - LCD Display Data"]
    #[inline(always)]
    pub fn seg(&self) -> SEG_R {
        SEG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - LCD Display Data"]
    #[inline(always)]
    #[must_use]
    pub fn seg(&mut self) -> SEG_W<SEG_SPEC, 0> {
        SEG_W::new(self)
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
#[doc = "LCD Display Data Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEG_SPEC;
impl crate::RegisterSpec for SEG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`seg::R`](R) reader structure"]
impl crate::Readable for SEG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`seg::W`](W) writer structure"]
impl crate::Writable for SEG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEG%s to value 0"]
impl crate::Resettable for SEG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
