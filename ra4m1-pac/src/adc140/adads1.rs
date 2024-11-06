#[doc = "Register `ADADS1` reader"]
pub type R = crate::R<ADADS1_SPEC>;
#[doc = "Register `ADADS1` writer"]
pub type W = crate::W<ADADS1_SPEC>;
#[doc = "Field `ADS16` reader - A/D-Converted Value Addition/Average Channel AN016 Select"]
pub type ADS16_R = crate::BitReader<ADS16_A>;
#[doc = "A/D-Converted Value Addition/Average Channel AN016 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS16_A {
    #[doc = "0: AN016 is not selected."]
    _0 = 0,
    #[doc = "1: AN016 is selected."]
    _1 = 1,
}
impl From<ADS16_A> for bool {
    #[inline(always)]
    fn from(variant: ADS16_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADS16_A {
        match self.bits {
            false => ADS16_A::_0,
            true => ADS16_A::_1,
        }
    }
    #[doc = "AN016 is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS16_A::_0
    }
    #[doc = "AN016 is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS16_A::_1
    }
}
#[doc = "Field `ADS16` writer - A/D-Converted Value Addition/Average Channel AN016 Select"]
pub type ADS16_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADS16_A>;
impl<'a, REG, const O: u8> ADS16_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN016 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS16_A::_0)
    }
    #[doc = "AN016 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS16_A::_1)
    }
}
#[doc = "Field `ADS17` reader - A/D-Converted Value Addition/Average Channel AN017 Select"]
pub type ADS17_R = crate::BitReader<ADS17_A>;
#[doc = "A/D-Converted Value Addition/Average Channel AN017 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS17_A {
    #[doc = "0: AN017 is not selected."]
    _0 = 0,
    #[doc = "1: AN017 is selected."]
    _1 = 1,
}
impl From<ADS17_A> for bool {
    #[inline(always)]
    fn from(variant: ADS17_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADS17_A {
        match self.bits {
            false => ADS17_A::_0,
            true => ADS17_A::_1,
        }
    }
    #[doc = "AN017 is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS17_A::_0
    }
    #[doc = "AN017 is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS17_A::_1
    }
}
#[doc = "Field `ADS17` writer - A/D-Converted Value Addition/Average Channel AN017 Select"]
pub type ADS17_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADS17_A>;
impl<'a, REG, const O: u8> ADS17_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN017 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS17_A::_0)
    }
    #[doc = "AN017 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS17_A::_1)
    }
}
#[doc = "Field `ADS18` reader - A/D-Converted Value Addition/Average Channel AN018 Select"]
pub type ADS18_R = crate::BitReader<ADS18_A>;
#[doc = "A/D-Converted Value Addition/Average Channel AN018 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS18_A {
    #[doc = "0: AN018 is not selected."]
    _0 = 0,
    #[doc = "1: AN018 is selected."]
    _1 = 1,
}
impl From<ADS18_A> for bool {
    #[inline(always)]
    fn from(variant: ADS18_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADS18_A {
        match self.bits {
            false => ADS18_A::_0,
            true => ADS18_A::_1,
        }
    }
    #[doc = "AN018 is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS18_A::_0
    }
    #[doc = "AN018 is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS18_A::_1
    }
}
#[doc = "Field `ADS18` writer - A/D-Converted Value Addition/Average Channel AN018 Select"]
pub type ADS18_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADS18_A>;
impl<'a, REG, const O: u8> ADS18_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN018 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS18_A::_0)
    }
    #[doc = "AN018 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS18_A::_1)
    }
}
#[doc = "Field `ADS19` reader - A/D-Converted Value Addition/Average Channel AN019 Select"]
pub type ADS19_R = crate::BitReader<ADS19_A>;
#[doc = "A/D-Converted Value Addition/Average Channel AN019 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS19_A {
    #[doc = "0: AN019 is not selected."]
    _0 = 0,
    #[doc = "1: AN019 is selected."]
    _1 = 1,
}
impl From<ADS19_A> for bool {
    #[inline(always)]
    fn from(variant: ADS19_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADS19_A {
        match self.bits {
            false => ADS19_A::_0,
            true => ADS19_A::_1,
        }
    }
    #[doc = "AN019 is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS19_A::_0
    }
    #[doc = "AN019 is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS19_A::_1
    }
}
#[doc = "Field `ADS19` writer - A/D-Converted Value Addition/Average Channel AN019 Select"]
pub type ADS19_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADS19_A>;
impl<'a, REG, const O: u8> ADS19_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN019 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS19_A::_0)
    }
    #[doc = "AN019 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS19_A::_1)
    }
}
#[doc = "Field `ADS20` reader - A/D-Converted Value Addition/Average Channel AN020 Select"]
pub type ADS20_R = crate::BitReader<ADS20_A>;
#[doc = "A/D-Converted Value Addition/Average Channel AN020 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS20_A {
    #[doc = "0: AN020 is not selected."]
    _0 = 0,
    #[doc = "1: AN020 is selected."]
    _1 = 1,
}
impl From<ADS20_A> for bool {
    #[inline(always)]
    fn from(variant: ADS20_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADS20_A {
        match self.bits {
            false => ADS20_A::_0,
            true => ADS20_A::_1,
        }
    }
    #[doc = "AN020 is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS20_A::_0
    }
    #[doc = "AN020 is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS20_A::_1
    }
}
#[doc = "Field `ADS20` writer - A/D-Converted Value Addition/Average Channel AN020 Select"]
pub type ADS20_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADS20_A>;
impl<'a, REG, const O: u8> ADS20_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN020 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS20_A::_0)
    }
    #[doc = "AN020 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS20_A::_1)
    }
}
#[doc = "Field `ADS21` reader - A/D-Converted Value Addition/Average Channel AN021 Select"]
pub type ADS21_R = crate::BitReader<ADS21_A>;
#[doc = "A/D-Converted Value Addition/Average Channel AN021 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS21_A {
    #[doc = "0: AN021 is not selected."]
    _0 = 0,
    #[doc = "1: AN021 is selected."]
    _1 = 1,
}
impl From<ADS21_A> for bool {
    #[inline(always)]
    fn from(variant: ADS21_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADS21_A {
        match self.bits {
            false => ADS21_A::_0,
            true => ADS21_A::_1,
        }
    }
    #[doc = "AN021 is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS21_A::_0
    }
    #[doc = "AN021 is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS21_A::_1
    }
}
#[doc = "Field `ADS21` writer - A/D-Converted Value Addition/Average Channel AN021 Select"]
pub type ADS21_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADS21_A>;
impl<'a, REG, const O: u8> ADS21_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN021 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS21_A::_0)
    }
    #[doc = "AN021 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS21_A::_1)
    }
}
#[doc = "Field `ADS22` reader - A/D-Converted Value Addition/Average Channel AN022 Select"]
pub type ADS22_R = crate::BitReader<ADS22_A>;
#[doc = "A/D-Converted Value Addition/Average Channel AN022 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS22_A {
    #[doc = "0: AN022 is not selected."]
    _0 = 0,
    #[doc = "1: AN022 is selected."]
    _1 = 1,
}
impl From<ADS22_A> for bool {
    #[inline(always)]
    fn from(variant: ADS22_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADS22_A {
        match self.bits {
            false => ADS22_A::_0,
            true => ADS22_A::_1,
        }
    }
    #[doc = "AN022 is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS22_A::_0
    }
    #[doc = "AN022 is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS22_A::_1
    }
}
#[doc = "Field `ADS22` writer - A/D-Converted Value Addition/Average Channel AN022 Select"]
pub type ADS22_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADS22_A>;
impl<'a, REG, const O: u8> ADS22_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN022 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS22_A::_0)
    }
    #[doc = "AN022 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS22_A::_1)
    }
}
#[doc = "Field `ADS23` reader - A/D-Converted Value Addition/Average Channel AN023 Select"]
pub type ADS23_R = crate::BitReader<ADS23_A>;
#[doc = "A/D-Converted Value Addition/Average Channel AN023 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS23_A {
    #[doc = "0: AN023 is not selected."]
    _0 = 0,
    #[doc = "1: AN023 is selected."]
    _1 = 1,
}
impl From<ADS23_A> for bool {
    #[inline(always)]
    fn from(variant: ADS23_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADS23_A {
        match self.bits {
            false => ADS23_A::_0,
            true => ADS23_A::_1,
        }
    }
    #[doc = "AN023 is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS23_A::_0
    }
    #[doc = "AN023 is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS23_A::_1
    }
}
#[doc = "Field `ADS23` writer - A/D-Converted Value Addition/Average Channel AN023 Select"]
pub type ADS23_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADS23_A>;
impl<'a, REG, const O: u8> ADS23_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN023 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS23_A::_0)
    }
    #[doc = "AN023 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS23_A::_1)
    }
}
#[doc = "Field `ADS24` reader - A/D-Converted Value Addition/Average Channel AN024 Select"]
pub type ADS24_R = crate::BitReader<ADS24_A>;
#[doc = "A/D-Converted Value Addition/Average Channel AN024 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS24_A {
    #[doc = "0: AN024 is not selected."]
    _0 = 0,
    #[doc = "1: AN024 is selected."]
    _1 = 1,
}
impl From<ADS24_A> for bool {
    #[inline(always)]
    fn from(variant: ADS24_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADS24_A {
        match self.bits {
            false => ADS24_A::_0,
            true => ADS24_A::_1,
        }
    }
    #[doc = "AN024 is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS24_A::_0
    }
    #[doc = "AN024 is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS24_A::_1
    }
}
#[doc = "Field `ADS24` writer - A/D-Converted Value Addition/Average Channel AN024 Select"]
pub type ADS24_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADS24_A>;
impl<'a, REG, const O: u8> ADS24_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN024 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS24_A::_0)
    }
    #[doc = "AN024 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS24_A::_1)
    }
}
#[doc = "Field `ADS25` reader - A/D-Converted Value Addition/Average Channel AN025 Select"]
pub type ADS25_R = crate::BitReader<ADS25_A>;
#[doc = "A/D-Converted Value Addition/Average Channel AN025 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS25_A {
    #[doc = "0: AN025 is not selected."]
    _0 = 0,
    #[doc = "1: AN025 is selected."]
    _1 = 1,
}
impl From<ADS25_A> for bool {
    #[inline(always)]
    fn from(variant: ADS25_A) -> Self {
        variant as u8 != 0
    }
}
impl ADS25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADS25_A {
        match self.bits {
            false => ADS25_A::_0,
            true => ADS25_A::_1,
        }
    }
    #[doc = "AN025 is not selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS25_A::_0
    }
    #[doc = "AN025 is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS25_A::_1
    }
}
#[doc = "Field `ADS25` writer - A/D-Converted Value Addition/Average Channel AN025 Select"]
pub type ADS25_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADS25_A>;
impl<'a, REG, const O: u8> ADS25_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AN025 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS25_A::_0)
    }
    #[doc = "AN025 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS25_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - A/D-Converted Value Addition/Average Channel AN016 Select"]
    #[inline(always)]
    pub fn ads16(&self) -> ADS16_R {
        ADS16_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A/D-Converted Value Addition/Average Channel AN017 Select"]
    #[inline(always)]
    pub fn ads17(&self) -> ADS17_R {
        ADS17_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A/D-Converted Value Addition/Average Channel AN018 Select"]
    #[inline(always)]
    pub fn ads18(&self) -> ADS18_R {
        ADS18_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - A/D-Converted Value Addition/Average Channel AN019 Select"]
    #[inline(always)]
    pub fn ads19(&self) -> ADS19_R {
        ADS19_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A/D-Converted Value Addition/Average Channel AN020 Select"]
    #[inline(always)]
    pub fn ads20(&self) -> ADS20_R {
        ADS20_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - A/D-Converted Value Addition/Average Channel AN021 Select"]
    #[inline(always)]
    pub fn ads21(&self) -> ADS21_R {
        ADS21_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - A/D-Converted Value Addition/Average Channel AN022 Select"]
    #[inline(always)]
    pub fn ads22(&self) -> ADS22_R {
        ADS22_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - A/D-Converted Value Addition/Average Channel AN023 Select"]
    #[inline(always)]
    pub fn ads23(&self) -> ADS23_R {
        ADS23_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - A/D-Converted Value Addition/Average Channel AN024 Select"]
    #[inline(always)]
    pub fn ads24(&self) -> ADS24_R {
        ADS24_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - A/D-Converted Value Addition/Average Channel AN025 Select"]
    #[inline(always)]
    pub fn ads25(&self) -> ADS25_R {
        ADS25_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A/D-Converted Value Addition/Average Channel AN016 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads16(&mut self) -> ADS16_W<ADADS1_SPEC, 0> {
        ADS16_W::new(self)
    }
    #[doc = "Bit 1 - A/D-Converted Value Addition/Average Channel AN017 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads17(&mut self) -> ADS17_W<ADADS1_SPEC, 1> {
        ADS17_W::new(self)
    }
    #[doc = "Bit 2 - A/D-Converted Value Addition/Average Channel AN018 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads18(&mut self) -> ADS18_W<ADADS1_SPEC, 2> {
        ADS18_W::new(self)
    }
    #[doc = "Bit 3 - A/D-Converted Value Addition/Average Channel AN019 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads19(&mut self) -> ADS19_W<ADADS1_SPEC, 3> {
        ADS19_W::new(self)
    }
    #[doc = "Bit 4 - A/D-Converted Value Addition/Average Channel AN020 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads20(&mut self) -> ADS20_W<ADADS1_SPEC, 4> {
        ADS20_W::new(self)
    }
    #[doc = "Bit 5 - A/D-Converted Value Addition/Average Channel AN021 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads21(&mut self) -> ADS21_W<ADADS1_SPEC, 5> {
        ADS21_W::new(self)
    }
    #[doc = "Bit 6 - A/D-Converted Value Addition/Average Channel AN022 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads22(&mut self) -> ADS22_W<ADADS1_SPEC, 6> {
        ADS22_W::new(self)
    }
    #[doc = "Bit 7 - A/D-Converted Value Addition/Average Channel AN023 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads23(&mut self) -> ADS23_W<ADADS1_SPEC, 7> {
        ADS23_W::new(self)
    }
    #[doc = "Bit 8 - A/D-Converted Value Addition/Average Channel AN024 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads24(&mut self) -> ADS24_W<ADADS1_SPEC, 8> {
        ADS24_W::new(self)
    }
    #[doc = "Bit 9 - A/D-Converted Value Addition/Average Channel AN025 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ads25(&mut self) -> ADS25_W<ADADS1_SPEC, 9> {
        ADS25_W::new(self)
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
#[doc = "A/D-Converted Value Addition/Average Channel Select Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adads1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adads1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADADS1_SPEC;
impl crate::RegisterSpec for ADADS1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adads1::R`](R) reader structure"]
impl crate::Readable for ADADS1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adads1::W`](W) writer structure"]
impl crate::Writable for ADADS1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADADS1 to value 0"]
impl crate::Resettable for ADADS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
