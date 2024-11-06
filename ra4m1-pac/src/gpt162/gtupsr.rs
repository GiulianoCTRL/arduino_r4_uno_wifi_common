#[doc = "Register `GTUPSR` reader"]
pub type R = crate::R<GTUPSR_SPEC>;
#[doc = "Register `GTUPSR` writer"]
pub type W = crate::W<GTUPSR_SPEC>;
#[doc = "Field `USGTRGAR` reader - GTETRGA Pin Rising Input Source Counter Count Up Enable"]
pub type USGTRGAR_R = crate::BitReader<USGTRGAR_A>;
#[doc = "GTETRGA Pin Rising Input Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USGTRGAR_A {
    #[doc = "0: Counter count up is disable at the rising edge of GTETRGA input"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the rising edge of GTETRGA input"]
    _1 = 1,
}
impl From<USGTRGAR_A> for bool {
    #[inline(always)]
    fn from(variant: USGTRGAR_A) -> Self {
        variant as u8 != 0
    }
}
impl USGTRGAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USGTRGAR_A {
        match self.bits {
            false => USGTRGAR_A::_0,
            true => USGTRGAR_A::_1,
        }
    }
    #[doc = "Counter count up is disable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USGTRGAR_A::_0
    }
    #[doc = "Counter count up is enable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USGTRGAR_A::_1
    }
}
#[doc = "Field `USGTRGAR` writer - GTETRGA Pin Rising Input Source Counter Count Up Enable"]
pub type USGTRGAR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USGTRGAR_A>;
impl<'a, REG, const O: u8> USGTRGAR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count up is disable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USGTRGAR_A::_0)
    }
    #[doc = "Counter count up is enable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USGTRGAR_A::_1)
    }
}
#[doc = "Field `USGTRGAF` reader - GTETRGA Pin Falling Input Source Counter Count Up Enable"]
pub type USGTRGAF_R = crate::BitReader<USGTRGAF_A>;
#[doc = "GTETRGA Pin Falling Input Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USGTRGAF_A {
    #[doc = "0: Counter count up is disable at the falling edge of GTETRGA input"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the falling edge of GTETRGA input"]
    _1 = 1,
}
impl From<USGTRGAF_A> for bool {
    #[inline(always)]
    fn from(variant: USGTRGAF_A) -> Self {
        variant as u8 != 0
    }
}
impl USGTRGAF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USGTRGAF_A {
        match self.bits {
            false => USGTRGAF_A::_0,
            true => USGTRGAF_A::_1,
        }
    }
    #[doc = "Counter count up is disable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USGTRGAF_A::_0
    }
    #[doc = "Counter count up is enable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USGTRGAF_A::_1
    }
}
#[doc = "Field `USGTRGAF` writer - GTETRGA Pin Falling Input Source Counter Count Up Enable"]
pub type USGTRGAF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USGTRGAF_A>;
impl<'a, REG, const O: u8> USGTRGAF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count up is disable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USGTRGAF_A::_0)
    }
    #[doc = "Counter count up is enable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USGTRGAF_A::_1)
    }
}
#[doc = "Field `USGTRGBR` reader - GTETRGB Pin Rising Input Source Counter Count Up Enable"]
pub type USGTRGBR_R = crate::BitReader<USGTRGBR_A>;
#[doc = "GTETRGB Pin Rising Input Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USGTRGBR_A {
    #[doc = "0: Counter count up is disable at the rising edge of GTETRGB input"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the rising edge of GTETRGB input"]
    _1 = 1,
}
impl From<USGTRGBR_A> for bool {
    #[inline(always)]
    fn from(variant: USGTRGBR_A) -> Self {
        variant as u8 != 0
    }
}
impl USGTRGBR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USGTRGBR_A {
        match self.bits {
            false => USGTRGBR_A::_0,
            true => USGTRGBR_A::_1,
        }
    }
    #[doc = "Counter count up is disable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USGTRGBR_A::_0
    }
    #[doc = "Counter count up is enable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USGTRGBR_A::_1
    }
}
#[doc = "Field `USGTRGBR` writer - GTETRGB Pin Rising Input Source Counter Count Up Enable"]
pub type USGTRGBR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USGTRGBR_A>;
impl<'a, REG, const O: u8> USGTRGBR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count up is disable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USGTRGBR_A::_0)
    }
    #[doc = "Counter count up is enable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USGTRGBR_A::_1)
    }
}
#[doc = "Field `USGTRGBF` reader - GTETRGB Pin Falling Input Source Counter Count Up Enable"]
pub type USGTRGBF_R = crate::BitReader<USGTRGBF_A>;
#[doc = "GTETRGB Pin Falling Input Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USGTRGBF_A {
    #[doc = "0: Counter count up is disable at the falling edge of GTETRGB input"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the falling edge of GTETRGB input"]
    _1 = 1,
}
impl From<USGTRGBF_A> for bool {
    #[inline(always)]
    fn from(variant: USGTRGBF_A) -> Self {
        variant as u8 != 0
    }
}
impl USGTRGBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USGTRGBF_A {
        match self.bits {
            false => USGTRGBF_A::_0,
            true => USGTRGBF_A::_1,
        }
    }
    #[doc = "Counter count up is disable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USGTRGBF_A::_0
    }
    #[doc = "Counter count up is enable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USGTRGBF_A::_1
    }
}
#[doc = "Field `USGTRGBF` writer - GTETRGB Pin Falling Input Source Counter Count Up Enable"]
pub type USGTRGBF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USGTRGBF_A>;
impl<'a, REG, const O: u8> USGTRGBF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count up is disable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USGTRGBF_A::_0)
    }
    #[doc = "Counter count up is enable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USGTRGBF_A::_1)
    }
}
#[doc = "Field `USCARBL` reader - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Up Enable"]
pub type USCARBL_R = crate::BitReader<USCARBL_A>;
#[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USCARBL_A {
    #[doc = "0: Counter count up is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 1,
}
impl From<USCARBL_A> for bool {
    #[inline(always)]
    fn from(variant: USCARBL_A) -> Self {
        variant as u8 != 0
    }
}
impl USCARBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USCARBL_A {
        match self.bits {
            false => USCARBL_A::_0,
            true => USCARBL_A::_1,
        }
    }
    #[doc = "Counter count up is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USCARBL_A::_0
    }
    #[doc = "Counter count up is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USCARBL_A::_1
    }
}
#[doc = "Field `USCARBL` writer - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Up Enable"]
pub type USCARBL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USCARBL_A>;
impl<'a, REG, const O: u8> USCARBL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count up is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USCARBL_A::_0)
    }
    #[doc = "Counter count up is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USCARBL_A::_1)
    }
}
#[doc = "Field `USCARBH` reader - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Up Enable"]
pub type USCARBH_R = crate::BitReader<USCARBH_A>;
#[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USCARBH_A {
    #[doc = "0: Counter count up is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 1,
}
impl From<USCARBH_A> for bool {
    #[inline(always)]
    fn from(variant: USCARBH_A) -> Self {
        variant as u8 != 0
    }
}
impl USCARBH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USCARBH_A {
        match self.bits {
            false => USCARBH_A::_0,
            true => USCARBH_A::_1,
        }
    }
    #[doc = "Counter count up is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USCARBH_A::_0
    }
    #[doc = "Counter count up is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USCARBH_A::_1
    }
}
#[doc = "Field `USCARBH` writer - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Up Enable"]
pub type USCARBH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USCARBH_A>;
impl<'a, REG, const O: u8> USCARBH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count up is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USCARBH_A::_0)
    }
    #[doc = "Counter count up is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USCARBH_A::_1)
    }
}
#[doc = "Field `USCAFBL` reader - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Up Enable"]
pub type USCAFBL_R = crate::BitReader<USCAFBL_A>;
#[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USCAFBL_A {
    #[doc = "0: Counter count up is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 1,
}
impl From<USCAFBL_A> for bool {
    #[inline(always)]
    fn from(variant: USCAFBL_A) -> Self {
        variant as u8 != 0
    }
}
impl USCAFBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USCAFBL_A {
        match self.bits {
            false => USCAFBL_A::_0,
            true => USCAFBL_A::_1,
        }
    }
    #[doc = "Counter count up is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USCAFBL_A::_0
    }
    #[doc = "Counter count up is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USCAFBL_A::_1
    }
}
#[doc = "Field `USCAFBL` writer - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Up Enable"]
pub type USCAFBL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USCAFBL_A>;
impl<'a, REG, const O: u8> USCAFBL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count up is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USCAFBL_A::_0)
    }
    #[doc = "Counter count up is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USCAFBL_A::_1)
    }
}
#[doc = "Field `USCAFBH` reader - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Up Enable"]
pub type USCAFBH_R = crate::BitReader<USCAFBH_A>;
#[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USCAFBH_A {
    #[doc = "0: Counter count up is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 1,
}
impl From<USCAFBH_A> for bool {
    #[inline(always)]
    fn from(variant: USCAFBH_A) -> Self {
        variant as u8 != 0
    }
}
impl USCAFBH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USCAFBH_A {
        match self.bits {
            false => USCAFBH_A::_0,
            true => USCAFBH_A::_1,
        }
    }
    #[doc = "Counter count up is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USCAFBH_A::_0
    }
    #[doc = "Counter count up is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USCAFBH_A::_1
    }
}
#[doc = "Field `USCAFBH` writer - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Up Enable"]
pub type USCAFBH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USCAFBH_A>;
impl<'a, REG, const O: u8> USCAFBH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count up is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USCAFBH_A::_0)
    }
    #[doc = "Counter count up is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USCAFBH_A::_1)
    }
}
#[doc = "Field `USCBRAL` reader - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Up Enable"]
pub type USCBRAL_R = crate::BitReader<USCBRAL_A>;
#[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USCBRAL_A {
    #[doc = "0: Counter count up is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 1,
}
impl From<USCBRAL_A> for bool {
    #[inline(always)]
    fn from(variant: USCBRAL_A) -> Self {
        variant as u8 != 0
    }
}
impl USCBRAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USCBRAL_A {
        match self.bits {
            false => USCBRAL_A::_0,
            true => USCBRAL_A::_1,
        }
    }
    #[doc = "Counter count up is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USCBRAL_A::_0
    }
    #[doc = "Counter count up is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USCBRAL_A::_1
    }
}
#[doc = "Field `USCBRAL` writer - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Up Enable"]
pub type USCBRAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USCBRAL_A>;
impl<'a, REG, const O: u8> USCBRAL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count up is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USCBRAL_A::_0)
    }
    #[doc = "Counter count up is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USCBRAL_A::_1)
    }
}
#[doc = "Field `USCBRAH` reader - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Up Enable"]
pub type USCBRAH_R = crate::BitReader<USCBRAH_A>;
#[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USCBRAH_A {
    #[doc = "0: Counter count up is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 1,
}
impl From<USCBRAH_A> for bool {
    #[inline(always)]
    fn from(variant: USCBRAH_A) -> Self {
        variant as u8 != 0
    }
}
impl USCBRAH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USCBRAH_A {
        match self.bits {
            false => USCBRAH_A::_0,
            true => USCBRAH_A::_1,
        }
    }
    #[doc = "Counter count up is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USCBRAH_A::_0
    }
    #[doc = "Counter count up is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USCBRAH_A::_1
    }
}
#[doc = "Field `USCBRAH` writer - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Up Enable"]
pub type USCBRAH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USCBRAH_A>;
impl<'a, REG, const O: u8> USCBRAH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count up is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USCBRAH_A::_0)
    }
    #[doc = "Counter count up is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USCBRAH_A::_1)
    }
}
#[doc = "Field `USCBFAL` reader - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Up Enable"]
pub type USCBFAL_R = crate::BitReader<USCBFAL_A>;
#[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USCBFAL_A {
    #[doc = "0: Counter count up is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 1,
}
impl From<USCBFAL_A> for bool {
    #[inline(always)]
    fn from(variant: USCBFAL_A) -> Self {
        variant as u8 != 0
    }
}
impl USCBFAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USCBFAL_A {
        match self.bits {
            false => USCBFAL_A::_0,
            true => USCBFAL_A::_1,
        }
    }
    #[doc = "Counter count up is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USCBFAL_A::_0
    }
    #[doc = "Counter count up is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USCBFAL_A::_1
    }
}
#[doc = "Field `USCBFAL` writer - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Up Enable"]
pub type USCBFAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USCBFAL_A>;
impl<'a, REG, const O: u8> USCBFAL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count up is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USCBFAL_A::_0)
    }
    #[doc = "Counter count up is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USCBFAL_A::_1)
    }
}
#[doc = "Field `USCBFAH` reader - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Up Enable"]
pub type USCBFAH_R = crate::BitReader<USCBFAH_A>;
#[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USCBFAH_A {
    #[doc = "0: Counter count up is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 1,
}
impl From<USCBFAH_A> for bool {
    #[inline(always)]
    fn from(variant: USCBFAH_A) -> Self {
        variant as u8 != 0
    }
}
impl USCBFAH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USCBFAH_A {
        match self.bits {
            false => USCBFAH_A::_0,
            true => USCBFAH_A::_1,
        }
    }
    #[doc = "Counter count up is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USCBFAH_A::_0
    }
    #[doc = "Counter count up is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USCBFAH_A::_1
    }
}
#[doc = "Field `USCBFAH` writer - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Up Enable"]
pub type USCBFAH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USCBFAH_A>;
impl<'a, REG, const O: u8> USCBFAH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count up is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USCBFAH_A::_0)
    }
    #[doc = "Counter count up is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USCBFAH_A::_1)
    }
}
#[doc = "Field `USELCA` reader - ELC_GPTA Event Source Counter Count Up Enable"]
pub type USELCA_R = crate::BitReader<USELCA_A>;
#[doc = "ELC_GPTA Event Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USELCA_A {
    #[doc = "0: Counter count up is disable at the ELC_GPTA input"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the ELC_GPTA input"]
    _1 = 1,
}
impl From<USELCA_A> for bool {
    #[inline(always)]
    fn from(variant: USELCA_A) -> Self {
        variant as u8 != 0
    }
}
impl USELCA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USELCA_A {
        match self.bits {
            false => USELCA_A::_0,
            true => USELCA_A::_1,
        }
    }
    #[doc = "Counter count up is disable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USELCA_A::_0
    }
    #[doc = "Counter count up is enable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USELCA_A::_1
    }
}
#[doc = "Field `USELCA` writer - ELC_GPTA Event Source Counter Count Up Enable"]
pub type USELCA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USELCA_A>;
impl<'a, REG, const O: u8> USELCA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count up is disable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USELCA_A::_0)
    }
    #[doc = "Counter count up is enable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USELCA_A::_1)
    }
}
#[doc = "Field `USELCB` reader - ELC_GPTB Event Source Counter Count Up Enable"]
pub type USELCB_R = crate::BitReader<USELCB_A>;
#[doc = "ELC_GPTB Event Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USELCB_A {
    #[doc = "0: Counter count up is disable at the ELC_GPTB input"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the ELC_GPTB input"]
    _1 = 1,
}
impl From<USELCB_A> for bool {
    #[inline(always)]
    fn from(variant: USELCB_A) -> Self {
        variant as u8 != 0
    }
}
impl USELCB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USELCB_A {
        match self.bits {
            false => USELCB_A::_0,
            true => USELCB_A::_1,
        }
    }
    #[doc = "Counter count up is disable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USELCB_A::_0
    }
    #[doc = "Counter count up is enable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USELCB_A::_1
    }
}
#[doc = "Field `USELCB` writer - ELC_GPTB Event Source Counter Count Up Enable"]
pub type USELCB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USELCB_A>;
impl<'a, REG, const O: u8> USELCB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count up is disable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USELCB_A::_0)
    }
    #[doc = "Counter count up is enable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USELCB_A::_1)
    }
}
#[doc = "Field `USELCC` reader - ELC_GPTC Event Source Counter Count Up Enable"]
pub type USELCC_R = crate::BitReader<USELCC_A>;
#[doc = "ELC_GPTC Event Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USELCC_A {
    #[doc = "0: Counter count up is disable at the ELC_GPTC input"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the ELC_GPTC input"]
    _1 = 1,
}
impl From<USELCC_A> for bool {
    #[inline(always)]
    fn from(variant: USELCC_A) -> Self {
        variant as u8 != 0
    }
}
impl USELCC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USELCC_A {
        match self.bits {
            false => USELCC_A::_0,
            true => USELCC_A::_1,
        }
    }
    #[doc = "Counter count up is disable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USELCC_A::_0
    }
    #[doc = "Counter count up is enable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USELCC_A::_1
    }
}
#[doc = "Field `USELCC` writer - ELC_GPTC Event Source Counter Count Up Enable"]
pub type USELCC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USELCC_A>;
impl<'a, REG, const O: u8> USELCC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count up is disable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USELCC_A::_0)
    }
    #[doc = "Counter count up is enable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USELCC_A::_1)
    }
}
#[doc = "Field `USELCD` reader - ELC_GPTD Event Source Counter Count Up Enable"]
pub type USELCD_R = crate::BitReader<USELCD_A>;
#[doc = "ELC_GPTD Event Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USELCD_A {
    #[doc = "0: Counter count up is disable at the ELC_GPTD input"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the ELC_GPTD input"]
    _1 = 1,
}
impl From<USELCD_A> for bool {
    #[inline(always)]
    fn from(variant: USELCD_A) -> Self {
        variant as u8 != 0
    }
}
impl USELCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USELCD_A {
        match self.bits {
            false => USELCD_A::_0,
            true => USELCD_A::_1,
        }
    }
    #[doc = "Counter count up is disable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USELCD_A::_0
    }
    #[doc = "Counter count up is enable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USELCD_A::_1
    }
}
#[doc = "Field `USELCD` writer - ELC_GPTD Event Source Counter Count Up Enable"]
pub type USELCD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USELCD_A>;
impl<'a, REG, const O: u8> USELCD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count up is disable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USELCD_A::_0)
    }
    #[doc = "Counter count up is enable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USELCD_A::_1)
    }
}
#[doc = "Field `USELCE` reader - ELC_GPTE Event Source Counter Count Up Enable"]
pub type USELCE_R = crate::BitReader<USELCE_A>;
#[doc = "ELC_GPTE Event Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USELCE_A {
    #[doc = "0: Counter count up is disable at the ELC_GPTE input"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the ELC_GPTE input"]
    _1 = 1,
}
impl From<USELCE_A> for bool {
    #[inline(always)]
    fn from(variant: USELCE_A) -> Self {
        variant as u8 != 0
    }
}
impl USELCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USELCE_A {
        match self.bits {
            false => USELCE_A::_0,
            true => USELCE_A::_1,
        }
    }
    #[doc = "Counter count up is disable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USELCE_A::_0
    }
    #[doc = "Counter count up is enable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USELCE_A::_1
    }
}
#[doc = "Field `USELCE` writer - ELC_GPTE Event Source Counter Count Up Enable"]
pub type USELCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USELCE_A>;
impl<'a, REG, const O: u8> USELCE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count up is disable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USELCE_A::_0)
    }
    #[doc = "Counter count up is enable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USELCE_A::_1)
    }
}
#[doc = "Field `USELCF` reader - ELC_GPTF Event Source Counter Count Up Enable"]
pub type USELCF_R = crate::BitReader<USELCF_A>;
#[doc = "ELC_GPTF Event Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USELCF_A {
    #[doc = "0: Counter count up is disable at the ELC_GPTF input"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the ELC_GPTF input"]
    _1 = 1,
}
impl From<USELCF_A> for bool {
    #[inline(always)]
    fn from(variant: USELCF_A) -> Self {
        variant as u8 != 0
    }
}
impl USELCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USELCF_A {
        match self.bits {
            false => USELCF_A::_0,
            true => USELCF_A::_1,
        }
    }
    #[doc = "Counter count up is disable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USELCF_A::_0
    }
    #[doc = "Counter count up is enable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USELCF_A::_1
    }
}
#[doc = "Field `USELCF` writer - ELC_GPTF Event Source Counter Count Up Enable"]
pub type USELCF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USELCF_A>;
impl<'a, REG, const O: u8> USELCF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count up is disable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USELCF_A::_0)
    }
    #[doc = "Counter count up is enable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USELCF_A::_1)
    }
}
#[doc = "Field `USELCG` reader - ELC_GPTG Event Source Counter Count Up Enable"]
pub type USELCG_R = crate::BitReader<USELCG_A>;
#[doc = "ELC_GPTG Event Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USELCG_A {
    #[doc = "0: Counter count up is disable at the ELC_GPTG input"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the ELC_GPTG input"]
    _1 = 1,
}
impl From<USELCG_A> for bool {
    #[inline(always)]
    fn from(variant: USELCG_A) -> Self {
        variant as u8 != 0
    }
}
impl USELCG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USELCG_A {
        match self.bits {
            false => USELCG_A::_0,
            true => USELCG_A::_1,
        }
    }
    #[doc = "Counter count up is disable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USELCG_A::_0
    }
    #[doc = "Counter count up is enable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USELCG_A::_1
    }
}
#[doc = "Field `USELCG` writer - ELC_GPTG Event Source Counter Count Up Enable"]
pub type USELCG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USELCG_A>;
impl<'a, REG, const O: u8> USELCG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count up is disable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USELCG_A::_0)
    }
    #[doc = "Counter count up is enable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USELCG_A::_1)
    }
}
#[doc = "Field `USELCH` reader - ELC_GPTH Event Source Counter Count Up Enable"]
pub type USELCH_R = crate::BitReader<USELCH_A>;
#[doc = "ELC_GPTH Event Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USELCH_A {
    #[doc = "0: Counter count up is disable at the ELC_GPTH input"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the ELC_GPTH input"]
    _1 = 1,
}
impl From<USELCH_A> for bool {
    #[inline(always)]
    fn from(variant: USELCH_A) -> Self {
        variant as u8 != 0
    }
}
impl USELCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USELCH_A {
        match self.bits {
            false => USELCH_A::_0,
            true => USELCH_A::_1,
        }
    }
    #[doc = "Counter count up is disable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USELCH_A::_0
    }
    #[doc = "Counter count up is enable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USELCH_A::_1
    }
}
#[doc = "Field `USELCH` writer - ELC_GPTH Event Source Counter Count Up Enable"]
pub type USELCH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USELCH_A>;
impl<'a, REG, const O: u8> USELCH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter count up is disable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USELCH_A::_0)
    }
    #[doc = "Counter count up is enable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USELCH_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn usgtrgar(&self) -> USGTRGAR_R {
        USGTRGAR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn usgtrgaf(&self) -> USGTRGAF_R {
        USGTRGAF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn usgtrgbr(&self) -> USGTRGBR_R {
        USGTRGBR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn usgtrgbf(&self) -> USGTRGBF_R {
        USGTRGBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscarbl(&self) -> USCARBL_R {
        USCARBL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscarbh(&self) -> USCARBH_R {
        USCARBH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscafbl(&self) -> USCAFBL_R {
        USCAFBL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscafbh(&self) -> USCAFBH_R {
        USCAFBH_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscbral(&self) -> USCBRAL_R {
        USCBRAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscbrah(&self) -> USCBRAH_R {
        USCBRAH_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscbfal(&self) -> USCBFAL_R {
        USCBFAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscbfah(&self) -> USCBFAH_R {
        USCBFAH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselca(&self) -> USELCA_R {
        USELCA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselcb(&self) -> USELCB_R {
        USELCB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselcc(&self) -> USELCC_R {
        USELCC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselcd(&self) -> USELCD_R {
        USELCD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ELC_GPTE Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselce(&self) -> USELCE_R {
        USELCE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ELC_GPTF Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselcf(&self) -> USELCF_R {
        USELCF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ELC_GPTG Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselcg(&self) -> USELCG_R {
        USELCG_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ELC_GPTH Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselch(&self) -> USELCH_R {
        USELCH_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usgtrgar(&mut self) -> USGTRGAR_W<GTUPSR_SPEC, 0> {
        USGTRGAR_W::new(self)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usgtrgaf(&mut self) -> USGTRGAF_W<GTUPSR_SPEC, 1> {
        USGTRGAF_W::new(self)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usgtrgbr(&mut self) -> USGTRGBR_W<GTUPSR_SPEC, 2> {
        USGTRGBR_W::new(self)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usgtrgbf(&mut self) -> USGTRGBF_W<GTUPSR_SPEC, 3> {
        USGTRGBF_W::new(self)
    }
    #[doc = "Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uscarbl(&mut self) -> USCARBL_W<GTUPSR_SPEC, 8> {
        USCARBL_W::new(self)
    }
    #[doc = "Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uscarbh(&mut self) -> USCARBH_W<GTUPSR_SPEC, 9> {
        USCARBH_W::new(self)
    }
    #[doc = "Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uscafbl(&mut self) -> USCAFBL_W<GTUPSR_SPEC, 10> {
        USCAFBL_W::new(self)
    }
    #[doc = "Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uscafbh(&mut self) -> USCAFBH_W<GTUPSR_SPEC, 11> {
        USCAFBH_W::new(self)
    }
    #[doc = "Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uscbral(&mut self) -> USCBRAL_W<GTUPSR_SPEC, 12> {
        USCBRAL_W::new(self)
    }
    #[doc = "Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uscbrah(&mut self) -> USCBRAH_W<GTUPSR_SPEC, 13> {
        USCBRAH_W::new(self)
    }
    #[doc = "Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uscbfal(&mut self) -> USCBFAL_W<GTUPSR_SPEC, 14> {
        USCBFAL_W::new(self)
    }
    #[doc = "Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uscbfah(&mut self) -> USCBFAH_W<GTUPSR_SPEC, 15> {
        USCBFAH_W::new(self)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uselca(&mut self) -> USELCA_W<GTUPSR_SPEC, 16> {
        USELCA_W::new(self)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uselcb(&mut self) -> USELCB_W<GTUPSR_SPEC, 17> {
        USELCB_W::new(self)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uselcc(&mut self) -> USELCC_W<GTUPSR_SPEC, 18> {
        USELCC_W::new(self)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uselcd(&mut self) -> USELCD_W<GTUPSR_SPEC, 19> {
        USELCD_W::new(self)
    }
    #[doc = "Bit 20 - ELC_GPTE Event Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uselce(&mut self) -> USELCE_W<GTUPSR_SPEC, 20> {
        USELCE_W::new(self)
    }
    #[doc = "Bit 21 - ELC_GPTF Event Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uselcf(&mut self) -> USELCF_W<GTUPSR_SPEC, 21> {
        USELCF_W::new(self)
    }
    #[doc = "Bit 22 - ELC_GPTG Event Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uselcg(&mut self) -> USELCG_W<GTUPSR_SPEC, 22> {
        USELCG_W::new(self)
    }
    #[doc = "Bit 23 - ELC_GPTH Event Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uselch(&mut self) -> USELCH_W<GTUPSR_SPEC, 23> {
        USELCH_W::new(self)
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
#[doc = "General PWM Timer Up Count Source Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtupsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtupsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTUPSR_SPEC;
impl crate::RegisterSpec for GTUPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtupsr::R`](R) reader structure"]
impl crate::Readable for GTUPSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gtupsr::W`](W) writer structure"]
impl crate::Writable for GTUPSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTUPSR to value 0"]
impl crate::Resettable for GTUPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
