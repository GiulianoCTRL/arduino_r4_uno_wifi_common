#[doc = "Register `SSIFSR` reader"]
pub type R = crate::R<SSIFSR_SPEC>;
#[doc = "Register `SSIFSR` writer"]
pub type W = crate::W<SSIFSR_SPEC>;
#[doc = "Field `RDF` reader - Receive Data Full Flag\n\nThe field is **modified** in some way after a read operation."]
pub type RDF_R = crate::BitReader<RDF_A>;
#[doc = "Receive Data Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDF_A {
    #[doc = "0: The size of received data in SSIFRDR is not more than the value of SSISCR.RDFS"]
    _0 = 0,
    #[doc = "1: The size of received data in SSIFRDR is not less than the value of SSISCR.RDFS plus one."]
    _1 = 1,
}
impl From<RDF_A> for bool {
    #[inline(always)]
    fn from(variant: RDF_A) -> Self {
        variant as u8 != 0
    }
}
impl RDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDF_A {
        match self.bits {
            false => RDF_A::_0,
            true => RDF_A::_1,
        }
    }
    #[doc = "The size of received data in SSIFRDR is not more than the value of SSISCR.RDFS"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDF_A::_0
    }
    #[doc = "The size of received data in SSIFRDR is not less than the value of SSISCR.RDFS plus one."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDF_A::_1
    }
}
#[doc = "Field `RDF` writer - Receive Data Full Flag"]
pub type RDF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, RDF_A>;
impl<'a, REG, const O: u8> RDF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The size of received data in SSIFRDR is not more than the value of SSISCR.RDFS"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RDF_A::_0)
    }
    #[doc = "The size of received data in SSIFRDR is not less than the value of SSISCR.RDFS plus one."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RDF_A::_1)
    }
}
#[doc = "Field `RDC` reader - Number of Receive FIFO Data Indication Flag Number of receive FIFO data indication flag."]
pub type RDC_R = crate::FieldReader;
#[doc = "Field `TDE` reader - Transmit Data Empty Flag\n\nThe field is **modified** in some way after a read operation."]
pub type TDE_R = crate::BitReader<TDE_A>;
#[doc = "Transmit Data Empty Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDE_A {
    #[doc = "0: The free space of SSIFTDR is not more than the value of SSISCR.TDES"]
    _0 = 0,
    #[doc = "1: The free space of SSIFTDR is not less than the value of SSISCR.TDES plus one."]
    _1 = 1,
}
impl From<TDE_A> for bool {
    #[inline(always)]
    fn from(variant: TDE_A) -> Self {
        variant as u8 != 0
    }
}
impl TDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TDE_A {
        match self.bits {
            false => TDE_A::_0,
            true => TDE_A::_1,
        }
    }
    #[doc = "The free space of SSIFTDR is not more than the value of SSISCR.TDES"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDE_A::_0
    }
    #[doc = "The free space of SSIFTDR is not less than the value of SSISCR.TDES plus one."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDE_A::_1
    }
}
#[doc = "Field `TDE` writer - Transmit Data Empty Flag"]
pub type TDE_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, TDE_A>;
impl<'a, REG, const O: u8> TDE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The free space of SSIFTDR is not more than the value of SSISCR.TDES"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TDE_A::_0)
    }
    #[doc = "The free space of SSIFTDR is not less than the value of SSISCR.TDES plus one."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TDE_A::_1)
    }
}
#[doc = "Field `TDC` reader - Number of Transmit FIFO Data Indication Flag Number of transmit FIFO data indication flag."]
pub type TDC_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Receive Data Full Flag"]
    #[inline(always)]
    pub fn rdf(&self) -> RDF_R {
        RDF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - Number of Receive FIFO Data Indication Flag Number of receive FIFO data indication flag."]
    #[inline(always)]
    pub fn rdc(&self) -> RDC_R {
        RDC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Transmit Data Empty Flag"]
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Number of Transmit FIFO Data Indication Flag Number of transmit FIFO data indication flag."]
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Data Full Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rdf(&mut self) -> RDF_W<SSIFSR_SPEC, 0> {
        RDF_W::new(self)
    }
    #[doc = "Bit 16 - Transmit Data Empty Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tde(&mut self) -> TDE_W<SSIFSR_SPEC, 16> {
        TDE_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssifsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssifsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSIFSR_SPEC;
impl crate::RegisterSpec for SSIFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssifsr::R`](R) reader structure"]
impl crate::Readable for SSIFSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssifsr::W`](W) writer structure"]
impl crate::Writable for SSIFSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0001_0001;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSIFSR to value 0x0001_0000"]
impl crate::Resettable for SSIFSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
