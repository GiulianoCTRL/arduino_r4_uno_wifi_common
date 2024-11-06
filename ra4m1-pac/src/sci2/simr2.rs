#[doc = "Register `SIMR2` reader"]
pub type R = crate::R<SIMR2_SPEC>;
#[doc = "Register `SIMR2` writer"]
pub type W = crate::W<SIMR2_SPEC>;
#[doc = "Field `IICINTM` reader - I2C Interrupt Mode Select"]
pub type IICINTM_R = crate::BitReader<IICINTM_A>;
#[doc = "I2C Interrupt Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICINTM_A {
    #[doc = "0: Use ACK/NACK interrupts."]
    _0 = 0,
    #[doc = "1: Use reception and transmission interrupts"]
    _1 = 1,
}
impl From<IICINTM_A> for bool {
    #[inline(always)]
    fn from(variant: IICINTM_A) -> Self {
        variant as u8 != 0
    }
}
impl IICINTM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IICINTM_A {
        match self.bits {
            false => IICINTM_A::_0,
            true => IICINTM_A::_1,
        }
    }
    #[doc = "Use ACK/NACK interrupts."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICINTM_A::_0
    }
    #[doc = "Use reception and transmission interrupts"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICINTM_A::_1
    }
}
#[doc = "Field `IICINTM` writer - I2C Interrupt Mode Select"]
pub type IICINTM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IICINTM_A>;
impl<'a, REG, const O: u8> IICINTM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use ACK/NACK interrupts."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IICINTM_A::_0)
    }
    #[doc = "Use reception and transmission interrupts"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IICINTM_A::_1)
    }
}
#[doc = "Field `IICCSC` reader - Clock Synchronization"]
pub type IICCSC_R = crate::BitReader<IICCSC_A>;
#[doc = "Clock Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICCSC_A {
    #[doc = "0: No synchronization with the clock signal"]
    _0 = 0,
    #[doc = "1: Synchronization with the clock signal"]
    _1 = 1,
}
impl From<IICCSC_A> for bool {
    #[inline(always)]
    fn from(variant: IICCSC_A) -> Self {
        variant as u8 != 0
    }
}
impl IICCSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IICCSC_A {
        match self.bits {
            false => IICCSC_A::_0,
            true => IICCSC_A::_1,
        }
    }
    #[doc = "No synchronization with the clock signal"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICCSC_A::_0
    }
    #[doc = "Synchronization with the clock signal"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICCSC_A::_1
    }
}
#[doc = "Field `IICCSC` writer - Clock Synchronization"]
pub type IICCSC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IICCSC_A>;
impl<'a, REG, const O: u8> IICCSC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No synchronization with the clock signal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IICCSC_A::_0)
    }
    #[doc = "Synchronization with the clock signal"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IICCSC_A::_1)
    }
}
#[doc = "Field `IICACKT` reader - ACK Transmission Data"]
pub type IICACKT_R = crate::BitReader<IICACKT_A>;
#[doc = "ACK Transmission Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICACKT_A {
    #[doc = "0: ACK transmission"]
    _0 = 0,
    #[doc = "1: NACK transmission and reception of ACK/NACK"]
    _1 = 1,
}
impl From<IICACKT_A> for bool {
    #[inline(always)]
    fn from(variant: IICACKT_A) -> Self {
        variant as u8 != 0
    }
}
impl IICACKT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IICACKT_A {
        match self.bits {
            false => IICACKT_A::_0,
            true => IICACKT_A::_1,
        }
    }
    #[doc = "ACK transmission"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICACKT_A::_0
    }
    #[doc = "NACK transmission and reception of ACK/NACK"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICACKT_A::_1
    }
}
#[doc = "Field `IICACKT` writer - ACK Transmission Data"]
pub type IICACKT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IICACKT_A>;
impl<'a, REG, const O: u8> IICACKT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ACK transmission"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IICACKT_A::_0)
    }
    #[doc = "NACK transmission and reception of ACK/NACK"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IICACKT_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - I2C Interrupt Mode Select"]
    #[inline(always)]
    pub fn iicintm(&self) -> IICINTM_R {
        IICINTM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Synchronization"]
    #[inline(always)]
    pub fn iiccsc(&self) -> IICCSC_R {
        IICCSC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK Transmission Data"]
    #[inline(always)]
    pub fn iicackt(&self) -> IICACKT_R {
        IICACKT_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Interrupt Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn iicintm(&mut self) -> IICINTM_W<SIMR2_SPEC, 0> {
        IICINTM_W::new(self)
    }
    #[doc = "Bit 1 - Clock Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn iiccsc(&mut self) -> IICCSC_W<SIMR2_SPEC, 1> {
        IICCSC_W::new(self)
    }
    #[doc = "Bit 5 - ACK Transmission Data"]
    #[inline(always)]
    #[must_use]
    pub fn iicackt(&mut self) -> IICACKT_W<SIMR2_SPEC, 5> {
        IICACKT_W::new(self)
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
#[doc = "I2C Mode Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`simr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`simr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIMR2_SPEC;
impl crate::RegisterSpec for SIMR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`simr2::R`](R) reader structure"]
impl crate::Readable for SIMR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`simr2::W`](W) writer structure"]
impl crate::Writable for SIMR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIMR2 to value 0"]
impl crate::Resettable for SIMR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
