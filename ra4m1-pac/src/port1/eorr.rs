#[doc = "Register `EORR` reader"]
pub type R = crate::R<EORR_SPEC>;
#[doc = "Register `EORR` writer"]
pub type W = crate::W<EORR_SPEC>;
#[doc = "Field `EORR` reader - Pmn Event Output Reset"]
pub type EORR_R = crate::FieldReader<EORR_A>;
#[doc = "Pmn Event Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum EORR_A {
    #[doc = "0: No affect to output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<EORR_A> for u16 {
    #[inline(always)]
    fn from(variant: EORR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EORR_A {
    type Ux = u16;
}
impl EORR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EORR_A> {
        match self.bits {
            0 => Some(EORR_A::_0),
            1 => Some(EORR_A::_1),
            _ => None,
        }
    }
    #[doc = "No affect to output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EORR_A::_0
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EORR_A::_1
    }
}
#[doc = "Field `EORR` writer - Pmn Event Output Reset"]
pub type EORR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, EORR_A>;
impl<'a, REG, const O: u8> EORR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "No affect to output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EORR_A::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EORR_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:15 - Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr(&self) -> EORR_R {
        EORR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Pmn Event Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eorr(&mut self) -> EORR_W<EORR_SPEC, 0> {
        EORR_W::new(self)
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
#[doc = "Event output set register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eorr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eorr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EORR_SPEC;
impl crate::RegisterSpec for EORR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`eorr::R`](R) reader structure"]
impl crate::Readable for EORR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eorr::W`](W) writer structure"]
impl crate::Writable for EORR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EORR to value 0"]
impl crate::Resettable for EORR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
