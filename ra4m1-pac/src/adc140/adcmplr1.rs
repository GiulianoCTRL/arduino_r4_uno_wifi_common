#[doc = "Register `ADCMPLR1` reader"]
pub type R = crate::R<ADCMPLR1_SPEC>;
#[doc = "Register `ADCMPLR1` writer"]
pub type W = crate::W<ADCMPLR1_SPEC>;
#[doc = "Field `CMPLCHA16` reader - Comparison condition of AN016"]
pub type CMPLCHA16_R = crate::BitReader<CMPLCHA16_A>;
#[doc = "Comparison condition of AN016\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA16_A {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or, ADCMPDR1 value &lt; A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value &lt; A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<CMPLCHA16_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA16_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMPLCHA16_A {
        match self.bits {
            false => CMPLCHA16_A::_0,
            true => CMPLCHA16_A::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or, ADCMPDR1 value &lt; A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA16_A::_0
    }
    #[doc = "ADCMPDR0 value &lt; A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA16_A::_1
    }
}
#[doc = "Field `CMPLCHA16` writer - Comparison condition of AN016"]
pub type CMPLCHA16_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMPLCHA16_A>;
impl<'a, REG, const O: u8> CMPLCHA16_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or, ADCMPDR1 value &lt; A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA16_A::_0)
    }
    #[doc = "ADCMPDR0 value &lt; A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA16_A::_1)
    }
}
#[doc = "Field `CMPLCHA17` reader - Comparison condition of AN017"]
pub type CMPLCHA17_R = crate::BitReader<CMPLCHA17_A>;
#[doc = "Comparison condition of AN017\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA17_A {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or, ADCMPDR1 value &lt; A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value &lt; A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<CMPLCHA17_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA17_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMPLCHA17_A {
        match self.bits {
            false => CMPLCHA17_A::_0,
            true => CMPLCHA17_A::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or, ADCMPDR1 value &lt; A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA17_A::_0
    }
    #[doc = "ADCMPDR0 value &lt; A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA17_A::_1
    }
}
#[doc = "Field `CMPLCHA17` writer - Comparison condition of AN017"]
pub type CMPLCHA17_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMPLCHA17_A>;
impl<'a, REG, const O: u8> CMPLCHA17_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or, ADCMPDR1 value &lt; A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA17_A::_0)
    }
    #[doc = "ADCMPDR0 value &lt; A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA17_A::_1)
    }
}
#[doc = "Field `CMPLCHA18` reader - Comparison condition of AN018"]
pub type CMPLCHA18_R = crate::BitReader<CMPLCHA18_A>;
#[doc = "Comparison condition of AN018\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA18_A {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or, ADCMPDR1 value &lt; A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value &lt; A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<CMPLCHA18_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA18_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMPLCHA18_A {
        match self.bits {
            false => CMPLCHA18_A::_0,
            true => CMPLCHA18_A::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or, ADCMPDR1 value &lt; A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA18_A::_0
    }
    #[doc = "ADCMPDR0 value &lt; A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA18_A::_1
    }
}
#[doc = "Field `CMPLCHA18` writer - Comparison condition of AN018"]
pub type CMPLCHA18_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMPLCHA18_A>;
impl<'a, REG, const O: u8> CMPLCHA18_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or, ADCMPDR1 value &lt; A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA18_A::_0)
    }
    #[doc = "ADCMPDR0 value &lt; A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA18_A::_1)
    }
}
#[doc = "Field `CMPLCHA19` reader - Comparison condition of AN019"]
pub type CMPLCHA19_R = crate::BitReader<CMPLCHA19_A>;
#[doc = "Comparison condition of AN019\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA19_A {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or, ADCMPDR1 value &lt; A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value &lt; A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<CMPLCHA19_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA19_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMPLCHA19_A {
        match self.bits {
            false => CMPLCHA19_A::_0,
            true => CMPLCHA19_A::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or, ADCMPDR1 value &lt; A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA19_A::_0
    }
    #[doc = "ADCMPDR0 value &lt; A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA19_A::_1
    }
}
#[doc = "Field `CMPLCHA19` writer - Comparison condition of AN019"]
pub type CMPLCHA19_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMPLCHA19_A>;
impl<'a, REG, const O: u8> CMPLCHA19_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or, ADCMPDR1 value &lt; A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA19_A::_0)
    }
    #[doc = "ADCMPDR0 value &lt; A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA19_A::_1)
    }
}
#[doc = "Field `CMPLCHA20` reader - Comparison condition of AN020"]
pub type CMPLCHA20_R = crate::BitReader<CMPLCHA20_A>;
#[doc = "Comparison condition of AN020\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA20_A {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or, ADCMPDR1 value &lt; A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value &lt; A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<CMPLCHA20_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA20_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMPLCHA20_A {
        match self.bits {
            false => CMPLCHA20_A::_0,
            true => CMPLCHA20_A::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or, ADCMPDR1 value &lt; A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA20_A::_0
    }
    #[doc = "ADCMPDR0 value &lt; A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA20_A::_1
    }
}
#[doc = "Field `CMPLCHA20` writer - Comparison condition of AN020"]
pub type CMPLCHA20_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMPLCHA20_A>;
impl<'a, REG, const O: u8> CMPLCHA20_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or, ADCMPDR1 value &lt; A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA20_A::_0)
    }
    #[doc = "ADCMPDR0 value &lt; A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA20_A::_1)
    }
}
#[doc = "Field `CMPLCHA21` reader - Comparison condition of AN021"]
pub type CMPLCHA21_R = crate::BitReader<CMPLCHA21_A>;
#[doc = "Comparison condition of AN021\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA21_A {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or, ADCMPDR1 value &lt; A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value &lt; A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<CMPLCHA21_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA21_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMPLCHA21_A {
        match self.bits {
            false => CMPLCHA21_A::_0,
            true => CMPLCHA21_A::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or, ADCMPDR1 value &lt; A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA21_A::_0
    }
    #[doc = "ADCMPDR0 value &lt; A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA21_A::_1
    }
}
#[doc = "Field `CMPLCHA21` writer - Comparison condition of AN021"]
pub type CMPLCHA21_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMPLCHA21_A>;
impl<'a, REG, const O: u8> CMPLCHA21_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or, ADCMPDR1 value &lt; A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA21_A::_0)
    }
    #[doc = "ADCMPDR0 value &lt; A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA21_A::_1)
    }
}
#[doc = "Field `CMPLCHA22` reader - Comparison condition of AN022"]
pub type CMPLCHA22_R = crate::BitReader<CMPLCHA22_A>;
#[doc = "Comparison condition of AN022\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA22_A {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or, ADCMPDR1 value &lt; A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value &lt; A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<CMPLCHA22_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA22_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMPLCHA22_A {
        match self.bits {
            false => CMPLCHA22_A::_0,
            true => CMPLCHA22_A::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or, ADCMPDR1 value &lt; A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA22_A::_0
    }
    #[doc = "ADCMPDR0 value &lt; A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA22_A::_1
    }
}
#[doc = "Field `CMPLCHA22` writer - Comparison condition of AN022"]
pub type CMPLCHA22_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMPLCHA22_A>;
impl<'a, REG, const O: u8> CMPLCHA22_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or, ADCMPDR1 value &lt; A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA22_A::_0)
    }
    #[doc = "ADCMPDR0 value &lt; A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA22_A::_1)
    }
}
#[doc = "Field `CMPLCHA23` reader - Comparison condition of AN023"]
pub type CMPLCHA23_R = crate::BitReader<CMPLCHA23_A>;
#[doc = "Comparison condition of AN023\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA23_A {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or, ADCMPDR1 value &lt; A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value &lt; A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<CMPLCHA23_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA23_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMPLCHA23_A {
        match self.bits {
            false => CMPLCHA23_A::_0,
            true => CMPLCHA23_A::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or, ADCMPDR1 value &lt; A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA23_A::_0
    }
    #[doc = "ADCMPDR0 value &lt; A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA23_A::_1
    }
}
#[doc = "Field `CMPLCHA23` writer - Comparison condition of AN023"]
pub type CMPLCHA23_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMPLCHA23_A>;
impl<'a, REG, const O: u8> CMPLCHA23_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or, ADCMPDR1 value &lt; A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA23_A::_0)
    }
    #[doc = "ADCMPDR0 value &lt; A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA23_A::_1)
    }
}
#[doc = "Field `CMPLCHA24` reader - Comparison condition of AN024"]
pub type CMPLCHA24_R = crate::BitReader<CMPLCHA24_A>;
#[doc = "Comparison condition of AN024\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA24_A {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or, ADCMPDR1 value &lt; A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value &lt; A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<CMPLCHA24_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA24_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMPLCHA24_A {
        match self.bits {
            false => CMPLCHA24_A::_0,
            true => CMPLCHA24_A::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or, ADCMPDR1 value &lt; A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA24_A::_0
    }
    #[doc = "ADCMPDR0 value &lt; A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA24_A::_1
    }
}
#[doc = "Field `CMPLCHA24` writer - Comparison condition of AN024"]
pub type CMPLCHA24_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMPLCHA24_A>;
impl<'a, REG, const O: u8> CMPLCHA24_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or, ADCMPDR1 value &lt; A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA24_A::_0)
    }
    #[doc = "ADCMPDR0 value &lt; A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA24_A::_1)
    }
}
#[doc = "Field `CMPLCHA25` reader - Comparison condition of AN025"]
pub type CMPLCHA25_R = crate::BitReader<CMPLCHA25_A>;
#[doc = "Comparison condition of AN025\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA25_A {
    #[doc = "0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or, ADCMPDR1 value &lt; A/D converted value (ADCMPCR.WCMPE=1)"]
    _0 = 0,
    #[doc = "1: ADCMPDR0 value &lt; A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    _1 = 1,
}
impl From<CMPLCHA25_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA25_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLCHA25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMPLCHA25_A {
        match self.bits {
            false => CMPLCHA25_A::_0,
            true => CMPLCHA25_A::_1,
        }
    }
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or, ADCMPDR1 value &lt; A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA25_A::_0
    }
    #[doc = "ADCMPDR0 value &lt; A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA25_A::_1
    }
}
#[doc = "Field `CMPLCHA25` writer - Comparison condition of AN025"]
pub type CMPLCHA25_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMPLCHA25_A>;
impl<'a, REG, const O: u8> CMPLCHA25_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or, ADCMPDR1 value &lt; A/D converted value (ADCMPCR.WCMPE=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA25_A::_0)
    }
    #[doc = "ADCMPDR0 value &lt; A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value &lt; A/D converted value &lt; ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA25_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Comparison condition of AN016"]
    #[inline(always)]
    pub fn cmplcha16(&self) -> CMPLCHA16_R {
        CMPLCHA16_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparison condition of AN017"]
    #[inline(always)]
    pub fn cmplcha17(&self) -> CMPLCHA17_R {
        CMPLCHA17_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparison condition of AN018"]
    #[inline(always)]
    pub fn cmplcha18(&self) -> CMPLCHA18_R {
        CMPLCHA18_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comparison condition of AN019"]
    #[inline(always)]
    pub fn cmplcha19(&self) -> CMPLCHA19_R {
        CMPLCHA19_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Comparison condition of AN020"]
    #[inline(always)]
    pub fn cmplcha20(&self) -> CMPLCHA20_R {
        CMPLCHA20_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comparison condition of AN021"]
    #[inline(always)]
    pub fn cmplcha21(&self) -> CMPLCHA21_R {
        CMPLCHA21_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Comparison condition of AN022"]
    #[inline(always)]
    pub fn cmplcha22(&self) -> CMPLCHA22_R {
        CMPLCHA22_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Comparison condition of AN023"]
    #[inline(always)]
    pub fn cmplcha23(&self) -> CMPLCHA23_R {
        CMPLCHA23_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Comparison condition of AN024"]
    #[inline(always)]
    pub fn cmplcha24(&self) -> CMPLCHA24_R {
        CMPLCHA24_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Comparison condition of AN025"]
    #[inline(always)]
    pub fn cmplcha25(&self) -> CMPLCHA25_R {
        CMPLCHA25_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparison condition of AN016"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha16(&mut self) -> CMPLCHA16_W<ADCMPLR1_SPEC, 0> {
        CMPLCHA16_W::new(self)
    }
    #[doc = "Bit 1 - Comparison condition of AN017"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha17(&mut self) -> CMPLCHA17_W<ADCMPLR1_SPEC, 1> {
        CMPLCHA17_W::new(self)
    }
    #[doc = "Bit 2 - Comparison condition of AN018"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha18(&mut self) -> CMPLCHA18_W<ADCMPLR1_SPEC, 2> {
        CMPLCHA18_W::new(self)
    }
    #[doc = "Bit 3 - Comparison condition of AN019"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha19(&mut self) -> CMPLCHA19_W<ADCMPLR1_SPEC, 3> {
        CMPLCHA19_W::new(self)
    }
    #[doc = "Bit 4 - Comparison condition of AN020"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha20(&mut self) -> CMPLCHA20_W<ADCMPLR1_SPEC, 4> {
        CMPLCHA20_W::new(self)
    }
    #[doc = "Bit 5 - Comparison condition of AN021"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha21(&mut self) -> CMPLCHA21_W<ADCMPLR1_SPEC, 5> {
        CMPLCHA21_W::new(self)
    }
    #[doc = "Bit 6 - Comparison condition of AN022"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha22(&mut self) -> CMPLCHA22_W<ADCMPLR1_SPEC, 6> {
        CMPLCHA22_W::new(self)
    }
    #[doc = "Bit 7 - Comparison condition of AN023"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha23(&mut self) -> CMPLCHA23_W<ADCMPLR1_SPEC, 7> {
        CMPLCHA23_W::new(self)
    }
    #[doc = "Bit 8 - Comparison condition of AN024"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha24(&mut self) -> CMPLCHA24_W<ADCMPLR1_SPEC, 8> {
        CMPLCHA24_W::new(self)
    }
    #[doc = "Bit 9 - Comparison condition of AN025"]
    #[inline(always)]
    #[must_use]
    pub fn cmplcha25(&mut self) -> CMPLCHA25_W<ADCMPLR1_SPEC, 9> {
        CMPLCHA25_W::new(self)
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
#[doc = "A/D Compare Function Window A Comparison Condition Setting Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmplr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmplr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCMPLR1_SPEC;
impl crate::RegisterSpec for ADCMPLR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcmplr1::R`](R) reader structure"]
impl crate::Readable for ADCMPLR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adcmplr1::W`](W) writer structure"]
impl crate::Writable for ADCMPLR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPLR1 to value 0"]
impl crate::Resettable for ADCMPLR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
