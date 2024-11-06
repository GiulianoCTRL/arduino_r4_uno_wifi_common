#[doc = "Register `COMPOCR` reader"]
pub type R = crate::R<COMPOCR_SPEC>;
#[doc = "Register `COMPOCR` writer"]
pub type W = crate::W<COMPOCR_SPEC>;
#[doc = "Field `C0OE` reader - ACMPLP0 VCOUT Pin Output Enable"]
pub type C0OE_R = crate::BitReader<C0OE_A>;
#[doc = "ACMPLP0 VCOUT Pin Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0OE_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<C0OE_A> for bool {
    #[inline(always)]
    fn from(variant: C0OE_A) -> Self {
        variant as u8 != 0
    }
}
impl C0OE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0OE_A {
        match self.bits {
            false => C0OE_A::_0,
            true => C0OE_A::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0OE_A::_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0OE_A::_1
    }
}
#[doc = "Field `C0OE` writer - ACMPLP0 VCOUT Pin Output Enable"]
pub type C0OE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, C0OE_A>;
impl<'a, REG, const O: u8> C0OE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C0OE_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C0OE_A::_1)
    }
}
#[doc = "Field `C0OP` reader - ACMPLP0 VCOUT Output Polarity Selection"]
pub type C0OP_R = crate::BitReader<C0OP_A>;
#[doc = "ACMPLP0 VCOUT Output Polarity Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0OP_A {
    #[doc = "0: Non inverted"]
    _0 = 0,
    #[doc = "1: Inverted"]
    _1 = 1,
}
impl From<C0OP_A> for bool {
    #[inline(always)]
    fn from(variant: C0OP_A) -> Self {
        variant as u8 != 0
    }
}
impl C0OP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0OP_A {
        match self.bits {
            false => C0OP_A::_0,
            true => C0OP_A::_1,
        }
    }
    #[doc = "Non inverted"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0OP_A::_0
    }
    #[doc = "Inverted"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0OP_A::_1
    }
}
#[doc = "Field `C0OP` writer - ACMPLP0 VCOUT Output Polarity Selection"]
pub type C0OP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, C0OP_A>;
impl<'a, REG, const O: u8> C0OP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Non inverted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C0OP_A::_0)
    }
    #[doc = "Inverted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C0OP_A::_1)
    }
}
#[doc = "Field `C1OE` reader - ACMPLP1 VCOUT Pin Output Enable"]
pub type C1OE_R = crate::BitReader<C1OE_A>;
#[doc = "ACMPLP1 VCOUT Pin Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1OE_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<C1OE_A> for bool {
    #[inline(always)]
    fn from(variant: C1OE_A) -> Self {
        variant as u8 != 0
    }
}
impl C1OE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1OE_A {
        match self.bits {
            false => C1OE_A::_0,
            true => C1OE_A::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1OE_A::_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1OE_A::_1
    }
}
#[doc = "Field `C1OE` writer - ACMPLP1 VCOUT Pin Output Enable"]
pub type C1OE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, C1OE_A>;
impl<'a, REG, const O: u8> C1OE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C1OE_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C1OE_A::_1)
    }
}
#[doc = "Field `C1OP` reader - ACMPLP1 VCOUT Output Polarity Selection"]
pub type C1OP_R = crate::BitReader<C1OP_A>;
#[doc = "ACMPLP1 VCOUT Output Polarity Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1OP_A {
    #[doc = "0: Non inverted"]
    _0 = 0,
    #[doc = "1: Inverted"]
    _1 = 1,
}
impl From<C1OP_A> for bool {
    #[inline(always)]
    fn from(variant: C1OP_A) -> Self {
        variant as u8 != 0
    }
}
impl C1OP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1OP_A {
        match self.bits {
            false => C1OP_A::_0,
            true => C1OP_A::_1,
        }
    }
    #[doc = "Non inverted"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1OP_A::_0
    }
    #[doc = "Inverted"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1OP_A::_1
    }
}
#[doc = "Field `C1OP` writer - ACMPLP1 VCOUT Output Polarity Selection"]
pub type C1OP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, C1OP_A>;
impl<'a, REG, const O: u8> C1OP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Non inverted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C1OP_A::_0)
    }
    #[doc = "Inverted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C1OP_A::_1)
    }
}
#[doc = "Field `SPDMD` reader - ACMPLP0/ACMPLP1 Speed Selection"]
pub type SPDMD_R = crate::BitReader<SPDMD_A>;
#[doc = "ACMPLP0/ACMPLP1 Speed Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPDMD_A {
    #[doc = "0: Comparator low-speed mode"]
    _0 = 0,
    #[doc = "1: Comparator high-speed mode"]
    _1 = 1,
}
impl From<SPDMD_A> for bool {
    #[inline(always)]
    fn from(variant: SPDMD_A) -> Self {
        variant as u8 != 0
    }
}
impl SPDMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPDMD_A {
        match self.bits {
            false => SPDMD_A::_0,
            true => SPDMD_A::_1,
        }
    }
    #[doc = "Comparator low-speed mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPDMD_A::_0
    }
    #[doc = "Comparator high-speed mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPDMD_A::_1
    }
}
#[doc = "Field `SPDMD` writer - ACMPLP0/ACMPLP1 Speed Selection"]
pub type SPDMD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SPDMD_A>;
impl<'a, REG, const O: u8> SPDMD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator low-speed mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPDMD_A::_0)
    }
    #[doc = "Comparator high-speed mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPDMD_A::_1)
    }
}
impl R {
    #[doc = "Bit 1 - ACMPLP0 VCOUT Pin Output Enable"]
    #[inline(always)]
    pub fn c0oe(&self) -> C0OE_R {
        C0OE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ACMPLP0 VCOUT Output Polarity Selection"]
    #[inline(always)]
    pub fn c0op(&self) -> C0OP_R {
        C0OP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - ACMPLP1 VCOUT Pin Output Enable"]
    #[inline(always)]
    pub fn c1oe(&self) -> C1OE_R {
        C1OE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ACMPLP1 VCOUT Output Polarity Selection"]
    #[inline(always)]
    pub fn c1op(&self) -> C1OP_R {
        C1OP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ACMPLP0/ACMPLP1 Speed Selection"]
    #[inline(always)]
    pub fn spdmd(&self) -> SPDMD_R {
        SPDMD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - ACMPLP0 VCOUT Pin Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn c0oe(&mut self) -> C0OE_W<COMPOCR_SPEC, 1> {
        C0OE_W::new(self)
    }
    #[doc = "Bit 2 - ACMPLP0 VCOUT Output Polarity Selection"]
    #[inline(always)]
    #[must_use]
    pub fn c0op(&mut self) -> C0OP_W<COMPOCR_SPEC, 2> {
        C0OP_W::new(self)
    }
    #[doc = "Bit 5 - ACMPLP1 VCOUT Pin Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn c1oe(&mut self) -> C1OE_W<COMPOCR_SPEC, 5> {
        C1OE_W::new(self)
    }
    #[doc = "Bit 6 - ACMPLP1 VCOUT Output Polarity Selection"]
    #[inline(always)]
    #[must_use]
    pub fn c1op(&mut self) -> C1OP_W<COMPOCR_SPEC, 6> {
        C1OP_W::new(self)
    }
    #[doc = "Bit 7 - ACMPLP0/ACMPLP1 Speed Selection"]
    #[inline(always)]
    #[must_use]
    pub fn spdmd(&mut self) -> SPDMD_W<COMPOCR_SPEC, 7> {
        SPDMD_W::new(self)
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
#[doc = "ACMPLP Output Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`compocr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`compocr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMPOCR_SPEC;
impl crate::RegisterSpec for COMPOCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`compocr::R`](R) reader structure"]
impl crate::Readable for COMPOCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`compocr::W`](W) writer structure"]
impl crate::Writable for COMPOCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMPOCR to value 0"]
impl crate::Resettable for COMPOCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
