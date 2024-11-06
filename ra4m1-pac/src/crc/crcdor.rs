#[doc = "Register `CRCDOR` reader"]
pub type R = crate::R<CRCDOR_SPEC>;
#[doc = "Register `CRCDOR` writer"]
pub type W = crate::W<CRCDOR_SPEC>;
#[doc = "Field `CRCDOR` reader - Calculation output Data (Case of CRC-32, CRC-32C )"]
pub type CRCDOR_R = crate::FieldReader<u32>;
#[doc = "Field `CRCDOR` writer - Calculation output Data (Case of CRC-32, CRC-32C )"]
pub type CRCDOR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Calculation output Data (Case of CRC-32, CRC-32C )"]
    #[inline(always)]
    pub fn crcdor(&self) -> CRCDOR_R {
        CRCDOR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Calculation output Data (Case of CRC-32, CRC-32C )"]
    #[inline(always)]
    #[must_use]
    pub fn crcdor(&mut self) -> CRCDOR_W<CRCDOR_SPEC, 0> {
        CRCDOR_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CRC Data Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcdor::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcdor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRCDOR_SPEC;
impl crate::RegisterSpec for CRCDOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcdor::R`](R) reader structure"]
impl crate::Readable for CRCDOR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crcdor::W`](W) writer structure"]
impl crate::Writable for CRCDOR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCDOR to value 0"]
impl crate::Resettable for CRCDOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
