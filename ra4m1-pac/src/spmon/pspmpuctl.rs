#[doc = "Register `PSPMPUCTL` reader"]
pub type R = crate::R<PSPMPUCTL_SPEC>;
#[doc = "Register `PSPMPUCTL` writer"]
pub type W = crate::W<PSPMPUCTL_SPEC>;
#[doc = "Field `ENABLE` reader - Stack Pointer Monitor Enable"]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
#[doc = "Stack Pointer Monitor Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_A {
    #[doc = "0: Stack pointer monitor is disabled"]
    _0 = 0,
    #[doc = "1: Stack pointer monitor is enabled"]
    _1 = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::_0,
            true => ENABLE_A::_1,
        }
    }
    #[doc = "Stack pointer monitor is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENABLE_A::_0
    }
    #[doc = "Stack pointer monitor is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENABLE_A::_1
    }
}
#[doc = "Field `ENABLE` writer - Stack Pointer Monitor Enable"]
pub type ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ENABLE_A>;
impl<'a, REG, const O: u8> ENABLE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stack pointer monitor is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE_A::_0)
    }
    #[doc = "Stack pointer monitor is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE_A::_1)
    }
}
#[doc = "Field `ERROR` reader - Stack Pointer Monitor Error Flag"]
pub type ERROR_R = crate::BitReader<ERROR_A>;
#[doc = "Stack Pointer Monitor Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERROR_A {
    #[doc = "0: Stack pointer has not overflowed or underflowed"]
    _0 = 0,
    #[doc = "1: Stack pointer has overflowed or underflowed"]
    _1 = 1,
}
impl From<ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_A) -> Self {
        variant as u8 != 0
    }
}
impl ERROR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERROR_A {
        match self.bits {
            false => ERROR_A::_0,
            true => ERROR_A::_1,
        }
    }
    #[doc = "Stack pointer has not overflowed or underflowed"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERROR_A::_0
    }
    #[doc = "Stack pointer has overflowed or underflowed"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERROR_A::_1
    }
}
#[doc = "Field `ERROR` writer - Stack Pointer Monitor Error Flag"]
pub type ERROR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ERROR_A>;
impl<'a, REG, const O: u8> ERROR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stack pointer has not overflowed or underflowed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ERROR_A::_0)
    }
    #[doc = "Stack pointer has overflowed or underflowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ERROR_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Stack Pointer Monitor Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Stack Pointer Monitor Error Flag"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stack Pointer Monitor Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<PSPMPUCTL_SPEC, 0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 8 - Stack Pointer Monitor Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<PSPMPUCTL_SPEC, 8> {
        ERROR_W::new(self)
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
#[doc = "Stack Pointer Monitor Access Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pspmpuctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pspmpuctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSPMPUCTL_SPEC;
impl crate::RegisterSpec for PSPMPUCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pspmpuctl::R`](R) reader structure"]
impl crate::Readable for PSPMPUCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pspmpuctl::W`](W) writer structure"]
impl crate::Writable for PSPMPUCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSPMPUCTL to value 0"]
impl crate::Resettable for PSPMPUCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
