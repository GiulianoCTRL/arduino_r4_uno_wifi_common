#[doc = "Register `SCR` reader"]
pub type R = crate::R<SCR_SPEC>;
#[doc = "Register `SCR` writer"]
pub type W = crate::W<SCR_SPEC>;
#[doc = "Field `CKE` reader - Clock Enable"]
pub type CKE_R = crate::FieldReader<CKE_A>;
#[doc = "Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKE_A {
    #[doc = "0: The SCKn pin is available for use as an I/O port in accord with the I/O port settings.(Asynchronous mode) / The SCKn pin functions as the clock output pin(Clock synchronous mode)"]
    _00 = 0,
    #[doc = "1: The clock with the same frequency as the bit rate is output from the SCKn pin.(Asynchronous mode) / The SCKn pin functions as the clock output pin(Clock synchronous mode)"]
    _01 = 1,
}
impl From<CKE_A> for u8 {
    #[inline(always)]
    fn from(variant: CKE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKE_A {
    type Ux = u8;
}
impl CKE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CKE_A> {
        match self.bits {
            0 => Some(CKE_A::_00),
            1 => Some(CKE_A::_01),
            _ => None,
        }
    }
    #[doc = "The SCKn pin is available for use as an I/O port in accord with the I/O port settings.(Asynchronous mode) / The SCKn pin functions as the clock output pin(Clock synchronous mode)"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CKE_A::_00
    }
    #[doc = "The clock with the same frequency as the bit rate is output from the SCKn pin.(Asynchronous mode) / The SCKn pin functions as the clock output pin(Clock synchronous mode)"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CKE_A::_01
    }
}
#[doc = "Field `CKE` writer - Clock Enable"]
pub type CKE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, CKE_A>;
impl<'a, REG, const O: u8> CKE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The SCKn pin is available for use as an I/O port in accord with the I/O port settings.(Asynchronous mode) / The SCKn pin functions as the clock output pin(Clock synchronous mode)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(CKE_A::_00)
    }
    #[doc = "The clock with the same frequency as the bit rate is output from the SCKn pin.(Asynchronous mode) / The SCKn pin functions as the clock output pin(Clock synchronous mode)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(CKE_A::_01)
    }
}
#[doc = "Field `TEIE` reader - Transmit End Interrupt Enable"]
pub type TEIE_R = crate::BitReader<TEIE_A>;
#[doc = "Transmit End Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIE_A {
    #[doc = "0: SCI_TEI interrupt request is disabled"]
    _0 = 0,
    #[doc = "1: SCI_TEI interrupt request is enabled"]
    _1 = 1,
}
impl From<TEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEIE_A {
        match self.bits {
            false => TEIE_A::_0,
            true => TEIE_A::_1,
        }
    }
    #[doc = "SCI_TEI interrupt request is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TEIE_A::_0
    }
    #[doc = "SCI_TEI interrupt request is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TEIE_A::_1
    }
}
#[doc = "Field `TEIE` writer - Transmit End Interrupt Enable"]
pub type TEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TEIE_A>;
impl<'a, REG, const O: u8> TEIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SCI_TEI interrupt request is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TEIE_A::_0)
    }
    #[doc = "SCI_TEI interrupt request is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TEIE_A::_1)
    }
}
#[doc = "Field `MPIE` reader - Multi-Processor Interrupt Enable (Valid in asynchronous mode when SMR.MP = 1)"]
pub type MPIE_R = crate::BitReader<MPIE_A>;
#[doc = "Multi-Processor Interrupt Enable (Valid in asynchronous mode when SMR.MP = 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPIE_A {
    #[doc = "0: Normal reception"]
    _0 = 0,
    #[doc = "1: When the data with the multi-processor bit set to 0 is received, the data is not read, and setting the status flags RDRF,ORER and FER in SSR to 1 is disabled. When the data with the multiprocessor bit set to 1 is received, the MPIE bit is automatically cleared to 0, and normal reception is resumed."]
    _1 = 1,
}
impl From<MPIE_A> for bool {
    #[inline(always)]
    fn from(variant: MPIE_A) -> Self {
        variant as u8 != 0
    }
}
impl MPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MPIE_A {
        match self.bits {
            false => MPIE_A::_0,
            true => MPIE_A::_1,
        }
    }
    #[doc = "Normal reception"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPIE_A::_0
    }
    #[doc = "When the data with the multi-processor bit set to 0 is received, the data is not read, and setting the status flags RDRF,ORER and FER in SSR to 1 is disabled. When the data with the multiprocessor bit set to 1 is received, the MPIE bit is automatically cleared to 0, and normal reception is resumed."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPIE_A::_1
    }
}
#[doc = "Field `MPIE` writer - Multi-Processor Interrupt Enable (Valid in asynchronous mode when SMR.MP = 1)"]
pub type MPIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MPIE_A>;
impl<'a, REG, const O: u8> MPIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MPIE_A::_0)
    }
    #[doc = "When the data with the multi-processor bit set to 0 is received, the data is not read, and setting the status flags RDRF,ORER and FER in SSR to 1 is disabled. When the data with the multiprocessor bit set to 1 is received, the MPIE bit is automatically cleared to 0, and normal reception is resumed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MPIE_A::_1)
    }
}
#[doc = "Field `RE` reader - Receive Enable"]
pub type RE_R = crate::BitReader<RE_A>;
#[doc = "Receive Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RE_A {
    #[doc = "0: Serial reception is disabled"]
    _0 = 0,
    #[doc = "1: Serial reception is enabled"]
    _1 = 1,
}
impl From<RE_A> for bool {
    #[inline(always)]
    fn from(variant: RE_A) -> Self {
        variant as u8 != 0
    }
}
impl RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RE_A {
        match self.bits {
            false => RE_A::_0,
            true => RE_A::_1,
        }
    }
    #[doc = "Serial reception is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RE_A::_0
    }
    #[doc = "Serial reception is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RE_A::_1
    }
}
#[doc = "Field `RE` writer - Receive Enable"]
pub type RE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RE_A>;
impl<'a, REG, const O: u8> RE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Serial reception is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RE_A::_0)
    }
    #[doc = "Serial reception is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RE_A::_1)
    }
}
#[doc = "Field `TE` reader - Transmit Enable"]
pub type TE_R = crate::BitReader<TE_A>;
#[doc = "Transmit Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TE_A {
    #[doc = "0: Serial transmission is disabled"]
    _0 = 0,
    #[doc = "1: Serial transmission is enabled"]
    _1 = 1,
}
impl From<TE_A> for bool {
    #[inline(always)]
    fn from(variant: TE_A) -> Self {
        variant as u8 != 0
    }
}
impl TE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TE_A {
        match self.bits {
            false => TE_A::_0,
            true => TE_A::_1,
        }
    }
    #[doc = "Serial transmission is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TE_A::_0
    }
    #[doc = "Serial transmission is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TE_A::_1
    }
}
#[doc = "Field `TE` writer - Transmit Enable"]
pub type TE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TE_A>;
impl<'a, REG, const O: u8> TE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Serial transmission is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TE_A::_0)
    }
    #[doc = "Serial transmission is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TE_A::_1)
    }
}
#[doc = "Field `RIE` reader - Receive Interrupt Enable"]
pub type RIE_R = crate::BitReader<RIE_A>;
#[doc = "Receive Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RIE_A {
    #[doc = "0: SCI_RXI and SCI_ERI interrupt requests are disabled"]
    _0 = 0,
    #[doc = "1: SCI_RXI and SCI_ERI interrupt requests are enabled"]
    _1 = 1,
}
impl From<RIE_A> for bool {
    #[inline(always)]
    fn from(variant: RIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RIE_A {
        match self.bits {
            false => RIE_A::_0,
            true => RIE_A::_1,
        }
    }
    #[doc = "SCI_RXI and SCI_ERI interrupt requests are disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RIE_A::_0
    }
    #[doc = "SCI_RXI and SCI_ERI interrupt requests are enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RIE_A::_1
    }
}
#[doc = "Field `RIE` writer - Receive Interrupt Enable"]
pub type RIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RIE_A>;
impl<'a, REG, const O: u8> RIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SCI_RXI and SCI_ERI interrupt requests are disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RIE_A::_0)
    }
    #[doc = "SCI_RXI and SCI_ERI interrupt requests are enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RIE_A::_1)
    }
}
#[doc = "Field `TIE` reader - Transmit Interrupt Enable"]
pub type TIE_R = crate::BitReader<TIE_A>;
#[doc = "Transmit Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIE_A {
    #[doc = "0: SCI_TXI interrupt request is disabled"]
    _0 = 0,
    #[doc = "1: SCI_TXI interrupt request is enabled"]
    _1 = 1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIE_A {
        match self.bits {
            false => TIE_A::_0,
            true => TIE_A::_1,
        }
    }
    #[doc = "SCI_TXI interrupt request is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIE_A::_0
    }
    #[doc = "SCI_TXI interrupt request is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIE_A::_1
    }
}
#[doc = "Field `TIE` writer - Transmit Interrupt Enable"]
pub type TIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TIE_A>;
impl<'a, REG, const O: u8> TIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SCI_TXI interrupt request is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TIE_A::_0)
    }
    #[doc = "SCI_TXI interrupt request is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TIE_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock Enable"]
    #[inline(always)]
    pub fn cke(&self) -> CKE_R {
        CKE_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Transmit End Interrupt Enable"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Multi-Processor Interrupt Enable (Valid in asynchronous mode when SMR.MP = 1)"]
    #[inline(always)]
    pub fn mpie(&self) -> MPIE_R {
        MPIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cke(&mut self) -> CKE_W<SCR_SPEC, 0> {
        CKE_W::new(self)
    }
    #[doc = "Bit 2 - Transmit End Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<SCR_SPEC, 2> {
        TEIE_W::new(self)
    }
    #[doc = "Bit 3 - Multi-Processor Interrupt Enable (Valid in asynchronous mode when SMR.MP = 1)"]
    #[inline(always)]
    #[must_use]
    pub fn mpie(&mut self) -> MPIE_W<SCR_SPEC, 3> {
        MPIE_W::new(self)
    }
    #[doc = "Bit 4 - Receive Enable"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<SCR_SPEC, 4> {
        RE_W::new(self)
    }
    #[doc = "Bit 5 - Transmit Enable"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<SCR_SPEC, 5> {
        TE_W::new(self)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<SCR_SPEC, 6> {
        RIE_W::new(self)
    }
    #[doc = "Bit 7 - Transmit Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<SCR_SPEC, 7> {
        TIE_W::new(self)
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
#[doc = "Serial Control Register (SCMR.SMIF = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`scr::R`](R) reader structure"]
impl crate::Readable for SCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
