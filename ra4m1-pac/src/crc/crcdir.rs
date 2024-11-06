#[doc = "Register `CRCDIR` reader"]
pub type R = crate::R<CRCDIR_SPEC>;
#[doc = "Register `CRCDIR` writer"]
pub type W = crate::W<CRCDIR_SPEC>;
#[doc = "Field `CRCDIR` reader - Calculation input Data (Case of CRC-32, CRC-32C )"]
pub type CRCDIR_R = crate::FieldReader<u32>;
#[doc = "Field `CRCDIR` writer - Calculation input Data (Case of CRC-32, CRC-32C )"]
pub type CRCDIR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Calculation input Data (Case of CRC-32, CRC-32C )"]
    #[inline(always)]
    pub fn crcdir(&self) -> CRCDIR_R {
        CRCDIR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Calculation input Data (Case of CRC-32, CRC-32C )"]
    #[inline(always)]
    #[must_use]
    pub fn crcdir(&mut self) -> CRCDIR_W<CRCDIR_SPEC, 0> {
        CRCDIR_W::new(self)
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
#[doc = "CRC Data Input Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcdir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcdir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRCDIR_SPEC;
impl crate::RegisterSpec for CRCDIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcdir::R`](R) reader structure"]
impl crate::Readable for CRCDIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crcdir::W`](W) writer structure"]
impl crate::Writable for CRCDIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCDIR to value 0"]
impl crate::Resettable for CRCDIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
