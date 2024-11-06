#[doc = "Register `BEMPENB` reader"]
pub type R = crate::R<BEMPENB_SPEC>;
#[doc = "Register `BEMPENB` writer"]
pub type W = crate::W<BEMPENB_SPEC>;
#[doc = "Field `PIPE0BEMPE` reader - BEMP Interrupt Enable for PIPE0"]
pub type PIPE0BEMPE_R = crate::BitReader<PIPE0BEMPE_A>;
#[doc = "BEMP Interrupt Enable for PIPE0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE0BEMPE_A {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<PIPE0BEMPE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE0BEMPE_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE0BEMPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIPE0BEMPE_A {
        match self.bits {
            false => PIPE0BEMPE_A::_0,
            true => PIPE0BEMPE_A::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE0BEMPE_A::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE0BEMPE_A::_1
    }
}
#[doc = "Field `PIPE0BEMPE` writer - BEMP Interrupt Enable for PIPE0"]
pub type PIPE0BEMPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PIPE0BEMPE_A>;
impl<'a, REG, const O: u8> PIPE0BEMPE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE0BEMPE_A::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE0BEMPE_A::_1)
    }
}
#[doc = "Field `PIPE1BEMPE` reader - BEMP Interrupt Enable for PIPE1"]
pub type PIPE1BEMPE_R = crate::BitReader<PIPE1BEMPE_A>;
#[doc = "BEMP Interrupt Enable for PIPE1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE1BEMPE_A {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<PIPE1BEMPE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE1BEMPE_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE1BEMPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIPE1BEMPE_A {
        match self.bits {
            false => PIPE1BEMPE_A::_0,
            true => PIPE1BEMPE_A::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE1BEMPE_A::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE1BEMPE_A::_1
    }
}
#[doc = "Field `PIPE1BEMPE` writer - BEMP Interrupt Enable for PIPE1"]
pub type PIPE1BEMPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PIPE1BEMPE_A>;
impl<'a, REG, const O: u8> PIPE1BEMPE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE1BEMPE_A::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE1BEMPE_A::_1)
    }
}
#[doc = "Field `PIPE2BEMPE` reader - BEMP Interrupt Enable for PIPE2"]
pub type PIPE2BEMPE_R = crate::BitReader<PIPE2BEMPE_A>;
#[doc = "BEMP Interrupt Enable for PIPE2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE2BEMPE_A {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<PIPE2BEMPE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE2BEMPE_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE2BEMPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIPE2BEMPE_A {
        match self.bits {
            false => PIPE2BEMPE_A::_0,
            true => PIPE2BEMPE_A::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE2BEMPE_A::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE2BEMPE_A::_1
    }
}
#[doc = "Field `PIPE2BEMPE` writer - BEMP Interrupt Enable for PIPE2"]
pub type PIPE2BEMPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PIPE2BEMPE_A>;
impl<'a, REG, const O: u8> PIPE2BEMPE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE2BEMPE_A::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE2BEMPE_A::_1)
    }
}
#[doc = "Field `PIPE3BEMPE` reader - BEMP Interrupt Enable for PIPE3"]
pub type PIPE3BEMPE_R = crate::BitReader<PIPE3BEMPE_A>;
#[doc = "BEMP Interrupt Enable for PIPE3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE3BEMPE_A {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<PIPE3BEMPE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE3BEMPE_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE3BEMPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIPE3BEMPE_A {
        match self.bits {
            false => PIPE3BEMPE_A::_0,
            true => PIPE3BEMPE_A::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE3BEMPE_A::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE3BEMPE_A::_1
    }
}
#[doc = "Field `PIPE3BEMPE` writer - BEMP Interrupt Enable for PIPE3"]
pub type PIPE3BEMPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PIPE3BEMPE_A>;
impl<'a, REG, const O: u8> PIPE3BEMPE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE3BEMPE_A::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE3BEMPE_A::_1)
    }
}
#[doc = "Field `PIPE4BEMPE` reader - BEMP Interrupt Enable for PIPE4"]
pub type PIPE4BEMPE_R = crate::BitReader<PIPE4BEMPE_A>;
#[doc = "BEMP Interrupt Enable for PIPE4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE4BEMPE_A {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<PIPE4BEMPE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE4BEMPE_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE4BEMPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIPE4BEMPE_A {
        match self.bits {
            false => PIPE4BEMPE_A::_0,
            true => PIPE4BEMPE_A::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE4BEMPE_A::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE4BEMPE_A::_1
    }
}
#[doc = "Field `PIPE4BEMPE` writer - BEMP Interrupt Enable for PIPE4"]
pub type PIPE4BEMPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PIPE4BEMPE_A>;
impl<'a, REG, const O: u8> PIPE4BEMPE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE4BEMPE_A::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE4BEMPE_A::_1)
    }
}
#[doc = "Field `PIPE5BEMPE` reader - BEMP Interrupt Enable for PIPE5"]
pub type PIPE5BEMPE_R = crate::BitReader<PIPE5BEMPE_A>;
#[doc = "BEMP Interrupt Enable for PIPE5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE5BEMPE_A {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<PIPE5BEMPE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE5BEMPE_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE5BEMPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIPE5BEMPE_A {
        match self.bits {
            false => PIPE5BEMPE_A::_0,
            true => PIPE5BEMPE_A::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE5BEMPE_A::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE5BEMPE_A::_1
    }
}
#[doc = "Field `PIPE5BEMPE` writer - BEMP Interrupt Enable for PIPE5"]
pub type PIPE5BEMPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PIPE5BEMPE_A>;
impl<'a, REG, const O: u8> PIPE5BEMPE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE5BEMPE_A::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE5BEMPE_A::_1)
    }
}
#[doc = "Field `PIPE6BEMPE` reader - BEMP Interrupt Enable for PIPE6"]
pub type PIPE6BEMPE_R = crate::BitReader<PIPE6BEMPE_A>;
#[doc = "BEMP Interrupt Enable for PIPE6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE6BEMPE_A {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<PIPE6BEMPE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE6BEMPE_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE6BEMPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIPE6BEMPE_A {
        match self.bits {
            false => PIPE6BEMPE_A::_0,
            true => PIPE6BEMPE_A::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE6BEMPE_A::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE6BEMPE_A::_1
    }
}
#[doc = "Field `PIPE6BEMPE` writer - BEMP Interrupt Enable for PIPE6"]
pub type PIPE6BEMPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PIPE6BEMPE_A>;
impl<'a, REG, const O: u8> PIPE6BEMPE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE6BEMPE_A::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE6BEMPE_A::_1)
    }
}
#[doc = "Field `PIPE7BEMPE` reader - BEMP Interrupt Enable for PIPE7"]
pub type PIPE7BEMPE_R = crate::BitReader<PIPE7BEMPE_A>;
#[doc = "BEMP Interrupt Enable for PIPE7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE7BEMPE_A {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<PIPE7BEMPE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE7BEMPE_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE7BEMPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIPE7BEMPE_A {
        match self.bits {
            false => PIPE7BEMPE_A::_0,
            true => PIPE7BEMPE_A::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE7BEMPE_A::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE7BEMPE_A::_1
    }
}
#[doc = "Field `PIPE7BEMPE` writer - BEMP Interrupt Enable for PIPE7"]
pub type PIPE7BEMPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PIPE7BEMPE_A>;
impl<'a, REG, const O: u8> PIPE7BEMPE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE7BEMPE_A::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE7BEMPE_A::_1)
    }
}
#[doc = "Field `PIPE8BEMPE` reader - BEMP Interrupt Enable for PIPE8"]
pub type PIPE8BEMPE_R = crate::BitReader<PIPE8BEMPE_A>;
#[doc = "BEMP Interrupt Enable for PIPE8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE8BEMPE_A {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<PIPE8BEMPE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE8BEMPE_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE8BEMPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIPE8BEMPE_A {
        match self.bits {
            false => PIPE8BEMPE_A::_0,
            true => PIPE8BEMPE_A::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE8BEMPE_A::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE8BEMPE_A::_1
    }
}
#[doc = "Field `PIPE8BEMPE` writer - BEMP Interrupt Enable for PIPE8"]
pub type PIPE8BEMPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PIPE8BEMPE_A>;
impl<'a, REG, const O: u8> PIPE8BEMPE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE8BEMPE_A::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE8BEMPE_A::_1)
    }
}
#[doc = "Field `PIPE9BEMPE` reader - BEMP Interrupt Enable for PIPE9"]
pub type PIPE9BEMPE_R = crate::BitReader<PIPE9BEMPE_A>;
#[doc = "BEMP Interrupt Enable for PIPE9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE9BEMPE_A {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<PIPE9BEMPE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE9BEMPE_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE9BEMPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIPE9BEMPE_A {
        match self.bits {
            false => PIPE9BEMPE_A::_0,
            true => PIPE9BEMPE_A::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE9BEMPE_A::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE9BEMPE_A::_1
    }
}
#[doc = "Field `PIPE9BEMPE` writer - BEMP Interrupt Enable for PIPE9"]
pub type PIPE9BEMPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PIPE9BEMPE_A>;
impl<'a, REG, const O: u8> PIPE9BEMPE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE9BEMPE_A::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE9BEMPE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - BEMP Interrupt Enable for PIPE0"]
    #[inline(always)]
    pub fn pipe0bempe(&self) -> PIPE0BEMPE_R {
        PIPE0BEMPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BEMP Interrupt Enable for PIPE1"]
    #[inline(always)]
    pub fn pipe1bempe(&self) -> PIPE1BEMPE_R {
        PIPE1BEMPE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BEMP Interrupt Enable for PIPE2"]
    #[inline(always)]
    pub fn pipe2bempe(&self) -> PIPE2BEMPE_R {
        PIPE2BEMPE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BEMP Interrupt Enable for PIPE3"]
    #[inline(always)]
    pub fn pipe3bempe(&self) -> PIPE3BEMPE_R {
        PIPE3BEMPE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BEMP Interrupt Enable for PIPE4"]
    #[inline(always)]
    pub fn pipe4bempe(&self) -> PIPE4BEMPE_R {
        PIPE4BEMPE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BEMP Interrupt Enable for PIPE5"]
    #[inline(always)]
    pub fn pipe5bempe(&self) -> PIPE5BEMPE_R {
        PIPE5BEMPE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BEMP Interrupt Enable for PIPE6"]
    #[inline(always)]
    pub fn pipe6bempe(&self) -> PIPE6BEMPE_R {
        PIPE6BEMPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BEMP Interrupt Enable for PIPE7"]
    #[inline(always)]
    pub fn pipe7bempe(&self) -> PIPE7BEMPE_R {
        PIPE7BEMPE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - BEMP Interrupt Enable for PIPE8"]
    #[inline(always)]
    pub fn pipe8bempe(&self) -> PIPE8BEMPE_R {
        PIPE8BEMPE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BEMP Interrupt Enable for PIPE9"]
    #[inline(always)]
    pub fn pipe9bempe(&self) -> PIPE9BEMPE_R {
        PIPE9BEMPE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BEMP Interrupt Enable for PIPE0"]
    #[inline(always)]
    #[must_use]
    pub fn pipe0bempe(&mut self) -> PIPE0BEMPE_W<BEMPENB_SPEC, 0> {
        PIPE0BEMPE_W::new(self)
    }
    #[doc = "Bit 1 - BEMP Interrupt Enable for PIPE1"]
    #[inline(always)]
    #[must_use]
    pub fn pipe1bempe(&mut self) -> PIPE1BEMPE_W<BEMPENB_SPEC, 1> {
        PIPE1BEMPE_W::new(self)
    }
    #[doc = "Bit 2 - BEMP Interrupt Enable for PIPE2"]
    #[inline(always)]
    #[must_use]
    pub fn pipe2bempe(&mut self) -> PIPE2BEMPE_W<BEMPENB_SPEC, 2> {
        PIPE2BEMPE_W::new(self)
    }
    #[doc = "Bit 3 - BEMP Interrupt Enable for PIPE3"]
    #[inline(always)]
    #[must_use]
    pub fn pipe3bempe(&mut self) -> PIPE3BEMPE_W<BEMPENB_SPEC, 3> {
        PIPE3BEMPE_W::new(self)
    }
    #[doc = "Bit 4 - BEMP Interrupt Enable for PIPE4"]
    #[inline(always)]
    #[must_use]
    pub fn pipe4bempe(&mut self) -> PIPE4BEMPE_W<BEMPENB_SPEC, 4> {
        PIPE4BEMPE_W::new(self)
    }
    #[doc = "Bit 5 - BEMP Interrupt Enable for PIPE5"]
    #[inline(always)]
    #[must_use]
    pub fn pipe5bempe(&mut self) -> PIPE5BEMPE_W<BEMPENB_SPEC, 5> {
        PIPE5BEMPE_W::new(self)
    }
    #[doc = "Bit 6 - BEMP Interrupt Enable for PIPE6"]
    #[inline(always)]
    #[must_use]
    pub fn pipe6bempe(&mut self) -> PIPE6BEMPE_W<BEMPENB_SPEC, 6> {
        PIPE6BEMPE_W::new(self)
    }
    #[doc = "Bit 7 - BEMP Interrupt Enable for PIPE7"]
    #[inline(always)]
    #[must_use]
    pub fn pipe7bempe(&mut self) -> PIPE7BEMPE_W<BEMPENB_SPEC, 7> {
        PIPE7BEMPE_W::new(self)
    }
    #[doc = "Bit 8 - BEMP Interrupt Enable for PIPE8"]
    #[inline(always)]
    #[must_use]
    pub fn pipe8bempe(&mut self) -> PIPE8BEMPE_W<BEMPENB_SPEC, 8> {
        PIPE8BEMPE_W::new(self)
    }
    #[doc = "Bit 9 - BEMP Interrupt Enable for PIPE9"]
    #[inline(always)]
    #[must_use]
    pub fn pipe9bempe(&mut self) -> PIPE9BEMPE_W<BEMPENB_SPEC, 9> {
        PIPE9BEMPE_W::new(self)
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
#[doc = "BEMP Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bempenb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bempenb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BEMPENB_SPEC;
impl crate::RegisterSpec for BEMPENB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bempenb::R`](R) reader structure"]
impl crate::Readable for BEMPENB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bempenb::W`](W) writer structure"]
impl crate::Writable for BEMPENB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BEMPENB to value 0"]
impl crate::Resettable for BEMPENB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
