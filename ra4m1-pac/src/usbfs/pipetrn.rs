#[doc = "Register `PIPE%sTRN` reader"]
pub type R = crate::R<PIPETRN_SPEC>;
#[doc = "Register `PIPE%sTRN` writer"]
pub type W = crate::W<PIPETRN_SPEC>;
#[doc = "Field `TRNCNT` reader - Transaction Counter"]
pub type TRNCNT_R = crate::FieldReader<u16>;
#[doc = "Field `TRNCNT` writer - Transaction Counter"]
pub type TRNCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Transaction Counter"]
    #[inline(always)]
    pub fn trncnt(&self) -> TRNCNT_R {
        TRNCNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transaction Counter"]
    #[inline(always)]
    #[must_use]
    pub fn trncnt(&mut self) -> TRNCNT_W<PIPETRN_SPEC, 0> {
        TRNCNT_W::new(self)
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
#[doc = "Pipe %s Transaction Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pipetrn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pipetrn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIPETRN_SPEC;
impl crate::RegisterSpec for PIPETRN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pipetrn::R`](R) reader structure"]
impl crate::Readable for PIPETRN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pipetrn::W`](W) writer structure"]
impl crate::Writable for PIPETRN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIPE%sTRN to value 0"]
impl crate::Resettable for PIPETRN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
