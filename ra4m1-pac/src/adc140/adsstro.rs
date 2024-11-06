#[doc = "Register `ADSSTRO` reader"]
pub type R = crate::R<ADSSTRO_SPEC>;
#[doc = "Register `ADSSTRO` writer"]
pub type W = crate::W<ADSSTRO_SPEC>;
#[doc = "Field `SST` reader - Sampling Time Setting (Internal reference voltage)"]
pub type SST_R = crate::FieldReader;
#[doc = "Field `SST` writer - Sampling Time Setting (Internal reference voltage)"]
pub type SST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Sampling Time Setting (Internal reference voltage)"]
    #[inline(always)]
    pub fn sst(&self) -> SST_R {
        SST_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sampling Time Setting (Internal reference voltage)"]
    #[inline(always)]
    #[must_use]
    pub fn sst(&mut self) -> SST_W<ADSSTRO_SPEC, 0> {
        SST_W::new(self)
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
#[doc = "A/D Sampling State Register O\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adsstro::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adsstro::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADSSTRO_SPEC;
impl crate::RegisterSpec for ADSSTRO_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adsstro::R`](R) reader structure"]
impl crate::Readable for ADSSTRO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adsstro::W`](W) writer structure"]
impl crate::Writable for ADSSTRO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSSTRO to value 0x0d"]
impl crate::Resettable for ADSSTRO_SPEC {
    const RESET_VALUE: Self::Ux = 0x0d;
}
