#[doc = "Register `PODR` reader"]
pub type R = crate::R<PODR_SPEC>;
#[doc = "Register `PODR` writer"]
pub type W = crate::W<PODR_SPEC>;
#[doc = "Field `PODR` reader - Pmn Output Data"]
pub type PODR_R = crate::FieldReader<PODR_A>;
#[doc = "Pmn Output Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PODR_A {
    #[doc = "0: Low output"]
    _0 = 0,
    #[doc = "1: High output."]
    _1 = 1,
}
impl From<PODR_A> for u16 {
    #[inline(always)]
    fn from(variant: PODR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PODR_A {
    type Ux = u16;
}
impl PODR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PODR_A> {
        match self.bits {
            0 => Some(PODR_A::_0),
            1 => Some(PODR_A::_1),
            _ => None,
        }
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PODR_A::_0
    }
    #[doc = "High output."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PODR_A::_1
    }
}
#[doc = "Field `PODR` writer - Pmn Output Data"]
pub type PODR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, PODR_A>;
impl<'a, REG, const O: u8> PODR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PODR_A::_0)
    }
    #[doc = "High output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PODR_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:15 - Pmn Output Data"]
    #[inline(always)]
    pub fn podr(&self) -> PODR_R {
        PODR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Pmn Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn podr(&mut self) -> PODR_W<PODR_SPEC, 0> {
        PODR_W::new(self)
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
#[doc = "Output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`podr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`podr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PODR_SPEC;
impl crate::RegisterSpec for PODR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`podr::R`](R) reader structure"]
impl crate::Readable for PODR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`podr::W`](W) writer structure"]
impl crate::Writable for PODR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PODR to value 0"]
impl crate::Resettable for PODR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
