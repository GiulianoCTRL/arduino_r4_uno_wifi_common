#[doc = "Register `ICDRT` reader"]
pub type R = crate::R<ICDRT_SPEC>;
#[doc = "Register `ICDRT` writer"]
pub type W = crate::W<ICDRT_SPEC>;
#[doc = "Field `ICDRT` reader - 8-bit read-write register that stores transmit data."]
pub type ICDRT_R = crate::FieldReader;
#[doc = "Field `ICDRT` writer - 8-bit read-write register that stores transmit data."]
pub type ICDRT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 8-bit read-write register that stores transmit data."]
    #[inline(always)]
    pub fn icdrt(&self) -> ICDRT_R {
        ICDRT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit read-write register that stores transmit data."]
    #[inline(always)]
    #[must_use]
    pub fn icdrt(&mut self) -> ICDRT_W<ICDRT_SPEC, 0> {
        ICDRT_W::new(self)
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
#[doc = "I2C Bus Transmit Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icdrt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icdrt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICDRT_SPEC;
impl crate::RegisterSpec for ICDRT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`icdrt::R`](R) reader structure"]
impl crate::Readable for ICDRT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icdrt::W`](W) writer structure"]
impl crate::Writable for ICDRT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICDRT to value 0xff"]
impl crate::Resettable for ICDRT_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
