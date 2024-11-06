#[doc = "Register `GTSTP` reader"]
pub type R = crate::R<GTSTP_SPEC>;
#[doc = "Register `GTSTP` writer"]
pub type W = crate::W<GTSTP_SPEC>;
#[doc = "Field `CSTOP0` reader - Channel 0 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP0_R = crate::BitReader<CSTOP0_A>;
#[doc = "Channel 0 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP0_A {
    #[doc = "0: No effect (write) / counter running (read)"]
    _0 = 0,
    #[doc = "1: GPT320.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 1,
}
impl From<CSTOP0_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP0_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSTOP0_A {
        match self.bits {
            false => CSTOP0_A::_0,
            true => CSTOP0_A::_1,
        }
    }
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP0_A::_0
    }
    #[doc = "GPT320.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP0_A::_1
    }
}
#[doc = "Field `CSTOP0` writer - Channel 0 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSTOP0_A>;
impl<'a, REG, const O: u8> CSTOP0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP0_A::_0)
    }
    #[doc = "GPT320.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP0_A::_1)
    }
}
#[doc = "Field `CSTOP1` reader - Channel 1 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP1_R = crate::BitReader<CSTOP1_A>;
#[doc = "Channel 1 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP1_A {
    #[doc = "0: No effect (write) / counter running (read)"]
    _0 = 0,
    #[doc = "1: GPT321.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 1,
}
impl From<CSTOP1_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP1_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSTOP1_A {
        match self.bits {
            false => CSTOP1_A::_0,
            true => CSTOP1_A::_1,
        }
    }
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP1_A::_0
    }
    #[doc = "GPT321.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP1_A::_1
    }
}
#[doc = "Field `CSTOP1` writer - Channel 1 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSTOP1_A>;
impl<'a, REG, const O: u8> CSTOP1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP1_A::_0)
    }
    #[doc = "GPT321.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP1_A::_1)
    }
}
#[doc = "Field `CSTOP2` reader - Channel 2 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP2_R = crate::BitReader<CSTOP2_A>;
#[doc = "Channel 2 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP2_A {
    #[doc = "0: No effect (write) / counter running (read)"]
    _0 = 0,
    #[doc = "1: GPT322.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 1,
}
impl From<CSTOP2_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP2_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSTOP2_A {
        match self.bits {
            false => CSTOP2_A::_0,
            true => CSTOP2_A::_1,
        }
    }
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP2_A::_0
    }
    #[doc = "GPT322.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP2_A::_1
    }
}
#[doc = "Field `CSTOP2` writer - Channel 2 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSTOP2_A>;
impl<'a, REG, const O: u8> CSTOP2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP2_A::_0)
    }
    #[doc = "GPT322.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP2_A::_1)
    }
}
#[doc = "Field `CSTOP3` reader - Channel 3 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP3_R = crate::BitReader<CSTOP3_A>;
#[doc = "Channel 3 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP3_A {
    #[doc = "0: No effect (write) / counter running (read)"]
    _0 = 0,
    #[doc = "1: GPT323.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 1,
}
impl From<CSTOP3_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP3_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSTOP3_A {
        match self.bits {
            false => CSTOP3_A::_0,
            true => CSTOP3_A::_1,
        }
    }
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP3_A::_0
    }
    #[doc = "GPT323.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP3_A::_1
    }
}
#[doc = "Field `CSTOP3` writer - Channel 3 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSTOP3_A>;
impl<'a, REG, const O: u8> CSTOP3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP3_A::_0)
    }
    #[doc = "GPT323.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP3_A::_1)
    }
}
#[doc = "Field `CSTOP4` reader - Channel 4 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP4_R = crate::BitReader<CSTOP4_A>;
#[doc = "Channel 4 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP4_A {
    #[doc = "0: No effect (write) / counter running (read)"]
    _0 = 0,
    #[doc = "1: GPT164.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 1,
}
impl From<CSTOP4_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP4_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSTOP4_A {
        match self.bits {
            false => CSTOP4_A::_0,
            true => CSTOP4_A::_1,
        }
    }
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP4_A::_0
    }
    #[doc = "GPT164.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP4_A::_1
    }
}
#[doc = "Field `CSTOP4` writer - Channel 4 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSTOP4_A>;
impl<'a, REG, const O: u8> CSTOP4_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP4_A::_0)
    }
    #[doc = "GPT164.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP4_A::_1)
    }
}
#[doc = "Field `CSTOP5` reader - Channel 5 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP5_R = crate::BitReader<CSTOP5_A>;
#[doc = "Channel 5 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP5_A {
    #[doc = "0: No effect (write) / counter running (read)"]
    _0 = 0,
    #[doc = "1: GPT165.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 1,
}
impl From<CSTOP5_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP5_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSTOP5_A {
        match self.bits {
            false => CSTOP5_A::_0,
            true => CSTOP5_A::_1,
        }
    }
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP5_A::_0
    }
    #[doc = "GPT165.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP5_A::_1
    }
}
#[doc = "Field `CSTOP5` writer - Channel 5 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSTOP5_A>;
impl<'a, REG, const O: u8> CSTOP5_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP5_A::_0)
    }
    #[doc = "GPT165.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP5_A::_1)
    }
}
#[doc = "Field `CSTOP6` reader - Channel 6 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP6_R = crate::BitReader<CSTOP6_A>;
#[doc = "Channel 6 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP6_A {
    #[doc = "0: No effect (write) / counter running (read)"]
    _0 = 0,
    #[doc = "1: GPT166.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 1,
}
impl From<CSTOP6_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP6_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSTOP6_A {
        match self.bits {
            false => CSTOP6_A::_0,
            true => CSTOP6_A::_1,
        }
    }
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP6_A::_0
    }
    #[doc = "GPT166.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP6_A::_1
    }
}
#[doc = "Field `CSTOP6` writer - Channel 6 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSTOP6_A>;
impl<'a, REG, const O: u8> CSTOP6_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP6_A::_0)
    }
    #[doc = "GPT166.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP6_A::_1)
    }
}
#[doc = "Field `CSTOP7` reader - Channel 7 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP7_R = crate::BitReader<CSTOP7_A>;
#[doc = "Channel 7 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP7_A {
    #[doc = "0: No effect (write) / counter running (read)"]
    _0 = 0,
    #[doc = "1: GPT167.GTCNT counter stops (write) / Counter stop (read)"]
    _1 = 1,
}
impl From<CSTOP7_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP7_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTOP7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSTOP7_A {
        match self.bits {
            false => CSTOP7_A::_0,
            true => CSTOP7_A::_1,
        }
    }
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP7_A::_0
    }
    #[doc = "GPT167.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP7_A::_1
    }
}
#[doc = "Field `CSTOP7` writer - Channel 7 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
pub type CSTOP7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CSTOP7_A>;
impl<'a, REG, const O: u8> CSTOP7_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect (write) / counter running (read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP7_A::_0)
    }
    #[doc = "GPT167.GTCNT counter stops (write) / Counter stop (read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP7_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop0(&self) -> CSTOP0_R {
        CSTOP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop1(&self) -> CSTOP1_R {
        CSTOP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop2(&self) -> CSTOP2_R {
        CSTOP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop3(&self) -> CSTOP3_R {
        CSTOP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop4(&self) -> CSTOP4_R {
        CSTOP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop5(&self) -> CSTOP5_R {
        CSTOP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop6(&self) -> CSTOP6_R {
        CSTOP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop7(&self) -> CSTOP7_R {
        CSTOP7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    #[must_use]
    pub fn cstop0(&mut self) -> CSTOP0_W<GTSTP_SPEC, 0> {
        CSTOP0_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    #[must_use]
    pub fn cstop1(&mut self) -> CSTOP1_W<GTSTP_SPEC, 1> {
        CSTOP1_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    #[must_use]
    pub fn cstop2(&mut self) -> CSTOP2_W<GTSTP_SPEC, 2> {
        CSTOP2_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    #[must_use]
    pub fn cstop3(&mut self) -> CSTOP3_W<GTSTP_SPEC, 3> {
        CSTOP3_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    #[must_use]
    pub fn cstop4(&mut self) -> CSTOP4_W<GTSTP_SPEC, 4> {
        CSTOP4_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    #[must_use]
    pub fn cstop5(&mut self) -> CSTOP5_W<GTSTP_SPEC, 5> {
        CSTOP5_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    #[must_use]
    pub fn cstop6(&mut self) -> CSTOP6_W<GTSTP_SPEC, 6> {
        CSTOP6_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 GTCNT Count Stop Read data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    #[must_use]
    pub fn cstop7(&mut self) -> CSTOP7_W<GTSTP_SPEC, 7> {
        CSTOP7_W::new(self)
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
#[doc = "General PWM Timer Software Stop Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtstp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtstp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTSTP_SPEC;
impl crate::RegisterSpec for GTSTP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtstp::R`](R) reader structure"]
impl crate::Readable for GTSTP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gtstp::W`](W) writer structure"]
impl crate::Writable for GTSTP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTSTP to value 0xffff_ffff"]
impl crate::Resettable for GTSTP_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
