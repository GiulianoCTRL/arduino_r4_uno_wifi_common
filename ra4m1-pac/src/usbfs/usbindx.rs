#[doc = "Register `USBINDX` reader"]
pub type R = crate::R<USBINDX_SPEC>;
#[doc = "Register `USBINDX` writer"]
pub type W = crate::W<USBINDX_SPEC>;
#[doc = "Field `WINDEX` reader - Index These bits store the USB request wIndex value."]
pub type WINDEX_R = crate::FieldReader<u16>;
#[doc = "Field `WINDEX` writer - Index These bits store the USB request wIndex value."]
pub type WINDEX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Index These bits store the USB request wIndex value."]
    #[inline(always)]
    pub fn windex(&self) -> WINDEX_R {
        WINDEX_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Index These bits store the USB request wIndex value."]
    #[inline(always)]
    #[must_use]
    pub fn windex(&mut self) -> WINDEX_W<USBINDX_SPEC, 0> {
        WINDEX_W::new(self)
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
#[doc = "USB Request Index Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbindx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbindx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBINDX_SPEC;
impl crate::RegisterSpec for USBINDX_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`usbindx::R`](R) reader structure"]
impl crate::Readable for USBINDX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbindx::W`](W) writer structure"]
impl crate::Writable for USBINDX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBINDX to value 0"]
impl crate::Resettable for USBINDX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
