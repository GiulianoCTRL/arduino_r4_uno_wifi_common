#[doc = "Register `CSSR` reader"]
pub type R = crate::R<CSSR_SPEC>;
#[doc = "Register `CSSR` writer"]
pub type W = crate::W<CSSR_SPEC>;
#[doc = "Field `CSSR` reader - When the value for the channel search is input, the channel number is output to MSSR."]
pub type CSSR_R = crate::FieldReader;
#[doc = "Field `CSSR` writer - When the value for the channel search is input, the channel number is output to MSSR."]
pub type CSSR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - When the value for the channel search is input, the channel number is output to MSSR."]
    #[inline(always)]
    pub fn cssr(&self) -> CSSR_R {
        CSSR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - When the value for the channel search is input, the channel number is output to MSSR."]
    #[inline(always)]
    #[must_use]
    pub fn cssr(&mut self) -> CSSR_W<CSSR_SPEC, 0> {
        CSSR_W::new(self)
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
#[doc = "Channel Search Support Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cssr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cssr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSSR_SPEC;
impl crate::RegisterSpec for CSSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cssr::R`](R) reader structure"]
impl crate::Readable for CSSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cssr::W`](W) writer structure"]
impl crate::Writable for CSSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSSR to value 0"]
impl crate::Resettable for CSSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
