#[doc = "Register `SPPCR` reader"]
pub type R = crate::R<SPPCR_SPEC>;
#[doc = "Register `SPPCR` writer"]
pub type W = crate::W<SPPCR_SPEC>;
#[doc = "Field `SPLP` reader - RSPI Loopback"]
pub type SPLP_R = crate::BitReader<SPLP_A>;
#[doc = "RSPI Loopback\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPLP_A {
    #[doc = "0: Normal mode"]
    _0 = 0,
    #[doc = "1: Loopback mode (data is inverted for transmission)"]
    _1 = 1,
}
impl From<SPLP_A> for bool {
    #[inline(always)]
    fn from(variant: SPLP_A) -> Self {
        variant as u8 != 0
    }
}
impl SPLP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPLP_A {
        match self.bits {
            false => SPLP_A::_0,
            true => SPLP_A::_1,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPLP_A::_0
    }
    #[doc = "Loopback mode (data is inverted for transmission)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPLP_A::_1
    }
}
#[doc = "Field `SPLP` writer - RSPI Loopback"]
pub type SPLP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SPLP_A>;
impl<'a, REG, const O: u8> SPLP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPLP_A::_0)
    }
    #[doc = "Loopback mode (data is inverted for transmission)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPLP_A::_1)
    }
}
#[doc = "Field `SPLP2` reader - RSPI Loopback 2"]
pub type SPLP2_R = crate::BitReader<SPLP2_A>;
#[doc = "RSPI Loopback 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPLP2_A {
    #[doc = "0: Normal mode"]
    _0 = 0,
    #[doc = "1: Loopback mode (data is not inverted for transmission)"]
    _1 = 1,
}
impl From<SPLP2_A> for bool {
    #[inline(always)]
    fn from(variant: SPLP2_A) -> Self {
        variant as u8 != 0
    }
}
impl SPLP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPLP2_A {
        match self.bits {
            false => SPLP2_A::_0,
            true => SPLP2_A::_1,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPLP2_A::_0
    }
    #[doc = "Loopback mode (data is not inverted for transmission)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPLP2_A::_1
    }
}
#[doc = "Field `SPLP2` writer - RSPI Loopback 2"]
pub type SPLP2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SPLP2_A>;
impl<'a, REG, const O: u8> SPLP2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPLP2_A::_0)
    }
    #[doc = "Loopback mode (data is not inverted for transmission)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPLP2_A::_1)
    }
}
#[doc = "Field `MOIFV` reader - MOSI Idle Fixed Value"]
pub type MOIFV_R = crate::BitReader<MOIFV_A>;
#[doc = "MOSI Idle Fixed Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOIFV_A {
    #[doc = "0: The level output on the MOSIn pin during MOSI idling corresponds to low."]
    _0 = 0,
    #[doc = "1: The level output on the MOSIn pin during MOSI idling corresponds to high."]
    _1 = 1,
}
impl From<MOIFV_A> for bool {
    #[inline(always)]
    fn from(variant: MOIFV_A) -> Self {
        variant as u8 != 0
    }
}
impl MOIFV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MOIFV_A {
        match self.bits {
            false => MOIFV_A::_0,
            true => MOIFV_A::_1,
        }
    }
    #[doc = "The level output on the MOSIn pin during MOSI idling corresponds to low."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MOIFV_A::_0
    }
    #[doc = "The level output on the MOSIn pin during MOSI idling corresponds to high."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MOIFV_A::_1
    }
}
#[doc = "Field `MOIFV` writer - MOSI Idle Fixed Value"]
pub type MOIFV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MOIFV_A>;
impl<'a, REG, const O: u8> MOIFV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The level output on the MOSIn pin during MOSI idling corresponds to low."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MOIFV_A::_0)
    }
    #[doc = "The level output on the MOSIn pin during MOSI idling corresponds to high."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MOIFV_A::_1)
    }
}
#[doc = "Field `MOIFE` reader - MOSI Idle Value Fixing Enable"]
pub type MOIFE_R = crate::BitReader<MOIFE_A>;
#[doc = "MOSI Idle Value Fixing Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOIFE_A {
    #[doc = "0: MOSI output value equals final data from previous transfer"]
    _0 = 0,
    #[doc = "1: MOSI output value equals the value set in the MOIFV bit"]
    _1 = 1,
}
impl From<MOIFE_A> for bool {
    #[inline(always)]
    fn from(variant: MOIFE_A) -> Self {
        variant as u8 != 0
    }
}
impl MOIFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MOIFE_A {
        match self.bits {
            false => MOIFE_A::_0,
            true => MOIFE_A::_1,
        }
    }
    #[doc = "MOSI output value equals final data from previous transfer"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MOIFE_A::_0
    }
    #[doc = "MOSI output value equals the value set in the MOIFV bit"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MOIFE_A::_1
    }
}
#[doc = "Field `MOIFE` writer - MOSI Idle Value Fixing Enable"]
pub type MOIFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MOIFE_A>;
impl<'a, REG, const O: u8> MOIFE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MOSI output value equals final data from previous transfer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MOIFE_A::_0)
    }
    #[doc = "MOSI output value equals the value set in the MOIFV bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MOIFE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - RSPI Loopback"]
    #[inline(always)]
    pub fn splp(&self) -> SPLP_R {
        SPLP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RSPI Loopback 2"]
    #[inline(always)]
    pub fn splp2(&self) -> SPLP2_R {
        SPLP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - MOSI Idle Fixed Value"]
    #[inline(always)]
    pub fn moifv(&self) -> MOIFV_R {
        MOIFV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MOSI Idle Value Fixing Enable"]
    #[inline(always)]
    pub fn moife(&self) -> MOIFE_R {
        MOIFE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RSPI Loopback"]
    #[inline(always)]
    #[must_use]
    pub fn splp(&mut self) -> SPLP_W<SPPCR_SPEC, 0> {
        SPLP_W::new(self)
    }
    #[doc = "Bit 1 - RSPI Loopback 2"]
    #[inline(always)]
    #[must_use]
    pub fn splp2(&mut self) -> SPLP2_W<SPPCR_SPEC, 1> {
        SPLP2_W::new(self)
    }
    #[doc = "Bit 4 - MOSI Idle Fixed Value"]
    #[inline(always)]
    #[must_use]
    pub fn moifv(&mut self) -> MOIFV_W<SPPCR_SPEC, 4> {
        MOIFV_W::new(self)
    }
    #[doc = "Bit 5 - MOSI Idle Value Fixing Enable"]
    #[inline(always)]
    #[must_use]
    pub fn moife(&mut self) -> MOIFE_W<SPPCR_SPEC, 5> {
        MOIFE_W::new(self)
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
#[doc = "SPI Pin Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sppcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sppcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPPCR_SPEC;
impl crate::RegisterSpec for SPPCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sppcr::R`](R) reader structure"]
impl crate::Readable for SPPCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sppcr::W`](W) writer structure"]
impl crate::Writable for SPPCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPPCR to value 0"]
impl crate::Resettable for SPPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
