#[doc = "Register `GTSTR` reader"]
pub type R = crate::R<GTSTR_SPEC>;
#[doc = "Register `GTSTR` writer"]
pub type W = crate::W<GTSTR_SPEC>;
#[doc = "Field `CSTRT0` reader - Channel 0 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT0_R = crate::BitReader<CSTRT0_A>;
#[doc = "Channel 0 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT0_A {
    #[doc = "0: No effect (write) / counter stop (read)"]
    _0 = 0,
    #[doc = "1: GPT320.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 1,
}
impl From<CSTRT0_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT0_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT0_A {
        match self.bits {
            false => CSTRT0_A::_0,
            true => CSTRT0_A::_1,
        }
    }
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT0_A::_0
    }
    #[doc = "GPT320.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT0_A::_1
    }
}
#[doc = "Field `CSTRT0` writer - Channel 0 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSTRT0_A>;
impl<'a, REG, const O: u8> CSTRT0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT0_A::_0)
    }
    #[doc = "GPT320.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT0_A::_1)
    }
}
#[doc = "Field `CSTRT1` reader - Channel 1 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT1_R = crate::BitReader<CSTRT1_A>;
#[doc = "Channel 1 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT1_A {
    #[doc = "0: No effect (write) / counter stop (read)"]
    _0 = 0,
    #[doc = "1: GPT321.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 1,
}
impl From<CSTRT1_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT1_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT1_A {
        match self.bits {
            false => CSTRT1_A::_0,
            true => CSTRT1_A::_1,
        }
    }
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT1_A::_0
    }
    #[doc = "GPT321.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT1_A::_1
    }
}
#[doc = "Field `CSTRT1` writer - Channel 1 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSTRT1_A>;
impl<'a, REG, const O: u8> CSTRT1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT1_A::_0)
    }
    #[doc = "GPT321.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT1_A::_1)
    }
}
#[doc = "Field `CSTRT2` reader - Channel 2 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT2_R = crate::BitReader<CSTRT2_A>;
#[doc = "Channel 2 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT2_A {
    #[doc = "0: No effect (write) / counter stop (read)"]
    _0 = 0,
    #[doc = "1: GPT322.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 1,
}
impl From<CSTRT2_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT2_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT2_A {
        match self.bits {
            false => CSTRT2_A::_0,
            true => CSTRT2_A::_1,
        }
    }
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT2_A::_0
    }
    #[doc = "GPT322.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT2_A::_1
    }
}
#[doc = "Field `CSTRT2` writer - Channel 2 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSTRT2_A>;
impl<'a, REG, const O: u8> CSTRT2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT2_A::_0)
    }
    #[doc = "GPT322.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT2_A::_1)
    }
}
#[doc = "Field `CSTRT3` reader - Channel 3 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT3_R = crate::BitReader<CSTRT3_A>;
#[doc = "Channel 3 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT3_A {
    #[doc = "0: No effect (write) / counter stop (read)"]
    _0 = 0,
    #[doc = "1: GPT323.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 1,
}
impl From<CSTRT3_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT3_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT3_A {
        match self.bits {
            false => CSTRT3_A::_0,
            true => CSTRT3_A::_1,
        }
    }
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT3_A::_0
    }
    #[doc = "GPT323.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT3_A::_1
    }
}
#[doc = "Field `CSTRT3` writer - Channel 3 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSTRT3_A>;
impl<'a, REG, const O: u8> CSTRT3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT3_A::_0)
    }
    #[doc = "GPT323.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT3_A::_1)
    }
}
#[doc = "Field `CSTRT4` reader - Channel 4 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT4_R = crate::BitReader<CSTRT4_A>;
#[doc = "Channel 4 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT4_A {
    #[doc = "0: No effect (write) / counter stop (read)"]
    _0 = 0,
    #[doc = "1: GPT164.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 1,
}
impl From<CSTRT4_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT4_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT4_A {
        match self.bits {
            false => CSTRT4_A::_0,
            true => CSTRT4_A::_1,
        }
    }
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT4_A::_0
    }
    #[doc = "GPT164.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT4_A::_1
    }
}
#[doc = "Field `CSTRT4` writer - Channel 4 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSTRT4_A>;
impl<'a, REG, const O: u8> CSTRT4_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT4_A::_0)
    }
    #[doc = "GPT164.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT4_A::_1)
    }
}
#[doc = "Field `CSTRT5` reader - Channel 5 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT5_R = crate::BitReader<CSTRT5_A>;
#[doc = "Channel 5 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT5_A {
    #[doc = "0: No effect (write) / counter stop (read)"]
    _0 = 0,
    #[doc = "1: GPT165.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 1,
}
impl From<CSTRT5_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT5_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT5_A {
        match self.bits {
            false => CSTRT5_A::_0,
            true => CSTRT5_A::_1,
        }
    }
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT5_A::_0
    }
    #[doc = "GPT165.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT5_A::_1
    }
}
#[doc = "Field `CSTRT5` writer - Channel 5 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSTRT5_A>;
impl<'a, REG, const O: u8> CSTRT5_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT5_A::_0)
    }
    #[doc = "GPT165.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT5_A::_1)
    }
}
#[doc = "Field `CSTRT6` reader - Channel 6 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT6_R = crate::BitReader<CSTRT6_A>;
#[doc = "Channel 6 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT6_A {
    #[doc = "0: No effect (write) / counter stop (read)"]
    _0 = 0,
    #[doc = "1: GPT166.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 1,
}
impl From<CSTRT6_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT6_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT6_A {
        match self.bits {
            false => CSTRT6_A::_0,
            true => CSTRT6_A::_1,
        }
    }
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT6_A::_0
    }
    #[doc = "GPT166.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT6_A::_1
    }
}
#[doc = "Field `CSTRT6` writer - Channel 6 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSTRT6_A>;
impl<'a, REG, const O: u8> CSTRT6_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT6_A::_0)
    }
    #[doc = "GPT166.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT6_A::_1)
    }
}
#[doc = "Field `CSTRT7` reader - Channel 7 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT7_R = crate::BitReader<CSTRT7_A>;
#[doc = "Channel 7 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT7_A {
    #[doc = "0: No effect (write) / counter stop (read)"]
    _0 = 0,
    #[doc = "1: GPT167.GTCNT counter starts (write) / Counter running (read)"]
    _1 = 1,
}
impl From<CSTRT7_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT7_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTRT7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT7_A {
        match self.bits {
            false => CSTRT7_A::_0,
            true => CSTRT7_A::_1,
        }
    }
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT7_A::_0
    }
    #[doc = "GPT167.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT7_A::_1
    }
}
#[doc = "Field `CSTRT7` writer - Channel 7 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
pub type CSTRT7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSTRT7_A>;
impl<'a, REG, const O: u8> CSTRT7_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter stop (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT7_A::_0)
    }
    #[doc = "GPT167.GTCNT counter starts (write) / Counter running (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT7_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt0(&self) -> CSTRT0_R {
        CSTRT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt1(&self) -> CSTRT1_R {
        CSTRT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt2(&self) -> CSTRT2_R {
        CSTRT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt3(&self) -> CSTRT3_R {
        CSTRT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt4(&self) -> CSTRT4_R {
        CSTRT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt5(&self) -> CSTRT5_R {
        CSTRT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt6(&self) -> CSTRT6_R {
        CSTRT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt7(&self) -> CSTRT7_R {
        CSTRT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    #[must_use]
    pub fn cstrt0(&mut self) -> CSTRT0_W<GTSTR_SPEC, 0> {
        CSTRT0_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    #[must_use]
    pub fn cstrt1(&mut self) -> CSTRT1_W<GTSTR_SPEC, 1> {
        CSTRT1_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    #[must_use]
    pub fn cstrt2(&mut self) -> CSTRT2_W<GTSTR_SPEC, 2> {
        CSTRT2_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    #[must_use]
    pub fn cstrt3(&mut self) -> CSTRT3_W<GTSTR_SPEC, 3> {
        CSTRT3_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    #[must_use]
    pub fn cstrt4(&mut self) -> CSTRT4_W<GTSTR_SPEC, 4> {
        CSTRT4_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    #[must_use]
    pub fn cstrt5(&mut self) -> CSTRT5_W<GTSTR_SPEC, 5> {
        CSTRT5_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    #[must_use]
    pub fn cstrt6(&mut self) -> CSTRT6_W<GTSTR_SPEC, 6> {
        CSTRT6_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    #[must_use]
    pub fn cstrt7(&mut self) -> CSTRT7_W<GTSTR_SPEC, 7> {
        CSTRT7_W::new(self)
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
#[doc = "General PWM Timer Software Start Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTSTR_SPEC;
impl crate::RegisterSpec for GTSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtstr::R`](R) reader structure"]
impl crate::Readable for GTSTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gtstr::W`](W) writer structure"]
impl crate::Writable for GTSTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTSTR to value 0"]
impl crate::Resettable for GTSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
