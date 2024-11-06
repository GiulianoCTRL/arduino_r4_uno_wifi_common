#[doc = "Register `SNZREQCR` reader"]
pub type R = crate::R<SNZREQCR_SPEC>;
#[doc = "Register `SNZREQCR` writer"]
pub type W = crate::W<SNZREQCR_SPEC>;
#[doc = "Field `SNZREQEN0` reader - Snooze Request Enable 0 Enable IRQ0 pin snooze request"]
pub type SNZREQEN0_R = crate::BitReader<SNZREQEN0_A>;
#[doc = "Snooze Request Enable 0 Enable IRQ0 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN0_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN0_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN0_A {
        match self.bits {
            false => SNZREQEN0_A::_0,
            true => SNZREQEN0_A::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN0_A::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN0_A::_1
    }
}
#[doc = "Field `SNZREQEN0` writer - Snooze Request Enable 0 Enable IRQ0 pin snooze request"]
pub type SNZREQEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SNZREQEN0_A>;
impl<'a, REG, const O: u8> SNZREQEN0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN0_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN0_A::_1)
    }
}
#[doc = "Field `SNZREQEN1` reader - Snooze Request Enable 1 Enable IRQ1 pin snooze request"]
pub type SNZREQEN1_R = crate::BitReader<SNZREQEN1_A>;
#[doc = "Snooze Request Enable 1 Enable IRQ1 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN1_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN1_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN1_A {
        match self.bits {
            false => SNZREQEN1_A::_0,
            true => SNZREQEN1_A::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN1_A::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN1_A::_1
    }
}
#[doc = "Field `SNZREQEN1` writer - Snooze Request Enable 1 Enable IRQ1 pin snooze request"]
pub type SNZREQEN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SNZREQEN1_A>;
impl<'a, REG, const O: u8> SNZREQEN1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN1_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN1_A::_1)
    }
}
#[doc = "Field `SNZREQEN2` reader - Snooze Request Enable 2 Enable IRQ2 pin snooze request"]
pub type SNZREQEN2_R = crate::BitReader<SNZREQEN2_A>;
#[doc = "Snooze Request Enable 2 Enable IRQ2 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN2_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN2_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN2_A {
        match self.bits {
            false => SNZREQEN2_A::_0,
            true => SNZREQEN2_A::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN2_A::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN2_A::_1
    }
}
#[doc = "Field `SNZREQEN2` writer - Snooze Request Enable 2 Enable IRQ2 pin snooze request"]
pub type SNZREQEN2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SNZREQEN2_A>;
impl<'a, REG, const O: u8> SNZREQEN2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN2_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN2_A::_1)
    }
}
#[doc = "Field `SNZREQEN3` reader - Snooze Request Enable 3 Enable IRQ3 pin snooze request"]
pub type SNZREQEN3_R = crate::BitReader<SNZREQEN3_A>;
#[doc = "Snooze Request Enable 3 Enable IRQ3 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN3_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN3_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN3_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN3_A {
        match self.bits {
            false => SNZREQEN3_A::_0,
            true => SNZREQEN3_A::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN3_A::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN3_A::_1
    }
}
#[doc = "Field `SNZREQEN3` writer - Snooze Request Enable 3 Enable IRQ3 pin snooze request"]
pub type SNZREQEN3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SNZREQEN3_A>;
impl<'a, REG, const O: u8> SNZREQEN3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN3_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN3_A::_1)
    }
}
#[doc = "Field `SNZREQEN4` reader - Snooze Request Enable 4 Enable IRQ4 pin snooze request"]
pub type SNZREQEN4_R = crate::BitReader<SNZREQEN4_A>;
#[doc = "Snooze Request Enable 4 Enable IRQ4 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN4_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN4_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN4_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN4_A {
        match self.bits {
            false => SNZREQEN4_A::_0,
            true => SNZREQEN4_A::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN4_A::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN4_A::_1
    }
}
#[doc = "Field `SNZREQEN4` writer - Snooze Request Enable 4 Enable IRQ4 pin snooze request"]
pub type SNZREQEN4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SNZREQEN4_A>;
impl<'a, REG, const O: u8> SNZREQEN4_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN4_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN4_A::_1)
    }
}
#[doc = "Field `SNZREQEN5` reader - Snooze Request Enable 5 Enable IRQ5 pin snooze request"]
pub type SNZREQEN5_R = crate::BitReader<SNZREQEN5_A>;
#[doc = "Snooze Request Enable 5 Enable IRQ5 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN5_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN5_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN5_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN5_A {
        match self.bits {
            false => SNZREQEN5_A::_0,
            true => SNZREQEN5_A::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN5_A::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN5_A::_1
    }
}
#[doc = "Field `SNZREQEN5` writer - Snooze Request Enable 5 Enable IRQ5 pin snooze request"]
pub type SNZREQEN5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SNZREQEN5_A>;
impl<'a, REG, const O: u8> SNZREQEN5_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN5_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN5_A::_1)
    }
}
#[doc = "Field `SNZREQEN6` reader - Snooze Request Enable 6 Enable IRQ6 pin snooze request"]
pub type SNZREQEN6_R = crate::BitReader<SNZREQEN6_A>;
#[doc = "Snooze Request Enable 6 Enable IRQ6 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN6_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN6_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN6_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN6_A {
        match self.bits {
            false => SNZREQEN6_A::_0,
            true => SNZREQEN6_A::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN6_A::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN6_A::_1
    }
}
#[doc = "Field `SNZREQEN6` writer - Snooze Request Enable 6 Enable IRQ6 pin snooze request"]
pub type SNZREQEN6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SNZREQEN6_A>;
impl<'a, REG, const O: u8> SNZREQEN6_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN6_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN6_A::_1)
    }
}
#[doc = "Field `SNZREQEN7` reader - Snooze Request Enable 7 Enable IRQ7 pin snooze request"]
pub type SNZREQEN7_R = crate::BitReader<SNZREQEN7_A>;
#[doc = "Snooze Request Enable 7 Enable IRQ7 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN7_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN7_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN7_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN7_A {
        match self.bits {
            false => SNZREQEN7_A::_0,
            true => SNZREQEN7_A::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN7_A::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN7_A::_1
    }
}
#[doc = "Field `SNZREQEN7` writer - Snooze Request Enable 7 Enable IRQ7 pin snooze request"]
pub type SNZREQEN7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SNZREQEN7_A>;
impl<'a, REG, const O: u8> SNZREQEN7_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN7_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN7_A::_1)
    }
}
#[doc = "Field `SNZREQEN8` reader - Snooze Request Enable 8 Enable IRQ8 pin snooze request"]
pub type SNZREQEN8_R = crate::BitReader<SNZREQEN8_A>;
#[doc = "Snooze Request Enable 8 Enable IRQ8 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN8_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN8_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN8_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN8_A {
        match self.bits {
            false => SNZREQEN8_A::_0,
            true => SNZREQEN8_A::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN8_A::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN8_A::_1
    }
}
#[doc = "Field `SNZREQEN8` writer - Snooze Request Enable 8 Enable IRQ8 pin snooze request"]
pub type SNZREQEN8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SNZREQEN8_A>;
impl<'a, REG, const O: u8> SNZREQEN8_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN8_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN8_A::_1)
    }
}
#[doc = "Field `SNZREQEN9` reader - Snooze Request Enable 9 Enable IRQ9 pin snooze request"]
pub type SNZREQEN9_R = crate::BitReader<SNZREQEN9_A>;
#[doc = "Snooze Request Enable 9 Enable IRQ9 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN9_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN9_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN9_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN9_A {
        match self.bits {
            false => SNZREQEN9_A::_0,
            true => SNZREQEN9_A::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN9_A::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN9_A::_1
    }
}
#[doc = "Field `SNZREQEN9` writer - Snooze Request Enable 9 Enable IRQ9 pin snooze request"]
pub type SNZREQEN9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SNZREQEN9_A>;
impl<'a, REG, const O: u8> SNZREQEN9_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN9_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN9_A::_1)
    }
}
#[doc = "Field `SNZREQEN10` reader - Snooze Request Enable 10 Enable IRQ10 pin snooze request"]
pub type SNZREQEN10_R = crate::BitReader<SNZREQEN10_A>;
#[doc = "Snooze Request Enable 10 Enable IRQ10 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN10_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN10_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN10_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN10_A {
        match self.bits {
            false => SNZREQEN10_A::_0,
            true => SNZREQEN10_A::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN10_A::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN10_A::_1
    }
}
#[doc = "Field `SNZREQEN10` writer - Snooze Request Enable 10 Enable IRQ10 pin snooze request"]
pub type SNZREQEN10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SNZREQEN10_A>;
impl<'a, REG, const O: u8> SNZREQEN10_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN10_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN10_A::_1)
    }
}
#[doc = "Field `SNZREQEN11` reader - Snooze Request Enable 11 Enable IRQ11 pin snooze request"]
pub type SNZREQEN11_R = crate::BitReader<SNZREQEN11_A>;
#[doc = "Snooze Request Enable 11 Enable IRQ11 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN11_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN11_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN11_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN11_A {
        match self.bits {
            false => SNZREQEN11_A::_0,
            true => SNZREQEN11_A::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN11_A::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN11_A::_1
    }
}
#[doc = "Field `SNZREQEN11` writer - Snooze Request Enable 11 Enable IRQ11 pin snooze request"]
pub type SNZREQEN11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SNZREQEN11_A>;
impl<'a, REG, const O: u8> SNZREQEN11_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN11_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN11_A::_1)
    }
}
#[doc = "Field `SNZREQEN12` reader - Snooze Request Enable 12 Enable IRQ12 pin snooze request"]
pub type SNZREQEN12_R = crate::BitReader<SNZREQEN12_A>;
#[doc = "Snooze Request Enable 12 Enable IRQ12 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN12_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN12_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN12_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN12_A {
        match self.bits {
            false => SNZREQEN12_A::_0,
            true => SNZREQEN12_A::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN12_A::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN12_A::_1
    }
}
#[doc = "Field `SNZREQEN12` writer - Snooze Request Enable 12 Enable IRQ12 pin snooze request"]
pub type SNZREQEN12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SNZREQEN12_A>;
impl<'a, REG, const O: u8> SNZREQEN12_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN12_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN12_A::_1)
    }
}
#[doc = "Field `SNZREQEN14` reader - Snooze Request Enable 14 Enable IRQ14 pin snooze request"]
pub type SNZREQEN14_R = crate::BitReader<SNZREQEN14_A>;
#[doc = "Snooze Request Enable 14 Enable IRQ14 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN14_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN14_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN14_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN14_A {
        match self.bits {
            false => SNZREQEN14_A::_0,
            true => SNZREQEN14_A::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN14_A::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN14_A::_1
    }
}
#[doc = "Field `SNZREQEN14` writer - Snooze Request Enable 14 Enable IRQ14 pin snooze request"]
pub type SNZREQEN14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SNZREQEN14_A>;
impl<'a, REG, const O: u8> SNZREQEN14_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN14_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN14_A::_1)
    }
}
#[doc = "Field `SNZREQEN15` reader - Snooze Request Enable 15 Enable IRQ15 pin snooze request"]
pub type SNZREQEN15_R = crate::BitReader<SNZREQEN15_A>;
#[doc = "Snooze Request Enable 15 Enable IRQ15 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN15_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN15_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN15_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN15_A {
        match self.bits {
            false => SNZREQEN15_A::_0,
            true => SNZREQEN15_A::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN15_A::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN15_A::_1
    }
}
#[doc = "Field `SNZREQEN15` writer - Snooze Request Enable 15 Enable IRQ15 pin snooze request"]
pub type SNZREQEN15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SNZREQEN15_A>;
impl<'a, REG, const O: u8> SNZREQEN15_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN15_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN15_A::_1)
    }
}
#[doc = "Field `SNZREQEN17` reader - Snooze Request Enable 17 Enable KINT snooze request"]
pub type SNZREQEN17_R = crate::BitReader<SNZREQEN17_A>;
#[doc = "Snooze Request Enable 17 Enable KINT snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN17_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN17_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN17_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN17_A {
        match self.bits {
            false => SNZREQEN17_A::_0,
            true => SNZREQEN17_A::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN17_A::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN17_A::_1
    }
}
#[doc = "Field `SNZREQEN17` writer - Snooze Request Enable 17 Enable KINT snooze request"]
pub type SNZREQEN17_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SNZREQEN17_A>;
impl<'a, REG, const O: u8> SNZREQEN17_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN17_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN17_A::_1)
    }
}
#[doc = "Field `SNZREQEN23` reader - Snooze Request Enable 23 Enable RTC alarm snooze request"]
pub type SNZREQEN23_R = crate::BitReader<SNZREQEN23_A>;
#[doc = "Snooze Request Enable 23 Enable RTC alarm snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN23_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN23_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN23_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN23_A {
        match self.bits {
            false => SNZREQEN23_A::_0,
            true => SNZREQEN23_A::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN23_A::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN23_A::_1
    }
}
#[doc = "Field `SNZREQEN23` writer - Snooze Request Enable 23 Enable RTC alarm snooze request"]
pub type SNZREQEN23_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SNZREQEN23_A>;
impl<'a, REG, const O: u8> SNZREQEN23_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN23_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN23_A::_1)
    }
}
#[doc = "Field `SNZREQEN24` reader - Snooze Request Enable 24 Enable RTC alarm snooze request"]
pub type SNZREQEN24_R = crate::BitReader<SNZREQEN24_A>;
#[doc = "Snooze Request Enable 24 Enable RTC alarm snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN24_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN24_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN24_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN24_A {
        match self.bits {
            false => SNZREQEN24_A::_0,
            true => SNZREQEN24_A::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN24_A::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN24_A::_1
    }
}
#[doc = "Field `SNZREQEN24` writer - Snooze Request Enable 24 Enable RTC alarm snooze request"]
pub type SNZREQEN24_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SNZREQEN24_A>;
impl<'a, REG, const O: u8> SNZREQEN24_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN24_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN24_A::_1)
    }
}
#[doc = "Field `SNZREQEN25` reader - Snooze Request Enable 25 Enable RTC period snooze request"]
pub type SNZREQEN25_R = crate::BitReader<SNZREQEN25_A>;
#[doc = "Snooze Request Enable 25 Enable RTC period snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN25_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN25_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN25_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN25_A {
        match self.bits {
            false => SNZREQEN25_A::_0,
            true => SNZREQEN25_A::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN25_A::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN25_A::_1
    }
}
#[doc = "Field `SNZREQEN25` writer - Snooze Request Enable 25 Enable RTC period snooze request"]
pub type SNZREQEN25_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SNZREQEN25_A>;
impl<'a, REG, const O: u8> SNZREQEN25_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN25_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN25_A::_1)
    }
}
#[doc = "Field `SNZREQEN28` reader - Snooze Request Enable 28 Enable AGT1 underflow snooze request"]
pub type SNZREQEN28_R = crate::BitReader<SNZREQEN28_A>;
#[doc = "Snooze Request Enable 28 Enable AGT1 underflow snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN28_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN28_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN28_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN28_A {
        match self.bits {
            false => SNZREQEN28_A::_0,
            true => SNZREQEN28_A::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN28_A::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN28_A::_1
    }
}
#[doc = "Field `SNZREQEN28` writer - Snooze Request Enable 28 Enable AGT1 underflow snooze request"]
pub type SNZREQEN28_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SNZREQEN28_A>;
impl<'a, REG, const O: u8> SNZREQEN28_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN28_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN28_A::_1)
    }
}
#[doc = "Field `SNZREQEN29` reader - Snooze Request Enable 29 Enable AGT1 compare match A snooze request"]
pub type SNZREQEN29_R = crate::BitReader<SNZREQEN29_A>;
#[doc = "Snooze Request Enable 29 Enable AGT1 compare match A snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN29_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN29_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN29_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN29_A {
        match self.bits {
            false => SNZREQEN29_A::_0,
            true => SNZREQEN29_A::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN29_A::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN29_A::_1
    }
}
#[doc = "Field `SNZREQEN29` writer - Snooze Request Enable 29 Enable AGT1 compare match A snooze request"]
pub type SNZREQEN29_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SNZREQEN29_A>;
impl<'a, REG, const O: u8> SNZREQEN29_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN29_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN29_A::_1)
    }
}
#[doc = "Field `SNZREQEN30` reader - Snooze Request Enable 30 Enable AGT1 compare match B snooze request"]
pub type SNZREQEN30_R = crate::BitReader<SNZREQEN30_A>;
#[doc = "Snooze Request Enable 30 Enable AGT1 compare match B snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN30_A {
    #[doc = "0: Disable snooze request"]
    _0 = 0,
    #[doc = "1: Enable snooze request"]
    _1 = 1,
}
impl From<SNZREQEN30_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN30_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZREQEN30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN30_A {
        match self.bits {
            false => SNZREQEN30_A::_0,
            true => SNZREQEN30_A::_1,
        }
    }
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN30_A::_0
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN30_A::_1
    }
}
#[doc = "Field `SNZREQEN30` writer - Snooze Request Enable 30 Enable AGT1 compare match B snooze request"]
pub type SNZREQEN30_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SNZREQEN30_A>;
impl<'a, REG, const O: u8> SNZREQEN30_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN30_A::_0)
    }
    #[doc = "Enable snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN30_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Snooze Request Enable 0 Enable IRQ0 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen0(&self) -> SNZREQEN0_R {
        SNZREQEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Snooze Request Enable 1 Enable IRQ1 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen1(&self) -> SNZREQEN1_R {
        SNZREQEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Snooze Request Enable 2 Enable IRQ2 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen2(&self) -> SNZREQEN2_R {
        SNZREQEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Snooze Request Enable 3 Enable IRQ3 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen3(&self) -> SNZREQEN3_R {
        SNZREQEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Snooze Request Enable 4 Enable IRQ4 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen4(&self) -> SNZREQEN4_R {
        SNZREQEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Snooze Request Enable 5 Enable IRQ5 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen5(&self) -> SNZREQEN5_R {
        SNZREQEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Snooze Request Enable 6 Enable IRQ6 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen6(&self) -> SNZREQEN6_R {
        SNZREQEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Snooze Request Enable 7 Enable IRQ7 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen7(&self) -> SNZREQEN7_R {
        SNZREQEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Snooze Request Enable 8 Enable IRQ8 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen8(&self) -> SNZREQEN8_R {
        SNZREQEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Snooze Request Enable 9 Enable IRQ9 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen9(&self) -> SNZREQEN9_R {
        SNZREQEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Snooze Request Enable 10 Enable IRQ10 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen10(&self) -> SNZREQEN10_R {
        SNZREQEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Snooze Request Enable 11 Enable IRQ11 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen11(&self) -> SNZREQEN11_R {
        SNZREQEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Snooze Request Enable 12 Enable IRQ12 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen12(&self) -> SNZREQEN12_R {
        SNZREQEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Snooze Request Enable 14 Enable IRQ14 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen14(&self) -> SNZREQEN14_R {
        SNZREQEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Snooze Request Enable 15 Enable IRQ15 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen15(&self) -> SNZREQEN15_R {
        SNZREQEN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Snooze Request Enable 17 Enable KINT snooze request"]
    #[inline(always)]
    pub fn snzreqen17(&self) -> SNZREQEN17_R {
        SNZREQEN17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 23 - Snooze Request Enable 23 Enable RTC alarm snooze request"]
    #[inline(always)]
    pub fn snzreqen23(&self) -> SNZREQEN23_R {
        SNZREQEN23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Snooze Request Enable 24 Enable RTC alarm snooze request"]
    #[inline(always)]
    pub fn snzreqen24(&self) -> SNZREQEN24_R {
        SNZREQEN24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Snooze Request Enable 25 Enable RTC period snooze request"]
    #[inline(always)]
    pub fn snzreqen25(&self) -> SNZREQEN25_R {
        SNZREQEN25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Snooze Request Enable 28 Enable AGT1 underflow snooze request"]
    #[inline(always)]
    pub fn snzreqen28(&self) -> SNZREQEN28_R {
        SNZREQEN28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Snooze Request Enable 29 Enable AGT1 compare match A snooze request"]
    #[inline(always)]
    pub fn snzreqen29(&self) -> SNZREQEN29_R {
        SNZREQEN29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Snooze Request Enable 30 Enable AGT1 compare match B snooze request"]
    #[inline(always)]
    pub fn snzreqen30(&self) -> SNZREQEN30_R {
        SNZREQEN30_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Snooze Request Enable 0 Enable IRQ0 pin snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen0(&mut self) -> SNZREQEN0_W<SNZREQCR_SPEC, 0> {
        SNZREQEN0_W::new(self)
    }
    #[doc = "Bit 1 - Snooze Request Enable 1 Enable IRQ1 pin snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen1(&mut self) -> SNZREQEN1_W<SNZREQCR_SPEC, 1> {
        SNZREQEN1_W::new(self)
    }
    #[doc = "Bit 2 - Snooze Request Enable 2 Enable IRQ2 pin snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen2(&mut self) -> SNZREQEN2_W<SNZREQCR_SPEC, 2> {
        SNZREQEN2_W::new(self)
    }
    #[doc = "Bit 3 - Snooze Request Enable 3 Enable IRQ3 pin snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen3(&mut self) -> SNZREQEN3_W<SNZREQCR_SPEC, 3> {
        SNZREQEN3_W::new(self)
    }
    #[doc = "Bit 4 - Snooze Request Enable 4 Enable IRQ4 pin snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen4(&mut self) -> SNZREQEN4_W<SNZREQCR_SPEC, 4> {
        SNZREQEN4_W::new(self)
    }
    #[doc = "Bit 5 - Snooze Request Enable 5 Enable IRQ5 pin snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen5(&mut self) -> SNZREQEN5_W<SNZREQCR_SPEC, 5> {
        SNZREQEN5_W::new(self)
    }
    #[doc = "Bit 6 - Snooze Request Enable 6 Enable IRQ6 pin snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen6(&mut self) -> SNZREQEN6_W<SNZREQCR_SPEC, 6> {
        SNZREQEN6_W::new(self)
    }
    #[doc = "Bit 7 - Snooze Request Enable 7 Enable IRQ7 pin snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen7(&mut self) -> SNZREQEN7_W<SNZREQCR_SPEC, 7> {
        SNZREQEN7_W::new(self)
    }
    #[doc = "Bit 8 - Snooze Request Enable 8 Enable IRQ8 pin snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen8(&mut self) -> SNZREQEN8_W<SNZREQCR_SPEC, 8> {
        SNZREQEN8_W::new(self)
    }
    #[doc = "Bit 9 - Snooze Request Enable 9 Enable IRQ9 pin snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen9(&mut self) -> SNZREQEN9_W<SNZREQCR_SPEC, 9> {
        SNZREQEN9_W::new(self)
    }
    #[doc = "Bit 10 - Snooze Request Enable 10 Enable IRQ10 pin snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen10(&mut self) -> SNZREQEN10_W<SNZREQCR_SPEC, 10> {
        SNZREQEN10_W::new(self)
    }
    #[doc = "Bit 11 - Snooze Request Enable 11 Enable IRQ11 pin snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen11(&mut self) -> SNZREQEN11_W<SNZREQCR_SPEC, 11> {
        SNZREQEN11_W::new(self)
    }
    #[doc = "Bit 12 - Snooze Request Enable 12 Enable IRQ12 pin snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen12(&mut self) -> SNZREQEN12_W<SNZREQCR_SPEC, 12> {
        SNZREQEN12_W::new(self)
    }
    #[doc = "Bit 14 - Snooze Request Enable 14 Enable IRQ14 pin snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen14(&mut self) -> SNZREQEN14_W<SNZREQCR_SPEC, 14> {
        SNZREQEN14_W::new(self)
    }
    #[doc = "Bit 15 - Snooze Request Enable 15 Enable IRQ15 pin snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen15(&mut self) -> SNZREQEN15_W<SNZREQCR_SPEC, 15> {
        SNZREQEN15_W::new(self)
    }
    #[doc = "Bit 17 - Snooze Request Enable 17 Enable KINT snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen17(&mut self) -> SNZREQEN17_W<SNZREQCR_SPEC, 17> {
        SNZREQEN17_W::new(self)
    }
    #[doc = "Bit 23 - Snooze Request Enable 23 Enable RTC alarm snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen23(&mut self) -> SNZREQEN23_W<SNZREQCR_SPEC, 23> {
        SNZREQEN23_W::new(self)
    }
    #[doc = "Bit 24 - Snooze Request Enable 24 Enable RTC alarm snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen24(&mut self) -> SNZREQEN24_W<SNZREQCR_SPEC, 24> {
        SNZREQEN24_W::new(self)
    }
    #[doc = "Bit 25 - Snooze Request Enable 25 Enable RTC period snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen25(&mut self) -> SNZREQEN25_W<SNZREQCR_SPEC, 25> {
        SNZREQEN25_W::new(self)
    }
    #[doc = "Bit 28 - Snooze Request Enable 28 Enable AGT1 underflow snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen28(&mut self) -> SNZREQEN28_W<SNZREQCR_SPEC, 28> {
        SNZREQEN28_W::new(self)
    }
    #[doc = "Bit 29 - Snooze Request Enable 29 Enable AGT1 compare match A snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen29(&mut self) -> SNZREQEN29_W<SNZREQCR_SPEC, 29> {
        SNZREQEN29_W::new(self)
    }
    #[doc = "Bit 30 - Snooze Request Enable 30 Enable AGT1 compare match B snooze request"]
    #[inline(always)]
    #[must_use]
    pub fn snzreqen30(&mut self) -> SNZREQEN30_W<SNZREQCR_SPEC, 30> {
        SNZREQEN30_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Snooze Request Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`snzreqcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`snzreqcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SNZREQCR_SPEC;
impl crate::RegisterSpec for SNZREQCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`snzreqcr::R`](R) reader structure"]
impl crate::Readable for SNZREQCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`snzreqcr::W`](W) writer structure"]
impl crate::Writable for SNZREQCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SNZREQCR to value 0"]
impl crate::Resettable for SNZREQCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
