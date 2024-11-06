#[doc = "Register `RCR2` reader"]
pub type R = crate::R<RCR2_SPEC>;
#[doc = "Register `RCR2` writer"]
pub type W = crate::W<RCR2_SPEC>;
#[doc = "Field `START` reader - Start"]
pub type START_R = crate::BitReader<START_A>;
#[doc = "Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START_A {
    #[doc = "0: Prescaler and time counter are stopped."]
    _0 = 0,
    #[doc = "1: Prescaler and time counter operate normally."]
    _1 = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> START_A {
        match self.bits {
            false => START_A::_0,
            true => START_A::_1,
        }
    }
    #[doc = "Prescaler and time counter are stopped."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == START_A::_0
    }
    #[doc = "Prescaler and time counter operate normally."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == START_A::_1
    }
}
#[doc = "Field `START` writer - Start"]
pub type START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, START_A>;
impl<'a, REG, const O: u8> START_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Prescaler and time counter are stopped."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::_0)
    }
    #[doc = "Prescaler and time counter operate normally."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::_1)
    }
}
#[doc = "Field `RESET` reader - RTC Software Reset"]
pub type RESET_R = crate::BitReader<RESET_A>;
#[doc = "RTC Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_A {
    #[doc = "0: Writing is invalid.(write) / In normal time operation, or an RTC software reset has completed.(read)"]
    _0 = 0,
    #[doc = "1: The prescaler and the target registers for RTC software reset *1 are initialized.(write) / During an RTC software reset.(read)"]
    _1 = 1,
}
impl From<RESET_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_A) -> Self {
        variant as u8 != 0
    }
}
impl RESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RESET_A {
        match self.bits {
            false => RESET_A::_0,
            true => RESET_A::_1,
        }
    }
    #[doc = "Writing is invalid.(write) / In normal time operation, or an RTC software reset has completed.(read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RESET_A::_0
    }
    #[doc = "The prescaler and the target registers for RTC software reset *1 are initialized.(write) / During an RTC software reset.(read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RESET_A::_1
    }
}
#[doc = "Field `RESET` writer - RTC Software Reset"]
pub type RESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RESET_A>;
impl<'a, REG, const O: u8> RESET_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing is invalid.(write) / In normal time operation, or an RTC software reset has completed.(read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_A::_0)
    }
    #[doc = "The prescaler and the target registers for RTC software reset *1 are initialized.(write) / During an RTC software reset.(read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_A::_1)
    }
}
#[doc = "Field `ADJ30` reader - 30-Second Adjustment"]
pub type ADJ30_R = crate::BitReader<ADJ30_A>;
#[doc = "30-Second Adjustment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADJ30_A {
    #[doc = "0: Writing is invalid.(write) / In normal time operation, or 30-second adjustment has completed.(read)"]
    _0 = 0,
    #[doc = "1: 30-second adjustment is executed.(write) / During 30-second adjustment.(read)"]
    _1 = 1,
}
impl From<ADJ30_A> for bool {
    #[inline(always)]
    fn from(variant: ADJ30_A) -> Self {
        variant as u8 != 0
    }
}
impl ADJ30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADJ30_A {
        match self.bits {
            false => ADJ30_A::_0,
            true => ADJ30_A::_1,
        }
    }
    #[doc = "Writing is invalid.(write) / In normal time operation, or 30-second adjustment has completed.(read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADJ30_A::_0
    }
    #[doc = "30-second adjustment is executed.(write) / During 30-second adjustment.(read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADJ30_A::_1
    }
}
#[doc = "Field `ADJ30` writer - 30-Second Adjustment"]
pub type ADJ30_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADJ30_A>;
impl<'a, REG, const O: u8> ADJ30_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing is invalid.(write) / In normal time operation, or 30-second adjustment has completed.(read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADJ30_A::_0)
    }
    #[doc = "30-second adjustment is executed.(write) / During 30-second adjustment.(read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADJ30_A::_1)
    }
}
#[doc = "Field `RTCOE` reader - RTCOUT Output Enable"]
pub type RTCOE_R = crate::BitReader<RTCOE_A>;
#[doc = "RTCOUT Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCOE_A {
    #[doc = "0: RTCOUT output disabled."]
    _0 = 0,
    #[doc = "1: RTCOUT output enabled."]
    _1 = 1,
}
impl From<RTCOE_A> for bool {
    #[inline(always)]
    fn from(variant: RTCOE_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTCOE_A {
        match self.bits {
            false => RTCOE_A::_0,
            true => RTCOE_A::_1,
        }
    }
    #[doc = "RTCOUT output disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTCOE_A::_0
    }
    #[doc = "RTCOUT output enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTCOE_A::_1
    }
}
#[doc = "Field `RTCOE` writer - RTCOUT Output Enable"]
pub type RTCOE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RTCOE_A>;
impl<'a, REG, const O: u8> RTCOE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTCOUT output disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RTCOE_A::_0)
    }
    #[doc = "RTCOUT output enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RTCOE_A::_1)
    }
}
#[doc = "Field `AADJE` reader - Automatic Adjustment Enable (When the LOCO clock is selected, the setting of this bit is disabled.)"]
pub type AADJE_R = crate::BitReader<AADJE_A>;
#[doc = "Automatic Adjustment Enable (When the LOCO clock is selected, the setting of this bit is disabled.)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AADJE_A {
    #[doc = "0: Automatic adjustment is disabled."]
    _0 = 0,
    #[doc = "1: Automatic adjustment is enabled."]
    _1 = 1,
}
impl From<AADJE_A> for bool {
    #[inline(always)]
    fn from(variant: AADJE_A) -> Self {
        variant as u8 != 0
    }
}
impl AADJE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AADJE_A {
        match self.bits {
            false => AADJE_A::_0,
            true => AADJE_A::_1,
        }
    }
    #[doc = "Automatic adjustment is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AADJE_A::_0
    }
    #[doc = "Automatic adjustment is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AADJE_A::_1
    }
}
#[doc = "Field `AADJE` writer - Automatic Adjustment Enable (When the LOCO clock is selected, the setting of this bit is disabled.)"]
pub type AADJE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AADJE_A>;
impl<'a, REG, const O: u8> AADJE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic adjustment is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AADJE_A::_0)
    }
    #[doc = "Automatic adjustment is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AADJE_A::_1)
    }
}
#[doc = "Field `AADJP` reader - Automatic Adjustment Period Select (When the LOCO clock is selected, the setting of this bit is disabled.)"]
pub type AADJP_R = crate::BitReader<AADJP_A>;
#[doc = "Automatic Adjustment Period Select (When the LOCO clock is selected, the setting of this bit is disabled.)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AADJP_A {
    #[doc = "0: The RADJ.ADJ\\[5:0\\]
