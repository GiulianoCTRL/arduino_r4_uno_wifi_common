#[doc = "Register `CRCDOR_BY` reader"]
pub type R = crate::R<CRCDOR_BY_SPEC>;
#[doc = "Register `CRCDOR_BY` writer"]
pub type W = crate::W<CRCDOR_BY_SPEC>;
#[doc = "Field `CRCDOR_BY` reader - Calculation output Data (Case of CRC-8 )"]
pub type CRCDOR_BY_R = crate::FieldReader;
#[doc = "Field `CRCDOR_BY` writer - Calculation output Data (Case of CRC-8 )"]
pub type CRCDOR_BY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Calculation output Data (Case of CRC-8 )"]
    #[inline(always)]
    pub fn crcdor_by(&self) -> CRCDOR_BY_R {
        CRCDOR_BY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Calculation output Data (Case of CRC-8 )"]
    #[inline(always)]
    #[must_use]
    pub fn crcdor_by(&mut self) -> CRCDOR_BY_W<CRCDOR_BY_SPEC, 0> {
        CRCDOR_BY_W::new(self)
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
#[doc = "CRC Data Output Register(byte access)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcdor_by::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcdor_by::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRCDOR_BY_SPEC;
impl crate::RegisterSpec for CRCDOR_BY_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`crcdor_by::R`](R) reader structure"]
impl crate::Readable for CRCDOR_BY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crcdor_by::W`](W) writer structure"]
impl crate::Writable for CRCDOR_BY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCDOR_BY to value 0"]
impl crate::Resettable for CRCDOR_BY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
