#[doc = "Register `DADR0` reader"]
pub type R = crate::R<DADR0_SPEC>;
#[doc = "Register `DADR0` writer"]
pub type W = crate::W<DADR0_SPEC>;
#[doc = "Field `DADR` reader - D/A Data Register NOTE: When DADPR.DPSEL = 0, the high-order 4 bits are fixed to 0: right justified format. When DADPR.DPSEL = 1, the low-order 4 bits are fixed to 0: left justified format."]
pub type DADR_R = crate::FieldReader<u16>;
#[doc = "Field `DADR` writer - D/A Data Register NOTE: When DADPR.DPSEL = 0, the high-order 4 bits are fixed to 0: right justified format. When DADPR.DPSEL = 1, the low-order 4 bits are fixed to 0: left justified format."]
pub type DADR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - D/A Data Register NOTE: When DADPR.DPSEL = 0, the high-order 4 bits are fixed to 0: right justified format. When DADPR.DPSEL = 1, the low-order 4 bits are fixed to 0: left justified format."]
    #[inline(always)]
    pub fn dadr(&self) -> DADR_R {
        DADR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - D/A Data Register NOTE: When DADPR.DPSEL = 0, the high-order 4 bits are fixed to 0: right justified format. When DADPR.DPSEL = 1, the low-order 4 bits are fixed to 0: left justified format."]
    #[inline(always)]
    #[must_use]
    pub fn dadr(&mut self) -> DADR_W<DADR0_SPEC, 0> {
        DADR_W::new(self)
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
#[doc = "D/A Data Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dadr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dadr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DADR0_SPEC;
impl crate::RegisterSpec for DADR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dadr0::R`](R) reader structure"]
impl crate::Readable for DADR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dadr0::W`](W) writer structure"]
impl crate::Writable for DADR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DADR0 to value 0"]
impl crate::Resettable for DADR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
