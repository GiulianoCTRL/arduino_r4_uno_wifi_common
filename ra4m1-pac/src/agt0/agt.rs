#[doc = "Register `AGT` reader"]
pub type R = crate::R<AGT_SPEC>;
#[doc = "Register `AGT` writer"]
pub type W = crate::W<AGT_SPEC>;
#[doc = "Field `AGT` reader - 16bit counter and reload register NOTE : When 1 is written to the TSTOP bit in the AGTCRn register, the 16-bit counter is forcibly stopped and set to FFFFH."]
pub type AGT_R = crate::FieldReader<u16>;
#[doc = "Field `AGT` writer - 16bit counter and reload register NOTE : When 1 is written to the TSTOP bit in the AGTCRn register, the 16-bit counter is forcibly stopped and set to FFFFH."]
pub type AGT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - 16bit counter and reload register NOTE : When 1 is written to the TSTOP bit in the AGTCRn register, the 16-bit counter is forcibly stopped and set to FFFFH."]
    #[inline(always)]
    pub fn agt(&self) -> AGT_R {
        AGT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 16bit counter and reload register NOTE : When 1 is written to the TSTOP bit in the AGTCRn register, the 16-bit counter is forcibly stopped and set to FFFFH."]
    #[inline(always)]
    #[must_use]
    pub fn agt(&mut self) -> AGT_W<AGT_SPEC, 0> {
        AGT_W::new(self)
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
#[doc = "AGT Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`agt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`agt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AGT_SPEC;
impl crate::RegisterSpec for AGT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`agt::R`](R) reader structure"]
impl crate::Readable for AGT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`agt::W`](W) writer structure"]
impl crate::Writable for AGT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AGT to value 0xffff"]
impl crate::Resettable for AGT_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
