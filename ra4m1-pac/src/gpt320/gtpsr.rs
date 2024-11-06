#[doc = "Register `GTPSR` reader"]
pub type R = crate::R<GTPSR_SPEC>;
#[doc = "Register `GTPSR` writer"]
pub type W = crate::W<GTPSR_SPEC>;
#[doc = "Field `PSGTRGAR` reader - GTETRGA Pin Rising Input Source Counter Stop Enable"]
pub type PSGTRGAR_R = crate::BitReader<PSGTRGAR_A>;
#[doc = "GTETRGA Pin Rising Input Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSGTRGAR_A {
    #[doc = "0: Counter stop is disable at the rising edge of GTETRGA input"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the rising edge of GTETRGA input"]
    _1 = 1,
}
impl From<PSGTRGAR_A> for bool {
    #[inline(always)]
    fn from(variant: PSGTRGAR_A) -> Self {
        variant as u8 != 0
    }
}
impl PSGTRGAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSGTRGAR_A {
        match self.bits {
            false => PSGTRGAR_A::_0,
            true => PSGTRGAR_A::_1,
        }
    }
    #[doc = "Counter stop is disable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSGTRGAR_A::_0
    }
    #[doc = "Counter stop is enable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSGTRGAR_A::_1
    }
}
#[doc = "Field `PSGTRGAR` writer - GTETRGA Pin Rising Input Source Counter Stop Enable"]
pub type PSGTRGAR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PSGTRGAR_A>;
impl<'a, REG, const O: u8> PSGTRGAR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSGTRGAR_A::_0)
    }
    #[doc = "Counter stop is enable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSGTRGAR_A::_1)
    }
}
#[doc = "Field `PSGTRGAF` reader - GTETRGA Pin Falling Input Source Counter Stop Enable"]
pub type PSGTRGAF_R = crate::BitReader<PSGTRGAF_A>;
#[doc = "GTETRGA Pin Falling Input Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSGTRGAF_A {
    #[doc = "0: Counter stop is disable at the falling edge of GTETRGA input"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the falling edge of GTETRGA input"]
    _1 = 1,
}
impl From<PSGTRGAF_A> for bool {
    #[inline(always)]
    fn from(variant: PSGTRGAF_A) -> Self {
        variant as u8 != 0
    }
}
impl PSGTRGAF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSGTRGAF_A {
        match self.bits {
            false => PSGTRGAF_A::_0,
            true => PSGTRGAF_A::_1,
        }
    }
    #[doc = "Counter stop is disable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSGTRGAF_A::_0
    }
    #[doc = "Counter stop is enable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSGTRGAF_A::_1
    }
}
#[doc = "Field `PSGTRGAF` writer - GTETRGA Pin Falling Input Source Counter Stop Enable"]
pub type PSGTRGAF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PSGTRGAF_A>;
impl<'a, REG, const O: u8> PSGTRGAF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSGTRGAF_A::_0)
    }
    #[doc = "Counter stop is enable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSGTRGAF_A::_1)
    }
}
#[doc = "Field `PSGTRGBR` reader - GTETRGB Pin Rising Input Source Counter Stop Enable"]
pub type PSGTRGBR_R = crate::BitReader<PSGTRGBR_A>;
#[doc = "GTETRGB Pin Rising Input Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSGTRGBR_A {
    #[doc = "0: Counter stop is disable at the rising edge of GTETRGB input"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the rising edge of GTETRGB input"]
    _1 = 1,
}
impl From<PSGTRGBR_A> for bool {
    #[inline(always)]
    fn from(variant: PSGTRGBR_A) -> Self {
        variant as u8 != 0
    }
}
impl PSGTRGBR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSGTRGBR_A {
        match self.bits {
            false => PSGTRGBR_A::_0,
            true => PSGTRGBR_A::_1,
        }
    }
    #[doc = "Counter stop is disable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSGTRGBR_A::_0
    }
    #[doc = "Counter stop is enable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSGTRGBR_A::_1
    }
}
#[doc = "Field `PSGTRGBR` writer - GTETRGB Pin Rising Input Source Counter Stop Enable"]
pub type PSGTRGBR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PSGTRGBR_A>;
impl<'a, REG, const O: u8> PSGTRGBR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSGTRGBR_A::_0)
    }
    #[doc = "Counter stop is enable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSGTRGBR_A::_1)
    }
}
#[doc = "Field `PSGTRGBF` reader - GTETRGB Pin Falling Input Source Counter Stop Enable"]
pub type PSGTRGBF_R = crate::BitReader<PSGTRGBF_A>;
#[doc = "GTETRGB Pin Falling Input Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSGTRGBF_A {
    #[doc = "0: Counter stop is disable at the falling edge of GTETRGB input"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the falling edge of GTETRGB input"]
    _1 = 1,
}
impl From<PSGTRGBF_A> for bool {
    #[inline(always)]
    fn from(variant: PSGTRGBF_A) -> Self {
        variant as u8 != 0
    }
}
impl PSGTRGBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSGTRGBF_A {
        match self.bits {
            false => PSGTRGBF_A::_0,
            true => PSGTRGBF_A::_1,
        }
    }
    #[doc = "Counter stop is disable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSGTRGBF_A::_0
    }
    #[doc = "Counter stop is enable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSGTRGBF_A::_1
    }
}
#[doc = "Field `PSGTRGBF` writer - GTETRGB Pin Falling Input Source Counter Stop Enable"]
pub type PSGTRGBF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PSGTRGBF_A>;
impl<'a, REG, const O: u8> PSGTRGBF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSGTRGBF_A::_0)
    }
    #[doc = "Counter stop is enable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSGTRGBF_A::_1)
    }
}
#[doc = "Field `PSCARBL` reader - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Stop Enable"]
pub type PSCARBL_R = crate::BitReader<PSCARBL_A>;
#[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSCARBL_A {
    #[doc = "0: Counter stop is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 1,
}
impl From<PSCARBL_A> for bool {
    #[inline(always)]
    fn from(variant: PSCARBL_A) -> Self {
        variant as u8 != 0
    }
}
impl PSCARBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSCARBL_A {
        match self.bits {
            false => PSCARBL_A::_0,
            true => PSCARBL_A::_1,
        }
    }
    #[doc = "Counter stop is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSCARBL_A::_0
    }
    #[doc = "Counter stop is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSCARBL_A::_1
    }
}
#[doc = "Field `PSCARBL` writer - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Stop Enable"]
pub type PSCARBL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PSCARBL_A>;
impl<'a, REG, const O: u8> PSCARBL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSCARBL_A::_0)
    }
    #[doc = "Counter stop is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSCARBL_A::_1)
    }
}
#[doc = "Field `PSCARBH` reader - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Stop Enable"]
pub type PSCARBH_R = crate::BitReader<PSCARBH_A>;
#[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSCARBH_A {
    #[doc = "0: Counter stop is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 1,
}
impl From<PSCARBH_A> for bool {
    #[inline(always)]
    fn from(variant: PSCARBH_A) -> Self {
        variant as u8 != 0
    }
}
impl PSCARBH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSCARBH_A {
        match self.bits {
            false => PSCARBH_A::_0,
            true => PSCARBH_A::_1,
        }
    }
    #[doc = "Counter stop is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSCARBH_A::_0
    }
    #[doc = "Counter stop is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSCARBH_A::_1
    }
}
#[doc = "Field `PSCARBH` writer - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Stop Enable"]
pub type PSCARBH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PSCARBH_A>;
impl<'a, REG, const O: u8> PSCARBH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSCARBH_A::_0)
    }
    #[doc = "Counter stop is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSCARBH_A::_1)
    }
}
#[doc = "Field `PSCAFBL` reader - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Stop Enable"]
pub type PSCAFBL_R = crate::BitReader<PSCAFBL_A>;
#[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSCAFBL_A {
    #[doc = "0: Counter stop is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 1,
}
impl From<PSCAFBL_A> for bool {
    #[inline(always)]
    fn from(variant: PSCAFBL_A) -> Self {
        variant as u8 != 0
    }
}
impl PSCAFBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSCAFBL_A {
        match self.bits {
            false => PSCAFBL_A::_0,
            true => PSCAFBL_A::_1,
        }
    }
    #[doc = "Counter stop is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSCAFBL_A::_0
    }
    #[doc = "Counter stop is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSCAFBL_A::_1
    }
}
#[doc = "Field `PSCAFBL` writer - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Stop Enable"]
pub type PSCAFBL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PSCAFBL_A>;
impl<'a, REG, const O: u8> PSCAFBL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSCAFBL_A::_0)
    }
    #[doc = "Counter stop is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSCAFBL_A::_1)
    }
}
#[doc = "Field `PSCAFBH` reader - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Stop Enable"]
pub type PSCAFBH_R = crate::BitReader<PSCAFBH_A>;
#[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSCAFBH_A {
    #[doc = "0: Counter stop is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 1,
}
impl From<PSCAFBH_A> for bool {
    #[inline(always)]
    fn from(variant: PSCAFBH_A) -> Self {
        variant as u8 != 0
    }
}
impl PSCAFBH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSCAFBH_A {
        match self.bits {
            false => PSCAFBH_A::_0,
            true => PSCAFBH_A::_1,
        }
    }
    #[doc = "Counter stop is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSCAFBH_A::_0
    }
    #[doc = "Counter stop is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSCAFBH_A::_1
    }
}
#[doc = "Field `PSCAFBH` writer - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Stop Enable"]
pub type PSCAFBH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PSCAFBH_A>;
impl<'a, REG, const O: u8> PSCAFBH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSCAFBH_A::_0)
    }
    #[doc = "Counter stop is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSCAFBH_A::_1)
    }
}
#[doc = "Field `PSCBRAL` reader - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Stop Enable"]
pub type PSCBRAL_R = crate::BitReader<PSCBRAL_A>;
#[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSCBRAL_A {
    #[doc = "0: Counter stop is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 1,
}
impl From<PSCBRAL_A> for bool {
    #[inline(always)]
    fn from(variant: PSCBRAL_A) -> Self {
        variant as u8 != 0
    }
}
impl PSCBRAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSCBRAL_A {
        match self.bits {
            false => PSCBRAL_A::_0,
            true => PSCBRAL_A::_1,
        }
    }
    #[doc = "Counter stop is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSCBRAL_A::_0
    }
    #[doc = "Counter stop is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSCBRAL_A::_1
    }
}
#[doc = "Field `PSCBRAL` writer - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Stop Enable"]
pub type PSCBRAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PSCBRAL_A>;
impl<'a, REG, const O: u8> PSCBRAL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSCBRAL_A::_0)
    }
    #[doc = "Counter stop is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSCBRAL_A::_1)
    }
}
#[doc = "Field `PSCBRAH` reader - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Stop Enable"]
pub type PSCBRAH_R = crate::BitReader<PSCBRAH_A>;
#[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSCBRAH_A {
    #[doc = "0: Counter stop is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 1,
}
impl From<PSCBRAH_A> for bool {
    #[inline(always)]
    fn from(variant: PSCBRAH_A) -> Self {
        variant as u8 != 0
    }
}
impl PSCBRAH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSCBRAH_A {
        match self.bits {
            false => PSCBRAH_A::_0,
            true => PSCBRAH_A::_1,
        }
    }
    #[doc = "Counter stop is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSCBRAH_A::_0
    }
    #[doc = "Counter stop is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSCBRAH_A::_1
    }
}
#[doc = "Field `PSCBRAH` writer - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Stop Enable"]
pub type PSCBRAH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PSCBRAH_A>;
impl<'a, REG, const O: u8> PSCBRAH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSCBRAH_A::_0)
    }
    #[doc = "Counter stop is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSCBRAH_A::_1)
    }
}
#[doc = "Field `PSCBFAL` reader - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Stop Enable"]
pub type PSCBFAL_R = crate::BitReader<PSCBFAL_A>;
#[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSCBFAL_A {
    #[doc = "0: Counter stop is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 1,
}
impl From<PSCBFAL_A> for bool {
    #[inline(always)]
    fn from(variant: PSCBFAL_A) -> Self {
        variant as u8 != 0
    }
}
impl PSCBFAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSCBFAL_A {
        match self.bits {
            false => PSCBFAL_A::_0,
            true => PSCBFAL_A::_1,
        }
    }
    #[doc = "Counter stop is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSCBFAL_A::_0
    }
    #[doc = "Counter stop is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSCBFAL_A::_1
    }
}
#[doc = "Field `PSCBFAL` writer - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Stop Enable"]
pub type PSCBFAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PSCBFAL_A>;
impl<'a, REG, const O: u8> PSCBFAL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSCBFAL_A::_0)
    }
    #[doc = "Counter stop is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSCBFAL_A::_1)
    }
}
#[doc = "Field `PSCBFAH` reader - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Stop Enable"]
pub type PSCBFAH_R = crate::BitReader<PSCBFAH_A>;
#[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSCBFAH_A {
    #[doc = "0: Counter stop is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 1,
}
impl From<PSCBFAH_A> for bool {
    #[inline(always)]
    fn from(variant: PSCBFAH_A) -> Self {
        variant as u8 != 0
    }
}
impl PSCBFAH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSCBFAH_A {
        match self.bits {
            false => PSCBFAH_A::_0,
            true => PSCBFAH_A::_1,
        }
    }
    #[doc = "Counter stop is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSCBFAH_A::_0
    }
    #[doc = "Counter stop is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSCBFAH_A::_1
    }
}
#[doc = "Field `PSCBFAH` writer - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Stop Enable"]
pub type PSCBFAH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PSCBFAH_A>;
impl<'a, REG, const O: u8> PSCBFAH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSCBFAH_A::_0)
    }
    #[doc = "Counter stop is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSCBFAH_A::_1)
    }
}
#[doc = "Field `PSELCA` reader - ELC_GPTA Event Source Counter Stop Enable"]
pub type PSELCA_R = crate::BitReader<PSELCA_A>;
#[doc = "ELC_GPTA Event Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSELCA_A {
    #[doc = "0: Counter stop is disable at the ELC_GPTA input"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the ELC_GPTA input"]
    _1 = 1,
}
impl From<PSELCA_A> for bool {
    #[inline(always)]
    fn from(variant: PSELCA_A) -> Self {
        variant as u8 != 0
    }
}
impl PSELCA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSELCA_A {
        match self.bits {
            false => PSELCA_A::_0,
            true => PSELCA_A::_1,
        }
    }
    #[doc = "Counter stop is disable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSELCA_A::_0
    }
    #[doc = "Counter stop is enable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSELCA_A::_1
    }
}
#[doc = "Field `PSELCA` writer - ELC_GPTA Event Source Counter Stop Enable"]
pub type PSELCA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PSELCA_A>;
impl<'a, REG, const O: u8> PSELCA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCA_A::_0)
    }
    #[doc = "Counter stop is enable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCA_A::_1)
    }
}
#[doc = "Field `PSELCB` reader - ELC_GPTB Event Source Counter Stop Enable"]
pub type PSELCB_R = crate::BitReader<PSELCB_A>;
#[doc = "ELC_GPTB Event Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSELCB_A {
    #[doc = "0: Counter stop is disable at the ELC_GPTB input"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the ELC_GPTB input"]
    _1 = 1,
}
impl From<PSELCB_A> for bool {
    #[inline(always)]
    fn from(variant: PSELCB_A) -> Self {
        variant as u8 != 0
    }
}
impl PSELCB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSELCB_A {
        match self.bits {
            false => PSELCB_A::_0,
            true => PSELCB_A::_1,
        }
    }
    #[doc = "Counter stop is disable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSELCB_A::_0
    }
    #[doc = "Counter stop is enable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSELCB_A::_1
    }
}
#[doc = "Field `PSELCB` writer - ELC_GPTB Event Source Counter Stop Enable"]
pub type PSELCB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PSELCB_A>;
impl<'a, REG, const O: u8> PSELCB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCB_A::_0)
    }
    #[doc = "Counter stop is enable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCB_A::_1)
    }
}
#[doc = "Field `PSELCC` reader - ELC_GPTC Event Source Counter Stop Enable"]
pub type PSELCC_R = crate::BitReader<PSELCC_A>;
#[doc = "ELC_GPTC Event Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSELCC_A {
    #[doc = "0: Counter stop is disable at the ELC_GPTC input"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the ELC_GPTC input"]
    _1 = 1,
}
impl From<PSELCC_A> for bool {
    #[inline(always)]
    fn from(variant: PSELCC_A) -> Self {
        variant as u8 != 0
    }
}
impl PSELCC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSELCC_A {
        match self.bits {
            false => PSELCC_A::_0,
            true => PSELCC_A::_1,
        }
    }
    #[doc = "Counter stop is disable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSELCC_A::_0
    }
    #[doc = "Counter stop is enable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSELCC_A::_1
    }
}
#[doc = "Field `PSELCC` writer - ELC_GPTC Event Source Counter Stop Enable"]
pub type PSELCC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PSELCC_A>;
impl<'a, REG, const O: u8> PSELCC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCC_A::_0)
    }
    #[doc = "Counter stop is enable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCC_A::_1)
    }
}
#[doc = "Field `PSELCD` reader - ELC_GPTD Event Source Counter Stop Enable"]
pub type PSELCD_R = crate::BitReader<PSELCD_A>;
#[doc = "ELC_GPTD Event Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSELCD_A {
    #[doc = "0: Counter stop is disable at the ELC_GPTD input"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the ELC_GPTD input"]
    _1 = 1,
}
impl From<PSELCD_A> for bool {
    #[inline(always)]
    fn from(variant: PSELCD_A) -> Self {
        variant as u8 != 0
    }
}
impl PSELCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSELCD_A {
        match self.bits {
            false => PSELCD_A::_0,
            true => PSELCD_A::_1,
        }
    }
    #[doc = "Counter stop is disable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSELCD_A::_0
    }
    #[doc = "Counter stop is enable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSELCD_A::_1
    }
}
#[doc = "Field `PSELCD` writer - ELC_GPTD Event Source Counter Stop Enable"]
pub type PSELCD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PSELCD_A>;
impl<'a, REG, const O: u8> PSELCD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCD_A::_0)
    }
    #[doc = "Counter stop is enable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCD_A::_1)
    }
}
#[doc = "Field `PSELCE` reader - ELC_GPTE Event Source Counter Stop Enable"]
pub type PSELCE_R = crate::BitReader<PSELCE_A>;
#[doc = "ELC_GPTE Event Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSELCE_A {
    #[doc = "0: Counter stop is disable at the ELC_GPTE input"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the ELC_GPTE input"]
    _1 = 1,
}
impl From<PSELCE_A> for bool {
    #[inline(always)]
    fn from(variant: PSELCE_A) -> Self {
        variant as u8 != 0
    }
}
impl PSELCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSELCE_A {
        match self.bits {
            false => PSELCE_A::_0,
            true => PSELCE_A::_1,
        }
    }
    #[doc = "Counter stop is disable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSELCE_A::_0
    }
    #[doc = "Counter stop is enable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSELCE_A::_1
    }
}
#[doc = "Field `PSELCE` writer - ELC_GPTE Event Source Counter Stop Enable"]
pub type PSELCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PSELCE_A>;
impl<'a, REG, const O: u8> PSELCE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCE_A::_0)
    }
    #[doc = "Counter stop is enable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCE_A::_1)
    }
}
#[doc = "Field `PSELCF` reader - ELC_GPTF Event Source Counter Stop Enable"]
pub type PSELCF_R = crate::BitReader<PSELCF_A>;
#[doc = "ELC_GPTF Event Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSELCF_A {
    #[doc = "0: Counter stop is disable at the ELC_GPTF input"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the ELC_GPTF input"]
    _1 = 1,
}
impl From<PSELCF_A> for bool {
    #[inline(always)]
    fn from(variant: PSELCF_A) -> Self {
        variant as u8 != 0
    }
}
impl PSELCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSELCF_A {
        match self.bits {
            false => PSELCF_A::_0,
            true => PSELCF_A::_1,
        }
    }
    #[doc = "Counter stop is disable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSELCF_A::_0
    }
    #[doc = "Counter stop is enable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSELCF_A::_1
    }
}
#[doc = "Field `PSELCF` writer - ELC_GPTF Event Source Counter Stop Enable"]
pub type PSELCF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PSELCF_A>;
impl<'a, REG, const O: u8> PSELCF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCF_A::_0)
    }
    #[doc = "Counter stop is enable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCF_A::_1)
    }
}
#[doc = "Field `PSELCG` reader - ELC_GPTG Event Source Counter Stop Enable"]
pub type PSELCG_R = crate::BitReader<PSELCG_A>;
#[doc = "ELC_GPTG Event Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSELCG_A {
    #[doc = "0: Counter stop is disable at the ELC_GPTG input"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the ELC_GPTG input"]
    _1 = 1,
}
impl From<PSELCG_A> for bool {
    #[inline(always)]
    fn from(variant: PSELCG_A) -> Self {
        variant as u8 != 0
    }
}
impl PSELCG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSELCG_A {
        match self.bits {
            false => PSELCG_A::_0,
            true => PSELCG_A::_1,
        }
    }
    #[doc = "Counter stop is disable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSELCG_A::_0
    }
    #[doc = "Counter stop is enable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSELCG_A::_1
    }
}
#[doc = "Field `PSELCG` writer - ELC_GPTG Event Source Counter Stop Enable"]
pub type PSELCG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PSELCG_A>;
impl<'a, REG, const O: u8> PSELCG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCG_A::_0)
    }
    #[doc = "Counter stop is enable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCG_A::_1)
    }
}
#[doc = "Field `PSELCH` reader - ELCH Event Source Counter Stop Enable"]
pub type PSELCH_R = crate::BitReader<PSELCH_A>;
#[doc = "ELCH Event Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSELCH_A {
    #[doc = "0: Counter stop is disable at the ELC_GPTH input"]
    _0 = 0,
    #[doc = "1: Counter stop is enable at the ELC_GPTH input"]
    _1 = 1,
}
impl From<PSELCH_A> for bool {
    #[inline(always)]
    fn from(variant: PSELCH_A) -> Self {
        variant as u8 != 0
    }
}
impl PSELCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSELCH_A {
        match self.bits {
            false => PSELCH_A::_0,
            true => PSELCH_A::_1,
        }
    }
    #[doc = "Counter stop is disable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSELCH_A::_0
    }
    #[doc = "Counter stop is enable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSELCH_A::_1
    }
}
#[doc = "Field `PSELCH` writer - ELCH Event Source Counter Stop Enable"]
pub type PSELCH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PSELCH_A>;
impl<'a, REG, const O: u8> PSELCH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCH_A::_0)
    }
    #[doc = "Counter stop is enable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCH_A::_1)
    }
}
#[doc = "Field `CSTOP` reader - Software Source Counter Stop Enable"]
pub type CSTOP_R = crate::BitReader<CSTOP_A>;
#[doc = "Software Source Counter Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP_A {
    #[doc = "0: Counter stop is disable by the GTSTP register"]
    _0 = 0,
    #[doc = "1: Counter stop is enable by the GTSTP register"]
    _1 = 1,
}
impl From<CSTOP_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSTOP_A {
        match self.bits {
            false => CSTOP_A::_0,
            true => CSTOP_A::_1,
        }
    }
    #[doc = "Counter stop is disable by the GTSTP register"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP_A::_0
    }
    #[doc = "Counter stop is enable by the GTSTP register"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP_A::_1
    }
}
#[doc = "Field `CSTOP` writer - Software Source Counter Stop Enable"]
pub type CSTOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSTOP_A>;
impl<'a, REG, const O: u8> CSTOP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter stop is disable by the GTSTP register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP_A::_0)
    }
    #[doc = "Counter stop is enable by the GTSTP register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source Counter Stop Enable"]
    #[inline(always)]
    pub fn psgtrgar(&self) -> PSGTRGAR_R {
        PSGTRGAR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source Counter Stop Enable"]
    #[inline(always)]
    pub fn psgtrgaf(&self) -> PSGTRGAF_R {
        PSGTRGAF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source Counter Stop Enable"]
    #[inline(always)]
    pub fn psgtrgbr(&self) -> PSGTRGBR_R {
        PSGTRGBR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source Counter Stop Enable"]
    #[inline(always)]
    pub fn psgtrgbf(&self) -> PSGTRGBF_R {
        PSGTRGBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscarbl(&self) -> PSCARBL_R {
        PSCARBL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscarbh(&self) -> PSCARBH_R {
        PSCARBH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscafbl(&self) -> PSCAFBL_R {
        PSCAFBL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscafbh(&self) -> PSCAFBH_R {
        PSCAFBH_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscbral(&self) -> PSCBRAL_R {
        PSCBRAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscbrah(&self) -> PSCBRAH_R {
        PSCBRAH_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscbfal(&self) -> PSCBFAL_R {
        PSCBFAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscbfah(&self) -> PSCBFAH_R {
        PSCBFAH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselca(&self) -> PSELCA_R {
        PSELCA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselcb(&self) -> PSELCB_R {
        PSELCB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselcc(&self) -> PSELCC_R {
        PSELCC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselcd(&self) -> PSELCD_R {
        PSELCD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ELC_GPTE Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselce(&self) -> PSELCE_R {
        PSELCE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ELC_GPTF Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselcf(&self) -> PSELCF_R {
        PSELCF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ELC_GPTG Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselcg(&self) -> PSELCG_R {
        PSELCG_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ELCH Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselch(&self) -> PSELCH_R {
        PSELCH_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - Software Source Counter Stop Enable"]
    #[inline(always)]
    pub fn cstop(&self) -> CSTOP_R {
        CSTOP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source Counter Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn psgtrgar(&mut self) -> PSGTRGAR_W<GTPSR_SPEC, 0> {
        PSGTRGAR_W::new(self)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source Counter Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn psgtrgaf(&mut self) -> PSGTRGAF_W<GTPSR_SPEC, 1> {
        PSGTRGAF_W::new(self)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source Counter Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn psgtrgbr(&mut self) -> PSGTRGBR_W<GTPSR_SPEC, 2> {
        PSGTRGBR_W::new(self)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source Counter Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn psgtrgbf(&mut self) -> PSGTRGBF_W<GTPSR_SPEC, 3> {
        PSGTRGBF_W::new(self)
    }
    #[doc = "Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pscarbl(&mut self) -> PSCARBL_W<GTPSR_SPEC, 8> {
        PSCARBL_W::new(self)
    }
    #[doc = "Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pscarbh(&mut self) -> PSCARBH_W<GTPSR_SPEC, 9> {
        PSCARBH_W::new(self)
    }
    #[doc = "Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pscafbl(&mut self) -> PSCAFBL_W<GTPSR_SPEC, 10> {
        PSCAFBL_W::new(self)
    }
    #[doc = "Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pscafbh(&mut self) -> PSCAFBH_W<GTPSR_SPEC, 11> {
        PSCAFBH_W::new(self)
    }
    #[doc = "Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pscbral(&mut self) -> PSCBRAL_W<GTPSR_SPEC, 12> {
        PSCBRAL_W::new(self)
    }
    #[doc = "Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pscbrah(&mut self) -> PSCBRAH_W<GTPSR_SPEC, 13> {
        PSCBRAH_W::new(self)
    }
    #[doc = "Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pscbfal(&mut self) -> PSCBFAL_W<GTPSR_SPEC, 14> {
        PSCBFAL_W::new(self)
    }
    #[doc = "Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pscbfah(&mut self) -> PSCBFAH_W<GTPSR_SPEC, 15> {
        PSCBFAH_W::new(self)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source Counter Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pselca(&mut self) -> PSELCA_W<GTPSR_SPEC, 16> {
        PSELCA_W::new(self)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source Counter Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pselcb(&mut self) -> PSELCB_W<GTPSR_SPEC, 17> {
        PSELCB_W::new(self)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source Counter Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pselcc(&mut self) -> PSELCC_W<GTPSR_SPEC, 18> {
        PSELCC_W::new(self)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source Counter Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pselcd(&mut self) -> PSELCD_W<GTPSR_SPEC, 19> {
        PSELCD_W::new(self)
    }
    #[doc = "Bit 20 - ELC_GPTE Event Source Counter Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pselce(&mut self) -> PSELCE_W<GTPSR_SPEC, 20> {
        PSELCE_W::new(self)
    }
    #[doc = "Bit 21 - ELC_GPTF Event Source Counter Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pselcf(&mut self) -> PSELCF_W<GTPSR_SPEC, 21> {
        PSELCF_W::new(self)
    }
    #[doc = "Bit 22 - ELC_GPTG Event Source Counter Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pselcg(&mut self) -> PSELCG_W<GTPSR_SPEC, 22> {
        PSELCG_W::new(self)
    }
    #[doc = "Bit 23 - ELCH Event Source Counter Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pselch(&mut self) -> PSELCH_W<GTPSR_SPEC, 23> {
        PSELCH_W::new(self)
    }
    #[doc = "Bit 31 - Software Source Counter Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cstop(&mut self) -> CSTOP_W<GTPSR_SPEC, 31> {
        CSTOP_W::new(self)
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
#[doc = "General PWM Timer Stop Source Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtpsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtpsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTPSR_SPEC;
impl crate::RegisterSpec for GTPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtpsr::R`](R) reader structure"]
impl crate::Readable for GTPSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gtpsr::W`](W) writer structure"]
impl crate::Writable for GTPSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTPSR to value 0"]
impl crate::Resettable for GTPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
