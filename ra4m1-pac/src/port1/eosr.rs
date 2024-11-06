#[doc = "Register `EOSR` reader"]
pub type R = crate::R<EOSR_SPEC>;
#[doc = "Register `EOSR` writer"]
pub type W = crate::W<EOSR_SPEC>;
#[doc = "Field `EOSR` reader - Pmn Event Output Set"]
pub type EOSR_R = crate::FieldReader<EOSR_A>;
#[doc = "Pmn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum EOSR_A {
    #[doc = "0: No affect to output"]
    _0 = 0,
    #[doc = "1: High output."]
    _1 = 1,
}
impl From<EOSR_A> for u16 {
    #[inline(always)]
    fn from(variant: EOSR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EOSR_A {
    type Ux = u16;
}
impl EOSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EOSR_A> {
        match self.bits {
            0 => Some(EOSR_A::_0),
            1 => Some(EOSR_A::_1),
            _ => None,
        }
    }
    #[doc = "No affect to output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOSR_A::_0
    }
    #[doc = "High output."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOSR_A::_1
    }
}
#[doc = "Field `EOSR` writer - Pmn Event Output Set"]
pub type EOSR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, EOSR_A>;
impl<'a, REG, const O: u8> EOSR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "No affect to output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EOSR_A::_0)
    }
    #[doc = "High output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EOSR_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:15 - Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr(&self) -> EOSR_R {
        EOSR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Pmn Event Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn eosr(&mut self) -> EOSR_W<EOSR_SPEC, 0> {
        EOSR_W::new(self)
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
#[doc = "Event output reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eosr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eosr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EOSR_SPEC;
impl crate::RegisterSpec for EOSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`eosr::R`](R) reader structure"]
impl crate::Readable for EOSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eosr::W`](W) writer structure"]
impl crate::Writable for EOSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EOSR to value 0"]
impl crate::Resettable for EOSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
