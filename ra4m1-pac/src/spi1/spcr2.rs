#[doc = "Register `SPCR2` reader"]
pub type R = crate::R<SPCR2_SPEC>;
#[doc = "Register `SPCR2` writer"]
pub type W = crate::W<SPCR2_SPEC>;
#[doc = "Field `SPPE` reader - Parity Enable"]
pub type SPPE_R = crate::BitReader<SPPE_A>;
#[doc = "Parity Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPPE_A {
    #[doc = "0: Does not add the parity bit to transmit data and does not check the parity bit of receive data"]
    _0 = 0,
    #[doc = "1: Adds the parity bit to transmit data and checks the parity bit of receive data (when SPCR.TXMD = 0) / Adds the parity bit to transmit data but does not check the parity bit of receive data (when SPCR.TXMD = 1)"]
    _1 = 1,
}
impl From<SPPE_A> for bool {
    #[inline(always)]
    fn from(variant: SPPE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPPE_A {
        match self.bits {
            false => SPPE_A::_0,
            true => SPPE_A::_1,
        }
    }
    #[doc = "Does not add the parity bit to transmit data and does not check the parity bit of receive data"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPPE_A::_0
    }
    #[doc = "Adds the parity bit to transmit data and checks the parity bit of receive data (when SPCR.TXMD = 0) / Adds the parity bit to transmit data but does not check the parity bit of receive data (when SPCR.TXMD = 1)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPPE_A::_1
    }
}
#[doc = "Field `SPPE` writer - Parity Enable"]
pub type SPPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SPPE_A>;
impl<'a, REG, const O: u8> SPPE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not add the parity bit to transmit data and does not check the parity bit of receive data"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPPE_A::_0)
    }
    #[doc = "Adds the parity bit to transmit data and checks the parity bit of receive data (when SPCR.TXMD = 0) / Adds the parity bit to transmit data but does not check the parity bit of receive data (when SPCR.TXMD = 1)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPPE_A::_1)
    }
}
#[doc = "Field `SPOE` reader - Parity Mode"]
pub type SPOE_R = crate::BitReader<SPOE_A>;
#[doc = "Parity Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPOE_A {
    #[doc = "0: Selects even parity for use in transmission and reception"]
    _0 = 0,
    #[doc = "1: Selects odd parity for use in transmission and reception"]
    _1 = 1,
}
impl From<SPOE_A> for bool {
    #[inline(always)]
    fn from(variant: SPOE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPOE_A {
        match self.bits {
            false => SPOE_A::_0,
            true => SPOE_A::_1,
        }
    }
    #[doc = "Selects even parity for use in transmission and reception"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPOE_A::_0
    }
    #[doc = "Selects odd parity for use in transmission and reception"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPOE_A::_1
    }
}
#[doc = "Field `SPOE` writer - Parity Mode"]
pub type SPOE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SPOE_A>;
impl<'a, REG, const O: u8> SPOE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selects even parity for use in transmission and reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPOE_A::_0)
    }
    #[doc = "Selects odd parity for use in transmission and reception"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPOE_A::_1)
    }
}
#[doc = "Field `SPIIE` reader - SPI Idle Interrupt Enable"]
pub type SPIIE_R = crate::BitReader<SPIIE_A>;
#[doc = "SPI Idle Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPIIE_A {
    #[doc = "0: Disables the generation of idle interrupt requests"]
    _0 = 0,
    #[doc = "1: Enables the generation of idle interrupt requests"]
    _1 = 1,
}
impl From<SPIIE_A> for bool {
    #[inline(always)]
    fn from(variant: SPIIE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPIIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPIIE_A {
        match self.bits {
            false => SPIIE_A::_0,
            true => SPIIE_A::_1,
        }
    }
    #[doc = "Disables the generation of idle interrupt requests"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPIIE_A::_0
    }
    #[doc = "Enables the generation of idle interrupt requests"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPIIE_A::_1
    }
}
#[doc = "Field `SPIIE` writer - SPI Idle Interrupt Enable"]
pub type SPIIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SPIIE_A>;
impl<'a, REG, const O: u8> SPIIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the generation of idle interrupt requests"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPIIE_A::_0)
    }
    #[doc = "Enables the generation of idle interrupt requests"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPIIE_A::_1)
    }
}
#[doc = "Field `PTE` reader - Parity Self-Testing"]
pub type PTE_R = crate::BitReader<PTE_A>;
#[doc = "Parity Self-Testing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTE_A {
    #[doc = "0: Disables the self-diagnosis function of the parity circuit"]
    _0 = 0,
    #[doc = "1: Enables the self-diagnosis function of the parity circuit"]
    _1 = 1,
}
impl From<PTE_A> for bool {
    #[inline(always)]
    fn from(variant: PTE_A) -> Self {
        variant as u8 != 0
    }
}
impl PTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PTE_A {
        match self.bits {
            false => PTE_A::_0,
            true => PTE_A::_1,
        }
    }
    #[doc = "Disables the self-diagnosis function of the parity circuit"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTE_A::_0
    }
    #[doc = "Enables the self-diagnosis function of the parity circuit"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTE_A::_1
    }
}
#[doc = "Field `PTE` writer - Parity Self-Testing"]
pub type PTE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PTE_A>;
impl<'a, REG, const O: u8> PTE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the self-diagnosis function of the parity circuit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PTE_A::_0)
    }
    #[doc = "Enables the self-diagnosis function of the parity circuit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PTE_A::_1)
    }
}
#[doc = "Field `SCKASE` reader - RSPCK Auto-Stop Function Enable"]
pub type SCKASE_R = crate::BitReader<SCKASE_A>;
#[doc = "RSPCK Auto-Stop Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCKASE_A {
    #[doc = "0: Disables the RSPCK auto-stop function"]
    _0 = 0,
    #[doc = "1: Enables the RSPCK auto-stop function"]
    _1 = 1,
}
impl From<SCKASE_A> for bool {
    #[inline(always)]
    fn from(variant: SCKASE_A) -> Self {
        variant as u8 != 0
    }
}
impl SCKASE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SCKASE_A {
        match self.bits {
            false => SCKASE_A::_0,
            true => SCKASE_A::_1,
        }
    }
    #[doc = "Disables the RSPCK auto-stop function"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCKASE_A::_0
    }
    #[doc = "Enables the RSPCK auto-stop function"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCKASE_A::_1
    }
}
#[doc = "Field `SCKASE` writer - RSPCK Auto-Stop Function Enable"]
pub type SCKASE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SCKASE_A>;
impl<'a, REG, const O: u8> SCKASE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the RSPCK auto-stop function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SCKASE_A::_0)
    }
    #[doc = "Enables the RSPCK auto-stop function"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SCKASE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Parity Enable"]
    #[inline(always)]
    pub fn sppe(&self) -> SPPE_R {
        SPPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity Mode"]
    #[inline(always)]
    pub fn spoe(&self) -> SPOE_R {
        SPOE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI Idle Interrupt Enable"]
    #[inline(always)]
    pub fn spiie(&self) -> SPIIE_R {
        SPIIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity Self-Testing"]
    #[inline(always)]
    pub fn pte(&self) -> PTE_R {
        PTE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RSPCK Auto-Stop Function Enable"]
    #[inline(always)]
    pub fn sckase(&self) -> SCKASE_R {
        SCKASE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sppe(&mut self) -> SPPE_W<SPCR2_SPEC, 0> {
        SPPE_W::new(self)
    }
    #[doc = "Bit 1 - Parity Mode"]
    #[inline(always)]
    #[must_use]
    pub fn spoe(&mut self) -> SPOE_W<SPCR2_SPEC, 1> {
        SPOE_W::new(self)
    }
    #[doc = "Bit 2 - SPI Idle Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spiie(&mut self) -> SPIIE_W<SPCR2_SPEC, 2> {
        SPIIE_W::new(self)
    }
    #[doc = "Bit 3 - Parity Self-Testing"]
    #[inline(always)]
    #[must_use]
    pub fn pte(&mut self) -> PTE_W<SPCR2_SPEC, 3> {
        PTE_W::new(self)
    }
    #[doc = "Bit 4 - RSPCK Auto-Stop Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sckase(&mut self) -> SCKASE_W<SPCR2_SPEC, 4> {
        SCKASE_W::new(self)
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
#[doc = "SPI Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spcr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spcr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPCR2_SPEC;
impl crate::RegisterSpec for SPCR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spcr2::R`](R) reader structure"]
impl crate::Readable for SPCR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spcr2::W`](W) writer structure"]
impl crate::Writable for SPCR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPCR2 to value 0"]
impl crate::Resettable for SPCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
