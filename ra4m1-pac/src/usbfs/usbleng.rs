#[doc = "Register `USBLENG` reader"]
pub type R = crate::R<USBLENG_SPEC>;
#[doc = "Register `USBLENG` writer"]
pub type W = crate::W<USBLENG_SPEC>;
#[doc = "Field `WLENTUH` reader - Length These bits store the USB request wLength value."]
pub type WLENTUH_R = crate::FieldReader<u16>;
#[doc = "Field `WLENTUH` writer - Length These bits store the USB request wLength value."]
pub type WLENTUH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Length These bits store the USB request wLength value."]
    #[inline(always)]
    pub fn wlentuh(&self) -> WLENTUH_R {
        WLENTUH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Length These bits store the USB request wLength value."]
    #[inline(always)]
    #[must_use]
    pub fn wlentuh(&mut self) -> WLENTUH_W<USBLENG_SPEC, 0> {
        WLENTUH_W::new(self)
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
#[doc = "USB Request Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbleng::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbleng::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBLENG_SPEC;
impl crate::RegisterSpec for USBLENG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`usbleng::R`](R) reader structure"]
impl crate::Readable for USBLENG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbleng::W`](W) writer structure"]
impl crate::Writable for USBLENG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBLENG to value 0"]
impl crate::Resettable for USBLENG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
