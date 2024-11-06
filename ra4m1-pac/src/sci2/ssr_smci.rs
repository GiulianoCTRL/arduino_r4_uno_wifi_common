#[doc = "Register `SSR_SMCI` reader"]
pub type R = crate::R<SSR_SMCI_SPEC>;
#[doc = "Register `SSR_SMCI` writer"]
pub type W = crate::W<SSR_SMCI_SPEC>;
#[doc = "Field `MPBT` reader - Multi-Processor Bit Transfer This bit should be 0 in smart card interface mode."]
pub type MPBT_R = crate::BitReader;
#[doc = "Field `MPBT` writer - Multi-Processor Bit Transfer This bit should be 0 in smart card interface mode."]
pub type MPBT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MPB` reader - Multi-Processor This bit should be 0 in smart card interface mode."]
pub type MPB_R = crate::BitReader;
#[doc = "Field `TEND` reader - Transmit End Flag"]
pub type TEND_R = crate::BitReader<TEND_A>;
#[doc = "Transmit End Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEND_A {
    #[doc = "0: A character is being transmitted."]
    _0 = 0,
    #[doc = "1: Character transfer has been completed."]
    _1 = 1,
}
impl From<TEND_A> for bool {
    #[inline(always)]
    fn from(variant: TEND_A) -> Self {
        variant as u8 != 0
    }
}
impl TEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEND_A {
        match self.bits {
            false => TEND_A::_0,
            true => TEND_A::_1,
        }
    }
    #[doc = "A character is being transmitted."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TEND_A::_0
    }
    #[doc = "Character transfer has been completed."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TEND_A::_1
    }
}
#[doc = "Field `PER` reader - Parity Error Flag\n\nThe field is **modified** in some way after a read operation."]
pub type PER_R = crate::BitReader<PER_A>;
#[doc = "Parity Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PER_A {
    #[doc = "0: No parity error occurred"]
    _0 = 0,
    #[doc = "1: A parity error has occurred"]
    _1 = 1,
}
impl From<PER_A> for bool {
    #[inline(always)]
    fn from(variant: PER_A) -> Self {
        variant as u8 != 0
    }
}
impl PER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PER_A {
        match self.bits {
            false => PER_A::_0,
            true => PER_A::_1,
        }
    }
    #[doc = "No parity error occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PER_A::_0
    }
    #[doc = "A parity error has occurred"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PER_A::_1
    }
}
#[doc = "Field `PER` writer - Parity Error Flag"]
pub type PER_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, PER_A>;
impl<'a, REG, const O: u8> PER_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No parity error occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PER_A::_0)
    }
    #[doc = "A parity error has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PER_A::_1)
    }
}
#[doc = "Field `ERS` reader - Error Signal Status Flag\n\nThe field is **modified** in some way after a read operation."]
pub type ERS_R = crate::BitReader<ERS_A>;
#[doc = "Error Signal Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERS_A {
    #[doc = "0: Low error signal not responded"]
    _0 = 0,
    #[doc = "1: Low error signal responded"]
    _1 = 1,
}
impl From<ERS_A> for bool {
    #[inline(always)]
    fn from(variant: ERS_A) -> Self {
        variant as u8 != 0
    }
}
impl ERS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERS_A {
        match self.bits {
            false => ERS_A::_0,
            true => ERS_A::_1,
        }
    }
    #[doc = "Low error signal not responded"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERS_A::_0
    }
    #[doc = "Low error signal responded"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERS_A::_1
    }
}
#[doc = "Field `ERS` writer - Error Signal Status Flag"]
pub type ERS_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, ERS_A>;
impl<'a, REG, const O: u8> ERS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low error signal not responded"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ERS_A::_0)
    }
    #[doc = "Low error signal responded"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ERS_A::_1)
    }
}
#[doc = "Field `ORER` reader - Overrun Error Flag\n\nThe field is **modified** in some way after a read operation."]
pub type ORER_R = crate::BitReader<ORER_A>;
#[doc = "Overrun Error Flag\n\nValue on reset: 0"]
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
#[doc = "Field `ORER` writer - Overrun Error Flag"]
pub type ORER_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, ORER_A>;
impl<'a, REG, const O: u8> ORER_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No overrun error occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ORER_A::_0)
    }
    #[doc = "An overrun error has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ORER_A::_1)
    }
}
#[doc = "Field `RDRF` reader - Receive Data Full Flag\n\nThe field is **modified** in some way after a read operation."]
pub type RDRF_R = crate::BitReader<RDRF_A>;
#[doc = "Receive Data Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDRF_A {
    #[doc = "0: No received data is in RDR register"]
    _0 = 0,
    #[doc = "1: Received data is in RDR register"]
    _1 = 1,
}
impl From<RDRF_A> for bool {
    #[inline(always)]
    fn from(variant: RDRF_A) -> Self {
        variant as u8 != 0
    }
}
impl RDRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDRF_A {
        match self.bits {
            false => RDRF_A::_0,
            true => RDRF_A::_1,
        }
    }
    #[doc = "No received data is in RDR register"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDRF_A::_0
    }
    #[doc = "Received data is in RDR register"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDRF_A::_1
    }
}
#[doc = "Field `RDRF` writer - Receive Data Full Flag"]
pub type RDRF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, RDRF_A>;
impl<'a, REG, const O: u8> RDRF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No received data is in RDR register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RDRF_A::_0)
    }
    #[doc = "Received data is in RDR register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RDRF_A::_1)
    }
}
#[doc = "Field `TDRE` reader - Transmit Data Empty Flag\n\nThe field is **modified** in some way after a read operation."]
pub type TDRE_R = crate::BitReader<TDRE_A>;
#[doc = "Transmit Data Empty Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDRE_A {
    #[doc = "0: Transmit data is in TDR register"]
    _0 = 0,
    #[doc = "1: No transmit data is in TDR register"]
    _1 = 1,
}
impl From<TDRE_A> for bool {
    #[inline(always)]
    fn from(variant: TDRE_A) -> Self {
        variant as u8 != 0
    }
}
impl TDRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TDRE_A {
        match self.bits {
            false => TDRE_A::_0,
            true => TDRE_A::_1,
        }
    }
    #[doc = "Transmit data is in TDR register"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDRE_A::_0
    }
    #[doc = "No transmit data is in TDR register"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDRE_A::_1
    }
}
#[doc = "Field `TDRE` writer - Transmit Data Empty Flag"]
pub type TDRE_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, TDRE_A>;
impl<'a, REG, const O: u8> TDRE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit data is in TDR register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TDRE_A::_0)
    }
    #[doc = "No transmit data is in TDR register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TDRE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Multi-Processor Bit Transfer This bit should be 0 in smart card interface mode."]
    #[inline(always)]
    pub fn mpbt(&self) -> MPBT_R {
        MPBT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Multi-Processor This bit should be 0 in smart card interface mode."]
    #[inline(always)]
    pub fn mpb(&self) -> MPB_R {
        MPB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit End Flag"]
    #[inline(always)]
    pub fn tend(&self) -> TEND_R {
        TEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity Error Flag"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Error Signal Status Flag"]
    #[inline(always)]
    pub fn ers(&self) -> ERS_R {
        ERS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Error Flag"]
    #[inline(always)]
    pub fn orer(&self) -> ORER_R {
        ORER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Data Full Flag"]
    #[inline(always)]
    pub fn rdrf(&self) -> RDRF_R {
        RDRF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Data Empty Flag"]
    #[inline(always)]
    pub fn tdre(&self) -> TDRE_R {
        TDRE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Multi-Processor Bit Transfer This bit should be 0 in smart card interface mode."]
    #[inline(always)]
    #[must_use]
    pub fn mpbt(&mut self) -> MPBT_W<SSR_SMCI_SPEC, 0> {
        MPBT_W::new(self)
    }
    #[doc = "Bit 3 - Parity Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<SSR_SMCI_SPEC, 3> {
        PER_W::new(self)
    }
    #[doc = "Bit 4 - Error Signal Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ers(&mut self) -> ERS_W<SSR_SMCI_SPEC, 4> {
        ERS_W::new(self)
    }
    #[doc = "Bit 5 - Overrun Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn orer(&mut self) -> ORER_W<SSR_SMCI_SPEC, 5> {
        ORER_W::new(self)
    }
    #[doc = "Bit 6 - Receive Data Full Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rdrf(&mut self) -> RDRF_W<SSR_SMCI_SPEC, 6> {
        RDRF_W::new(self)
    }
    #[doc = "Bit 7 - Transmit Data Empty Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tdre(&mut self) -> TDRE_W<SSR_SMCI_SPEC, 7> {
        TDRE_W::new(self)
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
#[doc = "Serial Status Register(SCMR.SMIF = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssr_smci::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssr_smci::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSR_SMCI_SPEC;
impl crate::RegisterSpec for SSR_SMCI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ssr_smci::R`](R) reader structure"]
impl crate::Readable for SSR_SMCI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssr_smci::W`](W) writer structure"]
impl crate::Writable for SSR_SMCI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xf8;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSR_SMCI to value 0x84"]
impl crate::Resettable for SSR_SMCI_SPEC {
    const RESET_VALUE: Self::Ux = 0x84;
}
