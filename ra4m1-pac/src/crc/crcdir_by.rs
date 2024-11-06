#[doc = "Register `CRCDIR_BY` reader"]
pub type R = crate::R<CRCDIR_BY_SPEC>;
#[doc = "Register `CRCDIR_BY` writer"]
pub type W = crate::W<CRCDIR_BY_SPEC>;
#[doc = "Field `CRCDIR_BY` reader - Calculation input Data ( Case of CRC-8, CRC-16 or CRC-CCITT )"]
pub type CRCDIR_BY_R = crate::FieldReader;
#[doc = "Field `CRCDIR_BY` writer - Calculation input Data ( Case of CRC-8, CRC-16 or CRC-CCITT )"]
pub type CRCDIR_BY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Calculation input Data ( Case of CRC-8, CRC-16 or CRC-CCITT )"]
    #[inline(always)]
    pub fn crcdir_by(&self) -> CRCDIR_BY_R {
        CRCDIR_BY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Calculation input Data ( Case of CRC-8, CRC-16 or CRC-CCITT )"]
    #[inline(always)]
    #[must_use]
    pub fn crcdir_by(&mut self) -> CRCDIR_BY_W<CRCDIR_BY_SPEC, 0> {
        CRCDIR_BY_W::new(self)
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
#[doc = "CRC Data Input Register (byte access)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcdir_by::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcdir_by::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRCDIR_BY_SPEC;
impl crate::RegisterSpec for CRCDIR_BY_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`crcdir_by::R`](R) reader structure"]
impl crate::Readable for CRCDIR_BY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crcdir_by::W`](W) writer structure"]
impl crate::Writable for CRCDIR_BY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCDIR_BY to value 0"]
impl crate::Resettable for CRCDIR_BY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
