#[doc = "Register `GTSSR` reader"]
pub type R = crate::R<GTSSR_SPEC>;
#[doc = "Register `GTSSR` writer"]
pub type W = crate::W<GTSSR_SPEC>;
#[doc = "Field `SSGTRGAR` reader - GTETRGA Pin Rising Input Source Counter Start Enable"]
pub type SSGTRGAR_R = crate::BitReader<SSGTRGAR_A>;
#[doc = "GTETRGA Pin Rising Input Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSGTRGAR_A {
    #[doc = "0: Counter start is disable at the rising edge of GTETRGA input"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the rising edge of GTETRGA input"]
    _1 = 1,
}
impl From<SSGTRGAR_A> for bool {
    #[inline(always)]
    fn from(variant: SSGTRGAR_A) -> Self {
        variant as u8 != 0
    }
}
impl SSGTRGAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSGTRGAR_A {
        match self.bits {
            false => SSGTRGAR_A::_0,
            true => SSGTRGAR_A::_1,
        }
    }
    #[doc = "Counter start is disable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSGTRGAR_A::_0
    }
    #[doc = "Counter start is enable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSGTRGAR_A::_1
    }
}
#[doc = "Field `SSGTRGAR` writer - GTETRGA Pin Rising Input Source Counter Start Enable"]
pub type SSGTRGAR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SSGTRGAR_A>;
impl<'a, REG, const O: u8> SSGTRGAR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSGTRGAR_A::_0)
    }
    #[doc = "Counter start is enable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSGTRGAR_A::_1)
    }
}
#[doc = "Field `SSGTRGAF` reader - GTETRGA Pin Falling Input Source Counter Start Enable"]
pub type SSGTRGAF_R = crate::BitReader<SSGTRGAF_A>;
#[doc = "GTETRGA Pin Falling Input Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSGTRGAF_A {
    #[doc = "0: Counter start is disable at the falling edge of GTETRGA input"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the falling edge of GTETRGA input"]
    _1 = 1,
}
impl From<SSGTRGAF_A> for bool {
    #[inline(always)]
    fn from(variant: SSGTRGAF_A) -> Self {
        variant as u8 != 0
    }
}
impl SSGTRGAF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSGTRGAF_A {
        match self.bits {
            false => SSGTRGAF_A::_0,
            true => SSGTRGAF_A::_1,
        }
    }
    #[doc = "Counter start is disable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSGTRGAF_A::_0
    }
    #[doc = "Counter start is enable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSGTRGAF_A::_1
    }
}
#[doc = "Field `SSGTRGAF` writer - GTETRGA Pin Falling Input Source Counter Start Enable"]
pub type SSGTRGAF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SSGTRGAF_A>;
impl<'a, REG, const O: u8> SSGTRGAF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSGTRGAF_A::_0)
    }
    #[doc = "Counter start is enable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSGTRGAF_A::_1)
    }
}
#[doc = "Field `SSGTRGBR` reader - GTETRGB Pin Rising Input Source Counter Start Enable"]
pub type SSGTRGBR_R = crate::BitReader<SSGTRGBR_A>;
#[doc = "GTETRGB Pin Rising Input Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSGTRGBR_A {
    #[doc = "0: Counter start is disable at the rising edge of GTETRGB input"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the rising edge of GTETRGB input"]
    _1 = 1,
}
impl From<SSGTRGBR_A> for bool {
    #[inline(always)]
    fn from(variant: SSGTRGBR_A) -> Self {
        variant as u8 != 0
    }
}
impl SSGTRGBR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSGTRGBR_A {
        match self.bits {
            false => SSGTRGBR_A::_0,
            true => SSGTRGBR_A::_1,
        }
    }
    #[doc = "Counter start is disable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSGTRGBR_A::_0
    }
    #[doc = "Counter start is enable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSGTRGBR_A::_1
    }
}
#[doc = "Field `SSGTRGBR` writer - GTETRGB Pin Rising Input Source Counter Start Enable"]
pub type SSGTRGBR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SSGTRGBR_A>;
impl<'a, REG, const O: u8> SSGTRGBR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSGTRGBR_A::_0)
    }
    #[doc = "Counter start is enable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSGTRGBR_A::_1)
    }
}
#[doc = "Field `SSGTRGBF` reader - GTETRGB Pin Falling Input Source Counter Start Enable"]
pub type SSGTRGBF_R = crate::BitReader<SSGTRGBF_A>;
#[doc = "GTETRGB Pin Falling Input Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSGTRGBF_A {
    #[doc = "0: Counter start is disable at the falling edge of GTETRGB input"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the falling edge of GTETRGB input"]
    _1 = 1,
}
impl From<SSGTRGBF_A> for bool {
    #[inline(always)]
    fn from(variant: SSGTRGBF_A) -> Self {
        variant as u8 != 0
    }
}
impl SSGTRGBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSGTRGBF_A {
        match self.bits {
            false => SSGTRGBF_A::_0,
            true => SSGTRGBF_A::_1,
        }
    }
    #[doc = "Counter start is disable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSGTRGBF_A::_0
    }
    #[doc = "Counter start is enable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSGTRGBF_A::_1
    }
}
#[doc = "Field `SSGTRGBF` writer - GTETRGB Pin Falling Input Source Counter Start Enable"]
pub type SSGTRGBF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SSGTRGBF_A>;
impl<'a, REG, const O: u8> SSGTRGBF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSGTRGBF_A::_0)
    }
    #[doc = "Counter start is enable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSGTRGBF_A::_1)
    }
}
#[doc = "Field `SSCARBL` reader - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Start Enable"]
pub type SSCARBL_R = crate::BitReader<SSCARBL_A>;
#[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSCARBL_A {
    #[doc = "0: Counter start is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 1,
}
impl From<SSCARBL_A> for bool {
    #[inline(always)]
    fn from(variant: SSCARBL_A) -> Self {
        variant as u8 != 0
    }
}
impl SSCARBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSCARBL_A {
        match self.bits {
            false => SSCARBL_A::_0,
            true => SSCARBL_A::_1,
        }
    }
    #[doc = "Counter start is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSCARBL_A::_0
    }
    #[doc = "Counter start is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSCARBL_A::_1
    }
}
#[doc = "Field `SSCARBL` writer - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Start Enable"]
pub type SSCARBL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SSCARBL_A>;
impl<'a, REG, const O: u8> SSCARBL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSCARBL_A::_0)
    }
    #[doc = "Counter start is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSCARBL_A::_1)
    }
}
#[doc = "Field `SSCARBH` reader - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Start Enable"]
pub type SSCARBH_R = crate::BitReader<SSCARBH_A>;
#[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSCARBH_A {
    #[doc = "0: Counter start is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 1,
}
impl From<SSCARBH_A> for bool {
    #[inline(always)]
    fn from(variant: SSCARBH_A) -> Self {
        variant as u8 != 0
    }
}
impl SSCARBH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSCARBH_A {
        match self.bits {
            false => SSCARBH_A::_0,
            true => SSCARBH_A::_1,
        }
    }
    #[doc = "Counter start is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSCARBH_A::_0
    }
    #[doc = "Counter start is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSCARBH_A::_1
    }
}
#[doc = "Field `SSCARBH` writer - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Start Enable"]
pub type SSCARBH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SSCARBH_A>;
impl<'a, REG, const O: u8> SSCARBH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSCARBH_A::_0)
    }
    #[doc = "Counter start is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSCARBH_A::_1)
    }
}
#[doc = "Field `SSCAFBL` reader - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Start Enable"]
pub type SSCAFBL_R = crate::BitReader<SSCAFBL_A>;
#[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSCAFBL_A {
    #[doc = "0: Counter start is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 1,
}
impl From<SSCAFBL_A> for bool {
    #[inline(always)]
    fn from(variant: SSCAFBL_A) -> Self {
        variant as u8 != 0
    }
}
impl SSCAFBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSCAFBL_A {
        match self.bits {
            false => SSCAFBL_A::_0,
            true => SSCAFBL_A::_1,
        }
    }
    #[doc = "Counter start is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSCAFBL_A::_0
    }
    #[doc = "Counter start is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSCAFBL_A::_1
    }
}
#[doc = "Field `SSCAFBL` writer - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Start Enable"]
pub type SSCAFBL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SSCAFBL_A>;
impl<'a, REG, const O: u8> SSCAFBL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSCAFBL_A::_0)
    }
    #[doc = "Counter start is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSCAFBL_A::_1)
    }
}
#[doc = "Field `SSCAFBH` reader - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Start Enable"]
pub type SSCAFBH_R = crate::BitReader<SSCAFBH_A>;
#[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSCAFBH_A {
    #[doc = "0: Counter start is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 1,
}
impl From<SSCAFBH_A> for bool {
    #[inline(always)]
    fn from(variant: SSCAFBH_A) -> Self {
        variant as u8 != 0
    }
}
impl SSCAFBH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSCAFBH_A {
        match self.bits {
            false => SSCAFBH_A::_0,
            true => SSCAFBH_A::_1,
        }
    }
    #[doc = "Counter start is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSCAFBH_A::_0
    }
    #[doc = "Counter start is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSCAFBH_A::_1
    }
}
#[doc = "Field `SSCAFBH` writer - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Start Enable"]
pub type SSCAFBH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SSCAFBH_A>;
impl<'a, REG, const O: u8> SSCAFBH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSCAFBH_A::_0)
    }
    #[doc = "Counter start is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSCAFBH_A::_1)
    }
}
#[doc = "Field `SSCBRAL` reader - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Start Enable"]
pub type SSCBRAL_R = crate::BitReader<SSCBRAL_A>;
#[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSCBRAL_A {
    #[doc = "0: Counter start is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 1,
}
impl From<SSCBRAL_A> for bool {
    #[inline(always)]
    fn from(variant: SSCBRAL_A) -> Self {
        variant as u8 != 0
    }
}
impl SSCBRAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSCBRAL_A {
        match self.bits {
            false => SSCBRAL_A::_0,
            true => SSCBRAL_A::_1,
        }
    }
    #[doc = "Counter start is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSCBRAL_A::_0
    }
    #[doc = "Counter start is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSCBRAL_A::_1
    }
}
#[doc = "Field `SSCBRAL` writer - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Start Enable"]
pub type SSCBRAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SSCBRAL_A>;
impl<'a, REG, const O: u8> SSCBRAL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSCBRAL_A::_0)
    }
    #[doc = "Counter start is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSCBRAL_A::_1)
    }
}
#[doc = "Field `SSCBRAH` reader - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Start Enable"]
pub type SSCBRAH_R = crate::BitReader<SSCBRAH_A>;
#[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSCBRAH_A {
    #[doc = "0: Counter start is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 1,
}
impl From<SSCBRAH_A> for bool {
    #[inline(always)]
    fn from(variant: SSCBRAH_A) -> Self {
        variant as u8 != 0
    }
}
impl SSCBRAH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSCBRAH_A {
        match self.bits {
            false => SSCBRAH_A::_0,
            true => SSCBRAH_A::_1,
        }
    }
    #[doc = "Counter start is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSCBRAH_A::_0
    }
    #[doc = "Counter start is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSCBRAH_A::_1
    }
}
#[doc = "Field `SSCBRAH` writer - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Start Enable"]
pub type SSCBRAH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SSCBRAH_A>;
impl<'a, REG, const O: u8> SSCBRAH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSCBRAH_A::_0)
    }
    #[doc = "Counter start is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSCBRAH_A::_1)
    }
}
#[doc = "Field `SSCBFAL` reader - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Start Enable"]
pub type SSCBFAL_R = crate::BitReader<SSCBFAL_A>;
#[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSCBFAL_A {
    #[doc = "0: Counter start is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 1,
}
impl From<SSCBFAL_A> for bool {
    #[inline(always)]
    fn from(variant: SSCBFAL_A) -> Self {
        variant as u8 != 0
    }
}
impl SSCBFAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSCBFAL_A {
        match self.bits {
            false => SSCBFAL_A::_0,
            true => SSCBFAL_A::_1,
        }
    }
    #[doc = "Counter start is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSCBFAL_A::_0
    }
    #[doc = "Counter start is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSCBFAL_A::_1
    }
}
#[doc = "Field `SSCBFAL` writer - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Start Enable"]
pub type SSCBFAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SSCBFAL_A>;
impl<'a, REG, const O: u8> SSCBFAL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSCBFAL_A::_0)
    }
    #[doc = "Counter start is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSCBFAL_A::_1)
    }
}
#[doc = "Field `SSCBFAH` reader - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Start Enable"]
pub type SSCBFAH_R = crate::BitReader<SSCBFAH_A>;
#[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSCBFAH_A {
    #[doc = "0: Counter start is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 1,
}
impl From<SSCBFAH_A> for bool {
    #[inline(always)]
    fn from(variant: SSCBFAH_A) -> Self {
        variant as u8 != 0
    }
}
impl SSCBFAH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSCBFAH_A {
        match self.bits {
            false => SSCBFAH_A::_0,
            true => SSCBFAH_A::_1,
        }
    }
    #[doc = "Counter start is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSCBFAH_A::_0
    }
    #[doc = "Counter start is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSCBFAH_A::_1
    }
}
#[doc = "Field `SSCBFAH` writer - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Start Enable"]
pub type SSCBFAH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SSCBFAH_A>;
impl<'a, REG, const O: u8> SSCBFAH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSCBFAH_A::_0)
    }
    #[doc = "Counter start is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSCBFAH_A::_1)
    }
}
#[doc = "Field `SSELCA` reader - ELC_GPTA Event Source Counter Start Enable"]
pub type SSELCA_R = crate::BitReader<SSELCA_A>;
#[doc = "ELC_GPTA Event Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSELCA_A {
    #[doc = "0: Counter start is disable at the ELC_GPTA input"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the ELC_GPTA input"]
    _1 = 1,
}
impl From<SSELCA_A> for bool {
    #[inline(always)]
    fn from(variant: SSELCA_A) -> Self {
        variant as u8 != 0
    }
}
impl SSELCA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSELCA_A {
        match self.bits {
            false => SSELCA_A::_0,
            true => SSELCA_A::_1,
        }
    }
    #[doc = "Counter start is disable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSELCA_A::_0
    }
    #[doc = "Counter start is enable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSELCA_A::_1
    }
}
#[doc = "Field `SSELCA` writer - ELC_GPTA Event Source Counter Start Enable"]
pub type SSELCA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SSELCA_A>;
impl<'a, REG, const O: u8> SSELCA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCA_A::_0)
    }
    #[doc = "Counter start is enable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCA_A::_1)
    }
}
#[doc = "Field `SSELCB` reader - ELC_GPTB Event Source Counter Start Enable"]
pub type SSELCB_R = crate::BitReader<SSELCB_A>;
#[doc = "ELC_GPTB Event Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSELCB_A {
    #[doc = "0: Counter start is disable at the ELC_GPTB input"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the ELC_GPTB input"]
    _1 = 1,
}
impl From<SSELCB_A> for bool {
    #[inline(always)]
    fn from(variant: SSELCB_A) -> Self {
        variant as u8 != 0
    }
}
impl SSELCB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSELCB_A {
        match self.bits {
            false => SSELCB_A::_0,
            true => SSELCB_A::_1,
        }
    }
    #[doc = "Counter start is disable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSELCB_A::_0
    }
    #[doc = "Counter start is enable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSELCB_A::_1
    }
}
#[doc = "Field `SSELCB` writer - ELC_GPTB Event Source Counter Start Enable"]
pub type SSELCB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SSELCB_A>;
impl<'a, REG, const O: u8> SSELCB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCB_A::_0)
    }
    #[doc = "Counter start is enable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCB_A::_1)
    }
}
#[doc = "Field `SSELCC` reader - ELC_GPTC Event Source Counter Start Enable"]
pub type SSELCC_R = crate::BitReader<SSELCC_A>;
#[doc = "ELC_GPTC Event Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSELCC_A {
    #[doc = "0: Counter start is disable at the ELC_GPTC input"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the ELC_GPTC input"]
    _1 = 1,
}
impl From<SSELCC_A> for bool {
    #[inline(always)]
    fn from(variant: SSELCC_A) -> Self {
        variant as u8 != 0
    }
}
impl SSELCC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSELCC_A {
        match self.bits {
            false => SSELCC_A::_0,
            true => SSELCC_A::_1,
        }
    }
    #[doc = "Counter start is disable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSELCC_A::_0
    }
    #[doc = "Counter start is enable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSELCC_A::_1
    }
}
#[doc = "Field `SSELCC` writer - ELC_GPTC Event Source Counter Start Enable"]
pub type SSELCC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SSELCC_A>;
impl<'a, REG, const O: u8> SSELCC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCC_A::_0)
    }
    #[doc = "Counter start is enable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCC_A::_1)
    }
}
#[doc = "Field `SSELCD` reader - ELC_GPTD Event Source Counter Start Enable"]
pub type SSELCD_R = crate::BitReader<SSELCD_A>;
#[doc = "ELC_GPTD Event Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSELCD_A {
    #[doc = "0: Counter start is disable at the ELC_GPTD input"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the ELC_GPTD input"]
    _1 = 1,
}
impl From<SSELCD_A> for bool {
    #[inline(always)]
    fn from(variant: SSELCD_A) -> Self {
        variant as u8 != 0
    }
}
impl SSELCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSELCD_A {
        match self.bits {
            false => SSELCD_A::_0,
            true => SSELCD_A::_1,
        }
    }
    #[doc = "Counter start is disable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSELCD_A::_0
    }
    #[doc = "Counter start is enable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSELCD_A::_1
    }
}
#[doc = "Field `SSELCD` writer - ELC_GPTD Event Source Counter Start Enable"]
pub type SSELCD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SSELCD_A>;
impl<'a, REG, const O: u8> SSELCD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCD_A::_0)
    }
    #[doc = "Counter start is enable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCD_A::_1)
    }
}
#[doc = "Field `SSELCE` reader - ELC_GPTE Event Source Counter Start Enable"]
pub type SSELCE_R = crate::BitReader<SSELCE_A>;
#[doc = "ELC_GPTE Event Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSELCE_A {
    #[doc = "0: Counter start is disable at the ELC_GPTE input"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the ELC_GPTE input"]
    _1 = 1,
}
impl From<SSELCE_A> for bool {
    #[inline(always)]
    fn from(variant: SSELCE_A) -> Self {
        variant as u8 != 0
    }
}
impl SSELCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSELCE_A {
        match self.bits {
            false => SSELCE_A::_0,
            true => SSELCE_A::_1,
        }
    }
    #[doc = "Counter start is disable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSELCE_A::_0
    }
    #[doc = "Counter start is enable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSELCE_A::_1
    }
}
#[doc = "Field `SSELCE` writer - ELC_GPTE Event Source Counter Start Enable"]
pub type SSELCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SSELCE_A>;
impl<'a, REG, const O: u8> SSELCE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCE_A::_0)
    }
    #[doc = "Counter start is enable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCE_A::_1)
    }
}
#[doc = "Field `SSELCF` reader - ELC_GPTF Event Source Counter Start Enable"]
pub type SSELCF_R = crate::BitReader<SSELCF_A>;
#[doc = "ELC_GPTF Event Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSELCF_A {
    #[doc = "0: Counter start is disable at the ELC_GPTF input"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the ELC_GPTF input"]
    _1 = 1,
}
impl From<SSELCF_A> for bool {
    #[inline(always)]
    fn from(variant: SSELCF_A) -> Self {
        variant as u8 != 0
    }
}
impl SSELCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSELCF_A {
        match self.bits {
            false => SSELCF_A::_0,
            true => SSELCF_A::_1,
        }
    }
    #[doc = "Counter start is disable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSELCF_A::_0
    }
    #[doc = "Counter start is enable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSELCF_A::_1
    }
}
#[doc = "Field `SSELCF` writer - ELC_GPTF Event Source Counter Start Enable"]
pub type SSELCF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SSELCF_A>;
impl<'a, REG, const O: u8> SSELCF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCF_A::_0)
    }
    #[doc = "Counter start is enable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCF_A::_1)
    }
}
#[doc = "Field `SSELCG` reader - ELC_GPTG Event Source Counter Start Enable"]
pub type SSELCG_R = crate::BitReader<SSELCG_A>;
#[doc = "ELC_GPTG Event Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSELCG_A {
    #[doc = "0: Counter start is disable at the ELC_GPTG input"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the ELC_GPTG input"]
    _1 = 1,
}
impl From<SSELCG_A> for bool {
    #[inline(always)]
    fn from(variant: SSELCG_A) -> Self {
        variant as u8 != 0
    }
}
impl SSELCG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSELCG_A {
        match self.bits {
            false => SSELCG_A::_0,
            true => SSELCG_A::_1,
        }
    }
    #[doc = "Counter start is disable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSELCG_A::_0
    }
    #[doc = "Counter start is enable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSELCG_A::_1
    }
}
#[doc = "Field `SSELCG` writer - ELC_GPTG Event Source Counter Start Enable"]
pub type SSELCG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SSELCG_A>;
impl<'a, REG, const O: u8> SSELCG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCG_A::_0)
    }
    #[doc = "Counter start is enable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCG_A::_1)
    }
}
#[doc = "Field `SSELCH` reader - ELCH Event Source Counter Start Enable"]
pub type SSELCH_R = crate::BitReader<SSELCH_A>;
#[doc = "ELCH Event Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSELCH_A {
    #[doc = "0: Counter start is disable at the ELC_GPTH input"]
    _0 = 0,
    #[doc = "1: Counter start is enable at the ELC_GPTH input"]
    _1 = 1,
}
impl From<SSELCH_A> for bool {
    #[inline(always)]
    fn from(variant: SSELCH_A) -> Self {
        variant as u8 != 0
    }
}
impl SSELCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSELCH_A {
        match self.bits {
            false => SSELCH_A::_0,
            true => SSELCH_A::_1,
        }
    }
    #[doc = "Counter start is disable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSELCH_A::_0
    }
    #[doc = "Counter start is enable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSELCH_A::_1
    }
}
#[doc = "Field `SSELCH` writer - ELCH Event Source Counter Start Enable"]
pub type SSELCH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SSELCH_A>;
impl<'a, REG, const O: u8> SSELCH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCH_A::_0)
    }
    #[doc = "Counter start is enable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCH_A::_1)
    }
}
#[doc = "Field `CSTRT` reader - Software Source Counter Start Enable"]
pub type CSTRT_R = crate::BitReader<CSTRT_A>;
#[doc = "Software Source Counter Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT_A {
    #[doc = "0: Counter start is disable by the GTSTR register"]
    _0 = 0,
    #[doc = "1: Counter start is enable by the GTSTR register"]
    _1 = 1,
}
impl From<CSTRT_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT_A {
        match self.bits {
            false => CSTRT_A::_0,
            true => CSTRT_A::_1,
        }
    }
    #[doc = "Counter start is disable by the GTSTR register"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT_A::_0
    }
    #[doc = "Counter start is enable by the GTSTR register"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT_A::_1
    }
}
#[doc = "Field `CSTRT` writer - Software Source Counter Start Enable"]
pub type CSTRT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSTRT_A>;
impl<'a, REG, const O: u8> CSTRT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter start is disable by the GTSTR register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT_A::_0)
    }
    #[doc = "Counter start is enable by the GTSTR register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source Counter Start Enable"]
    #[inline(always)]
    pub fn ssgtrgar(&self) -> SSGTRGAR_R {
        SSGTRGAR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source Counter Start Enable"]
    #[inline(always)]
    pub fn ssgtrgaf(&self) -> SSGTRGAF_R {
        SSGTRGAF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source Counter Start Enable"]
    #[inline(always)]
    pub fn ssgtrgbr(&self) -> SSGTRGBR_R {
        SSGTRGBR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source Counter Start Enable"]
    #[inline(always)]
    pub fn ssgtrgbf(&self) -> SSGTRGBF_R {
        SSGTRGBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscarbl(&self) -> SSCARBL_R {
        SSCARBL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscarbh(&self) -> SSCARBH_R {
        SSCARBH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscafbl(&self) -> SSCAFBL_R {
        SSCAFBL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscafbh(&self) -> SSCAFBH_R {
        SSCAFBH_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscbral(&self) -> SSCBRAL_R {
        SSCBRAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscbrah(&self) -> SSCBRAH_R {
        SSCBRAH_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscbfal(&self) -> SSCBFAL_R {
        SSCBFAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscbfah(&self) -> SSCBFAH_R {
        SSCBFAH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselca(&self) -> SSELCA_R {
        SSELCA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselcb(&self) -> SSELCB_R {
        SSELCB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselcc(&self) -> SSELCC_R {
        SSELCC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselcd(&self) -> SSELCD_R {
        SSELCD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ELC_GPTE Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselce(&self) -> SSELCE_R {
        SSELCE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ELC_GPTF Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselcf(&self) -> SSELCF_R {
        SSELCF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ELC_GPTG Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselcg(&self) -> SSELCG_R {
        SSELCG_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ELCH Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselch(&self) -> SSELCH_R {
        SSELCH_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - Software Source Counter Start Enable"]
    #[inline(always)]
    pub fn cstrt(&self) -> CSTRT_R {
        CSTRT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source Counter Start Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssgtrgar(&mut self) -> SSGTRGAR_W<GTSSR_SPEC, 0> {
        SSGTRGAR_W::new(self)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source Counter Start Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssgtrgaf(&mut self) -> SSGTRGAF_W<GTSSR_SPEC, 1> {
        SSGTRGAF_W::new(self)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source Counter Start Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssgtrgbr(&mut self) -> SSGTRGBR_W<GTSSR_SPEC, 2> {
        SSGTRGBR_W::new(self)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source Counter Start Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssgtrgbf(&mut self) -> SSGTRGBF_W<GTSSR_SPEC, 3> {
        SSGTRGBF_W::new(self)
    }
    #[doc = "Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Start Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sscarbl(&mut self) -> SSCARBL_W<GTSSR_SPEC, 8> {
        SSCARBL_W::new(self)
    }
    #[doc = "Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Start Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sscarbh(&mut self) -> SSCARBH_W<GTSSR_SPEC, 9> {
        SSCARBH_W::new(self)
    }
    #[doc = "Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Start Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sscafbl(&mut self) -> SSCAFBL_W<GTSSR_SPEC, 10> {
        SSCAFBL_W::new(self)
    }
    #[doc = "Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Start Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sscafbh(&mut self) -> SSCAFBH_W<GTSSR_SPEC, 11> {
        SSCAFBH_W::new(self)
    }
    #[doc = "Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Start Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sscbral(&mut self) -> SSCBRAL_W<GTSSR_SPEC, 12> {
        SSCBRAL_W::new(self)
    }
    #[doc = "Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Start Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sscbrah(&mut self) -> SSCBRAH_W<GTSSR_SPEC, 13> {
        SSCBRAH_W::new(self)
    }
    #[doc = "Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Start Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sscbfal(&mut self) -> SSCBFAL_W<GTSSR_SPEC, 14> {
        SSCBFAL_W::new(self)
    }
    #[doc = "Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Start Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sscbfah(&mut self) -> SSCBFAH_W<GTSSR_SPEC, 15> {
        SSCBFAH_W::new(self)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source Counter Start Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sselca(&mut self) -> SSELCA_W<GTSSR_SPEC, 16> {
        SSELCA_W::new(self)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source Counter Start Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sselcb(&mut self) -> SSELCB_W<GTSSR_SPEC, 17> {
        SSELCB_W::new(self)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source Counter Start Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sselcc(&mut self) -> SSELCC_W<GTSSR_SPEC, 18> {
        SSELCC_W::new(self)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source Counter Start Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sselcd(&mut self) -> SSELCD_W<GTSSR_SPEC, 19> {
        SSELCD_W::new(self)
    }
    #[doc = "Bit 20 - ELC_GPTE Event Source Counter Start Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sselce(&mut self) -> SSELCE_W<GTSSR_SPEC, 20> {
        SSELCE_W::new(self)
    }
    #[doc = "Bit 21 - ELC_GPTF Event Source Counter Start Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sselcf(&mut self) -> SSELCF_W<GTSSR_SPEC, 21> {
        SSELCF_W::new(self)
    }
    #[doc = "Bit 22 - ELC_GPTG Event Source Counter Start Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sselcg(&mut self) -> SSELCG_W<GTSSR_SPEC, 22> {
        SSELCG_W::new(self)
    }
    #[doc = "Bit 23 - ELCH Event Source Counter Start Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sselch(&mut self) -> SSELCH_W<GTSSR_SPEC, 23> {
        SSELCH_W::new(self)
    }
    #[doc = "Bit 31 - Software Source Counter Start Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cstrt(&mut self) -> CSTRT_W<GTSSR_SPEC, 31> {
        CSTRT_W::new(self)
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
#[doc = "General PWM Timer Start Source Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtssr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtssr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTSSR_SPEC;
impl crate::RegisterSpec for GTSSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtssr::R`](R) reader structure"]
impl crate::Readable for GTSSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gtssr::W`](W) writer structure"]
impl crate::Writable for GTSSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTSSR to value 0"]
impl crate::Resettable for GTSSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
