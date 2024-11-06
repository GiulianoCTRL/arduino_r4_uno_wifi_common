#[doc = "Register `RSTSR1` reader"]
pub type R = crate::R<RSTSR1_SPEC>;
#[doc = "Register `RSTSR1` writer"]
pub type W = crate::W<RSTSR1_SPEC>;
#[doc = "Field `IWDTRF` reader - Independent Watchdog Timer Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\nThe field is **modified** in some way after a read operation."]
pub type IWDTRF_R = crate::BitReader<IWDTRF_A>;
#[doc = "Independent Watchdog Timer Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDTRF_A {
    #[doc = "0: Independent watchdog timer reset not detected."]
    _0 = 0,
    #[doc = "1: Independent watchdog timer reset detected."]
    _1 = 1,
}
impl From<IWDTRF_A> for bool {
    #[inline(always)]
    fn from(variant: IWDTRF_A) -> Self {
        variant as u8 != 0
    }
}
impl IWDTRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IWDTRF_A {
        match self.bits {
            false => IWDTRF_A::_0,
            true => IWDTRF_A::_1,
        }
    }
    #[doc = "Independent watchdog timer reset not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IWDTRF_A::_0
    }
    #[doc = "Independent watchdog timer reset detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IWDTRF_A::_1
    }
}
#[doc = "Field `IWDTRF` writer - Independent Watchdog Timer Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
pub type IWDTRF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, IWDTRF_A>;
impl<'a, REG, const O: u8> IWDTRF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Independent watchdog timer reset not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IWDTRF_A::_0)
    }
    #[doc = "Independent watchdog timer reset detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IWDTRF_A::_1)
    }
}
#[doc = "Field `WDTRF` reader - Watchdog Timer Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\nThe field is **modified** in some way after a read operation."]
pub type WDTRF_R = crate::BitReader<WDTRF_A>;
#[doc = "Watchdog Timer Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTRF_A {
    #[doc = "0: Watchdog timer reset not detected."]
    _0 = 0,
    #[doc = "1: Watchdog timer reset detected."]
    _1 = 1,
}
impl From<WDTRF_A> for bool {
    #[inline(always)]
    fn from(variant: WDTRF_A) -> Self {
        variant as u8 != 0
    }
}
impl WDTRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDTRF_A {
        match self.bits {
            false => WDTRF_A::_0,
            true => WDTRF_A::_1,
        }
    }
    #[doc = "Watchdog timer reset not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WDTRF_A::_0
    }
    #[doc = "Watchdog timer reset detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WDTRF_A::_1
    }
}
#[doc = "Field `WDTRF` writer - Watchdog Timer Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
pub type WDTRF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, WDTRF_A>;
impl<'a, REG, const O: u8> WDTRF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Watchdog timer reset not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WDTRF_A::_0)
    }
    #[doc = "Watchdog timer reset detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WDTRF_A::_1)
    }
}
#[doc = "Field `SWRF` reader - Software Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\nThe field is **modified** in some way after a read operation."]
pub type SWRF_R = crate::BitReader<SWRF_A>;
#[doc = "Software Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWRF_A {
    #[doc = "0: Software reset not detected."]
    _0 = 0,
    #[doc = "1: Software reset detected."]
    _1 = 1,
}
impl From<SWRF_A> for bool {
    #[inline(always)]
    fn from(variant: SWRF_A) -> Self {
        variant as u8 != 0
    }
}
impl SWRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWRF_A {
        match self.bits {
            false => SWRF_A::_0,
            true => SWRF_A::_1,
        }
    }
    #[doc = "Software reset not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWRF_A::_0
    }
    #[doc = "Software reset detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWRF_A::_1
    }
}
#[doc = "Field `SWRF` writer - Software Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
pub type SWRF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, SWRF_A>;
impl<'a, REG, const O: u8> SWRF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software reset not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SWRF_A::_0)
    }
    #[doc = "Software reset detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SWRF_A::_1)
    }
}
#[doc = "Field `RPERF` reader - RAM Parity Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\nThe field is **modified** in some way after a read operation."]
pub type RPERF_R = crate::BitReader<RPERF_A>;
#[doc = "RAM Parity Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPERF_A {
    #[doc = "0: RAM parity error reset not detected."]
    _0 = 0,
    #[doc = "1: RAM parity error reset detected."]
    _1 = 1,
}
impl From<RPERF_A> for bool {
    #[inline(always)]
    fn from(variant: RPERF_A) -> Self {
        variant as u8 != 0
    }
}
impl RPERF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPERF_A {
        match self.bits {
            false => RPERF_A::_0,
            true => RPERF_A::_1,
        }
    }
    #[doc = "RAM parity error reset not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPERF_A::_0
    }
    #[doc = "RAM parity error reset detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPERF_A::_1
    }
}
#[doc = "Field `RPERF` writer - RAM Parity Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
pub type RPERF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, RPERF_A>;
impl<'a, REG, const O: u8> RPERF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RAM parity error reset not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RPERF_A::_0)
    }
    #[doc = "RAM parity error reset detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RPERF_A::_1)
    }
}
#[doc = "Field `REERF` reader - RAM ECC Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\nThe field is **modified** in some way after a read operation."]
pub type REERF_R = crate::BitReader<REERF_A>;
#[doc = "RAM ECC Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REERF_A {
    #[doc = "0: RAM ECC error reset not detected."]
    _0 = 0,
    #[doc = "1: RAM ECC error reset detected."]
    _1 = 1,
}
impl From<REERF_A> for bool {
    #[inline(always)]
    fn from(variant: REERF_A) -> Self {
        variant as u8 != 0
    }
}
impl REERF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REERF_A {
        match self.bits {
            false => REERF_A::_0,
            true => REERF_A::_1,
        }
    }
    #[doc = "RAM ECC error reset not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == REERF_A::_0
    }
    #[doc = "RAM ECC error reset detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == REERF_A::_1
    }
}
#[doc = "Field `REERF` writer - RAM ECC Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
pub type REERF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, REERF_A>;
impl<'a, REG, const O: u8> REERF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RAM ECC error reset not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(REERF_A::_0)
    }
    #[doc = "RAM ECC error reset detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(REERF_A::_1)
    }
}
#[doc = "Field `BUSSRF` reader - Bus Slave MPU Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\nThe field is **modified** in some way after a read operation."]
pub type BUSSRF_R = crate::BitReader<BUSSRF_A>;
#[doc = "Bus Slave MPU Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSSRF_A {
    #[doc = "0: Bus Slave MPU reset not detected."]
    _0 = 0,
    #[doc = "1: Bus Slave MPU reset detected."]
    _1 = 1,
}
impl From<BUSSRF_A> for bool {
    #[inline(always)]
    fn from(variant: BUSSRF_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSSRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUSSRF_A {
        match self.bits {
            false => BUSSRF_A::_0,
            true => BUSSRF_A::_1,
        }
    }
    #[doc = "Bus Slave MPU reset not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSSRF_A::_0
    }
    #[doc = "Bus Slave MPU reset detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSSRF_A::_1
    }
}
#[doc = "Field `BUSSRF` writer - Bus Slave MPU Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
pub type BUSSRF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, BUSSRF_A>;
impl<'a, REG, const O: u8> BUSSRF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bus Slave MPU reset not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BUSSRF_A::_0)
    }
    #[doc = "Bus Slave MPU reset detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BUSSRF_A::_1)
    }
}
#[doc = "Field `BUSMRF` reader - Bus Master MPU Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\nThe field is **modified** in some way after a read operation."]
pub type BUSMRF_R = crate::BitReader<BUSMRF_A>;
#[doc = "Bus Master MPU Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSMRF_A {
    #[doc = "0: Bus Master MPU reset not detected."]
    _0 = 0,
    #[doc = "1: Bus Master MPU reset detected."]
    _1 = 1,
}
impl From<BUSMRF_A> for bool {
    #[inline(always)]
    fn from(variant: BUSMRF_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSMRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUSMRF_A {
        match self.bits {
            false => BUSMRF_A::_0,
            true => BUSMRF_A::_1,
        }
    }
    #[doc = "Bus Master MPU reset not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSMRF_A::_0
    }
    #[doc = "Bus Master MPU reset detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSMRF_A::_1
    }
}
#[doc = "Field `BUSMRF` writer - Bus Master MPU Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
pub type BUSMRF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, BUSMRF_A>;
impl<'a, REG, const O: u8> BUSMRF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bus Master MPU reset not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BUSMRF_A::_0)
    }
    #[doc = "Bus Master MPU reset detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BUSMRF_A::_1)
    }
}
#[doc = "Field `SPERF` reader - SP Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\nThe field is **modified** in some way after a read operation."]
pub type SPERF_R = crate::BitReader<SPERF_A>;
#[doc = "SP Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPERF_A {
    #[doc = "0: SP error reset not detected."]
    _0 = 0,
    #[doc = "1: SP error reset detected."]
    _1 = 1,
}
impl From<SPERF_A> for bool {
    #[inline(always)]
    fn from(variant: SPERF_A) -> Self {
        variant as u8 != 0
    }
}
impl SPERF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPERF_A {
        match self.bits {
            false => SPERF_A::_0,
            true => SPERF_A::_1,
        }
    }
    #[doc = "SP error reset not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPERF_A::_0
    }
    #[doc = "SP error reset detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPERF_A::_1
    }
}
#[doc = "Field `SPERF` writer - SP Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
pub type SPERF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, SPERF_A>;
impl<'a, REG, const O: u8> SPERF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SP error reset not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPERF_A::_0)
    }
    #[doc = "SP error reset detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPERF_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Independent Watchdog Timer Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn iwdtrf(&self) -> IWDTRF_R {
        IWDTRF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog Timer Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn wdtrf(&self) -> WDTRF_R {
        WDTRF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn swrf(&self) -> SWRF_R {
        SWRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - RAM Parity Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn rperf(&self) -> RPERF_R {
        RPERF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RAM ECC Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn reerf(&self) -> REERF_R {
        REERF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bus Slave MPU Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn bussrf(&self) -> BUSSRF_R {
        BUSSRF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bus Master MPU Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn busmrf(&self) -> BUSMRF_R {
        BUSMRF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SP Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn sperf(&self) -> SPERF_R {
        SPERF_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Independent Watchdog Timer Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    #[must_use]
    pub fn iwdtrf(&mut self) -> IWDTRF_W<RSTSR1_SPEC, 0> {
        IWDTRF_W::new(self)
    }
    #[doc = "Bit 1 - Watchdog Timer Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    #[must_use]
    pub fn wdtrf(&mut self) -> WDTRF_W<RSTSR1_SPEC, 1> {
        WDTRF_W::new(self)
    }
    #[doc = "Bit 2 - Software Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    #[must_use]
    pub fn swrf(&mut self) -> SWRF_W<RSTSR1_SPEC, 2> {
        SWRF_W::new(self)
    }
    #[doc = "Bit 8 - RAM Parity Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    #[must_use]
    pub fn rperf(&mut self) -> RPERF_W<RSTSR1_SPEC, 8> {
        RPERF_W::new(self)
    }
    #[doc = "Bit 9 - RAM ECC Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    #[must_use]
    pub fn reerf(&mut self) -> REERF_W<RSTSR1_SPEC, 9> {
        REERF_W::new(self)
    }
    #[doc = "Bit 10 - Bus Slave MPU Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    #[must_use]
    pub fn bussrf(&mut self) -> BUSSRF_W<RSTSR1_SPEC, 10> {
        BUSSRF_W::new(self)
    }
    #[doc = "Bit 11 - Bus Master MPU Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    #[must_use]
    pub fn busmrf(&mut self) -> BUSMRF_W<RSTSR1_SPEC, 11> {
        BUSMRF_W::new(self)
    }
    #[doc = "Bit 12 - SP Error Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    #[must_use]
    pub fn sperf(&mut self) -> SPERF_W<RSTSR1_SPEC, 12> {
        SPERF_W::new(self)
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
#[doc = "Reset Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstsr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstsr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSTSR1_SPEC;
impl crate::RegisterSpec for RSTSR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rstsr1::R`](R) reader structure"]
impl crate::Readable for RSTSR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rstsr1::W`](W) writer structure"]
impl crate::Writable for RSTSR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x1f07;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSTSR1 to value 0"]
impl crate::Resettable for RSTSR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