setting value is adjusted from the count value of the prescaler every minute."]
    _0 = 0,
    #[doc = "1: The RADJ.ADJ\\[5:0\\]
setting value is adjusted from the count value of the prescaler every 10 seconds."]
    _1 = 1,
}
impl From<AADJP_A> for bool {
    #[inline(always)]
    fn from(variant: AADJP_A) -> Self {
        variant as u8 != 0
    }
}
impl AADJP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AADJP_A {
        match self.bits {
            false => AADJP_A::_0,
            true => AADJP_A::_1,
        }
    }
    #[doc = "The RADJ.ADJ\\[5:0\\]
setting value is adjusted from the count value of the prescaler every minute."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AADJP_A::_0
    }
    #[doc = "The RADJ.ADJ\\[5:0\\]
setting value is adjusted from the count value of the prescaler every 10 seconds."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AADJP_A::_1
    }
}
#[doc = "Field `AADJP` writer - Automatic Adjustment Period Select (When the LOCO clock is selected, the setting of this bit is disabled.)"]
pub type AADJP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AADJP_A>;
impl<'a, REG, const O: u8> AADJP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The RADJ.ADJ\\[5:0\\]
setting value is adjusted from the count value of the prescaler every minute."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AADJP_A::_0)
    }
    #[doc = "The RADJ.ADJ\\[5:0\\]
