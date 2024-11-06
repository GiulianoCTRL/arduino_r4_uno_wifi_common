#[doc = "Register `PORR` writer"]
pub type W = crate::W<PORR_SPEC>;
#[doc = "Pmn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PORR_AW {
    #[doc = "0: No affect to output"]
    _0 = 0,
    #[doc = "1: Low output."]
    _1 = 1,
}
impl From<PORR_AW> for u16 {
    #[inline(always)]
    fn from(variant: PORR_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PORR_AW {
    type Ux = u16;
}
#[doc = "Field `PORR` writer - Pmn Output Reset"]
pub type PORR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, PORR_AW>;
impl<'a, REG, const O: u8> PORR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "No affect to output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PORR_AW::_0)
    }
    #[doc = "Low output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PORR_AW::_1)
    }
}
impl W {
    #[doc = "Bits 0:15 - Pmn Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn porr(&mut self) -> PORR_W<PORR_SPEC, 0> {
        PORR_W::new(self)
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
#[doc = "Output reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`porr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PORR_SPEC;
impl crate::RegisterSpec for PORR_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`porr::W`](W) writer structure"]
impl crate::Writable for PORR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PORR to value 0"]
impl crate::Resettable for PORR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
