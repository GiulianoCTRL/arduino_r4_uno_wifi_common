#[doc = "Register `ADCMPANSR1` reader"]
pub type R = crate::R<ADCMPANSR1_SPEC>;
#[doc = "Register `ADCMPANSR1` writer"]
pub type W = crate::W<ADCMPANSR1_SPEC>;
#[doc = "Field `CMPCHA16` reader - AN016 Select"]
pub type CMPCHA16_R = crate::BitReader<CMPCHA16_A>;
#[doc = "AN016 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA16_A {
    #[doc = "0: Excludes AN016 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN016 from the compare window A target range."]
    _1 = 1,
}
impl From<CMPCHA16_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA16_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMPCHA16_A {
        match self.bits {
            false => CMPCHA16_A::_0,
            true => CMPCHA16_A::_1,
        }
    }
    #[doc = "Excludes AN016 from the compare window A target range."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA16_A::_0
    }
    #[doc = "Includes AN016 from the compare window A target range."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA16_A::_1
    }
}
#[doc = "Field `CMPCHA16` writer - AN016 Select"]
pub type CMPCHA16_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMPCHA16_A>;
impl<'a, REG, const O: u8> CMPCHA16_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Excludes AN016 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA16_A::_0)
    }
    #[doc = "Includes AN016 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA16_A::_1)
    }
}
#[doc = "Field `CMPCHA17` reader - AN017 Select"]
pub type CMPCHA17_R = crate::BitReader<CMPCHA17_A>;
#[doc = "AN017 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA17_A {
    #[doc = "0: Excludes AN017 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN017 from the compare window A target range."]
    _1 = 1,
}
impl From<CMPCHA17_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA17_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMPCHA17_A {
        match self.bits {
            false => CMPCHA17_A::_0,
            true => CMPCHA17_A::_1,
        }
    }
    #[doc = "Excludes AN017 from the compare window A target range."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA17_A::_0
    }
    #[doc = "Includes AN017 from the compare window A target range."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA17_A::_1
    }
}
#[doc = "Field `CMPCHA17` writer - AN017 Select"]
pub type CMPCHA17_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMPCHA17_A>;
impl<'a, REG, const O: u8> CMPCHA17_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Excludes AN017 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA17_A::_0)
    }
    #[doc = "Includes AN017 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA17_A::_1)
    }
}
#[doc = "Field `CMPCHA18` reader - AN018 Select"]
pub type CMPCHA18_R = crate::BitReader<CMPCHA18_A>;
#[doc = "AN018 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA18_A {
    #[doc = "0: Excludes AN018 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN018 from the compare window A target range."]
    _1 = 1,
}
impl From<CMPCHA18_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA18_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMPCHA18_A {
        match self.bits {
            false => CMPCHA18_A::_0,
            true => CMPCHA18_A::_1,
        }
    }
    #[doc = "Excludes AN018 from the compare window A target range."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA18_A::_0
    }
    #[doc = "Includes AN018 from the compare window A target range."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA18_A::_1
    }
}
#[doc = "Field `CMPCHA18` writer - AN018 Select"]
pub type CMPCHA18_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMPCHA18_A>;
impl<'a, REG, const O: u8> CMPCHA18_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Excludes AN018 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA18_A::_0)
    }
    #[doc = "Includes AN018 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA18_A::_1)
    }
}
#[doc = "Field `CMPCHA19` reader - AN019 Select"]
pub type CMPCHA19_R = crate::BitReader<CMPCHA19_A>;
#[doc = "AN019 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA19_A {
    #[doc = "0: Excludes AN019 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN019 from the compare window A target range."]
    _1 = 1,
}
impl From<CMPCHA19_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA19_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMPCHA19_A {
        match self.bits {
            false => CMPCHA19_A::_0,
            true => CMPCHA19_A::_1,
        }
    }
    #[doc = "Excludes AN019 from the compare window A target range."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA19_A::_0
    }
    #[doc = "Includes AN019 from the compare window A target range."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA19_A::_1
    }
}
#[doc = "Field `CMPCHA19` writer - AN019 Select"]
pub type CMPCHA19_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMPCHA19_A>;
impl<'a, REG, const O: u8> CMPCHA19_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Excludes AN019 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA19_A::_0)
    }
    #[doc = "Includes AN019 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA19_A::_1)
    }
}
#[doc = "Field `CMPCHA20` reader - AN020 Select"]
pub type CMPCHA20_R = crate::BitReader<CMPCHA20_A>;
#[doc = "AN020 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA20_A {
    #[doc = "0: Excludes AN020 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN020 from the compare window A target range."]
    _1 = 1,
}
impl From<CMPCHA20_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA20_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMPCHA20_A {
        match self.bits {
            false => CMPCHA20_A::_0,
            true => CMPCHA20_A::_1,
        }
    }
    #[doc = "Excludes AN020 from the compare window A target range."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA20_A::_0
    }
    #[doc = "Includes AN020 from the compare window A target range."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA20_A::_1
    }
}
#[doc = "Field `CMPCHA20` writer - AN020 Select"]
pub type CMPCHA20_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMPCHA20_A>;
impl<'a, REG, const O: u8> CMPCHA20_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Excludes AN020 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA20_A::_0)
    }
    #[doc = "Includes AN020 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA20_A::_1)
    }
}
#[doc = "Field `CMPCHA21` reader - AN021 Select"]
pub type CMPCHA21_R = crate::BitReader<CMPCHA21_A>;
#[doc = "AN021 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA21_A {
    #[doc = "0: Excludes AN021 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN021 from the compare window A target range."]
    _1 = 1,
}
impl From<CMPCHA21_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA21_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMPCHA21_A {
        match self.bits {
            false => CMPCHA21_A::_0,
            true => CMPCHA21_A::_1,
        }
    }
    #[doc = "Excludes AN021 from the compare window A target range."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA21_A::_0
    }
    #[doc = "Includes AN021 from the compare window A target range."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA21_A::_1
    }
}
#[doc = "Field `CMPCHA21` writer - AN021 Select"]
pub type CMPCHA21_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMPCHA21_A>;
impl<'a, REG, const O: u8> CMPCHA21_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Excludes AN021 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA21_A::_0)
    }
    #[doc = "Includes AN021 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA21_A::_1)
    }
}
#[doc = "Field `CMPCHA22` reader - AN022 Select"]
pub type CMPCHA22_R = crate::BitReader<CMPCHA22_A>;
#[doc = "AN022 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA22_A {
    #[doc = "0: Excludes AN022 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN022 from the compare window A target range."]
    _1 = 1,
}
impl From<CMPCHA22_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA22_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMPCHA22_A {
        match self.bits {
            false => CMPCHA22_A::_0,
            true => CMPCHA22_A::_1,
        }
    }
    #[doc = "Excludes AN022 from the compare window A target range."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA22_A::_0
    }
    #[doc = "Includes AN022 from the compare window A target range."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA22_A::_1
    }
}
#[doc = "Field `CMPCHA22` writer - AN022 Select"]
pub type CMPCHA22_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMPCHA22_A>;
impl<'a, REG, const O: u8> CMPCHA22_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Excludes AN022 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA22_A::_0)
    }
    #[doc = "Includes AN022 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA22_A::_1)
    }
}
#[doc = "Field `CMPCHA23` reader - AN023 Select"]
pub type CMPCHA23_R = crate::BitReader<CMPCHA23_A>;
#[doc = "AN023 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA23_A {
    #[doc = "0: Excludes AN023 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN023 from the compare window A target range."]
    _1 = 1,
}
impl From<CMPCHA23_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA23_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMPCHA23_A {
        match self.bits {
            false => CMPCHA23_A::_0,
            true => CMPCHA23_A::_1,
        }
    }
    #[doc = "Excludes AN023 from the compare window A target range."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA23_A::_0
    }
    #[doc = "Includes AN023 from the compare window A target range."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA23_A::_1
    }
}
#[doc = "Field `CMPCHA23` writer - AN023 Select"]
pub type CMPCHA23_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMPCHA23_A>;
impl<'a, REG, const O: u8> CMPCHA23_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Excludes AN023 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA23_A::_0)
    }
    #[doc = "Includes AN023 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA23_A::_1)
    }
}
#[doc = "Field `CMPCHA24` reader - AN024 Select"]
pub type CMPCHA24_R = crate::BitReader<CMPCHA24_A>;
#[doc = "AN024 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA24_A {
    #[doc = "0: Excludes AN024 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN024 from the compare window A target range."]
    _1 = 1,
}
impl From<CMPCHA24_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA24_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMPCHA24_A {
        match self.bits {
            false => CMPCHA24_A::_0,
            true => CMPCHA24_A::_1,
        }
    }
    #[doc = "Excludes AN024 from the compare window A target range."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA24_A::_0
    }
    #[doc = "Includes AN024 from the compare window A target range."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA24_A::_1
    }
}
#[doc = "Field `CMPCHA24` writer - AN024 Select"]
pub type CMPCHA24_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMPCHA24_A>;
impl<'a, REG, const O: u8> CMPCHA24_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Excludes AN024 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA24_A::_0)
    }
    #[doc = "Includes AN024 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA24_A::_1)
    }
}
#[doc = "Field `CMPCHA25` reader - AN025 Select"]
pub type CMPCHA25_R = crate::BitReader<CMPCHA25_A>;
#[doc = "AN025 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA25_A {
    #[doc = "0: Excludes AN025 from the compare window A target range."]
    _0 = 0,
    #[doc = "1: Includes AN025 from the compare window A target range."]
    _1 = 1,
}
impl From<CMPCHA25_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA25_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPCHA25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMPCHA25_A {
        match self.bits {
            false => CMPCHA25_A::_0,
            true => CMPCHA25_A::_1,
        }
    }
    #[doc = "Excludes AN025 from the compare window A target range."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA25_A::_0
    }
    #[doc = "Includes AN025 from the compare window A target range."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA25_A::_1
    }
}
#[doc = "Field `CMPCHA25` writer - AN025 Select"]
pub type CMPCHA25_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMPCHA25_A>;
impl<'a, REG, const O: u8> CMPCHA25_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Excludes AN025 from the compare window A target range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA25_A::_0)
    }
    #[doc = "Includes AN025 from the compare window A target range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA25_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AN016 Select"]
    #[inline(always)]
    pub fn cmpcha16(&self) -> CMPCHA16_R {
        CMPCHA16_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AN017 Select"]
    #[inline(always)]
    pub fn cmpcha17(&self) -> CMPCHA17_R {
        CMPCHA17_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AN018 Select"]
    #[inline(always)]
    pub fn cmpcha18(&self) -> CMPCHA18_R {
        CMPCHA18_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AN019 Select"]
    #[inline(always)]
    pub fn cmpcha19(&self) -> CMPCHA19_R {
        CMPCHA19_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AN020 Select"]
    #[inline(always)]
    pub fn cmpcha20(&self) -> CMPCHA20_R {
        CMPCHA20_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AN021 Select"]
    #[inline(always)]
    pub fn cmpcha21(&self) -> CMPCHA21_R {
        CMPCHA21_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AN022 Select"]
    #[inline(always)]
    pub fn cmpcha22(&self) -> CMPCHA22_R {
        CMPCHA22_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AN023 Select"]
    #[inline(always)]
    pub fn cmpcha23(&self) -> CMPCHA23_R {
        CMPCHA23_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AN024 Select"]
    #[inline(always)]
    pub fn cmpcha24(&self) -> CMPCHA24_R {
        CMPCHA24_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AN025 Select"]
    #[inline(always)]
    pub fn cmpcha25(&self) -> CMPCHA25_R {
        CMPCHA25_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AN016 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha16(&mut self) -> CMPCHA16_W<ADCMPANSR1_SPEC, 0> {
        CMPCHA16_W::new(self)
    }
    #[doc = "Bit 1 - AN017 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha17(&mut self) -> CMPCHA17_W<ADCMPANSR1_SPEC, 1> {
        CMPCHA17_W::new(self)
    }
    #[doc = "Bit 2 - AN018 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha18(&mut self) -> CMPCHA18_W<ADCMPANSR1_SPEC, 2> {
        CMPCHA18_W::new(self)
    }
    #[doc = "Bit 3 - AN019 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha19(&mut self) -> CMPCHA19_W<ADCMPANSR1_SPEC, 3> {
        CMPCHA19_W::new(self)
    }
    #[doc = "Bit 4 - AN020 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha20(&mut self) -> CMPCHA20_W<ADCMPANSR1_SPEC, 4> {
        CMPCHA20_W::new(self)
    }
    #[doc = "Bit 5 - AN021 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha21(&mut self) -> CMPCHA21_W<ADCMPANSR1_SPEC, 5> {
        CMPCHA21_W::new(self)
    }
    #[doc = "Bit 6 - AN022 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha22(&mut self) -> CMPCHA22_W<ADCMPANSR1_SPEC, 6> {
        CMPCHA22_W::new(self)
    }
    #[doc = "Bit 7 - AN023 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha23(&mut self) -> CMPCHA23_W<ADCMPANSR1_SPEC, 7> {
        CMPCHA23_W::new(self)
    }
    #[doc = "Bit 8 - AN024 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha24(&mut self) -> CMPCHA24_W<ADCMPANSR1_SPEC, 8> {
        CMPCHA24_W::new(self)
    }
    #[doc = "Bit 9 - AN025 Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcha25(&mut self) -> CMPCHA25_W<ADCMPANSR1_SPEC, 9> {
        CMPCHA25_W::new(self)
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
#[doc = "A/D Compare Function Window A Channel Select Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmpansr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmpansr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCMPANSR1_SPEC;
impl crate::RegisterSpec for ADCMPANSR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcmpansr1::R`](R) reader structure"]
impl crate::Readable for ADCMPANSR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adcmpansr1::W`](W) writer structure"]
impl crate::Writable for ADCMPANSR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPANSR1 to value 0"]
impl crate::Resettable for ADCMPANSR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
