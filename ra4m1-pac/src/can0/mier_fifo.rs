#[doc = "Register `MIER_FIFO` reader"]
pub type R = crate::R<MIER_FIFO_SPEC>;
#[doc = "Register `MIER_FIFO` writer"]
pub type W = crate::W<MIER_FIFO_SPEC>;
#[doc = "Field `MB0` reader - mailbox 0 Interrupt Enable"]
pub type MB0_R = crate::BitReader<MB0_A>;
#[doc = "mailbox 0 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB0_A {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<MB0_A> for bool {
    #[inline(always)]
    fn from(variant: MB0_A) -> Self {
        variant as u8 != 0
    }
}
impl MB0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MB0_A {
        match self.bits {
            false => MB0_A::_0,
            true => MB0_A::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB0_A::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB0_A::_1
    }
}
#[doc = "Field `MB0` writer - mailbox 0 Interrupt Enable"]
pub type MB0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MB0_A>;
impl<'a, REG, const O: u8> MB0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB0_A::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB0_A::_1)
    }
}
#[doc = "Field `MB1` reader - mailbox 1 Interrupt Enable"]
pub type MB1_R = crate::BitReader<MB1_A>;
#[doc = "mailbox 1 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB1_A {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<MB1_A> for bool {
    #[inline(always)]
    fn from(variant: MB1_A) -> Self {
        variant as u8 != 0
    }
}
impl MB1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MB1_A {
        match self.bits {
            false => MB1_A::_0,
            true => MB1_A::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB1_A::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB1_A::_1
    }
}
#[doc = "Field `MB1` writer - mailbox 1 Interrupt Enable"]
pub type MB1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MB1_A>;
impl<'a, REG, const O: u8> MB1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB1_A::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB1_A::_1)
    }
}
#[doc = "Field `MB2` reader - mailbox 2 Interrupt Enable"]
pub type MB2_R = crate::BitReader<MB2_A>;
#[doc = "mailbox 2 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB2_A {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<MB2_A> for bool {
    #[inline(always)]
    fn from(variant: MB2_A) -> Self {
        variant as u8 != 0
    }
}
impl MB2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MB2_A {
        match self.bits {
            false => MB2_A::_0,
            true => MB2_A::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB2_A::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB2_A::_1
    }
}
#[doc = "Field `MB2` writer - mailbox 2 Interrupt Enable"]
pub type MB2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MB2_A>;
impl<'a, REG, const O: u8> MB2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB2_A::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB2_A::_1)
    }
}
#[doc = "Field `MB3` reader - mailbox 3 Interrupt Enable"]
pub type MB3_R = crate::BitReader<MB3_A>;
#[doc = "mailbox 3 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB3_A {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<MB3_A> for bool {
    #[inline(always)]
    fn from(variant: MB3_A) -> Self {
        variant as u8 != 0
    }
}
impl MB3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MB3_A {
        match self.bits {
            false => MB3_A::_0,
            true => MB3_A::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB3_A::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB3_A::_1
    }
}
#[doc = "Field `MB3` writer - mailbox 3 Interrupt Enable"]
pub type MB3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MB3_A>;
impl<'a, REG, const O: u8> MB3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB3_A::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB3_A::_1)
    }
}
#[doc = "Field `MB4` reader - mailbox 4 Interrupt Enable"]
pub type MB4_R = crate::BitReader<MB4_A>;
#[doc = "mailbox 4 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB4_A {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<MB4_A> for bool {
    #[inline(always)]
    fn from(variant: MB4_A) -> Self {
        variant as u8 != 0
    }
}
impl MB4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MB4_A {
        match self.bits {
            false => MB4_A::_0,
            true => MB4_A::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB4_A::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB4_A::_1
    }
}
#[doc = "Field `MB4` writer - mailbox 4 Interrupt Enable"]
pub type MB4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MB4_A>;
impl<'a, REG, const O: u8> MB4_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB4_A::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB4_A::_1)
    }
}
#[doc = "Field `MB5` reader - mailbox 5 Interrupt Enable"]
pub type MB5_R = crate::BitReader<MB5_A>;
#[doc = "mailbox 5 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB5_A {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<MB5_A> for bool {
    #[inline(always)]
    fn from(variant: MB5_A) -> Self {
        variant as u8 != 0
    }
}
impl MB5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MB5_A {
        match self.bits {
            false => MB5_A::_0,
            true => MB5_A::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB5_A::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB5_A::_1
    }
}
#[doc = "Field `MB5` writer - mailbox 5 Interrupt Enable"]
pub type MB5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MB5_A>;
impl<'a, REG, const O: u8> MB5_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB5_A::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB5_A::_1)
    }
}
#[doc = "Field `MB6` reader - mailbox 6 Interrupt Enable"]
pub type MB6_R = crate::BitReader<MB6_A>;
#[doc = "mailbox 6 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB6_A {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<MB6_A> for bool {
    #[inline(always)]
    fn from(variant: MB6_A) -> Self {
        variant as u8 != 0
    }
}
impl MB6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MB6_A {
        match self.bits {
            false => MB6_A::_0,
            true => MB6_A::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB6_A::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB6_A::_1
    }
}
#[doc = "Field `MB6` writer - mailbox 6 Interrupt Enable"]
pub type MB6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MB6_A>;
impl<'a, REG, const O: u8> MB6_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB6_A::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB6_A::_1)
    }
}
#[doc = "Field `MB7` reader - mailbox 7 Interrupt Enable"]
pub type MB7_R = crate::BitReader<MB7_A>;
#[doc = "mailbox 7 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB7_A {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<MB7_A> for bool {
    #[inline(always)]
    fn from(variant: MB7_A) -> Self {
        variant as u8 != 0
    }
}
impl MB7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MB7_A {
        match self.bits {
            false => MB7_A::_0,
            true => MB7_A::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB7_A::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB7_A::_1
    }
}
#[doc = "Field `MB7` writer - mailbox 7 Interrupt Enable"]
pub type MB7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MB7_A>;
impl<'a, REG, const O: u8> MB7_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB7_A::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB7_A::_1)
    }
}
#[doc = "Field `MB8` reader - mailbox 8 Interrupt Enable"]
pub type MB8_R = crate::BitReader<MB8_A>;
#[doc = "mailbox 8 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB8_A {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<MB8_A> for bool {
    #[inline(always)]
    fn from(variant: MB8_A) -> Self {
        variant as u8 != 0
    }
}
impl MB8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MB8_A {
        match self.bits {
            false => MB8_A::_0,
            true => MB8_A::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB8_A::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB8_A::_1
    }
}
#[doc = "Field `MB8` writer - mailbox 8 Interrupt Enable"]
pub type MB8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MB8_A>;
impl<'a, REG, const O: u8> MB8_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB8_A::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB8_A::_1)
    }
}
#[doc = "Field `MB9` reader - mailbox 9 Interrupt Enable"]
pub type MB9_R = crate::BitReader<MB9_A>;
#[doc = "mailbox 9 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB9_A {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<MB9_A> for bool {
    #[inline(always)]
    fn from(variant: MB9_A) -> Self {
        variant as u8 != 0
    }
}
impl MB9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MB9_A {
        match self.bits {
            false => MB9_A::_0,
            true => MB9_A::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB9_A::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB9_A::_1
    }
}
#[doc = "Field `MB9` writer - mailbox 9 Interrupt Enable"]
pub type MB9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MB9_A>;
impl<'a, REG, const O: u8> MB9_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB9_A::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB9_A::_1)
    }
}
#[doc = "Field `MB10` reader - mailbox 10 Interrupt Enable"]
pub type MB10_R = crate::BitReader<MB10_A>;
#[doc = "mailbox 10 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB10_A {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<MB10_A> for bool {
    #[inline(always)]
    fn from(variant: MB10_A) -> Self {
        variant as u8 != 0
    }
}
impl MB10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MB10_A {
        match self.bits {
            false => MB10_A::_0,
            true => MB10_A::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB10_A::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB10_A::_1
    }
}
#[doc = "Field `MB10` writer - mailbox 10 Interrupt Enable"]
pub type MB10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MB10_A>;
impl<'a, REG, const O: u8> MB10_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB10_A::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB10_A::_1)
    }
}
#[doc = "Field `MB11` reader - mailbox 11 Interrupt Enable"]
pub type MB11_R = crate::BitReader<MB11_A>;
#[doc = "mailbox 11 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB11_A {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<MB11_A> for bool {
    #[inline(always)]
    fn from(variant: MB11_A) -> Self {
        variant as u8 != 0
    }
}
impl MB11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MB11_A {
        match self.bits {
            false => MB11_A::_0,
            true => MB11_A::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB11_A::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB11_A::_1
    }
}
#[doc = "Field `MB11` writer - mailbox 11 Interrupt Enable"]
pub type MB11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MB11_A>;
impl<'a, REG, const O: u8> MB11_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB11_A::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB11_A::_1)
    }
}
#[doc = "Field `MB12` reader - mailbox 12 Interrupt Enable"]
pub type MB12_R = crate::BitReader<MB12_A>;
#[doc = "mailbox 12 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB12_A {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<MB12_A> for bool {
    #[inline(always)]
    fn from(variant: MB12_A) -> Self {
        variant as u8 != 0
    }
}
impl MB12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MB12_A {
        match self.bits {
            false => MB12_A::_0,
            true => MB12_A::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB12_A::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB12_A::_1
    }
}
#[doc = "Field `MB12` writer - mailbox 12 Interrupt Enable"]
pub type MB12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MB12_A>;
impl<'a, REG, const O: u8> MB12_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB12_A::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB12_A::_1)
    }
}
#[doc = "Field `MB13` reader - mailbox 13 Interrupt Enable"]
pub type MB13_R = crate::BitReader<MB13_A>;
#[doc = "mailbox 13 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB13_A {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<MB13_A> for bool {
    #[inline(always)]
    fn from(variant: MB13_A) -> Self {
        variant as u8 != 0
    }
}
impl MB13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MB13_A {
        match self.bits {
            false => MB13_A::_0,
            true => MB13_A::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB13_A::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB13_A::_1
    }
}
#[doc = "Field `MB13` writer - mailbox 13 Interrupt Enable"]
pub type MB13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MB13_A>;
impl<'a, REG, const O: u8> MB13_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB13_A::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB13_A::_1)
    }
}
#[doc = "Field `MB14` reader - mailbox 14 Interrupt Enable"]
pub type MB14_R = crate::BitReader<MB14_A>;
#[doc = "mailbox 14 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB14_A {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<MB14_A> for bool {
    #[inline(always)]
    fn from(variant: MB14_A) -> Self {
        variant as u8 != 0
    }
}
impl MB14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MB14_A {
        match self.bits {
            false => MB14_A::_0,
            true => MB14_A::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB14_A::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB14_A::_1
    }
}
#[doc = "Field `MB14` writer - mailbox 14 Interrupt Enable"]
pub type MB14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MB14_A>;
impl<'a, REG, const O: u8> MB14_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB14_A::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB14_A::_1)
    }
}
#[doc = "Field `MB15` reader - mailbox 15 Interrupt Enable"]
pub type MB15_R = crate::BitReader<MB15_A>;
#[doc = "mailbox 15 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB15_A {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<MB15_A> for bool {
    #[inline(always)]
    fn from(variant: MB15_A) -> Self {
        variant as u8 != 0
    }
}
impl MB15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MB15_A {
        match self.bits {
            false => MB15_A::_0,
            true => MB15_A::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB15_A::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB15_A::_1
    }
}
#[doc = "Field `MB15` writer - mailbox 15 Interrupt Enable"]
pub type MB15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MB15_A>;
impl<'a, REG, const O: u8> MB15_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB15_A::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB15_A::_1)
    }
}
#[doc = "Field `MB16` reader - mailbox 16 Interrupt Enable"]
pub type MB16_R = crate::BitReader<MB16_A>;
#[doc = "mailbox 16 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB16_A {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<MB16_A> for bool {
    #[inline(always)]
    fn from(variant: MB16_A) -> Self {
        variant as u8 != 0
    }
}
impl MB16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MB16_A {
        match self.bits {
            false => MB16_A::_0,
            true => MB16_A::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB16_A::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB16_A::_1
    }
}
#[doc = "Field `MB16` writer - mailbox 16 Interrupt Enable"]
pub type MB16_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MB16_A>;
impl<'a, REG, const O: u8> MB16_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB16_A::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB16_A::_1)
    }
}
#[doc = "Field `MB17` reader - mailbox 17 Interrupt Enable"]
pub type MB17_R = crate::BitReader<MB17_A>;
#[doc = "mailbox 17 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB17_A {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<MB17_A> for bool {
    #[inline(always)]
    fn from(variant: MB17_A) -> Self {
        variant as u8 != 0
    }
}
impl MB17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MB17_A {
        match self.bits {
            false => MB17_A::_0,
            true => MB17_A::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB17_A::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB17_A::_1
    }
}
#[doc = "Field `MB17` writer - mailbox 17 Interrupt Enable"]
pub type MB17_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MB17_A>;
impl<'a, REG, const O: u8> MB17_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB17_A::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB17_A::_1)
    }
}
#[doc = "Field `MB18` reader - mailbox 18 Interrupt Enable"]
pub type MB18_R = crate::BitReader<MB18_A>;
#[doc = "mailbox 18 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB18_A {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<MB18_A> for bool {
    #[inline(always)]
    fn from(variant: MB18_A) -> Self {
        variant as u8 != 0
    }
}
impl MB18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MB18_A {
        match self.bits {
            false => MB18_A::_0,
            true => MB18_A::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB18_A::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB18_A::_1
    }
}
#[doc = "Field `MB18` writer - mailbox 18 Interrupt Enable"]
pub type MB18_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MB18_A>;
impl<'a, REG, const O: u8> MB18_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB18_A::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB18_A::_1)
    }
}
#[doc = "Field `MB19` reader - mailbox 19 Interrupt Enable"]
pub type MB19_R = crate::BitReader<MB19_A>;
#[doc = "mailbox 19 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB19_A {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<MB19_A> for bool {
    #[inline(always)]
    fn from(variant: MB19_A) -> Self {
        variant as u8 != 0
    }
}
impl MB19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MB19_A {
        match self.bits {
            false => MB19_A::_0,
            true => MB19_A::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB19_A::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB19_A::_1
    }
}
#[doc = "Field `MB19` writer - mailbox 19 Interrupt Enable"]
pub type MB19_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MB19_A>;
impl<'a, REG, const O: u8> MB19_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB19_A::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB19_A::_1)
    }
}
#[doc = "Field `MB20` reader - mailbox 20 Interrupt Enable"]
pub type MB20_R = crate::BitReader<MB20_A>;
#[doc = "mailbox 20 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB20_A {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<MB20_A> for bool {
    #[inline(always)]
    fn from(variant: MB20_A) -> Self {
        variant as u8 != 0
    }
}
impl MB20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MB20_A {
        match self.bits {
            false => MB20_A::_0,
            true => MB20_A::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB20_A::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB20_A::_1
    }
}
#[doc = "Field `MB20` writer - mailbox 20 Interrupt Enable"]
pub type MB20_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MB20_A>;
impl<'a, REG, const O: u8> MB20_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB20_A::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB20_A::_1)
    }
}
#[doc = "Field `MB21` reader - mailbox 21 Interrupt Enable"]
pub type MB21_R = crate::BitReader<MB21_A>;
#[doc = "mailbox 21 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB21_A {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<MB21_A> for bool {
    #[inline(always)]
    fn from(variant: MB21_A) -> Self {
        variant as u8 != 0
    }
}
impl MB21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MB21_A {
        match self.bits {
            false => MB21_A::_0,
            true => MB21_A::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB21_A::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB21_A::_1
    }
}
#[doc = "Field `MB21` writer - mailbox 21 Interrupt Enable"]
pub type MB21_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MB21_A>;
impl<'a, REG, const O: u8> MB21_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB21_A::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB21_A::_1)
    }
}
#[doc = "Field `MB22` reader - mailbox 22 Interrupt Enable"]
pub type MB22_R = crate::BitReader<MB22_A>;
#[doc = "mailbox 22 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB22_A {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<MB22_A> for bool {
    #[inline(always)]
    fn from(variant: MB22_A) -> Self {
        variant as u8 != 0
    }
}
impl MB22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MB22_A {
        match self.bits {
            false => MB22_A::_0,
            true => MB22_A::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB22_A::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB22_A::_1
    }
}
#[doc = "Field `MB22` writer - mailbox 22 Interrupt Enable"]
pub type MB22_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MB22_A>;
impl<'a, REG, const O: u8> MB22_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB22_A::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB22_A::_1)
    }
}
#[doc = "Field `MB23` reader - mailbox 23 Interrupt Enable"]
pub type MB23_R = crate::BitReader<MB23_A>;
#[doc = "mailbox 23 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB23_A {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<MB23_A> for bool {
    #[inline(always)]
    fn from(variant: MB23_A) -> Self {
        variant as u8 != 0
    }
}
impl MB23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MB23_A {
        match self.bits {
            false => MB23_A::_0,
            true => MB23_A::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB23_A::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB23_A::_1
    }
}
#[doc = "Field `MB23` writer - mailbox 23 Interrupt Enable"]
pub type MB23_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MB23_A>;
impl<'a, REG, const O: u8> MB23_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB23_A::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB23_A::_1)
    }
}
#[doc = "Field `MB24` reader - Transmit FIFO Interrupt Enable"]
pub type MB24_R = crate::BitReader<MB24_A>;
#[doc = "Transmit FIFO Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB24_A {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<MB24_A> for bool {
    #[inline(always)]
    fn from(variant: MB24_A) -> Self {
        variant as u8 != 0
    }
}
impl MB24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MB24_A {
        match self.bits {
            false => MB24_A::_0,
            true => MB24_A::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB24_A::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB24_A::_1
    }
}
#[doc = "Field `MB24` writer - Transmit FIFO Interrupt Enable"]
pub type MB24_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MB24_A>;
impl<'a, REG, const O: u8> MB24_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB24_A::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB24_A::_1)
    }
}
#[doc = "Field `MB25` reader - Transmit FIFO Interrupt Generation Timing Control"]
pub type MB25_R = crate::BitReader<MB25_A>;
#[doc = "Transmit FIFO Interrupt Generation Timing Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB25_A {
    #[doc = "0: Every time transmission is completed"]
    _0 = 0,
    #[doc = "1: When the transmit FIFO becomes empty due to completion of transmission"]
    _1 = 1,
}
impl From<MB25_A> for bool {
    #[inline(always)]
    fn from(variant: MB25_A) -> Self {
        variant as u8 != 0
    }
}
impl MB25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MB25_A {
        match self.bits {
            false => MB25_A::_0,
            true => MB25_A::_1,
        }
    }
    #[doc = "Every time transmission is completed"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB25_A::_0
    }
    #[doc = "When the transmit FIFO becomes empty due to completion of transmission"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB25_A::_1
    }
}
#[doc = "Field `MB25` writer - Transmit FIFO Interrupt Generation Timing Control"]
pub type MB25_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MB25_A>;
impl<'a, REG, const O: u8> MB25_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Every time transmission is completed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB25_A::_0)
    }
    #[doc = "When the transmit FIFO becomes empty due to completion of transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB25_A::_1)
    }
}
#[doc = "Field `MB28` reader - Receive FIFO Interrupt Enable"]
pub type MB28_R = crate::BitReader<MB28_A>;
#[doc = "Receive FIFO Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB28_A {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<MB28_A> for bool {
    #[inline(always)]
    fn from(variant: MB28_A) -> Self {
        variant as u8 != 0
    }
}
impl MB28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MB28_A {
        match self.bits {
            false => MB28_A::_0,
            true => MB28_A::_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB28_A::_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB28_A::_1
    }
}
#[doc = "Field `MB28` writer - Receive FIFO Interrupt Enable"]
pub type MB28_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MB28_A>;
impl<'a, REG, const O: u8> MB28_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB28_A::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB28_A::_1)
    }
}
#[doc = "Field `MB29` reader - Receive FIFO Interrupt Generation Timing Control"]
pub type MB29_R = crate::BitReader<MB29_A>;
#[doc = "Receive FIFO Interrupt Generation Timing Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB29_A {
    #[doc = "0: Every time reception is completed"]
    _0 = 0,
    #[doc = "1: When the receive FIFO becomes buffer warning by completion of reception"]
    _1 = 1,
}
impl From<MB29_A> for bool {
    #[inline(always)]
    fn from(variant: MB29_A) -> Self {
        variant as u8 != 0
    }
}
impl MB29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MB29_A {
        match self.bits {
            false => MB29_A::_0,
            true => MB29_A::_1,
        }
    }
    #[doc = "Every time reception is completed"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB29_A::_0
    }
    #[doc = "When the receive FIFO becomes buffer warning by completion of reception"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB29_A::_1
    }
}
#[doc = "Field `MB29` writer - Receive FIFO Interrupt Generation Timing Control"]
pub type MB29_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MB29_A>;
impl<'a, REG, const O: u8> MB29_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Every time reception is completed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB29_A::_0)
    }
    #[doc = "When the receive FIFO becomes buffer warning by completion of reception"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB29_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - mailbox 0 Interrupt Enable"]
    #[inline(always)]
    pub fn mb0(&self) -> MB0_R {
        MB0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - mailbox 1 Interrupt Enable"]
    #[inline(always)]
    pub fn mb1(&self) -> MB1_R {
        MB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - mailbox 2 Interrupt Enable"]
    #[inline(always)]
    pub fn mb2(&self) -> MB2_R {
        MB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - mailbox 3 Interrupt Enable"]
    #[inline(always)]
    pub fn mb3(&self) -> MB3_R {
        MB3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - mailbox 4 Interrupt Enable"]
    #[inline(always)]
    pub fn mb4(&self) -> MB4_R {
        MB4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - mailbox 5 Interrupt Enable"]
    #[inline(always)]
    pub fn mb5(&self) -> MB5_R {
        MB5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - mailbox 6 Interrupt Enable"]
    #[inline(always)]
    pub fn mb6(&self) -> MB6_R {
        MB6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - mailbox 7 Interrupt Enable"]
    #[inline(always)]
    pub fn mb7(&self) -> MB7_R {
        MB7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - mailbox 8 Interrupt Enable"]
    #[inline(always)]
    pub fn mb8(&self) -> MB8_R {
        MB8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - mailbox 9 Interrupt Enable"]
    #[inline(always)]
    pub fn mb9(&self) -> MB9_R {
        MB9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - mailbox 10 Interrupt Enable"]
    #[inline(always)]
    pub fn mb10(&self) -> MB10_R {
        MB10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - mailbox 11 Interrupt Enable"]
    #[inline(always)]
    pub fn mb11(&self) -> MB11_R {
        MB11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - mailbox 12 Interrupt Enable"]
    #[inline(always)]
    pub fn mb12(&self) -> MB12_R {
        MB12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - mailbox 13 Interrupt Enable"]
    #[inline(always)]
    pub fn mb13(&self) -> MB13_R {
        MB13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - mailbox 14 Interrupt Enable"]
    #[inline(always)]
    pub fn mb14(&self) -> MB14_R {
        MB14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - mailbox 15 Interrupt Enable"]
    #[inline(always)]
    pub fn mb15(&self) -> MB15_R {
        MB15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - mailbox 16 Interrupt Enable"]
    #[inline(always)]
    pub fn mb16(&self) -> MB16_R {
        MB16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - mailbox 17 Interrupt Enable"]
    #[inline(always)]
    pub fn mb17(&self) -> MB17_R {
        MB17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - mailbox 18 Interrupt Enable"]
    #[inline(always)]
    pub fn mb18(&self) -> MB18_R {
        MB18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - mailbox 19 Interrupt Enable"]
    #[inline(always)]
    pub fn mb19(&self) -> MB19_R {
        MB19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - mailbox 20 Interrupt Enable"]
    #[inline(always)]
    pub fn mb20(&self) -> MB20_R {
        MB20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - mailbox 21 Interrupt Enable"]
    #[inline(always)]
    pub fn mb21(&self) -> MB21_R {
        MB21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - mailbox 22 Interrupt Enable"]
    #[inline(always)]
    pub fn mb22(&self) -> MB22_R {
        MB22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - mailbox 23 Interrupt Enable"]
    #[inline(always)]
    pub fn mb23(&self) -> MB23_R {
        MB23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Transmit FIFO Interrupt Enable"]
    #[inline(always)]
    pub fn mb24(&self) -> MB24_R {
        MB24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Transmit FIFO Interrupt Generation Timing Control"]
    #[inline(always)]
    pub fn mb25(&self) -> MB25_R {
        MB25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Receive FIFO Interrupt Enable"]
    #[inline(always)]
    pub fn mb28(&self) -> MB28_R {
        MB28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Receive FIFO Interrupt Generation Timing Control"]
    #[inline(always)]
    pub fn mb29(&self) -> MB29_R {
        MB29_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - mailbox 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb0(&mut self) -> MB0_W<MIER_FIFO_SPEC, 0> {
        MB0_W::new(self)
    }
    #[doc = "Bit 1 - mailbox 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb1(&mut self) -> MB1_W<MIER_FIFO_SPEC, 1> {
        MB1_W::new(self)
    }
    #[doc = "Bit 2 - mailbox 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb2(&mut self) -> MB2_W<MIER_FIFO_SPEC, 2> {
        MB2_W::new(self)
    }
    #[doc = "Bit 3 - mailbox 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb3(&mut self) -> MB3_W<MIER_FIFO_SPEC, 3> {
        MB3_W::new(self)
    }
    #[doc = "Bit 4 - mailbox 4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb4(&mut self) -> MB4_W<MIER_FIFO_SPEC, 4> {
        MB4_W::new(self)
    }
    #[doc = "Bit 5 - mailbox 5 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb5(&mut self) -> MB5_W<MIER_FIFO_SPEC, 5> {
        MB5_W::new(self)
    }
    #[doc = "Bit 6 - mailbox 6 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb6(&mut self) -> MB6_W<MIER_FIFO_SPEC, 6> {
        MB6_W::new(self)
    }
    #[doc = "Bit 7 - mailbox 7 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb7(&mut self) -> MB7_W<MIER_FIFO_SPEC, 7> {
        MB7_W::new(self)
    }
    #[doc = "Bit 8 - mailbox 8 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb8(&mut self) -> MB8_W<MIER_FIFO_SPEC, 8> {
        MB8_W::new(self)
    }
    #[doc = "Bit 9 - mailbox 9 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb9(&mut self) -> MB9_W<MIER_FIFO_SPEC, 9> {
        MB9_W::new(self)
    }
    #[doc = "Bit 10 - mailbox 10 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb10(&mut self) -> MB10_W<MIER_FIFO_SPEC, 10> {
        MB10_W::new(self)
    }
    #[doc = "Bit 11 - mailbox 11 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb11(&mut self) -> MB11_W<MIER_FIFO_SPEC, 11> {
        MB11_W::new(self)
    }
    #[doc = "Bit 12 - mailbox 12 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb12(&mut self) -> MB12_W<MIER_FIFO_SPEC, 12> {
        MB12_W::new(self)
    }
    #[doc = "Bit 13 - mailbox 13 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb13(&mut self) -> MB13_W<MIER_FIFO_SPEC, 13> {
        MB13_W::new(self)
    }
    #[doc = "Bit 14 - mailbox 14 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb14(&mut self) -> MB14_W<MIER_FIFO_SPEC, 14> {
        MB14_W::new(self)
    }
    #[doc = "Bit 15 - mailbox 15 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb15(&mut self) -> MB15_W<MIER_FIFO_SPEC, 15> {
        MB15_W::new(self)
    }
    #[doc = "Bit 16 - mailbox 16 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb16(&mut self) -> MB16_W<MIER_FIFO_SPEC, 16> {
        MB16_W::new(self)
    }
    #[doc = "Bit 17 - mailbox 17 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb17(&mut self) -> MB17_W<MIER_FIFO_SPEC, 17> {
        MB17_W::new(self)
    }
    #[doc = "Bit 18 - mailbox 18 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb18(&mut self) -> MB18_W<MIER_FIFO_SPEC, 18> {
        MB18_W::new(self)
    }
    #[doc = "Bit 19 - mailbox 19 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb19(&mut self) -> MB19_W<MIER_FIFO_SPEC, 19> {
        MB19_W::new(self)
    }
    #[doc = "Bit 20 - mailbox 20 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb20(&mut self) -> MB20_W<MIER_FIFO_SPEC, 20> {
        MB20_W::new(self)
    }
    #[doc = "Bit 21 - mailbox 21 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb21(&mut self) -> MB21_W<MIER_FIFO_SPEC, 21> {
        MB21_W::new(self)
    }
    #[doc = "Bit 22 - mailbox 22 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb22(&mut self) -> MB22_W<MIER_FIFO_SPEC, 22> {
        MB22_W::new(self)
    }
    #[doc = "Bit 23 - mailbox 23 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb23(&mut self) -> MB23_W<MIER_FIFO_SPEC, 23> {
        MB23_W::new(self)
    }
    #[doc = "Bit 24 - Transmit FIFO Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb24(&mut self) -> MB24_W<MIER_FIFO_SPEC, 24> {
        MB24_W::new(self)
    }
    #[doc = "Bit 25 - Transmit FIFO Interrupt Generation Timing Control"]
    #[inline(always)]
    #[must_use]
    pub fn mb25(&mut self) -> MB25_W<MIER_FIFO_SPEC, 25> {
        MB25_W::new(self)
    }
    #[doc = "Bit 28 - Receive FIFO Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mb28(&mut self) -> MB28_W<MIER_FIFO_SPEC, 28> {
        MB28_W::new(self)
    }
    #[doc = "Bit 29 - Receive FIFO Interrupt Generation Timing Control"]
    #[inline(always)]
    #[must_use]
    pub fn mb29(&mut self) -> MB29_W<MIER_FIFO_SPEC, 29> {
        MB29_W::new(self)
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
#[doc = "Mailbox Interrupt Enable Register for FIFO Mailbox Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mier_fifo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mier_fifo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIER_FIFO_SPEC;
impl crate::RegisterSpec for MIER_FIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mier_fifo::R`](R) reader structure"]
impl crate::Readable for MIER_FIFO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mier_fifo::W`](W) writer structure"]
impl crate::Writable for MIER_FIFO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIER_FIFO to value 0"]
impl crate::Resettable for MIER_FIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
