#[doc = "Register `VBTBKR[%s]` reader"]
pub type R = crate::R<VBTBKR_SPEC>;
#[doc = "Register `VBTBKR[%s]` writer"]
pub type W = crate::W<VBTBKR_SPEC>;
#[doc = "Field `VBTBKR` reader - VBTBKR is a 512-byte readable/writable register to store data powered by VBATT. The value of this register is retained even when VCC is not powered but VBATT is powered. VBTBKR is initialized by VBATT selected voltage power-on-reset."]
pub type VBTBKR_R = crate::FieldReader;
#[doc = "Field `VBTBKR` writer - VBTBKR is a 512-byte readable/writable register to store data powered by VBATT. The value of this register is retained even when VCC is not powered but VBATT is powered. VBTBKR is initialized by VBATT selected voltage power-on-reset."]
pub type VBTBKR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - VBTBKR is a 512-byte readable/writable register to store data powered by VBATT. The value of this register is retained even when VCC is not powered but VBATT is powered. VBTBKR is initialized by VBATT selected voltage power-on-reset."]
    #[inline(always)]
    pub fn vbtbkr(&self) -> VBTBKR_R {
        VBTBKR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - VBTBKR is a 512-byte readable/writable register to store data powered by VBATT. The value of this register is retained even when VCC is not powered but VBATT is powered. VBTBKR is initialized by VBATT selected voltage power-on-reset."]
    #[inline(always)]
    #[must_use]
    pub fn vbtbkr(&mut self) -> VBTBKR_W<VBTBKR_SPEC, 0> {
        VBTBKR_W::new(self)
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
#[doc = "VBATT Backup Register \\[%s\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbtbkr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbtbkr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VBTBKR_SPEC;
impl crate::RegisterSpec for VBTBKR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vbtbkr::R`](R) reader structure"]
impl crate::Readable for VBTBKR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vbtbkr::W`](W) writer structure"]
impl crate::Writable for VBTBKR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VBTBKR[%s]
to value 0"]
impl crate::Resettable for VBTBKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