setting value is adjusted from the count value of the prescaler every 10 seconds."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AADJP_A::_1)
    }
}
#[doc = "Field `HR24` reader - Hours Mode"]
pub type HR24_R = crate::BitReader<HR24_A>;
#[doc = "Hours Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HR24_A {
    #[doc = "0: The RTC operates in 12-hour mode."]
    _0 = 0,
    #[doc = "1: The RTC operates in 24-hour mode."]
    _1 = 1,
}
impl From<HR24_A> for bool {
    #[inline(always)]
    fn from(variant: HR24_A) -> Self {
        variant as u8 != 0
    }
}
impl HR24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HR24_A {
        match self.bits {
            false => HR24_A::_0,
            true => HR24_A::_1,
        }
    }
    #[doc = "The RTC operates in 12-hour mode."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HR24_A::_0
    }
    #[doc = "The RTC operates in 24-hour mode."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HR24_A::_1
    }
}
#[doc = "Field `HR24` writer - Hours Mode"]
pub type HR24_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HR24_A>;
impl<'a, REG, const O: u8> HR24_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The RTC operates in 12-hour mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(HR24_A::_0)
    }
    #[doc = "The RTC operates in 24-hour mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(HR24_A::_1)
    }
}
#[doc = "Field `CNTMD` reader - Count Mode Select"]
pub type CNTMD_R = crate::BitReader<CNTMD_A>;
#[doc = "Count Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNTMD_A {
    #[doc = "0: The calendar count mode."]
    _0 = 0,
    #[doc = "1: The binary count mode."]
    _1 = 1,
}
impl From<CNTMD_A> for bool {
    #[inline(always)]
    fn from(variant: CNTMD_A) -> Self {
        variant as u8 != 0
    }
}
impl CNTMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CNTMD_A {
        match self.bits {
            false => CNTMD_A::_0,
            true => CNTMD_A::_1,
        }
    }
    #[doc = "The calendar count mode."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CNTMD_A::_0
    }
    #[doc = "The binary count mode."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CNTMD_A::_1
    }
}
#[doc = "Field `CNTMD` writer - Count Mode Select"]
pub type CNTMD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CNTMD_A>;
impl<'a, REG, const O: u8> CNTMD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The calendar count mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CNTMD_A::_0)
    }
    #[doc = "The binary count mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CNTMD_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Start"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC Software Reset"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 30-Second Adjustment"]
    #[inline(always)]
    pub fn adj30(&self) -> ADJ30_R {
        ADJ30_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTCOUT Output Enable"]
    #[inline(always)]
    pub fn rtcoe(&self) -> RTCOE_R {
        RTCOE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Automatic Adjustment Enable (When the LOCO clock is selected, the setting of this bit is disabled.)"]
    #[inline(always)]
    pub fn aadje(&self) -> AADJE_R {
        AADJE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Automatic Adjustment Period Select (When the LOCO clock is selected, the setting of this bit is disabled.)"]
    #[inline(always)]
    pub fn aadjp(&self) -> AADJP_R {
        AADJP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hours Mode"]
    #[inline(always)]
    pub fn hr24(&self) -> HR24_R {
        HR24_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Count Mode Select"]
    #[inline(always)]
    pub fn cntmd(&self) -> CNTMD_R {
        CNTMD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<RCR2_SPEC, 0> {
        START_W::new(self)
    }
    #[doc = "Bit 1 - RTC Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<RCR2_SPEC, 1> {
        RESET_W::new(self)
    }
    #[doc = "Bit 2 - 30-Second Adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn adj30(&mut self) -> ADJ30_W<RCR2_SPEC, 2> {
        ADJ30_W::new(self)
    }
    #[doc = "Bit 3 - RTCOUT Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcoe(&mut self) -> RTCOE_W<RCR2_SPEC, 3> {
        RTCOE_W::new(self)
    }
    #[doc = "Bit 4 - Automatic Adjustment Enable (When the LOCO clock is selected, the setting of this bit is disabled.)"]
    #[inline(always)]
    #[must_use]
    pub fn aadje(&mut self) -> AADJE_W<RCR2_SPEC, 4> {
        AADJE_W::new(self)
    }
    #[doc = "Bit 5 - Automatic Adjustment Period Select (When the LOCO clock is selected, the setting of this bit is disabled.)"]
    #[inline(always)]
    #[must_use]
    pub fn aadjp(&mut self) -> AADJP_W<RCR2_SPEC, 5> {
        AADJP_W::new(self)
    }
    #[doc = "Bit 6 - Hours Mode"]
    #[inline(always)]
    #[must_use]
    pub fn hr24(&mut self) -> HR24_W<RCR2_SPEC, 6> {
        HR24_W::new(self)
    }
    #[doc = "Bit 7 - Count Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn cntmd(&mut self) -> CNTMD_W<RCR2_SPEC, 7> {
        CNTMD_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RTC Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCR2_SPEC;
impl crate::RegisterSpec for RCR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rcr2::R`](R) reader structure"]
impl crate::Readable for RCR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rcr2::W`](W) writer structure"]
impl crate::Writable for RCR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCR2 to value 0"]
impl crate::Resettable for RCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
