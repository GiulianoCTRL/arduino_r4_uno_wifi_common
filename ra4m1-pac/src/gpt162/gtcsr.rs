#[doc = "Register `GTCSR` reader"]
pub type R = crate::R<GTCSR_SPEC>;
#[doc = "Register `GTCSR` writer"]
pub type W = crate::W<GTCSR_SPEC>;
#[doc = "Field `CSGTRGAR` reader - GTETRGA Pin Rising Input Source Counter Clear Enable"]
pub type CSGTRGAR_R = crate::BitReader<CSGTRGAR_A>;
#[doc = "GTETRGA Pin Rising Input Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSGTRGAR_A {
    #[doc = "0: Counter clear is disable at the rising edge of GTETRGA input"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the rising edge of GTETRGA input"]
    _1 = 1,
}
impl From<CSGTRGAR_A> for bool {
    #[inline(always)]
    fn from(variant: CSGTRGAR_A) -> Self {
        variant as u8 != 0
    }
}
impl CSGTRGAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSGTRGAR_A {
        match self.bits {
            false => CSGTRGAR_A::_0,
            true => CSGTRGAR_A::_1,
        }
    }
    #[doc = "Counter clear is disable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSGTRGAR_A::_0
    }
    #[doc = "Counter clear is enable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSGTRGAR_A::_1
    }
}
#[doc = "Field `CSGTRGAR` writer - GTETRGA Pin Rising Input Source Counter Clear Enable"]
pub type CSGTRGAR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSGTRGAR_A>;
impl<'a, REG, const O: u8> CSGTRGAR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSGTRGAR_A::_0)
    }
    #[doc = "Counter clear is enable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSGTRGAR_A::_1)
    }
}
#[doc = "Field `CSGTRGAF` reader - GTETRGA Pin Falling Input Source Counter Clear Enable"]
pub type CSGTRGAF_R = crate::BitReader<CSGTRGAF_A>;
#[doc = "GTETRGA Pin Falling Input Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSGTRGAF_A {
    #[doc = "0: Counter clear is disable at the falling edge of GTETRGA input"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the falling edge of GTETRGA input"]
    _1 = 1,
}
impl From<CSGTRGAF_A> for bool {
    #[inline(always)]
    fn from(variant: CSGTRGAF_A) -> Self {
        variant as u8 != 0
    }
}
impl CSGTRGAF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSGTRGAF_A {
        match self.bits {
            false => CSGTRGAF_A::_0,
            true => CSGTRGAF_A::_1,
        }
    }
    #[doc = "Counter clear is disable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSGTRGAF_A::_0
    }
    #[doc = "Counter clear is enable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSGTRGAF_A::_1
    }
}
#[doc = "Field `CSGTRGAF` writer - GTETRGA Pin Falling Input Source Counter Clear Enable"]
pub type CSGTRGAF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSGTRGAF_A>;
impl<'a, REG, const O: u8> CSGTRGAF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSGTRGAF_A::_0)
    }
    #[doc = "Counter clear is enable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSGTRGAF_A::_1)
    }
}
#[doc = "Field `CSGTRGBR` reader - GTETRGB Pin Rising Input Source Counter Clear Enable"]
pub type CSGTRGBR_R = crate::BitReader<CSGTRGBR_A>;
#[doc = "GTETRGB Pin Rising Input Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSGTRGBR_A {
    #[doc = "0: Counter clear is disable at the rising edge of GTETRGB input"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the rising edge of GTETRGB input"]
    _1 = 1,
}
impl From<CSGTRGBR_A> for bool {
    #[inline(always)]
    fn from(variant: CSGTRGBR_A) -> Self {
        variant as u8 != 0
    }
}
impl CSGTRGBR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSGTRGBR_A {
        match self.bits {
            false => CSGTRGBR_A::_0,
            true => CSGTRGBR_A::_1,
        }
    }
    #[doc = "Counter clear is disable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSGTRGBR_A::_0
    }
    #[doc = "Counter clear is enable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSGTRGBR_A::_1
    }
}
#[doc = "Field `CSGTRGBR` writer - GTETRGB Pin Rising Input Source Counter Clear Enable"]
pub type CSGTRGBR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSGTRGBR_A>;
impl<'a, REG, const O: u8> CSGTRGBR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSGTRGBR_A::_0)
    }
    #[doc = "Counter clear is enable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSGTRGBR_A::_1)
    }
}
#[doc = "Field `CSGTRGBF` reader - GTETRGB Pin Falling Input Source Counter Clear Enable"]
pub type CSGTRGBF_R = crate::BitReader<CSGTRGBF_A>;
#[doc = "GTETRGB Pin Falling Input Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSGTRGBF_A {
    #[doc = "0: Counter clear is disable at the falling edge of GTETRGB input"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the falling edge of GTETRGB input"]
    _1 = 1,
}
impl From<CSGTRGBF_A> for bool {
    #[inline(always)]
    fn from(variant: CSGTRGBF_A) -> Self {
        variant as u8 != 0
    }
}
impl CSGTRGBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSGTRGBF_A {
        match self.bits {
            false => CSGTRGBF_A::_0,
            true => CSGTRGBF_A::_1,
        }
    }
    #[doc = "Counter clear is disable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSGTRGBF_A::_0
    }
    #[doc = "Counter clear is enable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSGTRGBF_A::_1
    }
}
#[doc = "Field `CSGTRGBF` writer - GTETRGB Pin Falling Input Source Counter Clear Enable"]
pub type CSGTRGBF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSGTRGBF_A>;
impl<'a, REG, const O: u8> CSGTRGBF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSGTRGBF_A::_0)
    }
    #[doc = "Counter clear is enable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSGTRGBF_A::_1)
    }
}
#[doc = "Field `CSCARBL` reader - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Clear Enable"]
pub type CSCARBL_R = crate::BitReader<CSCARBL_A>;
#[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSCARBL_A {
    #[doc = "0: Counter clear is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 1,
}
impl From<CSCARBL_A> for bool {
    #[inline(always)]
    fn from(variant: CSCARBL_A) -> Self {
        variant as u8 != 0
    }
}
impl CSCARBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSCARBL_A {
        match self.bits {
            false => CSCARBL_A::_0,
            true => CSCARBL_A::_1,
        }
    }
    #[doc = "Counter clear is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSCARBL_A::_0
    }
    #[doc = "Counter clear is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSCARBL_A::_1
    }
}
#[doc = "Field `CSCARBL` writer - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Clear Enable"]
pub type CSCARBL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSCARBL_A>;
impl<'a, REG, const O: u8> CSCARBL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSCARBL_A::_0)
    }
    #[doc = "Counter clear is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSCARBL_A::_1)
    }
}
#[doc = "Field `CSCARBH` reader - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Clear Enable"]
pub type CSCARBH_R = crate::BitReader<CSCARBH_A>;
#[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSCARBH_A {
    #[doc = "0: Counter clear is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 1,
}
impl From<CSCARBH_A> for bool {
    #[inline(always)]
    fn from(variant: CSCARBH_A) -> Self {
        variant as u8 != 0
    }
}
impl CSCARBH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSCARBH_A {
        match self.bits {
            false => CSCARBH_A::_0,
            true => CSCARBH_A::_1,
        }
    }
    #[doc = "Counter clear is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSCARBH_A::_0
    }
    #[doc = "Counter clear is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSCARBH_A::_1
    }
}
#[doc = "Field `CSCARBH` writer - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Clear Enable"]
pub type CSCARBH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSCARBH_A>;
impl<'a, REG, const O: u8> CSCARBH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSCARBH_A::_0)
    }
    #[doc = "Counter clear is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSCARBH_A::_1)
    }
}
#[doc = "Field `CSCAFBL` reader - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Clear Enable"]
pub type CSCAFBL_R = crate::BitReader<CSCAFBL_A>;
#[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSCAFBL_A {
    #[doc = "0: Counter clear is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 1,
}
impl From<CSCAFBL_A> for bool {
    #[inline(always)]
    fn from(variant: CSCAFBL_A) -> Self {
        variant as u8 != 0
    }
}
impl CSCAFBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSCAFBL_A {
        match self.bits {
            false => CSCAFBL_A::_0,
            true => CSCAFBL_A::_1,
        }
    }
    #[doc = "Counter clear is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSCAFBL_A::_0
    }
    #[doc = "Counter clear is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSCAFBL_A::_1
    }
}
#[doc = "Field `CSCAFBL` writer - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Clear Enable"]
pub type CSCAFBL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSCAFBL_A>;
impl<'a, REG, const O: u8> CSCAFBL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSCAFBL_A::_0)
    }
    #[doc = "Counter clear is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSCAFBL_A::_1)
    }
}
#[doc = "Field `CSCAFBH` reader - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Clear Enable"]
pub type CSCAFBH_R = crate::BitReader<CSCAFBH_A>;
#[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSCAFBH_A {
    #[doc = "0: Counter clear is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 1,
}
impl From<CSCAFBH_A> for bool {
    #[inline(always)]
    fn from(variant: CSCAFBH_A) -> Self {
        variant as u8 != 0
    }
}
impl CSCAFBH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSCAFBH_A {
        match self.bits {
            false => CSCAFBH_A::_0,
            true => CSCAFBH_A::_1,
        }
    }
    #[doc = "Counter clear is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSCAFBH_A::_0
    }
    #[doc = "Counter clear is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSCAFBH_A::_1
    }
}
#[doc = "Field `CSCAFBH` writer - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Clear Enable"]
pub type CSCAFBH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSCAFBH_A>;
impl<'a, REG, const O: u8> CSCAFBH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSCAFBH_A::_0)
    }
    #[doc = "Counter clear is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSCAFBH_A::_1)
    }
}
#[doc = "Field `CSCBRAL` reader - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Clear Enable"]
pub type CSCBRAL_R = crate::BitReader<CSCBRAL_A>;
#[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSCBRAL_A {
    #[doc = "0: Counter clear is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 1,
}
impl From<CSCBRAL_A> for bool {
    #[inline(always)]
    fn from(variant: CSCBRAL_A) -> Self {
        variant as u8 != 0
    }
}
impl CSCBRAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSCBRAL_A {
        match self.bits {
            false => CSCBRAL_A::_0,
            true => CSCBRAL_A::_1,
        }
    }
    #[doc = "Counter clear is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSCBRAL_A::_0
    }
    #[doc = "Counter clear is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSCBRAL_A::_1
    }
}
#[doc = "Field `CSCBRAL` writer - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Clear Enable"]
pub type CSCBRAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSCBRAL_A>;
impl<'a, REG, const O: u8> CSCBRAL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSCBRAL_A::_0)
    }
    #[doc = "Counter clear is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSCBRAL_A::_1)
    }
}
#[doc = "Field `CSCBRAH` reader - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Clear Enable"]
pub type CSCBRAH_R = crate::BitReader<CSCBRAH_A>;
#[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSCBRAH_A {
    #[doc = "0: Counter clear is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 1,
}
impl From<CSCBRAH_A> for bool {
    #[inline(always)]
    fn from(variant: CSCBRAH_A) -> Self {
        variant as u8 != 0
    }
}
impl CSCBRAH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSCBRAH_A {
        match self.bits {
            false => CSCBRAH_A::_0,
            true => CSCBRAH_A::_1,
        }
    }
    #[doc = "Counter clear is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSCBRAH_A::_0
    }
    #[doc = "Counter clear is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSCBRAH_A::_1
    }
}
#[doc = "Field `CSCBRAH` writer - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Clear Enable"]
pub type CSCBRAH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSCBRAH_A>;
impl<'a, REG, const O: u8> CSCBRAH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSCBRAH_A::_0)
    }
    #[doc = "Counter clear is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSCBRAH_A::_1)
    }
}
#[doc = "Field `CSCBFAL` reader - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Clear Enable"]
pub type CSCBFAL_R = crate::BitReader<CSCBFAL_A>;
#[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSCBFAL_A {
    #[doc = "0: Counter clear is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 1,
}
impl From<CSCBFAL_A> for bool {
    #[inline(always)]
    fn from(variant: CSCBFAL_A) -> Self {
        variant as u8 != 0
    }
}
impl CSCBFAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSCBFAL_A {
        match self.bits {
            false => CSCBFAL_A::_0,
            true => CSCBFAL_A::_1,
        }
    }
    #[doc = "Counter clear is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSCBFAL_A::_0
    }
    #[doc = "Counter clear is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSCBFAL_A::_1
    }
}
#[doc = "Field `CSCBFAL` writer - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Clear Enable"]
pub type CSCBFAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSCBFAL_A>;
impl<'a, REG, const O: u8> CSCBFAL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSCBFAL_A::_0)
    }
    #[doc = "Counter clear is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSCBFAL_A::_1)
    }
}
#[doc = "Field `CSCBFAH` reader - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Clear Enable"]
pub type CSCBFAH_R = crate::BitReader<CSCBFAH_A>;
#[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSCBFAH_A {
    #[doc = "0: Counter clear is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 1,
}
impl From<CSCBFAH_A> for bool {
    #[inline(always)]
    fn from(variant: CSCBFAH_A) -> Self {
        variant as u8 != 0
    }
}
impl CSCBFAH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSCBFAH_A {
        match self.bits {
            false => CSCBFAH_A::_0,
            true => CSCBFAH_A::_1,
        }
    }
    #[doc = "Counter clear is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSCBFAH_A::_0
    }
    #[doc = "Counter clear is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSCBFAH_A::_1
    }
}
#[doc = "Field `CSCBFAH` writer - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Clear Enable"]
pub type CSCBFAH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSCBFAH_A>;
impl<'a, REG, const O: u8> CSCBFAH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSCBFAH_A::_0)
    }
    #[doc = "Counter clear is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSCBFAH_A::_1)
    }
}
#[doc = "Field `CSELCA` reader - ELC_GPTA Event Source Counter Clear Enable"]
pub type CSELCA_R = crate::BitReader<CSELCA_A>;
#[doc = "ELC_GPTA Event Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSELCA_A {
    #[doc = "0: Counter clear is disable at the ELC_GPTA input"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the ELC_GPTA input"]
    _1 = 1,
}
impl From<CSELCA_A> for bool {
    #[inline(always)]
    fn from(variant: CSELCA_A) -> Self {
        variant as u8 != 0
    }
}
impl CSELCA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSELCA_A {
        match self.bits {
            false => CSELCA_A::_0,
            true => CSELCA_A::_1,
        }
    }
    #[doc = "Counter clear is disable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSELCA_A::_0
    }
    #[doc = "Counter clear is enable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSELCA_A::_1
    }
}
#[doc = "Field `CSELCA` writer - ELC_GPTA Event Source Counter Clear Enable"]
pub type CSELCA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSELCA_A>;
impl<'a, REG, const O: u8> CSELCA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCA_A::_0)
    }
    #[doc = "Counter clear is enable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCA_A::_1)
    }
}
#[doc = "Field `CSELCB` reader - ELC_GPTB Event Source Counter Clear Enable"]
pub type CSELCB_R = crate::BitReader<CSELCB_A>;
#[doc = "ELC_GPTB Event Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSELCB_A {
    #[doc = "0: Counter clear is disable at the ELC_GPTB input"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the ELC_GPTB input"]
    _1 = 1,
}
impl From<CSELCB_A> for bool {
    #[inline(always)]
    fn from(variant: CSELCB_A) -> Self {
        variant as u8 != 0
    }
}
impl CSELCB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSELCB_A {
        match self.bits {
            false => CSELCB_A::_0,
            true => CSELCB_A::_1,
        }
    }
    #[doc = "Counter clear is disable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSELCB_A::_0
    }
    #[doc = "Counter clear is enable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSELCB_A::_1
    }
}
#[doc = "Field `CSELCB` writer - ELC_GPTB Event Source Counter Clear Enable"]
pub type CSELCB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSELCB_A>;
impl<'a, REG, const O: u8> CSELCB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCB_A::_0)
    }
    #[doc = "Counter clear is enable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCB_A::_1)
    }
}
#[doc = "Field `CSELCC` reader - ELC_GPTC Event Source Counter Clear Enable"]
pub type CSELCC_R = crate::BitReader<CSELCC_A>;
#[doc = "ELC_GPTC Event Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSELCC_A {
    #[doc = "0: Counter clear is disable at the ELC_GPTC input"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the ELC_GPTC input"]
    _1 = 1,
}
impl From<CSELCC_A> for bool {
    #[inline(always)]
    fn from(variant: CSELCC_A) -> Self {
        variant as u8 != 0
    }
}
impl CSELCC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSELCC_A {
        match self.bits {
            false => CSELCC_A::_0,
            true => CSELCC_A::_1,
        }
    }
    #[doc = "Counter clear is disable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSELCC_A::_0
    }
    #[doc = "Counter clear is enable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSELCC_A::_1
    }
}
#[doc = "Field `CSELCC` writer - ELC_GPTC Event Source Counter Clear Enable"]
pub type CSELCC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSELCC_A>;
impl<'a, REG, const O: u8> CSELCC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCC_A::_0)
    }
    #[doc = "Counter clear is enable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCC_A::_1)
    }
}
#[doc = "Field `CSELCD` reader - ELC_GPTD Event Source Counter Clear Enable"]
pub type CSELCD_R = crate::BitReader<CSELCD_A>;
#[doc = "ELC_GPTD Event Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSELCD_A {
    #[doc = "0: Counter clear is disable at the ELC_GPTD input"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the ELC_GPTD input"]
    _1 = 1,
}
impl From<CSELCD_A> for bool {
    #[inline(always)]
    fn from(variant: CSELCD_A) -> Self {
        variant as u8 != 0
    }
}
impl CSELCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSELCD_A {
        match self.bits {
            false => CSELCD_A::_0,
            true => CSELCD_A::_1,
        }
    }
    #[doc = "Counter clear is disable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSELCD_A::_0
    }
    #[doc = "Counter clear is enable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSELCD_A::_1
    }
}
#[doc = "Field `CSELCD` writer - ELC_GPTD Event Source Counter Clear Enable"]
pub type CSELCD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSELCD_A>;
impl<'a, REG, const O: u8> CSELCD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCD_A::_0)
    }
    #[doc = "Counter clear is enable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCD_A::_1)
    }
}
#[doc = "Field `CSELCE` reader - ELC_GPTE Event Source Counter Clear Enable"]
pub type CSELCE_R = crate::BitReader<CSELCE_A>;
#[doc = "ELC_GPTE Event Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSELCE_A {
    #[doc = "0: Counter clear is disable at the ELC_GPTE input"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the ELC_GPTE input"]
    _1 = 1,
}
impl From<CSELCE_A> for bool {
    #[inline(always)]
    fn from(variant: CSELCE_A) -> Self {
        variant as u8 != 0
    }
}
impl CSELCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSELCE_A {
        match self.bits {
            false => CSELCE_A::_0,
            true => CSELCE_A::_1,
        }
    }
    #[doc = "Counter clear is disable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSELCE_A::_0
    }
    #[doc = "Counter clear is enable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSELCE_A::_1
    }
}
#[doc = "Field `CSELCE` writer - ELC_GPTE Event Source Counter Clear Enable"]
pub type CSELCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSELCE_A>;
impl<'a, REG, const O: u8> CSELCE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCE_A::_0)
    }
    #[doc = "Counter clear is enable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCE_A::_1)
    }
}
#[doc = "Field `CSELCF` reader - ELC_GPTF Event Source Counter Clear Enable"]
pub type CSELCF_R = crate::BitReader<CSELCF_A>;
#[doc = "ELC_GPTF Event Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSELCF_A {
    #[doc = "0: Counter clear is disable at the ELC_GPTF input"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the ELC_GPTF input"]
    _1 = 1,
}
impl From<CSELCF_A> for bool {
    #[inline(always)]
    fn from(variant: CSELCF_A) -> Self {
        variant as u8 != 0
    }
}
impl CSELCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSELCF_A {
        match self.bits {
            false => CSELCF_A::_0,
            true => CSELCF_A::_1,
        }
    }
    #[doc = "Counter clear is disable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSELCF_A::_0
    }
    #[doc = "Counter clear is enable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSELCF_A::_1
    }
}
#[doc = "Field `CSELCF` writer - ELC_GPTF Event Source Counter Clear Enable"]
pub type CSELCF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSELCF_A>;
impl<'a, REG, const O: u8> CSELCF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCF_A::_0)
    }
    #[doc = "Counter clear is enable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCF_A::_1)
    }
}
#[doc = "Field `CSELCG` reader - ELC_GPTG Event Source Counter Clear Enable"]
pub type CSELCG_R = crate::BitReader<CSELCG_A>;
#[doc = "ELC_GPTG Event Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSELCG_A {
    #[doc = "0: Counter clear is disable at the ELC_GPTG input"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the ELC_GPTG input"]
    _1 = 1,
}
impl From<CSELCG_A> for bool {
    #[inline(always)]
    fn from(variant: CSELCG_A) -> Self {
        variant as u8 != 0
    }
}
impl CSELCG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSELCG_A {
        match self.bits {
            false => CSELCG_A::_0,
            true => CSELCG_A::_1,
        }
    }
    #[doc = "Counter clear is disable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSELCG_A::_0
    }
    #[doc = "Counter clear is enable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSELCG_A::_1
    }
}
#[doc = "Field `CSELCG` writer - ELC_GPTG Event Source Counter Clear Enable"]
pub type CSELCG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSELCG_A>;
impl<'a, REG, const O: u8> CSELCG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCG_A::_0)
    }
    #[doc = "Counter clear is enable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCG_A::_1)
    }
}
#[doc = "Field `CSELCH` reader - ELC_GPTH Event Source Counter Clear Enable"]
pub type CSELCH_R = crate::BitReader<CSELCH_A>;
#[doc = "ELC_GPTH Event Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSELCH_A {
    #[doc = "0: Counter clear is disable at the ELC_GPTH input"]
    _0 = 0,
    #[doc = "1: Counter clear is enable at the ELC_GPTH input"]
    _1 = 1,
}
impl From<CSELCH_A> for bool {
    #[inline(always)]
    fn from(variant: CSELCH_A) -> Self {
        variant as u8 != 0
    }
}
impl CSELCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSELCH_A {
        match self.bits {
            false => CSELCH_A::_0,
            true => CSELCH_A::_1,
        }
    }
    #[doc = "Counter clear is disable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSELCH_A::_0
    }
    #[doc = "Counter clear is enable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSELCH_A::_1
    }
}
#[doc = "Field `CSELCH` writer - ELC_GPTH Event Source Counter Clear Enable"]
pub type CSELCH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSELCH_A>;
impl<'a, REG, const O: u8> CSELCH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCH_A::_0)
    }
    #[doc = "Counter clear is enable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCH_A::_1)
    }
}
#[doc = "Field `CCLR` reader - Software Source Counter Clear Enable"]
pub type CCLR_R = crate::BitReader<CCLR_A>;
#[doc = "Software Source Counter Clear Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR_A {
    #[doc = "0: Counter clear is disable by the GTCLR register"]
    _0 = 0,
    #[doc = "1: Counter clear is enable by the GTCLR register"]
    _1 = 1,
}
impl From<CCLR_A> for bool {
    #[inline(always)]
    fn from(variant: CCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl CCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCLR_A {
        match self.bits {
            false => CCLR_A::_0,
            true => CCLR_A::_1,
        }
    }
    #[doc = "Counter clear is disable by the GTCLR register"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCLR_A::_0
    }
    #[doc = "Counter clear is enable by the GTCLR register"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCLR_A::_1
    }
}
#[doc = "Field `CCLR` writer - Software Source Counter Clear Enable"]
pub type CCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CCLR_A>;
impl<'a, REG, const O: u8> CCLR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clear is disable by the GTCLR register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR_A::_0)
    }
    #[doc = "Counter clear is enable by the GTCLR register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgar(&self) -> CSGTRGAR_R {
        CSGTRGAR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgaf(&self) -> CSGTRGAF_R {
        CSGTRGAF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgbr(&self) -> CSGTRGBR_R {
        CSGTRGBR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgbf(&self) -> CSGTRGBF_R {
        CSGTRGBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscarbl(&self) -> CSCARBL_R {
        CSCARBL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscarbh(&self) -> CSCARBH_R {
        CSCARBH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscafbl(&self) -> CSCAFBL_R {
        CSCAFBL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscafbh(&self) -> CSCAFBH_R {
        CSCAFBH_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscbral(&self) -> CSCBRAL_R {
        CSCBRAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscbrah(&self) -> CSCBRAH_R {
        CSCBRAH_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscbfal(&self) -> CSCBFAL_R {
        CSCBFAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscbfah(&self) -> CSCBFAH_R {
        CSCBFAH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselca(&self) -> CSELCA_R {
        CSELCA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselcb(&self) -> CSELCB_R {
        CSELCB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselcc(&self) -> CSELCC_R {
        CSELCC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselcd(&self) -> CSELCD_R {
        CSELCD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ELC_GPTE Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselce(&self) -> CSELCE_R {
        CSELCE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ELC_GPTF Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselcf(&self) -> CSELCF_R {
        CSELCF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ELC_GPTG Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselcg(&self) -> CSELCG_R {
        CSELCG_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ELC_GPTH Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselch(&self) -> CSELCH_R {
        CSELCH_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - Software Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cclr(&self) -> CCLR_R {
        CCLR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csgtrgar(&mut self) -> CSGTRGAR_W<GTCSR_SPEC, 0> {
        CSGTRGAR_W::new(self)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csgtrgaf(&mut self) -> CSGTRGAF_W<GTCSR_SPEC, 1> {
        CSGTRGAF_W::new(self)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csgtrgbr(&mut self) -> CSGTRGBR_W<GTCSR_SPEC, 2> {
        CSGTRGBR_W::new(self)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csgtrgbf(&mut self) -> CSGTRGBF_W<GTCSR_SPEC, 3> {
        CSGTRGBF_W::new(self)
    }
    #[doc = "Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cscarbl(&mut self) -> CSCARBL_W<GTCSR_SPEC, 8> {
        CSCARBL_W::new(self)
    }
    #[doc = "Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cscarbh(&mut self) -> CSCARBH_W<GTCSR_SPEC, 9> {
        CSCARBH_W::new(self)
    }
    #[doc = "Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cscafbl(&mut self) -> CSCAFBL_W<GTCSR_SPEC, 10> {
        CSCAFBL_W::new(self)
    }
    #[doc = "Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cscafbh(&mut self) -> CSCAFBH_W<GTCSR_SPEC, 11> {
        CSCAFBH_W::new(self)
    }
    #[doc = "Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cscbral(&mut self) -> CSCBRAL_W<GTCSR_SPEC, 12> {
        CSCBRAL_W::new(self)
    }
    #[doc = "Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cscbrah(&mut self) -> CSCBRAH_W<GTCSR_SPEC, 13> {
        CSCBRAH_W::new(self)
    }
    #[doc = "Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cscbfal(&mut self) -> CSCBFAL_W<GTCSR_SPEC, 14> {
        CSCBFAL_W::new(self)
    }
    #[doc = "Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cscbfah(&mut self) -> CSCBFAH_W<GTCSR_SPEC, 15> {
        CSCBFAH_W::new(self)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cselca(&mut self) -> CSELCA_W<GTCSR_SPEC, 16> {
        CSELCA_W::new(self)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cselcb(&mut self) -> CSELCB_W<GTCSR_SPEC, 17> {
        CSELCB_W::new(self)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cselcc(&mut self) -> CSELCC_W<GTCSR_SPEC, 18> {
        CSELCC_W::new(self)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cselcd(&mut self) -> CSELCD_W<GTCSR_SPEC, 19> {
        CSELCD_W::new(self)
    }
    #[doc = "Bit 20 - ELC_GPTE Event Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cselce(&mut self) -> CSELCE_W<GTCSR_SPEC, 20> {
        CSELCE_W::new(self)
    }
    #[doc = "Bit 21 - ELC_GPTF Event Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cselcf(&mut self) -> CSELCF_W<GTCSR_SPEC, 21> {
        CSELCF_W::new(self)
    }
    #[doc = "Bit 22 - ELC_GPTG Event Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cselcg(&mut self) -> CSELCG_W<GTCSR_SPEC, 22> {
        CSELCG_W::new(self)
    }
    #[doc = "Bit 23 - ELC_GPTH Event Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cselch(&mut self) -> CSELCH_W<GTCSR_SPEC, 23> {
        CSELCH_W::new(self)
    }
    #[doc = "Bit 31 - Software Source Counter Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cclr(&mut self) -> CCLR_W<GTCSR_SPEC, 31> {
        CCLR_W::new(self)
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
#[doc = "General PWM Timer Clear Source Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTCSR_SPEC;
impl crate::RegisterSpec for GTCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtcsr::R`](R) reader structure"]
impl crate::Readable for GTCSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gtcsr::W`](W) writer structure"]
impl crate::Writable for GTCSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTCSR to value 0"]
impl crate::Resettable for GTCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
