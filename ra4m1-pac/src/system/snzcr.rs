#[doc = "Register `SNZCR` reader"]
pub type R = crate::R<SNZCR_SPEC>;
#[doc = "Register `SNZCR` writer"]
pub type W = crate::W<SNZCR_SPEC>;
#[doc = "Field `RXDREQEN` reader - RXD0 Snooze Request Enable NOTE: Do not set to 1 other than in asynchronous mode."]
pub type RXDREQEN_R = crate::BitReader<RXDREQEN_A>;
#[doc = "RXD0 Snooze Request Enable NOTE: Do not set to 1 other than in asynchronous mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDREQEN_A {
    #[doc = "0: Ignore RXD0 falling edge in Software Standby mode."]
    _0 = 0,
    #[doc = "1: Accept RXD0 falling edge in Standby mode as a request to transit to Snooze mode."]
    _1 = 1,
}
impl From<RXDREQEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXDREQEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RXDREQEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXDREQEN_A {
        match self.bits {
            false => RXDREQEN_A::_0,
            true => RXDREQEN_A::_1,
        }
    }
    #[doc = "Ignore RXD0 falling edge in Software Standby mode."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXDREQEN_A::_0
    }
    #[doc = "Accept RXD0 falling edge in Standby mode as a request to transit to Snooze mode."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXDREQEN_A::_1
    }
}
#[doc = "Field `RXDREQEN` writer - RXD0 Snooze Request Enable NOTE: Do not set to 1 other than in asynchronous mode."]
pub type RXDREQEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RXDREQEN_A>;
impl<'a, REG, const O: u8> RXDREQEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore RXD0 falling edge in Software Standby mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RXDREQEN_A::_0)
    }
    #[doc = "Accept RXD0 falling edge in Standby mode as a request to transit to Snooze mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RXDREQEN_A::_1)
    }
}
#[doc = "Field `SNZDTCEN` reader - DTC Enable in Snooze Mode"]
pub type SNZDTCEN_R = crate::BitReader<SNZDTCEN_A>;
#[doc = "DTC Enable in Snooze Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZDTCEN_A {
    #[doc = "0: Disable DTC operation"]
    _0 = 0,
    #[doc = "1: Enable DTC operation"]
    _1 = 1,
}
impl From<SNZDTCEN_A> for bool {
    #[inline(always)]
    fn from(variant: SNZDTCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZDTCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SNZDTCEN_A {
        match self.bits {
            false => SNZDTCEN_A::_0,
            true => SNZDTCEN_A::_1,
        }
    }
    #[doc = "Disable DTC operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZDTCEN_A::_0
    }
    #[doc = "Enable DTC operation"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZDTCEN_A::_1
    }
}
#[doc = "Field `SNZDTCEN` writer - DTC Enable in Snooze Mode"]
pub type SNZDTCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SNZDTCEN_A>;
impl<'a, REG, const O: u8> SNZDTCEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable DTC operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZDTCEN_A::_0)
    }
    #[doc = "Enable DTC operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZDTCEN_A::_1)
    }
}
#[doc = "Field `SNZE` reader - Snooze Mode Enable"]
pub type SNZE_R = crate::BitReader<SNZE_A>;
#[doc = "Snooze Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZE_A {
    #[doc = "0: Disable Snooze Mode"]
    _0 = 0,
    #[doc = "1: Enable Snooze Mode"]
    _1 = 1,
}
impl From<SNZE_A> for bool {
    #[inline(always)]
    fn from(variant: SNZE_A) -> Self {
        variant as u8 != 0
    }
}
impl SNZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SNZE_A {
        match self.bits {
            false => SNZE_A::_0,
            true => SNZE_A::_1,
        }
    }
    #[doc = "Disable Snooze Mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZE_A::_0
    }
    #[doc = "Enable Snooze Mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZE_A::_1
    }
}
#[doc = "Field `SNZE` writer - Snooze Mode Enable"]
pub type SNZE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SNZE_A>;
impl<'a, REG, const O: u8> SNZE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Snooze Mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZE_A::_0)
    }
    #[doc = "Enable Snooze Mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - RXD0 Snooze Request Enable NOTE: Do not set to 1 other than in asynchronous mode."]
    #[inline(always)]
    pub fn rxdreqen(&self) -> RXDREQEN_R {
        RXDREQEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DTC Enable in Snooze Mode"]
    #[inline(always)]
    pub fn snzdtcen(&self) -> SNZDTCEN_R {
        SNZDTCEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - Snooze Mode Enable"]
    #[inline(always)]
    pub fn snze(&self) -> SNZE_R {
        SNZE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXD0 Snooze Request Enable NOTE: Do not set to 1 other than in asynchronous mode."]
    #[inline(always)]
    #[must_use]
    pub fn rxdreqen(&mut self) -> RXDREQEN_W<SNZCR_SPEC, 0> {
        RXDREQEN_W::new(self)
    }
    #[doc = "Bit 1 - DTC Enable in Snooze Mode"]
    #[inline(always)]
    #[must_use]
    pub fn snzdtcen(&mut self) -> SNZDTCEN_W<SNZCR_SPEC, 1> {
        SNZDTCEN_W::new(self)
    }
    #[doc = "Bit 7 - Snooze Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn snze(&mut self) -> SNZE_W<SNZCR_SPEC, 7> {
        SNZE_W::new(self)
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
#[doc = "Snooze Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`snzcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`snzcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SNZCR_SPEC;
impl crate::RegisterSpec for SNZCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`snzcr::R`](R) reader structure"]
impl crate::Readable for SNZCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`snzcr::W`](W) writer structure"]
impl crate::Writable for SNZCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SNZCR to value 0"]
impl crate::Resettable for SNZCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
