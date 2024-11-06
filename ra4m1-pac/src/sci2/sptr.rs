#[doc = "Register `SPTR` reader"]
pub type R = crate::R<SPTR_SPEC>;
#[doc = "Register `SPTR` writer"]
pub type W = crate::W<SPTR_SPEC>;
#[doc = "Field `RXDMON` reader - Serial input data monitor bit (The state of the RXD terminal is shown.)"]
pub type RXDMON_R = crate::BitReader<RXDMON_A>;
#[doc = "Serial input data monitor bit (The state of the RXD terminal is shown.)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDMON_A {
    #[doc = "0: RXD pin is low."]
    _0 = 0,
    #[doc = "1: RXD pin is high."]
    _1 = 1,
}
impl From<RXDMON_A> for bool {
    #[inline(always)]
    fn from(variant: RXDMON_A) -> Self {
        variant as u8 != 0
    }
}
impl RXDMON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXDMON_A {
        match self.bits {
            false => RXDMON_A::_0,
            true => RXDMON_A::_1,
        }
    }
    #[doc = "RXD pin is low."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXDMON_A::_0
    }
    #[doc = "RXD pin is high."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXDMON_A::_1
    }
}
#[doc = "Field `SPB2DT` reader - Serial port break data select bit (The output level of TxD terminal is selected when SCR.TE = 0.)"]
pub type SPB2DT_R = crate::BitReader<SPB2DT_A>;
#[doc = "Serial port break data select bit (The output level of TxD terminal is selected when SCR.TE = 0.)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPB2DT_A {
    #[doc = "0: Low level is output on TXD pin"]
    _0 = 0,
    #[doc = "1: High level is output on TXD pin"]
    _1 = 1,
}
impl From<SPB2DT_A> for bool {
    #[inline(always)]
    fn from(variant: SPB2DT_A) -> Self {
        variant as u8 != 0
    }
}
impl SPB2DT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPB2DT_A {
        match self.bits {
            false => SPB2DT_A::_0,
            true => SPB2DT_A::_1,
        }
    }
    #[doc = "Low level is output on TXD pin"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPB2DT_A::_0
    }
    #[doc = "High level is output on TXD pin"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPB2DT_A::_1
    }
}
#[doc = "Field `SPB2DT` writer - Serial port break data select bit (The output level of TxD terminal is selected when SCR.TE = 0.)"]
pub type SPB2DT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SPB2DT_A>;
impl<'a, REG, const O: u8> SPB2DT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level is output on TXD pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPB2DT_A::_0)
    }
    #[doc = "High level is output on TXD pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPB2DT_A::_1)
    }
}
#[doc = "Field `SPB2IO` reader - Serial port break I/O bit (It's selected whether the value of SPB2DT is output to TxD terminal.)"]
pub type SPB2IO_R = crate::BitReader<SPB2IO_A>;
#[doc = "Serial port break I/O bit (It's selected whether the value of SPB2DT is output to TxD terminal.)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPB2IO_A {
    #[doc = "0: The value of SPB2DT bit is not output in TXD pin."]
    _0 = 0,
    #[doc = "1: The value of SPB2DT bit is output in TXD pin."]
    _1 = 1,
}
impl From<SPB2IO_A> for bool {
    #[inline(always)]
    fn from(variant: SPB2IO_A) -> Self {
        variant as u8 != 0
    }
}
impl SPB2IO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPB2IO_A {
        match self.bits {
            false => SPB2IO_A::_0,
            true => SPB2IO_A::_1,
        }
    }
    #[doc = "The value of SPB2DT bit is not output in TXD pin."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPB2IO_A::_0
    }
    #[doc = "The value of SPB2DT bit is output in TXD pin."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPB2IO_A::_1
    }
}
#[doc = "Field `SPB2IO` writer - Serial port break I/O bit (It's selected whether the value of SPB2DT is output to TxD terminal.)"]
pub type SPB2IO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SPB2IO_A>;
impl<'a, REG, const O: u8> SPB2IO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The value of SPB2DT bit is not output in TXD pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPB2IO_A::_0)
    }
    #[doc = "The value of SPB2DT bit is output in TXD pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPB2IO_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Serial input data monitor bit (The state of the RXD terminal is shown.)"]
    #[inline(always)]
    pub fn rxdmon(&self) -> RXDMON_R {
        RXDMON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Serial port break data select bit (The output level of TxD terminal is selected when SCR.TE = 0.)"]
    #[inline(always)]
    pub fn spb2dt(&self) -> SPB2DT_R {
        SPB2DT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Serial port break I/O bit (It's selected whether the value of SPB2DT is output to TxD terminal.)"]
    #[inline(always)]
    pub fn spb2io(&self) -> SPB2IO_R {
        SPB2IO_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Serial port break data select bit (The output level of TxD terminal is selected when SCR.TE = 0.)"]
    #[inline(always)]
    #[must_use]
    pub fn spb2dt(&mut self) -> SPB2DT_W<SPTR_SPEC, 1> {
        SPB2DT_W::new(self)
    }
    #[doc = "Bit 2 - Serial port break I/O bit (It's selected whether the value of SPB2DT is output to TxD terminal.)"]
    #[inline(always)]
    #[must_use]
    pub fn spb2io(&mut self) -> SPB2IO_W<SPTR_SPEC, 2> {
        SPB2IO_W::new(self)
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
#[doc = "Serial Port Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sptr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sptr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPTR_SPEC;
impl crate::RegisterSpec for SPTR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sptr::R`](R) reader structure"]
impl crate::Readable for SPTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sptr::W`](W) writer structure"]
impl crate::Writable for SPTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPTR to value 0x03"]
impl crate::Resettable for SPTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
