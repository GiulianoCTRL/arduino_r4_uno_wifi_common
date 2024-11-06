#[doc = "Register `ADANSB0` reader"]
pub type R = crate::R<ADANSB0_SPEC>;
#[doc = "Register `ADANSB0` writer"]
pub type W = crate::W<ADANSB0_SPEC>;
#[doc = "Field `ANSB00` reader - AN000 Select"]
pub type ANSB00_R = crate::BitReader<ANSB00_A>;
#[doc = "AN000 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB00_A {
    #[doc = "0: AN000 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN000 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB00_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB00_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ANSB00_A {
        match self.bits {
            false => ANSB00_A::_0,
            true => ANSB00_A::_1,
        }
    }
    #[doc = "AN000 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB00_A::_0
    }
    #[doc = "AN000 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB00_A::_1
    }
}
#[doc = "Field `ANSB00` writer - AN000 Select"]
pub type ANSB00_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ANSB00_A>;
impl<'a, REG, const O: u8> ANSB00_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN000 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB00_A::_0)
    }
    #[doc = "AN000 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB00_A::_1)
    }
}
#[doc = "Field `ANSB01` reader - AN001 Select"]
pub type ANSB01_R = crate::BitReader<ANSB01_A>;
#[doc = "AN001 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB01_A {
    #[doc = "0: AN001 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN001 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB01_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB01_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ANSB01_A {
        match self.bits {
            false => ANSB01_A::_0,
            true => ANSB01_A::_1,
        }
    }
    #[doc = "AN001 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB01_A::_0
    }
    #[doc = "AN001 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB01_A::_1
    }
}
#[doc = "Field `ANSB01` writer - AN001 Select"]
pub type ANSB01_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ANSB01_A>;
impl<'a, REG, const O: u8> ANSB01_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN001 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB01_A::_0)
    }
    #[doc = "AN001 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB01_A::_1)
    }
}
#[doc = "Field `ANSB02` reader - AN002 Select"]
pub type ANSB02_R = crate::BitReader<ANSB02_A>;
#[doc = "AN002 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB02_A {
    #[doc = "0: AN002 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN002 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB02_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB02_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB02_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ANSB02_A {
        match self.bits {
            false => ANSB02_A::_0,
            true => ANSB02_A::_1,
        }
    }
    #[doc = "AN002 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB02_A::_0
    }
    #[doc = "AN002 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB02_A::_1
    }
}
#[doc = "Field `ANSB02` writer - AN002 Select"]
pub type ANSB02_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ANSB02_A>;
impl<'a, REG, const O: u8> ANSB02_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN002 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB02_A::_0)
    }
    #[doc = "AN002 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB02_A::_1)
    }
}
#[doc = "Field `ANSB03` reader - AN003 Select"]
pub type ANSB03_R = crate::BitReader<ANSB03_A>;
#[doc = "AN003 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB03_A {
    #[doc = "0: AN003 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN003 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB03_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB03_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB03_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ANSB03_A {
        match self.bits {
            false => ANSB03_A::_0,
            true => ANSB03_A::_1,
        }
    }
    #[doc = "AN003 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB03_A::_0
    }
    #[doc = "AN003 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB03_A::_1
    }
}
#[doc = "Field `ANSB03` writer - AN003 Select"]
pub type ANSB03_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ANSB03_A>;
impl<'a, REG, const O: u8> ANSB03_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN003 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB03_A::_0)
    }
    #[doc = "AN003 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB03_A::_1)
    }
}
#[doc = "Field `ANSB04` reader - AN004 Select"]
pub type ANSB04_R = crate::BitReader<ANSB04_A>;
#[doc = "AN004 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB04_A {
    #[doc = "0: AN004 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN004 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB04_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB04_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB04_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ANSB04_A {
        match self.bits {
            false => ANSB04_A::_0,
            true => ANSB04_A::_1,
        }
    }
    #[doc = "AN004 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB04_A::_0
    }
    #[doc = "AN004 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB04_A::_1
    }
}
#[doc = "Field `ANSB04` writer - AN004 Select"]
pub type ANSB04_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ANSB04_A>;
impl<'a, REG, const O: u8> ANSB04_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN004 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB04_A::_0)
    }
    #[doc = "AN004 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB04_A::_1)
    }
}
#[doc = "Field `ANSB05` reader - AN005 Select"]
pub type ANSB05_R = crate::BitReader<ANSB05_A>;
#[doc = "AN005 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB05_A {
    #[doc = "0: AN005 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN005 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB05_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB05_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB05_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ANSB05_A {
        match self.bits {
            false => ANSB05_A::_0,
            true => ANSB05_A::_1,
        }
    }
    #[doc = "AN005 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB05_A::_0
    }
    #[doc = "AN005 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB05_A::_1
    }
}
#[doc = "Field `ANSB05` writer - AN005 Select"]
pub type ANSB05_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ANSB05_A>;
impl<'a, REG, const O: u8> ANSB05_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN005 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB05_A::_0)
    }
    #[doc = "AN005 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB05_A::_1)
    }
}
#[doc = "Field `ANSB06` reader - AN006 Select"]
pub type ANSB06_R = crate::BitReader<ANSB06_A>;
#[doc = "AN006 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB06_A {
    #[doc = "0: AN006 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN006 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB06_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB06_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB06_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ANSB06_A {
        match self.bits {
            false => ANSB06_A::_0,
            true => ANSB06_A::_1,
        }
    }
    #[doc = "AN006 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB06_A::_0
    }
    #[doc = "AN006 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB06_A::_1
    }
}
#[doc = "Field `ANSB06` writer - AN006 Select"]
pub type ANSB06_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ANSB06_A>;
impl<'a, REG, const O: u8> ANSB06_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN006 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB06_A::_0)
    }
    #[doc = "AN006 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB06_A::_1)
    }
}
#[doc = "Field `ANSB07` reader - AN007 Select"]
pub type ANSB07_R = crate::BitReader<ANSB07_A>;
#[doc = "AN007 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB07_A {
    #[doc = "0: AN007 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN007 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB07_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB07_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB07_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ANSB07_A {
        match self.bits {
            false => ANSB07_A::_0,
            true => ANSB07_A::_1,
        }
    }
    #[doc = "AN007 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB07_A::_0
    }
    #[doc = "AN007 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB07_A::_1
    }
}
#[doc = "Field `ANSB07` writer - AN007 Select"]
pub type ANSB07_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ANSB07_A>;
impl<'a, REG, const O: u8> ANSB07_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN007 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB07_A::_0)
    }
    #[doc = "AN007 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB07_A::_1)
    }
}
#[doc = "Field `ANSB08` reader - AN008 Select"]
pub type ANSB08_R = crate::BitReader<ANSB08_A>;
#[doc = "AN008 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB08_A {
    #[doc = "0: AN008 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN008 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB08_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB08_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB08_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ANSB08_A {
        match self.bits {
            false => ANSB08_A::_0,
            true => ANSB08_A::_1,
        }
    }
    #[doc = "AN008 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB08_A::_0
    }
    #[doc = "AN008 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB08_A::_1
    }
}
#[doc = "Field `ANSB08` writer - AN008 Select"]
pub type ANSB08_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ANSB08_A>;
impl<'a, REG, const O: u8> ANSB08_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN008 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB08_A::_0)
    }
    #[doc = "AN008 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB08_A::_1)
    }
}
#[doc = "Field `ANSB09` reader - AN009 Select"]
pub type ANSB09_R = crate::BitReader<ANSB09_A>;
#[doc = "AN009 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB09_A {
    #[doc = "0: AN009 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN009 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB09_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB09_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB09_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ANSB09_A {
        match self.bits {
            false => ANSB09_A::_0,
            true => ANSB09_A::_1,
        }
    }
    #[doc = "AN009 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB09_A::_0
    }
    #[doc = "AN009 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB09_A::_1
    }
}
#[doc = "Field `ANSB09` writer - AN009 Select"]
pub type ANSB09_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ANSB09_A>;
impl<'a, REG, const O: u8> ANSB09_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN009 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB09_A::_0)
    }
    #[doc = "AN009 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB09_A::_1)
    }
}
#[doc = "Field `ANSB10` reader - AN010 Select"]
pub type ANSB10_R = crate::BitReader<ANSB10_A>;
#[doc = "AN010 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB10_A {
    #[doc = "0: AN010 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN010 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB10_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB10_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ANSB10_A {
        match self.bits {
            false => ANSB10_A::_0,
            true => ANSB10_A::_1,
        }
    }
    #[doc = "AN010 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB10_A::_0
    }
    #[doc = "AN010 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB10_A::_1
    }
}
#[doc = "Field `ANSB10` writer - AN010 Select"]
pub type ANSB10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ANSB10_A>;
impl<'a, REG, const O: u8> ANSB10_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN010 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB10_A::_0)
    }
    #[doc = "AN010 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB10_A::_1)
    }
}
#[doc = "Field `ANSB11` reader - AN011 Select"]
pub type ANSB11_R = crate::BitReader<ANSB11_A>;
#[doc = "AN011 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB11_A {
    #[doc = "0: AN011 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN011 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB11_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB11_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ANSB11_A {
        match self.bits {
            false => ANSB11_A::_0,
            true => ANSB11_A::_1,
        }
    }
    #[doc = "AN011 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB11_A::_0
    }
    #[doc = "AN011 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB11_A::_1
    }
}
#[doc = "Field `ANSB11` writer - AN011 Select"]
pub type ANSB11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ANSB11_A>;
impl<'a, REG, const O: u8> ANSB11_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN011 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB11_A::_0)
    }
    #[doc = "AN011 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB11_A::_1)
    }
}
#[doc = "Field `ANSB12` reader - AN012 Select"]
pub type ANSB12_R = crate::BitReader<ANSB12_A>;
#[doc = "AN012 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB12_A {
    #[doc = "0: AN012 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN012 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB12_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB12_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ANSB12_A {
        match self.bits {
            false => ANSB12_A::_0,
            true => ANSB12_A::_1,
        }
    }
    #[doc = "AN012 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB12_A::_0
    }
    #[doc = "AN012 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB12_A::_1
    }
}
#[doc = "Field `ANSB12` writer - AN012 Select"]
pub type ANSB12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ANSB12_A>;
impl<'a, REG, const O: u8> ANSB12_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN012 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB12_A::_0)
    }
    #[doc = "AN012 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB12_A::_1)
    }
}
#[doc = "Field `ANSB13` reader - AN013 Select"]
pub type ANSB13_R = crate::BitReader<ANSB13_A>;
#[doc = "AN013 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB13_A {
    #[doc = "0: AN013 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN013 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB13_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB13_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ANSB13_A {
        match self.bits {
            false => ANSB13_A::_0,
            true => ANSB13_A::_1,
        }
    }
    #[doc = "AN013 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB13_A::_0
    }
    #[doc = "AN013 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB13_A::_1
    }
}
#[doc = "Field `ANSB13` writer - AN013 Select"]
pub type ANSB13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ANSB13_A>;
impl<'a, REG, const O: u8> ANSB13_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN013 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB13_A::_0)
    }
    #[doc = "AN013 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB13_A::_1)
    }
}
#[doc = "Field `ANSB14` reader - AN014 Select"]
pub type ANSB14_R = crate::BitReader<ANSB14_A>;
#[doc = "AN014 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB14_A {
    #[doc = "0: AN014 is not subjected to conversion."]
    _0 = 0,
    #[doc = "1: AN014 is subjected to conversion."]
    _1 = 1,
}
impl From<ANSB14_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB14_A) -> Self {
        variant as u8 != 0
    }
}
impl ANSB14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ANSB14_A {
        match self.bits {
            false => ANSB14_A::_0,
            true => ANSB14_A::_1,
        }
    }
    #[doc = "AN014 is not subjected to conversion."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB14_A::_0
    }
    #[doc = "AN014 is subjected to conversion."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB14_A::_1
    }
}
#[doc = "Field `ANSB14` writer - AN014 Select"]
pub type ANSB14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ANSB14_A>;
impl<'a, REG, const O: u8> ANSB14_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN014 is not subjected to conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB14_A::_0)
    }
    #[doc = "AN014 is subjected to conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB14_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AN000 Select"]
    #[inline(always)]
    pub fn ansb00(&self) -> ANSB00_R {
        ANSB00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AN001 Select"]
    #[inline(always)]
    pub fn ansb01(&self) -> ANSB01_R {
        ANSB01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AN002 Select"]
    #[inline(always)]
    pub fn ansb02(&self) -> ANSB02_R {
        ANSB02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AN003 Select"]
    #[inline(always)]
    pub fn ansb03(&self) -> ANSB03_R {
        ANSB03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AN004 Select"]
    #[inline(always)]
    pub fn ansb04(&self) -> ANSB04_R {
        ANSB04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AN005 Select"]
    #[inline(always)]
    pub fn ansb05(&self) -> ANSB05_R {
        ANSB05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AN006 Select"]
    #[inline(always)]
    pub fn ansb06(&self) -> ANSB06_R {
        ANSB06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AN007 Select"]
    #[inline(always)]
    pub fn ansb07(&self) -> ANSB07_R {
        ANSB07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AN008 Select"]
    #[inline(always)]
    pub fn ansb08(&self) -> ANSB08_R {
        ANSB08_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AN009 Select"]
    #[inline(always)]
    pub fn ansb09(&self) -> ANSB09_R {
        ANSB09_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AN010 Select"]
    #[inline(always)]
    pub fn ansb10(&self) -> ANSB10_R {
        ANSB10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - AN011 Select"]
    #[inline(always)]
    pub fn ansb11(&self) -> ANSB11_R {
        ANSB11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AN012 Select"]
    #[inline(always)]
    pub fn ansb12(&self) -> ANSB12_R {
        ANSB12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AN013 Select"]
    #[inline(always)]
    pub fn ansb13(&self) -> ANSB13_R {
        ANSB13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AN014 Select"]
    #[inline(always)]
    pub fn ansb14(&self) -> ANSB14_R {
        ANSB14_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AN000 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb00(&mut self) -> ANSB00_W<ADANSB0_SPEC, 0> {
        ANSB00_W::new(self)
    }
    #[doc = "Bit 1 - AN001 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb01(&mut self) -> ANSB01_W<ADANSB0_SPEC, 1> {
        ANSB01_W::new(self)
    }
    #[doc = "Bit 2 - AN002 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb02(&mut self) -> ANSB02_W<ADANSB0_SPEC, 2> {
        ANSB02_W::new(self)
    }
    #[doc = "Bit 3 - AN003 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb03(&mut self) -> ANSB03_W<ADANSB0_SPEC, 3> {
        ANSB03_W::new(self)
    }
    #[doc = "Bit 4 - AN004 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb04(&mut self) -> ANSB04_W<ADANSB0_SPEC, 4> {
        ANSB04_W::new(self)
    }
    #[doc = "Bit 5 - AN005 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb05(&mut self) -> ANSB05_W<ADANSB0_SPEC, 5> {
        ANSB05_W::new(self)
    }
    #[doc = "Bit 6 - AN006 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb06(&mut self) -> ANSB06_W<ADANSB0_SPEC, 6> {
        ANSB06_W::new(self)
    }
    #[doc = "Bit 7 - AN007 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb07(&mut self) -> ANSB07_W<ADANSB0_SPEC, 7> {
        ANSB07_W::new(self)
    }
    #[doc = "Bit 8 - AN008 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb08(&mut self) -> ANSB08_W<ADANSB0_SPEC, 8> {
        ANSB08_W::new(self)
    }
    #[doc = "Bit 9 - AN009 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb09(&mut self) -> ANSB09_W<ADANSB0_SPEC, 9> {
        ANSB09_W::new(self)
    }
    #[doc = "Bit 10 - AN010 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb10(&mut self) -> ANSB10_W<ADANSB0_SPEC, 10> {
        ANSB10_W::new(self)
    }
    #[doc = "Bit 11 - AN011 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb11(&mut self) -> ANSB11_W<ADANSB0_SPEC, 11> {
        ANSB11_W::new(self)
    }
    #[doc = "Bit 12 - AN012 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb12(&mut self) -> ANSB12_W<ADANSB0_SPEC, 12> {
        ANSB12_W::new(self)
    }
    #[doc = "Bit 13 - AN013 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb13(&mut self) -> ANSB13_W<ADANSB0_SPEC, 13> {
        ANSB13_W::new(self)
    }
    #[doc = "Bit 14 - AN014 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ansb14(&mut self) -> ANSB14_W<ADANSB0_SPEC, 14> {
        ANSB14_W::new(self)
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
#[doc = "A/D Channel Select Register B0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adansb0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adansb0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADANSB0_SPEC;
impl crate::RegisterSpec for ADANSB0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adansb0::R`](R) reader structure"]
impl crate::Readable for ADANSB0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adansb0::W`](W) writer structure"]
impl crate::Writable for ADANSB0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADANSB0 to value 0"]
impl crate::Resettable for ADANSB0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
