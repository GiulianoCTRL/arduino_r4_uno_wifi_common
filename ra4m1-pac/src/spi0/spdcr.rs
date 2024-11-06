#[doc = "Register `SPDCR` reader"]
pub type R = crate::R<SPDCR_SPEC>;
#[doc = "Register `SPDCR` writer"]
pub type W = crate::W<SPDCR_SPEC>;
#[doc = "Field `SPRDTD` reader - RSPI Receive/Transmit Data Selection"]
pub type SPRDTD_R = crate::BitReader<SPRDTD_A>;
#[doc = "RSPI Receive/Transmit Data Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPRDTD_A {
    #[doc = "0: SPDR values are read from the receive buffer"]
    _0 = 0,
    #[doc = "1: SPDR values are read from the transmit buffer (but only if the transmit buffer is empty)"]
    _1 = 1,
}
impl From<SPRDTD_A> for bool {
    #[inline(always)]
    fn from(variant: SPRDTD_A) -> Self {
        variant as u8 != 0
    }
}
impl SPRDTD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPRDTD_A {
        match self.bits {
            false => SPRDTD_A::_0,
            true => SPRDTD_A::_1,
        }
    }
    #[doc = "SPDR values are read from the receive buffer"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPRDTD_A::_0
    }
    #[doc = "SPDR values are read from the transmit buffer (but only if the transmit buffer is empty)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPRDTD_A::_1
    }
}
#[doc = "Field `SPRDTD` writer - RSPI Receive/Transmit Data Selection"]
pub type SPRDTD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SPRDTD_A>;
impl<'a, REG, const O: u8> SPRDTD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPDR values are read from the receive buffer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPRDTD_A::_0)
    }
    #[doc = "SPDR values are read from the transmit buffer (but only if the transmit buffer is empty)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPRDTD_A::_1)
    }
}
#[doc = "Field `SPLW` reader - SPI Word Access/Halfword Access Specification"]
pub type SPLW_R = crate::BitReader<SPLW_A>;
#[doc = "SPI Word Access/Halfword Access Specification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPLW_A {
    #[doc = "0: SPDR_HA is valid to access in halfwords"]
    _0 = 0,
    #[doc = "1: SPDR is valid (to access in words)."]
    _1 = 1,
}
impl From<SPLW_A> for bool {
    #[inline(always)]
    fn from(variant: SPLW_A) -> Self {
        variant as u8 != 0
    }
}
impl SPLW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPLW_A {
        match self.bits {
            false => SPLW_A::_0,
            true => SPLW_A::_1,
        }
    }
    #[doc = "SPDR_HA is valid to access in halfwords"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPLW_A::_0
    }
    #[doc = "SPDR is valid (to access in words)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPLW_A::_1
    }
}
#[doc = "Field `SPLW` writer - SPI Word Access/Halfword Access Specification"]
pub type SPLW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SPLW_A>;
impl<'a, REG, const O: u8> SPLW_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPDR_HA is valid to access in halfwords"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPLW_A::_0)
    }
    #[doc = "SPDR is valid (to access in words)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPLW_A::_1)
    }
}
impl R {
    #[doc = "Bit 4 - RSPI Receive/Transmit Data Selection"]
    #[inline(always)]
    pub fn sprdtd(&self) -> SPRDTD_R {
        SPRDTD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI Word Access/Halfword Access Specification"]
    #[inline(always)]
    pub fn splw(&self) -> SPLW_R {
        SPLW_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - RSPI Receive/Transmit Data Selection"]
    #[inline(always)]
    #[must_use]
    pub fn sprdtd(&mut self) -> SPRDTD_W<SPDCR_SPEC, 4> {
        SPRDTD_W::new(self)
    }
    #[doc = "Bit 5 - SPI Word Access/Halfword Access Specification"]
    #[inline(always)]
    #[must_use]
    pub fn splw(&mut self) -> SPLW_W<SPDCR_SPEC, 5> {
        SPLW_W::new(self)
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
#[doc = "SPI Data Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPDCR_SPEC;
impl crate::RegisterSpec for SPDCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spdcr::R`](R) reader structure"]
impl crate::Readable for SPDCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spdcr::W`](W) writer structure"]
impl crate::Writable for SPDCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPDCR to value 0"]
impl crate::Resettable for SPDCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
