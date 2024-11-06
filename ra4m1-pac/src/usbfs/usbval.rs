#[doc = "Register `USBVAL` reader"]
pub type R = crate::R<USBVAL_SPEC>;
#[doc = "Register `USBVAL` writer"]
pub type W = crate::W<USBVAL_SPEC>;
#[doc = "Field `WVALUE` reader - Value These bits store the USB request Value value."]
pub type WVALUE_R = crate::FieldReader<u16>;
#[doc = "Field `WVALUE` writer - Value These bits store the USB request Value value."]
pub type WVALUE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Value These bits store the USB request Value value."]
    #[inline(always)]
    pub fn wvalue(&self) -> WVALUE_R {
        WVALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Value These bits store the USB request Value value."]
    #[inline(always)]
    #[must_use]
    pub fn wvalue(&mut self) -> WVALUE_W<USBVAL_SPEC, 0> {
        WVALUE_W::new(self)
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
#[doc = "USB Request Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbval::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbval::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBVAL_SPEC;
impl crate::RegisterSpec for USBVAL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`usbval::R`](R) reader structure"]
impl crate::Readable for USBVAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbval::W`](W) writer structure"]
impl crate::Writable for USBVAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBVAL to value 0"]
impl crate::Resettable for USBVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
