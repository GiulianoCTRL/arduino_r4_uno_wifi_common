#[doc = "Register `BEMPSTS` reader"]
pub type R = crate::R<BEMPSTS_SPEC>;
#[doc = "Register `BEMPSTS` writer"]
pub type W = crate::W<BEMPSTS_SPEC>;
#[doc = "Field `PIPE0BEMP` reader - BEMP Interrupt Status for PIPE0\n\nThe field is **modified** in some way after a read operation."]
pub type PIPE0BEMP_R = crate::BitReader<PIPE0BEMP_A>;
#[doc = "BEMP Interrupt Status for PIPE0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE0BEMP_A {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<PIPE0BEMP_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE0BEMP_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE0BEMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIPE0BEMP_A {
        match self.bits {
            false => PIPE0BEMP_A::_0,
            true => PIPE0BEMP_A::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE0BEMP_A::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE0BEMP_A::_1
    }
}
#[doc = "Field `PIPE0BEMP` writer - BEMP Interrupt Status for PIPE0"]
pub type PIPE0BEMP_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, PIPE0BEMP_A>;
impl<'a, REG, const O: u8> PIPE0BEMP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE0BEMP_A::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE0BEMP_A::_1)
    }
}
#[doc = "Field `PIPE1BEMP` reader - BEMP Interrupt Status for PIPE1\n\nThe field is **modified** in some way after a read operation."]
pub type PIPE1BEMP_R = crate::BitReader<PIPE1BEMP_A>;
#[doc = "BEMP Interrupt Status for PIPE1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE1BEMP_A {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<PIPE1BEMP_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE1BEMP_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE1BEMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIPE1BEMP_A {
        match self.bits {
            false => PIPE1BEMP_A::_0,
            true => PIPE1BEMP_A::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE1BEMP_A::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE1BEMP_A::_1
    }
}
#[doc = "Field `PIPE1BEMP` writer - BEMP Interrupt Status for PIPE1"]
pub type PIPE1BEMP_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, PIPE1BEMP_A>;
impl<'a, REG, const O: u8> PIPE1BEMP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE1BEMP_A::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE1BEMP_A::_1)
    }
}
#[doc = "Field `PIPE2BEMP` reader - BEMP Interrupt Status for PIPE2\n\nThe field is **modified** in some way after a read operation."]
pub type PIPE2BEMP_R = crate::BitReader<PIPE2BEMP_A>;
#[doc = "BEMP Interrupt Status for PIPE2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE2BEMP_A {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<PIPE2BEMP_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE2BEMP_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE2BEMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIPE2BEMP_A {
        match self.bits {
            false => PIPE2BEMP_A::_0,
            true => PIPE2BEMP_A::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE2BEMP_A::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE2BEMP_A::_1
    }
}
#[doc = "Field `PIPE2BEMP` writer - BEMP Interrupt Status for PIPE2"]
pub type PIPE2BEMP_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, PIPE2BEMP_A>;
impl<'a, REG, const O: u8> PIPE2BEMP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE2BEMP_A::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE2BEMP_A::_1)
    }
}
#[doc = "Field `PIPE3BEMP` reader - BEMP Interrupt Status for PIPE3\n\nThe field is **modified** in some way after a read operation."]
pub type PIPE3BEMP_R = crate::BitReader<PIPE3BEMP_A>;
#[doc = "BEMP Interrupt Status for PIPE3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE3BEMP_A {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<PIPE3BEMP_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE3BEMP_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE3BEMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIPE3BEMP_A {
        match self.bits {
            false => PIPE3BEMP_A::_0,
            true => PIPE3BEMP_A::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE3BEMP_A::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE3BEMP_A::_1
    }
}
#[doc = "Field `PIPE3BEMP` writer - BEMP Interrupt Status for PIPE3"]
pub type PIPE3BEMP_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, PIPE3BEMP_A>;
impl<'a, REG, const O: u8> PIPE3BEMP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE3BEMP_A::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE3BEMP_A::_1)
    }
}
#[doc = "Field `PIPE4BEMP` reader - BEMP Interrupt Status for PIPE4\n\nThe field is **modified** in some way after a read operation."]
pub type PIPE4BEMP_R = crate::BitReader<PIPE4BEMP_A>;
#[doc = "BEMP Interrupt Status for PIPE4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE4BEMP_A {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<PIPE4BEMP_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE4BEMP_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE4BEMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIPE4BEMP_A {
        match self.bits {
            false => PIPE4BEMP_A::_0,
            true => PIPE4BEMP_A::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE4BEMP_A::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE4BEMP_A::_1
    }
}
#[doc = "Field `PIPE4BEMP` writer - BEMP Interrupt Status for PIPE4"]
pub type PIPE4BEMP_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, PIPE4BEMP_A>;
impl<'a, REG, const O: u8> PIPE4BEMP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE4BEMP_A::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE4BEMP_A::_1)
    }
}
#[doc = "Field `PIPE5BEMP` reader - BEMP Interrupt Status for PIPE5\n\nThe field is **modified** in some way after a read operation."]
pub type PIPE5BEMP_R = crate::BitReader<PIPE5BEMP_A>;
#[doc = "BEMP Interrupt Status for PIPE5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE5BEMP_A {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<PIPE5BEMP_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE5BEMP_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE5BEMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIPE5BEMP_A {
        match self.bits {
            false => PIPE5BEMP_A::_0,
            true => PIPE5BEMP_A::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE5BEMP_A::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE5BEMP_A::_1
    }
}
#[doc = "Field `PIPE5BEMP` writer - BEMP Interrupt Status for PIPE5"]
pub type PIPE5BEMP_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, PIPE5BEMP_A>;
impl<'a, REG, const O: u8> PIPE5BEMP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE5BEMP_A::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE5BEMP_A::_1)
    }
}
#[doc = "Field `PIPE6BEMP` reader - BEMP Interrupt Status for PIPE6\n\nThe field is **modified** in some way after a read operation."]
pub type PIPE6BEMP_R = crate::BitReader<PIPE6BEMP_A>;
#[doc = "BEMP Interrupt Status for PIPE6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE6BEMP_A {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<PIPE6BEMP_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE6BEMP_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE6BEMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIPE6BEMP_A {
        match self.bits {
            false => PIPE6BEMP_A::_0,
            true => PIPE6BEMP_A::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE6BEMP_A::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE6BEMP_A::_1
    }
}
#[doc = "Field `PIPE6BEMP` writer - BEMP Interrupt Status for PIPE6"]
pub type PIPE6BEMP_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, PIPE6BEMP_A>;
impl<'a, REG, const O: u8> PIPE6BEMP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE6BEMP_A::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE6BEMP_A::_1)
    }
}
#[doc = "Field `PIPE7BEMP` reader - BEMP Interrupt Status for PIPE7\n\nThe field is **modified** in some way after a read operation."]
pub type PIPE7BEMP_R = crate::BitReader<PIPE7BEMP_A>;
#[doc = "BEMP Interrupt Status for PIPE7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE7BEMP_A {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<PIPE7BEMP_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE7BEMP_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE7BEMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIPE7BEMP_A {
        match self.bits {
            false => PIPE7BEMP_A::_0,
            true => PIPE7BEMP_A::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE7BEMP_A::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE7BEMP_A::_1
    }
}
#[doc = "Field `PIPE7BEMP` writer - BEMP Interrupt Status for PIPE7"]
pub type PIPE7BEMP_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, PIPE7BEMP_A>;
impl<'a, REG, const O: u8> PIPE7BEMP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE7BEMP_A::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE7BEMP_A::_1)
    }
}
#[doc = "Field `PIPE8BEMP` reader - BEMP Interrupt Status for PIPE8\n\nThe field is **modified** in some way after a read operation."]
pub type PIPE8BEMP_R = crate::BitReader<PIPE8BEMP_A>;
#[doc = "BEMP Interrupt Status for PIPE8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE8BEMP_A {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<PIPE8BEMP_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE8BEMP_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE8BEMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIPE8BEMP_A {
        match self.bits {
            false => PIPE8BEMP_A::_0,
            true => PIPE8BEMP_A::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE8BEMP_A::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE8BEMP_A::_1
    }
}
#[doc = "Field `PIPE8BEMP` writer - BEMP Interrupt Status for PIPE8"]
pub type PIPE8BEMP_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, PIPE8BEMP_A>;
impl<'a, REG, const O: u8> PIPE8BEMP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE8BEMP_A::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE8BEMP_A::_1)
    }
}
#[doc = "Field `PIPE9BEMP` reader - BEMP Interrupt Status for PIPE9\n\nThe field is **modified** in some way after a read operation."]
pub type PIPE9BEMP_R = crate::BitReader<PIPE9BEMP_A>;
#[doc = "BEMP Interrupt Status for PIPE9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE9BEMP_A {
    #[doc = "0: Interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Interrupts are generated."]
    _1 = 1,
}
impl From<PIPE9BEMP_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE9BEMP_A) -> Self {
        variant as u8 != 0
    }
}
impl PIPE9BEMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIPE9BEMP_A {
        match self.bits {
            false => PIPE9BEMP_A::_0,
            true => PIPE9BEMP_A::_1,
        }
    }
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE9BEMP_A::_0
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE9BEMP_A::_1
    }
}
#[doc = "Field `PIPE9BEMP` writer - BEMP Interrupt Status for PIPE9"]
pub type PIPE9BEMP_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, PIPE9BEMP_A>;
impl<'a, REG, const O: u8> PIPE9BEMP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE9BEMP_A::_0)
    }
    #[doc = "Interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE9BEMP_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - BEMP Interrupt Status for PIPE0"]
    #[inline(always)]
    pub fn pipe0bemp(&self) -> PIPE0BEMP_R {
        PIPE0BEMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BEMP Interrupt Status for PIPE1"]
    #[inline(always)]
    pub fn pipe1bemp(&self) -> PIPE1BEMP_R {
        PIPE1BEMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BEMP Interrupt Status for PIPE2"]
    #[inline(always)]
    pub fn pipe2bemp(&self) -> PIPE2BEMP_R {
        PIPE2BEMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BEMP Interrupt Status for PIPE3"]
    #[inline(always)]
    pub fn pipe3bemp(&self) -> PIPE3BEMP_R {
        PIPE3BEMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BEMP Interrupt Status for PIPE4"]
    #[inline(always)]
    pub fn pipe4bemp(&self) -> PIPE4BEMP_R {
        PIPE4BEMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BEMP Interrupt Status for PIPE5"]
    #[inline(always)]
    pub fn pipe5bemp(&self) -> PIPE5BEMP_R {
        PIPE5BEMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BEMP Interrupt Status for PIPE6"]
    #[inline(always)]
    pub fn pipe6bemp(&self) -> PIPE6BEMP_R {
        PIPE6BEMP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BEMP Interrupt Status for PIPE7"]
    #[inline(always)]
    pub fn pipe7bemp(&self) -> PIPE7BEMP_R {
        PIPE7BEMP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - BEMP Interrupt Status for PIPE8"]
    #[inline(always)]
    pub fn pipe8bemp(&self) -> PIPE8BEMP_R {
        PIPE8BEMP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BEMP Interrupt Status for PIPE9"]
    #[inline(always)]
    pub fn pipe9bemp(&self) -> PIPE9BEMP_R {
        PIPE9BEMP_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BEMP Interrupt Status for PIPE0"]
    #[inline(always)]
    #[must_use]
    pub fn pipe0bemp(&mut self) -> PIPE0BEMP_W<BEMPSTS_SPEC, 0> {
        PIPE0BEMP_W::new(self)
    }
    #[doc = "Bit 1 - BEMP Interrupt Status for PIPE1"]
    #[inline(always)]
    #[must_use]
    pub fn pipe1bemp(&mut self) -> PIPE1BEMP_W<BEMPSTS_SPEC, 1> {
        PIPE1BEMP_W::new(self)
    }
    #[doc = "Bit 2 - BEMP Interrupt Status for PIPE2"]
    #[inline(always)]
    #[must_use]
    pub fn pipe2bemp(&mut self) -> PIPE2BEMP_W<BEMPSTS_SPEC, 2> {
        PIPE2BEMP_W::new(self)
    }
    #[doc = "Bit 3 - BEMP Interrupt Status for PIPE3"]
    #[inline(always)]
    #[must_use]
    pub fn pipe3bemp(&mut self) -> PIPE3BEMP_W<BEMPSTS_SPEC, 3> {
        PIPE3BEMP_W::new(self)
    }
    #[doc = "Bit 4 - BEMP Interrupt Status for PIPE4"]
    #[inline(always)]
    #[must_use]
    pub fn pipe4bemp(&mut self) -> PIPE4BEMP_W<BEMPSTS_SPEC, 4> {
        PIPE4BEMP_W::new(self)
    }
    #[doc = "Bit 5 - BEMP Interrupt Status for PIPE5"]
    #[inline(always)]
    #[must_use]
    pub fn pipe5bemp(&mut self) -> PIPE5BEMP_W<BEMPSTS_SPEC, 5> {
        PIPE5BEMP_W::new(self)
    }
    #[doc = "Bit 6 - BEMP Interrupt Status for PIPE6"]
    #[inline(always)]
    #[must_use]
    pub fn pipe6bemp(&mut self) -> PIPE6BEMP_W<BEMPSTS_SPEC, 6> {
        PIPE6BEMP_W::new(self)
    }
    #[doc = "Bit 7 - BEMP Interrupt Status for PIPE7"]
    #[inline(always)]
    #[must_use]
    pub fn pipe7bemp(&mut self) -> PIPE7BEMP_W<BEMPSTS_SPEC, 7> {
        PIPE7BEMP_W::new(self)
    }
    #[doc = "Bit 8 - BEMP Interrupt Status for PIPE8"]
    #[inline(always)]
    #[must_use]
    pub fn pipe8bemp(&mut self) -> PIPE8BEMP_W<BEMPSTS_SPEC, 8> {
        PIPE8BEMP_W::new(self)
    }
    #[doc = "Bit 9 - BEMP Interrupt Status for PIPE9"]
    #[inline(always)]
    #[must_use]
    pub fn pipe9bemp(&mut self) -> PIPE9BEMP_W<BEMPSTS_SPEC, 9> {
        PIPE9BEMP_W::new(self)
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
#[doc = "BEMP Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bempsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bempsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BEMPSTS_SPEC;
impl crate::RegisterSpec for BEMPSTS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bempsts::R`](R) reader structure"]
impl crate::Readable for BEMPSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bempsts::W`](W) writer structure"]
impl crate::Writable for BEMPSTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x03ff;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BEMPSTS to value 0"]
impl crate::Resettable for BEMPSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
