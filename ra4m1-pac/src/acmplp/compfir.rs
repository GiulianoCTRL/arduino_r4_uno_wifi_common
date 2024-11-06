#[doc = "Register `COMPFIR` reader"]
pub type R = crate::R<COMPFIR_SPEC>;
#[doc = "Register `COMPFIR` writer"]
pub type W = crate::W<COMPFIR_SPEC>;
#[doc = "Field `C0FCK` reader - ACMPLP0 Filter Select"]
pub type C0FCK_R = crate::FieldReader<C0FCK_A>;
#[doc = "ACMPLP0 Filter Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C0FCK_A {
    #[doc = "0: No Sampling (bypass)"]
    _00 = 0,
    #[doc = "1: Sampling at PCLK"]
    _01 = 1,
    #[doc = "2: Sampling at PCLK/8"]
    _10 = 2,
    #[doc = "3: Sampling at PCLK/32"]
    _11 = 3,
}
impl From<C0FCK_A> for u8 {
    #[inline(always)]
    fn from(variant: C0FCK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C0FCK_A {
    type Ux = u8;
}
impl C0FCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0FCK_A {
        match self.bits {
            0 => C0FCK_A::_00,
            1 => C0FCK_A::_01,
            2 => C0FCK_A::_10,
            3 => C0FCK_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "No Sampling (bypass)"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == C0FCK_A::_00
    }
    #[doc = "Sampling at PCLK"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == C0FCK_A::_01
    }
    #[doc = "Sampling at PCLK/8"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == C0FCK_A::_10
    }
    #[doc = "Sampling at PCLK/32"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == C0FCK_A::_11
    }
}
#[doc = "Field `C0FCK` writer - ACMPLP0 Filter Select"]
pub type C0FCK_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, C0FCK_A>;
impl<'a, REG, const O: u8> C0FCK_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Sampling (bypass)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(C0FCK_A::_00)
    }
    #[doc = "Sampling at PCLK"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(C0FCK_A::_01)
    }
    #[doc = "Sampling at PCLK/8"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(C0FCK_A::_10)
    }
    #[doc = "Sampling at PCLK/32"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(C0FCK_A::_11)
    }
}
#[doc = "Field `C0EPO` reader - ACMPLP0 Edge Polarity Switching"]
pub type C0EPO_R = crate::BitReader<C0EPO_A>;
#[doc = "ACMPLP0 Edge Polarity Switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0EPO_A {
    #[doc = "0: Interrupt and ELC event request at rising edge"]
    _0 = 0,
    #[doc = "1: Interrupt and ELC event request at falling edge"]
    _1 = 1,
}
impl From<C0EPO_A> for bool {
    #[inline(always)]
    fn from(variant: C0EPO_A) -> Self {
        variant as u8 != 0
    }
}
impl C0EPO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0EPO_A {
        match self.bits {
            false => C0EPO_A::_0,
            true => C0EPO_A::_1,
        }
    }
    #[doc = "Interrupt and ELC event request at rising edge"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0EPO_A::_0
    }
    #[doc = "Interrupt and ELC event request at falling edge"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0EPO_A::_1
    }
}
#[doc = "Field `C0EPO` writer - ACMPLP0 Edge Polarity Switching"]
pub type C0EPO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, C0EPO_A>;
impl<'a, REG, const O: u8> C0EPO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt and ELC event request at rising edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C0EPO_A::_0)
    }
    #[doc = "Interrupt and ELC event request at falling edge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C0EPO_A::_1)
    }
}
#[doc = "Field `C0EDG` reader - ACMPLP0 Edge Detection Selection"]
pub type C0EDG_R = crate::BitReader<C0EDG_A>;
#[doc = "ACMPLP0 Edge Detection Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0EDG_A {
    #[doc = "0: Interrupt and ELC event request by one-edge detection"]
    _0 = 0,
    #[doc = "1: Interrupt and ELC event request by both-edge detection"]
    _1 = 1,
}
impl From<C0EDG_A> for bool {
    #[inline(always)]
    fn from(variant: C0EDG_A) -> Self {
        variant as u8 != 0
    }
}
impl C0EDG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0EDG_A {
        match self.bits {
            false => C0EDG_A::_0,
            true => C0EDG_A::_1,
        }
    }
    #[doc = "Interrupt and ELC event request by one-edge detection"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0EDG_A::_0
    }
    #[doc = "Interrupt and ELC event request by both-edge detection"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0EDG_A::_1
    }
}
#[doc = "Field `C0EDG` writer - ACMPLP0 Edge Detection Selection"]
pub type C0EDG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, C0EDG_A>;
impl<'a, REG, const O: u8> C0EDG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt and ELC event request by one-edge detection"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C0EDG_A::_0)
    }
    #[doc = "Interrupt and ELC event request by both-edge detection"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C0EDG_A::_1)
    }
}
#[doc = "Field `C1FCK` reader - ACMPLP1 Filter Select"]
pub type C1FCK_R = crate::FieldReader<C1FCK_A>;
#[doc = "ACMPLP1 Filter Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C1FCK_A {
    #[doc = "0: No Sampling (bypass)"]
    _00 = 0,
    #[doc = "1: Sampling at PCLK"]
    _01 = 1,
    #[doc = "2: Sampling at PCLK/8"]
    _10 = 2,
    #[doc = "3: Sampling at PCLK/32"]
    _11 = 3,
}
impl From<C1FCK_A> for u8 {
    #[inline(always)]
    fn from(variant: C1FCK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C1FCK_A {
    type Ux = u8;
}
impl C1FCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1FCK_A {
        match self.bits {
            0 => C1FCK_A::_00,
            1 => C1FCK_A::_01,
            2 => C1FCK_A::_10,
            3 => C1FCK_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "No Sampling (bypass)"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == C1FCK_A::_00
    }
    #[doc = "Sampling at PCLK"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == C1FCK_A::_01
    }
    #[doc = "Sampling at PCLK/8"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == C1FCK_A::_10
    }
    #[doc = "Sampling at PCLK/32"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == C1FCK_A::_11
    }
}
#[doc = "Field `C1FCK` writer - ACMPLP1 Filter Select"]
pub type C1FCK_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, C1FCK_A>;
impl<'a, REG, const O: u8> C1FCK_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Sampling (bypass)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(C1FCK_A::_00)
    }
    #[doc = "Sampling at PCLK"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(C1FCK_A::_01)
    }
    #[doc = "Sampling at PCLK/8"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(C1FCK_A::_10)
    }
    #[doc = "Sampling at PCLK/32"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(C1FCK_A::_11)
    }
}
#[doc = "Field `C1EPO` reader - ACMPLP1 Edge Polarity Switching"]
pub type C1EPO_R = crate::BitReader<C1EPO_A>;
#[doc = "ACMPLP1 Edge Polarity Switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1EPO_A {
    #[doc = "0: Interrupt and ELC event request at rising edge"]
    _0 = 0,
    #[doc = "1: Interrupt and ELC event request at falling edge"]
    _1 = 1,
}
impl From<C1EPO_A> for bool {
    #[inline(always)]
    fn from(variant: C1EPO_A) -> Self {
        variant as u8 != 0
    }
}
impl C1EPO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1EPO_A {
        match self.bits {
            false => C1EPO_A::_0,
            true => C1EPO_A::_1,
        }
    }
    #[doc = "Interrupt and ELC event request at rising edge"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1EPO_A::_0
    }
    #[doc = "Interrupt and ELC event request at falling edge"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1EPO_A::_1
    }
}
#[doc = "Field `C1EPO` writer - ACMPLP1 Edge Polarity Switching"]
pub type C1EPO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, C1EPO_A>;
impl<'a, REG, const O: u8> C1EPO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt and ELC event request at rising edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C1EPO_A::_0)
    }
    #[doc = "Interrupt and ELC event request at falling edge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C1EPO_A::_1)
    }
}
#[doc = "Field `C1EDG` reader - ACMPLP1 Edge Detection Selection"]
pub type C1EDG_R = crate::BitReader<C1EDG_A>;
#[doc = "ACMPLP1 Edge Detection Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1EDG_A {
    #[doc = "0: Interrupt and ELC event request by one-edge detection"]
    _0 = 0,
    #[doc = "1: Interrupt and ELC event request by both-edge detection"]
    _1 = 1,
}
impl From<C1EDG_A> for bool {
    #[inline(always)]
    fn from(variant: C1EDG_A) -> Self {
        variant as u8 != 0
    }
}
impl C1EDG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1EDG_A {
        match self.bits {
            false => C1EDG_A::_0,
            true => C1EDG_A::_1,
        }
    }
    #[doc = "Interrupt and ELC event request by one-edge detection"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1EDG_A::_0
    }
    #[doc = "Interrupt and ELC event request by both-edge detection"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1EDG_A::_1
    }
}
#[doc = "Field `C1EDG` writer - ACMPLP1 Edge Detection Selection"]
pub type C1EDG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, C1EDG_A>;
impl<'a, REG, const O: u8> C1EDG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt and ELC event request by one-edge detection"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C1EDG_A::_0)
    }
    #[doc = "Interrupt and ELC event request by both-edge detection"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C1EDG_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - ACMPLP0 Filter Select"]
    #[inline(always)]
    pub fn c0fck(&self) -> C0FCK_R {
        C0FCK_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - ACMPLP0 Edge Polarity Switching"]
    #[inline(always)]
    pub fn c0epo(&self) -> C0EPO_R {
        C0EPO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ACMPLP0 Edge Detection Selection"]
    #[inline(always)]
    pub fn c0edg(&self) -> C0EDG_R {
        C0EDG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - ACMPLP1 Filter Select"]
    #[inline(always)]
    pub fn c1fck(&self) -> C1FCK_R {
        C1FCK_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - ACMPLP1 Edge Polarity Switching"]
    #[inline(always)]
    pub fn c1epo(&self) -> C1EPO_R {
        C1EPO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ACMPLP1 Edge Detection Selection"]
    #[inline(always)]
    pub fn c1edg(&self) -> C1EDG_R {
        C1EDG_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ACMPLP0 Filter Select"]
    #[inline(always)]
    #[must_use]
    pub fn c0fck(&mut self) -> C0FCK_W<COMPFIR_SPEC, 0> {
        C0FCK_W::new(self)
    }
    #[doc = "Bit 2 - ACMPLP0 Edge Polarity Switching"]
    #[inline(always)]
    #[must_use]
    pub fn c0epo(&mut self) -> C0EPO_W<COMPFIR_SPEC, 2> {
        C0EPO_W::new(self)
    }
    #[doc = "Bit 3 - ACMPLP0 Edge Detection Selection"]
    #[inline(always)]
    #[must_use]
    pub fn c0edg(&mut self) -> C0EDG_W<COMPFIR_SPEC, 3> {
        C0EDG_W::new(self)
    }
    #[doc = "Bits 4:5 - ACMPLP1 Filter Select"]
    #[inline(always)]
    #[must_use]
    pub fn c1fck(&mut self) -> C1FCK_W<COMPFIR_SPEC, 4> {
        C1FCK_W::new(self)
    }
    #[doc = "Bit 6 - ACMPLP1 Edge Polarity Switching"]
    #[inline(always)]
    #[must_use]
    pub fn c1epo(&mut self) -> C1EPO_W<COMPFIR_SPEC, 6> {
        C1EPO_W::new(self)
    }
    #[doc = "Bit 7 - ACMPLP1 Edge Detection Selection"]
    #[inline(always)]
    #[must_use]
    pub fn c1edg(&mut self) -> C1EDG_W<COMPFIR_SPEC, 7> {
        C1EDG_W::new(self)
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
#[doc = "ACMPLP Filter Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`compfir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`compfir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMPFIR_SPEC;
impl crate::RegisterSpec for COMPFIR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`compfir::R`](R) reader structure"]
impl crate::Readable for COMPFIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`compfir::W`](W) writer structure"]
impl crate::Writable for COMPFIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMPFIR to value 0"]
impl crate::Resettable for COMPFIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
