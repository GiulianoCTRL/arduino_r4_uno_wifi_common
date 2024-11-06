#[doc = "Register `SARL%s` reader"]
pub type R = crate::R<SARL_SPEC>;
#[doc = "Register `SARL%s` writer"]
pub type W = crate::W<SARL_SPEC>;
#[doc = "Field `SVA` reader - A slave address is set. 7-Bit Address = SVA\\[7:1\\]
10-Bit Address = { SVA9,SVA8,SVA\\[7:0\\]
}"]
pub type SVA_R = crate::FieldReader;
#[doc = "Field `SVA` writer - A slave address is set. 7-Bit Address = SVA\\[7:1\\]
10-Bit Address = { SVA9,SVA8,SVA\\[7:0\\]
}"]
pub type SVA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - A slave address is set. 7-Bit Address = SVA\\[7:1\\]
10-Bit Address = { SVA9,SVA8,SVA\\[7:0\\]
}"]
    #[inline(always)]
    pub fn sva(&self) -> SVA_R {
        SVA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - A slave address is set. 7-Bit Address = SVA\\[7:1\\]
10-Bit Address = { SVA9,SVA8,SVA\\[7:0\\]
}"]
    #[inline(always)]
    #[must_use]
    pub fn sva(&mut self) -> SVA_W<SARL_SPEC, 0> {
        SVA_W::new(self)
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
#[doc = "Slave Address Register L%s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sarl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sarl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SARL_SPEC;
impl crate::RegisterSpec for SARL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sarl::R`](R) reader structure"]
impl crate::Readable for SARL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sarl::W`](W) writer structure"]
impl crate::Writable for SARL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SARL%s to value 0"]
impl crate::Resettable for SARL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
