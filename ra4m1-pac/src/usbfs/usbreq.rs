#[doc = "Register `USBREQ` reader"]
pub type R = crate::R<USBREQ_SPEC>;
#[doc = "Register `USBREQ` writer"]
pub type W = crate::W<USBREQ_SPEC>;
#[doc = "Field `BMREQUESTTYPE` reader - Request Type These bits store the USB request bmRequestType value."]
pub type BMREQUESTTYPE_R = crate::FieldReader;
#[doc = "Field `BMREQUESTTYPE` writer - Request Type These bits store the USB request bmRequestType value."]
pub type BMREQUESTTYPE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `BREQUEST` reader - Request These bits store the USB request bRequest value."]
pub type BREQUEST_R = crate::FieldReader;
#[doc = "Field `BREQUEST` writer - Request These bits store the USB request bRequest value."]
pub type BREQUEST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Request Type These bits store the USB request bmRequestType value."]
    #[inline(always)]
    pub fn bmrequesttype(&self) -> BMREQUESTTYPE_R {
        BMREQUESTTYPE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Request These bits store the USB request bRequest value."]
    #[inline(always)]
    pub fn brequest(&self) -> BREQUEST_R {
        BREQUEST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Request Type These bits store the USB request bmRequestType value."]
    #[inline(always)]
    #[must_use]
    pub fn bmrequesttype(&mut self) -> BMREQUESTTYPE_W<USBREQ_SPEC, 0> {
        BMREQUESTTYPE_W::new(self)
    }
    #[doc = "Bits 8:15 - Request These bits store the USB request bRequest value."]
    #[inline(always)]
    #[must_use]
    pub fn brequest(&mut self) -> BREQUEST_W<USBREQ_SPEC, 8> {
        BREQUEST_W::new(self)
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
#[doc = "USB Request Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbreq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbreq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBREQ_SPEC;
impl crate::RegisterSpec for USBREQ_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`usbreq::R`](R) reader structure"]
impl crate::Readable for USBREQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbreq::W`](W) writer structure"]
impl crate::Writable for USBREQ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBREQ to value 0"]
impl crate::Resettable for USBREQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
