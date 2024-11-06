#[doc = "Register `LSR` reader"]
pub type R = crate::R<LSR_SPEC>;
#[doc = "Field `ORER` reader - Overrun Error Flag (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
pub type ORER_R = crate::BitReader<ORER_A>;
#[doc = "Overrun Error Flag (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ORER_A {
    #[doc = "0: No overrun error occurred"]
    _0 = 0,
    #[doc = "1: An overrun error has occurred"]
    _1 = 1,
}
impl From<ORER_A> for bool {
    #[inline(always)]
    fn from(variant: ORER_A) -> Self {
        variant as u8 != 0
    }
}
impl ORER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ORER_A {
        match self.bits {
            false => ORER_A::_0,
            true => ORER_A::_1,
        }
    }
    #[doc = "No overrun error occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ORER_A::_0
    }
    #[doc = "An overrun error has occurred"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ORER_A::_1
    }
}
#[doc = "Field `FNUM` reader - Framing Error Count Indicates the quantity of data with a framing error among the receive data stored in the receive FIFO data register (FRDRH and FRDRL)."]
pub type FNUM_R = crate::FieldReader;
#[doc = "Field `PNUM` reader - Parity Error Count Indicates the quantity of data with a parity error among the receive data stored in the receive FIFO data register (FRDRH and FRDRL)."]
pub type PNUM_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Overrun Error Flag (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
    #[inline(always)]
    pub fn orer(&self) -> ORER_R {
        ORER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:6 - Framing Error Count Indicates the quantity of data with a framing error among the receive data stored in the receive FIFO data register (FRDRH and FRDRL)."]
    #[inline(always)]
    pub fn fnum(&self) -> FNUM_R {
        FNUM_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Parity Error Count Indicates the quantity of data with a parity error among the receive data stored in the receive FIFO data register (FRDRH and FRDRL)."]
    #[inline(always)]
    pub fn pnum(&self) -> PNUM_R {
        PNUM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
#[doc = "Line Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LSR_SPEC;
impl crate::RegisterSpec for LSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lsr::R`](R) reader structure"]
impl crate::Readable for LSR_SPEC {}
#[doc = "`reset()` method sets LSR to value 0"]
impl crate::Resettable for LSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
