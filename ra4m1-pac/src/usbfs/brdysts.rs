#[doc = "Register `BRDYSTS` reader"]
pub type R = crate::R<BRDYSTS_SPEC>;
#[doc = "Register `BRDYSTS` writer"]
pub type W = crate::W<BRDYSTS_SPEC>;
#[doc = "Field `PIPE0BRDY` reader - BRDY Interrupt Status for PIPE0\n\nThe field is **modified** in some way after a read operation."]
pub type PIPE0BRDY_R = crate::BitReader<PIPE0BRDY_A>;
#[doc = "BRDY Interrupt Status for PIPE0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE0BRDY_A {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<PIPE0BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE0BRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE0BRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIPE0BRDY_A {
        match self.bits {
            false => PIPE0BRDY_A::_0,
            true => PIPE0BRDY_A::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE0BRDY_A::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE0BRDY_A::_1
    }
}
#[doc = "Field `PIPE0BRDY` writer - BRDY Interrupt Status for PIPE0"]
pub type PIPE0BRDY_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, PIPE0BRDY_A>;
impl<'a, REG, const O: u8> PIPE0BRDY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE0BRDY_A::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE0BRDY_A::_1)
    }
}
#[doc = "Field `PIPE1BRDY` reader - BRDY Interrupt Status for PIPE1\n\nThe field is **modified** in some way after a read operation."]
pub type PIPE1BRDY_R = crate::BitReader<PIPE1BRDY_A>;
#[doc = "BRDY Interrupt Status for PIPE1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE1BRDY_A {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<PIPE1BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE1BRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE1BRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIPE1BRDY_A {
        match self.bits {
            false => PIPE1BRDY_A::_0,
            true => PIPE1BRDY_A::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE1BRDY_A::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE1BRDY_A::_1
    }
}
#[doc = "Field `PIPE1BRDY` writer - BRDY Interrupt Status for PIPE1"]
pub type PIPE1BRDY_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, PIPE1BRDY_A>;
impl<'a, REG, const O: u8> PIPE1BRDY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE1BRDY_A::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE1BRDY_A::_1)
    }
}
#[doc = "Field `PIPE2BRDY` reader - BRDY Interrupt Status for PIPE2\n\nThe field is **modified** in some way after a read operation."]
pub type PIPE2BRDY_R = crate::BitReader<PIPE2BRDY_A>;
#[doc = "BRDY Interrupt Status for PIPE2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE2BRDY_A {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<PIPE2BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE2BRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE2BRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIPE2BRDY_A {
        match self.bits {
            false => PIPE2BRDY_A::_0,
            true => PIPE2BRDY_A::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE2BRDY_A::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE2BRDY_A::_1
    }
}
#[doc = "Field `PIPE2BRDY` writer - BRDY Interrupt Status for PIPE2"]
pub type PIPE2BRDY_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, PIPE2BRDY_A>;
impl<'a, REG, const O: u8> PIPE2BRDY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE2BRDY_A::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE2BRDY_A::_1)
    }
}
#[doc = "Field `PIPE3BRDY` reader - BRDY Interrupt Status for PIPE3\n\nThe field is **modified** in some way after a read operation."]
pub type PIPE3BRDY_R = crate::BitReader<PIPE3BRDY_A>;
#[doc = "BRDY Interrupt Status for PIPE3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE3BRDY_A {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<PIPE3BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE3BRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE3BRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIPE3BRDY_A {
        match self.bits {
            false => PIPE3BRDY_A::_0,
            true => PIPE3BRDY_A::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE3BRDY_A::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE3BRDY_A::_1
    }
}
#[doc = "Field `PIPE3BRDY` writer - BRDY Interrupt Status for PIPE3"]
pub type PIPE3BRDY_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, PIPE3BRDY_A>;
impl<'a, REG, const O: u8> PIPE3BRDY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE3BRDY_A::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE3BRDY_A::_1)
    }
}
#[doc = "Field `PIPE4BRDY` reader - BRDY Interrupt Status for PIPE4\n\nThe field is **modified** in some way after a read operation."]
pub type PIPE4BRDY_R = crate::BitReader<PIPE4BRDY_A>;
#[doc = "BRDY Interrupt Status for PIPE4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE4BRDY_A {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<PIPE4BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE4BRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE4BRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIPE4BRDY_A {
        match self.bits {
            false => PIPE4BRDY_A::_0,
            true => PIPE4BRDY_A::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE4BRDY_A::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE4BRDY_A::_1
    }
}
#[doc = "Field `PIPE4BRDY` writer - BRDY Interrupt Status for PIPE4"]
pub type PIPE4BRDY_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, PIPE4BRDY_A>;
impl<'a, REG, const O: u8> PIPE4BRDY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE4BRDY_A::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE4BRDY_A::_1)
    }
}
#[doc = "Field `PIPE5BRDY` reader - BRDY Interrupt Status for PIPE5\n\nThe field is **modified** in some way after a read operation."]
pub type PIPE5BRDY_R = crate::BitReader<PIPE5BRDY_A>;
#[doc = "BRDY Interrupt Status for PIPE5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE5BRDY_A {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<PIPE5BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE5BRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE5BRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIPE5BRDY_A {
        match self.bits {
            false => PIPE5BRDY_A::_0,
            true => PIPE5BRDY_A::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE5BRDY_A::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE5BRDY_A::_1
    }
}
#[doc = "Field `PIPE5BRDY` writer - BRDY Interrupt Status for PIPE5"]
pub type PIPE5BRDY_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, PIPE5BRDY_A>;
impl<'a, REG, const O: u8> PIPE5BRDY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE5BRDY_A::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE5BRDY_A::_1)
    }
}
#[doc = "Field `PIPE6BRDY` reader - BRDY Interrupt Status for PIPE6\n\nThe field is **modified** in some way after a read operation."]
pub type PIPE6BRDY_R = crate::BitReader<PIPE6BRDY_A>;
#[doc = "BRDY Interrupt Status for PIPE6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE6BRDY_A {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<PIPE6BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE6BRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE6BRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIPE6BRDY_A {
        match self.bits {
            false => PIPE6BRDY_A::_0,
            true => PIPE6BRDY_A::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE6BRDY_A::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE6BRDY_A::_1
    }
}
#[doc = "Field `PIPE6BRDY` writer - BRDY Interrupt Status for PIPE6"]
pub type PIPE6BRDY_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, PIPE6BRDY_A>;
impl<'a, REG, const O: u8> PIPE6BRDY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE6BRDY_A::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE6BRDY_A::_1)
    }
}
#[doc = "Field `PIPE7BRDY` reader - BRDY Interrupt Status for PIPE7\n\nThe field is **modified** in some way after a read operation."]
pub type PIPE7BRDY_R = crate::BitReader<PIPE7BRDY_A>;
#[doc = "BRDY Interrupt Status for PIPE7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE7BRDY_A {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<PIPE7BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE7BRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE7BRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIPE7BRDY_A {
        match self.bits {
            false => PIPE7BRDY_A::_0,
            true => PIPE7BRDY_A::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE7BRDY_A::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE7BRDY_A::_1
    }
}
#[doc = "Field `PIPE7BRDY` writer - BRDY Interrupt Status for PIPE7"]
pub type PIPE7BRDY_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, PIPE7BRDY_A>;
impl<'a, REG, const O: u8> PIPE7BRDY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE7BRDY_A::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE7BRDY_A::_1)
    }
}
#[doc = "Field `PIPE8BRDY` reader - BRDY Interrupt Status for PIPE8\n\nThe field is **modified** in some way after a read operation."]
pub type PIPE8BRDY_R = crate::BitReader<PIPE8BRDY_A>;
#[doc = "BRDY Interrupt Status for PIPE8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE8BRDY_A {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<PIPE8BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE8BRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE8BRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIPE8BRDY_A {
        match self.bits {
            false => PIPE8BRDY_A::_0,
            true => PIPE8BRDY_A::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE8BRDY_A::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE8BRDY_A::_1
    }
}
#[doc = "Field `PIPE8BRDY` writer - BRDY Interrupt Status for PIPE8"]
pub type PIPE8BRDY_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, PIPE8BRDY_A>;
impl<'a, REG, const O: u8> PIPE8BRDY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE8BRDY_A::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE8BRDY_A::_1)
    }
}
#[doc = "Field `PIPE9BRDY` reader - BRDY Interrupt Status for PIPE9\n\nThe field is **modified** in some way after a read operation."]
pub type PIPE9BRDY_R = crate::BitReader<PIPE9BRDY_A>;
#[doc = "BRDY Interrupt Status for PIPE9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE9BRDY_A {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<PIPE9BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE9BRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE9BRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIPE9BRDY_A {
        match self.bits {
            false => PIPE9BRDY_A::_0,
            true => PIPE9BRDY_A::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE9BRDY_A::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE9BRDY_A::_1
    }
}
#[doc = "Field `PIPE9BRDY` writer - BRDY Interrupt Status for PIPE9"]
pub type PIPE9BRDY_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, PIPE9BRDY_A>;
impl<'a, REG, const O: u8> PIPE9BRDY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE9BRDY_A::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE9BRDY_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - BRDY Interrupt Status for PIPE0"]
    #[inline(always)]
    pub fn pipe0brdy(&self) -> PIPE0BRDY_R {
        PIPE0BRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BRDY Interrupt Status for PIPE1"]
    #[inline(always)]
    pub fn pipe1brdy(&self) -> PIPE1BRDY_R {
        PIPE1BRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BRDY Interrupt Status for PIPE2"]
    #[inline(always)]
    pub fn pipe2brdy(&self) -> PIPE2BRDY_R {
        PIPE2BRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BRDY Interrupt Status for PIPE3"]
    #[inline(always)]
    pub fn pipe3brdy(&self) -> PIPE3BRDY_R {
        PIPE3BRDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BRDY Interrupt Status for PIPE4"]
    #[inline(always)]
    pub fn pipe4brdy(&self) -> PIPE4BRDY_R {
        PIPE4BRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BRDY Interrupt Status for PIPE5"]
    #[inline(always)]
    pub fn pipe5brdy(&self) -> PIPE5BRDY_R {
        PIPE5BRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BRDY Interrupt Status for PIPE6"]
    #[inline(always)]
    pub fn pipe6brdy(&self) -> PIPE6BRDY_R {
        PIPE6BRDY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BRDY Interrupt Status for PIPE7"]
    #[inline(always)]
    pub fn pipe7brdy(&self) -> PIPE7BRDY_R {
        PIPE7BRDY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - BRDY Interrupt Status for PIPE8"]
    #[inline(always)]
    pub fn pipe8brdy(&self) -> PIPE8BRDY_R {
        PIPE8BRDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BRDY Interrupt Status for PIPE9"]
    #[inline(always)]
    pub fn pipe9brdy(&self) -> PIPE9BRDY_R {
        PIPE9BRDY_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BRDY Interrupt Status for PIPE0"]
    #[inline(always)]
    #[must_use]
    pub fn pipe0brdy(&mut self) -> PIPE0BRDY_W<BRDYSTS_SPEC, 0> {
        PIPE0BRDY_W::new(self)
    }
    #[doc = "Bit 1 - BRDY Interrupt Status for PIPE1"]
    #[inline(always)]
    #[must_use]
    pub fn pipe1brdy(&mut self) -> PIPE1BRDY_W<BRDYSTS_SPEC, 1> {
        PIPE1BRDY_W::new(self)
    }
    #[doc = "Bit 2 - BRDY Interrupt Status for PIPE2"]
    #[inline(always)]
    #[must_use]
    pub fn pipe2brdy(&mut self) -> PIPE2BRDY_W<BRDYSTS_SPEC, 2> {
        PIPE2BRDY_W::new(self)
    }
    #[doc = "Bit 3 - BRDY Interrupt Status for PIPE3"]
    #[inline(always)]
    #[must_use]
    pub fn pipe3brdy(&mut self) -> PIPE3BRDY_W<BRDYSTS_SPEC, 3> {
        PIPE3BRDY_W::new(self)
    }
    #[doc = "Bit 4 - BRDY Interrupt Status for PIPE4"]
    #[inline(always)]
    #[must_use]
    pub fn pipe4brdy(&mut self) -> PIPE4BRDY_W<BRDYSTS_SPEC, 4> {
        PIPE4BRDY_W::new(self)
    }
    #[doc = "Bit 5 - BRDY Interrupt Status for PIPE5"]
    #[inline(always)]
    #[must_use]
    pub fn pipe5brdy(&mut self) -> PIPE5BRDY_W<BRDYSTS_SPEC, 5> {
        PIPE5BRDY_W::new(self)
    }
    #[doc = "Bit 6 - BRDY Interrupt Status for PIPE6"]
    #[inline(always)]
    #[must_use]
    pub fn pipe6brdy(&mut self) -> PIPE6BRDY_W<BRDYSTS_SPEC, 6> {
        PIPE6BRDY_W::new(self)
    }
    #[doc = "Bit 7 - BRDY Interrupt Status for PIPE7"]
    #[inline(always)]
    #[must_use]
    pub fn pipe7brdy(&mut self) -> PIPE7BRDY_W<BRDYSTS_SPEC, 7> {
        PIPE7BRDY_W::new(self)
    }
    #[doc = "Bit 8 - BRDY Interrupt Status for PIPE8"]
    #[inline(always)]
    #[must_use]
    pub fn pipe8brdy(&mut self) -> PIPE8BRDY_W<BRDYSTS_SPEC, 8> {
        PIPE8BRDY_W::new(self)
    }
    #[doc = "Bit 9 - BRDY Interrupt Status for PIPE9"]
    #[inline(always)]
    #[must_use]
    pub fn pipe9brdy(&mut self) -> PIPE9BRDY_W<BRDYSTS_SPEC, 9> {
        PIPE9BRDY_W::new(self)
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
#[doc = "BRDY Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brdysts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brdysts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRDYSTS_SPEC;
impl crate::RegisterSpec for BRDYSTS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`brdysts::R`](R) reader structure"]
impl crate::Readable for BRDYSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`brdysts::W`](W) writer structure"]
impl crate::Writable for BRDYSTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x03ff;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BRDYSTS to value 0"]
impl crate::Resettable for BRDYSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
