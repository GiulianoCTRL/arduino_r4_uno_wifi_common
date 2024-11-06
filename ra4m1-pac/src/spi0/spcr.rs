#[doc = "Register `SPCR` reader"]
pub type R = crate::R<SPCR_SPEC>;
#[doc = "Register `SPCR` writer"]
pub type W = crate::W<SPCR_SPEC>;
#[doc = "Field `SPMS` reader - SPI Mode Select"]
pub type SPMS_R = crate::BitReader<SPMS_A>;
#[doc = "SPI Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPMS_A {
    #[doc = "0: SPI operation (4-wire method)"]
    _0 = 0,
    #[doc = "1: Clock synchronous operation (3-wire method)"]
    _1 = 1,
}
impl From<SPMS_A> for bool {
    #[inline(always)]
    fn from(variant: SPMS_A) -> Self {
        variant as u8 != 0
    }
}
impl SPMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPMS_A {
        match self.bits {
            false => SPMS_A::_0,
            true => SPMS_A::_1,
        }
    }
    #[doc = "SPI operation (4-wire method)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPMS_A::_0
    }
    #[doc = "Clock synchronous operation (3-wire method)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPMS_A::_1
    }
}
#[doc = "Field `SPMS` writer - SPI Mode Select"]
pub type SPMS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SPMS_A>;
impl<'a, REG, const O: u8> SPMS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI operation (4-wire method)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPMS_A::_0)
    }
    #[doc = "Clock synchronous operation (3-wire method)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPMS_A::_1)
    }
}
#[doc = "Field `TXMD` reader - Communications Operating Mode Select"]
pub type TXMD_R = crate::BitReader<TXMD_A>;
#[doc = "Communications Operating Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXMD_A {
    #[doc = "0: Full-duplex synchronous serial communications"]
    _0 = 0,
    #[doc = "1: Serial communications consisting of only transmit operations"]
    _1 = 1,
}
impl From<TXMD_A> for bool {
    #[inline(always)]
    fn from(variant: TXMD_A) -> Self {
        variant as u8 != 0
    }
}
impl TXMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXMD_A {
        match self.bits {
            false => TXMD_A::_0,
            true => TXMD_A::_1,
        }
    }
    #[doc = "Full-duplex synchronous serial communications"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXMD_A::_0
    }
    #[doc = "Serial communications consisting of only transmit operations"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXMD_A::_1
    }
}
#[doc = "Field `TXMD` writer - Communications Operating Mode Select"]
pub type TXMD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TXMD_A>;
impl<'a, REG, const O: u8> TXMD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Full-duplex synchronous serial communications"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TXMD_A::_0)
    }
    #[doc = "Serial communications consisting of only transmit operations"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TXMD_A::_1)
    }
}
#[doc = "Field `MODFEN` reader - Mode Fault Error Detection Enable"]
pub type MODFEN_R = crate::BitReader<MODFEN_A>;
#[doc = "Mode Fault Error Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODFEN_A {
    #[doc = "0: Disables the detection of mode fault error"]
    _0 = 0,
    #[doc = "1: Enables the detection of mode fault error"]
    _1 = 1,
}
impl From<MODFEN_A> for bool {
    #[inline(always)]
    fn from(variant: MODFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MODFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODFEN_A {
        match self.bits {
            false => MODFEN_A::_0,
            true => MODFEN_A::_1,
        }
    }
    #[doc = "Disables the detection of mode fault error"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MODFEN_A::_0
    }
    #[doc = "Enables the detection of mode fault error"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MODFEN_A::_1
    }
}
#[doc = "Field `MODFEN` writer - Mode Fault Error Detection Enable"]
pub type MODFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MODFEN_A>;
impl<'a, REG, const O: u8> MODFEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the detection of mode fault error"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MODFEN_A::_0)
    }
    #[doc = "Enables the detection of mode fault error"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MODFEN_A::_1)
    }
}
#[doc = "Field `MSTR` reader - SPI Master/Slave Mode Select"]
pub type MSTR_R = crate::BitReader<MSTR_A>;
#[doc = "SPI Master/Slave Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTR_A {
    #[doc = "0: Slave mode"]
    _0 = 0,
    #[doc = "1: Master mode"]
    _1 = 1,
}
impl From<MSTR_A> for bool {
    #[inline(always)]
    fn from(variant: MSTR_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSTR_A {
        match self.bits {
            false => MSTR_A::_0,
            true => MSTR_A::_1,
        }
    }
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTR_A::_0
    }
    #[doc = "Master mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTR_A::_1
    }
}
#[doc = "Field `MSTR` writer - SPI Master/Slave Mode Select"]
pub type MSTR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MSTR_A>;
impl<'a, REG, const O: u8> MSTR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTR_A::_0)
    }
    #[doc = "Master mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTR_A::_1)
    }
}
#[doc = "Field `SPEIE` reader - SPI Error Interrupt Enable"]
pub type SPEIE_R = crate::BitReader<SPEIE_A>;
#[doc = "SPI Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPEIE_A {
    #[doc = "0: Disables the generation of SPI error interrupt requests"]
    _0 = 0,
    #[doc = "1: Enables the generation of SPI error interrupt requests"]
    _1 = 1,
}
impl From<SPEIE_A> for bool {
    #[inline(always)]
    fn from(variant: SPEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPEIE_A {
        match self.bits {
            false => SPEIE_A::_0,
            true => SPEIE_A::_1,
        }
    }
    #[doc = "Disables the generation of SPI error interrupt requests"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPEIE_A::_0
    }
    #[doc = "Enables the generation of SPI error interrupt requests"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPEIE_A::_1
    }
}
#[doc = "Field `SPEIE` writer - SPI Error Interrupt Enable"]
pub type SPEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SPEIE_A>;
impl<'a, REG, const O: u8> SPEIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the generation of SPI error interrupt requests"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPEIE_A::_0)
    }
    #[doc = "Enables the generation of SPI error interrupt requests"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPEIE_A::_1)
    }
}
#[doc = "Field `SPTIE` reader - Transmit Buffer Empty Interrupt Enable"]
pub type SPTIE_R = crate::BitReader<SPTIE_A>;
#[doc = "Transmit Buffer Empty Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPTIE_A {
    #[doc = "0: Disables the generation of transmit buffer empty interrupt requests"]
    _0 = 0,
    #[doc = "1: Enables the generation of transmit buffer empty interrupt requests"]
    _1 = 1,
}
impl From<SPTIE_A> for bool {
    #[inline(always)]
    fn from(variant: SPTIE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPTIE_A {
        match self.bits {
            false => SPTIE_A::_0,
            true => SPTIE_A::_1,
        }
    }
    #[doc = "Disables the generation of transmit buffer empty interrupt requests"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPTIE_A::_0
    }
    #[doc = "Enables the generation of transmit buffer empty interrupt requests"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPTIE_A::_1
    }
}
#[doc = "Field `SPTIE` writer - Transmit Buffer Empty Interrupt Enable"]
pub type SPTIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SPTIE_A>;
impl<'a, REG, const O: u8> SPTIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the generation of transmit buffer empty interrupt requests"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPTIE_A::_0)
    }
    #[doc = "Enables the generation of transmit buffer empty interrupt requests"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPTIE_A::_1)
    }
}
#[doc = "Field `SPE` reader - SPI Function Enable"]
pub type SPE_R = crate::BitReader<SPE_A>;
#[doc = "SPI Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPE_A {
    #[doc = "0: Disables the SPI function"]
    _0 = 0,
    #[doc = "1: Enables the SPI function"]
    _1 = 1,
}
impl From<SPE_A> for bool {
    #[inline(always)]
    fn from(variant: SPE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPE_A {
        match self.bits {
            false => SPE_A::_0,
            true => SPE_A::_1,
        }
    }
    #[doc = "Disables the SPI function"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPE_A::_0
    }
    #[doc = "Enables the SPI function"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPE_A::_1
    }
}
#[doc = "Field `SPE` writer - SPI Function Enable"]
pub type SPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SPE_A>;
impl<'a, REG, const O: u8> SPE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the SPI function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPE_A::_0)
    }
    #[doc = "Enables the SPI function"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPE_A::_1)
    }
}
#[doc = "Field `SPRIE` reader - SPI Receive Buffer Full Interrupt Enable"]
pub type SPRIE_R = crate::BitReader<SPRIE_A>;
#[doc = "SPI Receive Buffer Full Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPRIE_A {
    #[doc = "0: Disables the generation of SPI receive buffer full interrupt requests"]
    _0 = 0,
    #[doc = "1: Enables the generation of SPI receive buffer full interrupt requests"]
    _1 = 1,
}
impl From<SPRIE_A> for bool {
    #[inline(always)]
    fn from(variant: SPRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPRIE_A {
        match self.bits {
            false => SPRIE_A::_0,
            true => SPRIE_A::_1,
        }
    }
    #[doc = "Disables the generation of SPI receive buffer full interrupt requests"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPRIE_A::_0
    }
    #[doc = "Enables the generation of SPI receive buffer full interrupt requests"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPRIE_A::_1
    }
}
#[doc = "Field `SPRIE` writer - SPI Receive Buffer Full Interrupt Enable"]
pub type SPRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SPRIE_A>;
impl<'a, REG, const O: u8> SPRIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the generation of SPI receive buffer full interrupt requests"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPRIE_A::_0)
    }
    #[doc = "Enables the generation of SPI receive buffer full interrupt requests"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPRIE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - SPI Mode Select"]
    #[inline(always)]
    pub fn spms(&self) -> SPMS_R {
        SPMS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Communications Operating Mode Select"]
    #[inline(always)]
    pub fn txmd(&self) -> TXMD_R {
        TXMD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mode Fault Error Detection Enable"]
    #[inline(always)]
    pub fn modfen(&self) -> MODFEN_R {
        MODFEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SPI Master/Slave Mode Select"]
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SPI Error Interrupt Enable"]
    #[inline(always)]
    pub fn speie(&self) -> SPEIE_R {
        SPEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn sptie(&self) -> SPTIE_R {
        SPTIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI Function Enable"]
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI Receive Buffer Full Interrupt Enable"]
    #[inline(always)]
    pub fn sprie(&self) -> SPRIE_R {
        SPRIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn spms(&mut self) -> SPMS_W<SPCR_SPEC, 0> {
        SPMS_W::new(self)
    }
    #[doc = "Bit 1 - Communications Operating Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn txmd(&mut self) -> TXMD_W<SPCR_SPEC, 1> {
        TXMD_W::new(self)
    }
    #[doc = "Bit 2 - Mode Fault Error Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn modfen(&mut self) -> MODFEN_W<SPCR_SPEC, 2> {
        MODFEN_W::new(self)
    }
    #[doc = "Bit 3 - SPI Master/Slave Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn mstr(&mut self) -> MSTR_W<SPCR_SPEC, 3> {
        MSTR_W::new(self)
    }
    #[doc = "Bit 4 - SPI Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn speie(&mut self) -> SPEIE_W<SPCR_SPEC, 4> {
        SPEIE_W::new(self)
    }
    #[doc = "Bit 5 - Transmit Buffer Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sptie(&mut self) -> SPTIE_W<SPCR_SPEC, 5> {
        SPTIE_W::new(self)
    }
    #[doc = "Bit 6 - SPI Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spe(&mut self) -> SPE_W<SPCR_SPEC, 6> {
        SPE_W::new(self)
    }
    #[doc = "Bit 7 - SPI Receive Buffer Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sprie(&mut self) -> SPRIE_W<SPCR_SPEC, 7> {
        SPRIE_W::new(self)
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
#[doc = "SPI Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPCR_SPEC;
impl crate::RegisterSpec for SPCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spcr::R`](R) reader structure"]
impl crate::Readable for SPCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spcr::W`](W) writer structure"]
impl crate::Writable for SPCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPCR to value 0"]
impl crate::Resettable for SPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
