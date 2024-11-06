#[doc = "Register `GTCCRB` reader"]
pub type R = crate::R<GTCCRB_SPEC>;
#[doc = "Register `GTCCRB` writer"]
pub type W = crate::W<GTCCRB_SPEC>;
#[doc = "Field `GTCCRB` reader - Compare Capture Register B"]
pub type GTCCRB_R = crate::FieldReader<u32>;
#[doc = "Field `GTCCRB` writer - Compare Capture Register B"]
pub type GTCCRB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Compare Capture Register B"]
    #[inline(always)]
    pub fn gtccrb(&self) -> GTCCRB_R {
        GTCCRB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare Capture Register B"]
    #[inline(always)]
    #[must_use]
    pub fn gtccrb(&mut self) -> GTCCRB_W<GTCCRB_SPEC, 0> {
        GTCCRB_W::new(self)
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
#[doc = "General PWM Timer Compare Capture Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtccrb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtccrb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTCCRB_SPEC;
impl crate::RegisterSpec for GTCCRB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtccrb::R`](R) reader structure"]
impl crate::Readable for GTCCRB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gtccrb::W`](W) writer structure"]
impl crate::Writable for GTCCRB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTCCRB to value 0xffff_ffff"]
impl crate::Resettable for GTCCRB_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
