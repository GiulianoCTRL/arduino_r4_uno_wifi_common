#[doc = "Register `LVD%sCR0` reader"]
pub type R = crate::R<LVDCR0_SPEC>;
#[doc = "Register `LVD%sCR0` writer"]
pub type W = crate::W<LVDCR0_SPEC>;
#[doc = "Field `RIE` reader - Voltage Monitor Interrupt/Reset Enable"]
pub type RIE_R = crate::BitReader<RIE_A>;
#[doc = "Voltage Monitor Interrupt/Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RIE_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
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
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RIE_A::_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RIE_A::_1
    }
}
#[doc = "Field `RIE` writer - Voltage Monitor Interrupt/Reset Enable"]
pub type RIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RIE_A>;
impl<'a, REG, const O: u8> RIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RIE_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RIE_A::_1)
    }
}
#[doc = "Field `CMPE` reader - Voltage Monitor Circuit Comparison Result Output Enable"]
pub type CMPE_R = crate::BitReader<CMPE_A>;
#[doc = "Voltage Monitor Circuit Comparison Result Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPE_A {
    #[doc = "0: Voltage Monitor circuit comparison result output disabled."]
    _0 = 0,
    #[doc = "1: Voltage Monitor circuit comparison result output enabled."]
    _1 = 1,
}
impl From<CMPE_A> for bool {
    #[inline(always)]
    fn from(variant: CMPE_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMPE_A {
        match self.bits {
            false => CMPE_A::_0,
            true => CMPE_A::_1,
        }
    }
    #[doc = "Voltage Monitor circuit comparison result output disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPE_A::_0
    }
    #[doc = "Voltage Monitor circuit comparison result output enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPE_A::_1
    }
}
#[doc = "Field `CMPE` writer - Voltage Monitor Circuit Comparison Result Output Enable"]
pub type CMPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMPE_A>;
impl<'a, REG, const O: u8> CMPE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Voltage Monitor circuit comparison result output disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPE_A::_0)
    }
    #[doc = "Voltage Monitor circuit comparison result output enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPE_A::_1)
    }
}
#[doc = "Field `RI` reader - Voltage Monitor Circuit Mode Select"]
pub type RI_R = crate::BitReader<RI_A>;
#[doc = "Voltage Monitor Circuit Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RI_A {
    #[doc = "0: Voltage Monitor interrupt during Vdet1 passage"]
    _0 = 0,
    #[doc = "1: Voltage Monitor reset enabled when the voltage falls to and below Vdet1"]
    _1 = 1,
}
impl From<RI_A> for bool {
    #[inline(always)]
    fn from(variant: RI_A) -> Self {
        variant as u8 != 0
    }
}
impl RI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RI_A {
        match self.bits {
            false => RI_A::_0,
            true => RI_A::_1,
        }
    }
    #[doc = "Voltage Monitor interrupt during Vdet1 passage"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RI_A::_0
    }
    #[doc = "Voltage Monitor reset enabled when the voltage falls to and below Vdet1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RI_A::_1
    }
}
#[doc = "Field `RI` writer - Voltage Monitor Circuit Mode Select"]
pub type RI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RI_A>;
impl<'a, REG, const O: u8> RI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Voltage Monitor interrupt during Vdet1 passage"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RI_A::_0)
    }
    #[doc = "Voltage Monitor reset enabled when the voltage falls to and below Vdet1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RI_A::_1)
    }
}
#[doc = "Field `RN` reader - Voltage Monitor Reset Negate Select"]
pub type RN_R = crate::BitReader<RN_A>;
#[doc = "Voltage Monitor Reset Negate Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RN_A {
    #[doc = "0: Negation follows a stabilization time (tLVD) after VCC > Vdet1 is detected."]
    _0 = 0,
    #[doc = "1: Negation follows a stabilization time (tLVD) after assertion of the LVD reset."]
    _1 = 1,
}
impl From<RN_A> for bool {
    #[inline(always)]
    fn from(variant: RN_A) -> Self {
        variant as u8 != 0
    }
}
impl RN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RN_A {
        match self.bits {
            false => RN_A::_0,
            true => RN_A::_1,
        }
    }
    #[doc = "Negation follows a stabilization time (tLVD) after VCC > Vdet1 is detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RN_A::_0
    }
    #[doc = "Negation follows a stabilization time (tLVD) after assertion of the LVD reset."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RN_A::_1
    }
}
#[doc = "Field `RN` writer - Voltage Monitor Reset Negate Select"]
pub type RN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RN_A>;
impl<'a, REG, const O: u8> RN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Negation follows a stabilization time (tLVD) after VCC > Vdet1 is detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RN_A::_0)
    }
    #[doc = "Negation follows a stabilization time (tLVD) after assertion of the LVD reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Voltage Monitor Interrupt/Reset Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Voltage Monitor Circuit Comparison Result Output Enable"]
    #[inline(always)]
    pub fn cmpe(&self) -> CMPE_R {
        CMPE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Voltage Monitor Circuit Mode Select"]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Voltage Monitor Reset Negate Select"]
    #[inline(always)]
    pub fn rn(&self) -> RN_R {
        RN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage Monitor Interrupt/Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<LVDCR0_SPEC, 0> {
        RIE_W::new(self)
    }
    #[doc = "Bit 2 - Voltage Monitor Circuit Comparison Result Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpe(&mut self) -> CMPE_W<LVDCR0_SPEC, 2> {
        CMPE_W::new(self)
    }
    #[doc = "Bit 6 - Voltage Monitor Circuit Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn ri(&mut self) -> RI_W<LVDCR0_SPEC, 6> {
        RI_W::new(self)
    }
    #[doc = "Bit 7 - Voltage Monitor Reset Negate Select"]
    #[inline(always)]
    #[must_use]
    pub fn rn(&mut self) -> RN_W<LVDCR0_SPEC, 7> {
        RN_W::new(self)
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
#[doc = "Voltage Monitor %s Circuit Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lvdcr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lvdcr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LVDCR0_SPEC;
impl crate::RegisterSpec for LVDCR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lvdcr0::R`](R) reader structure"]
impl crate::Readable for LVDCR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lvdcr0::W`](W) writer structure"]
impl crate::Writable for LVDCR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LVD%sCR0 to value 0x80"]
impl crate::Resettable for LVDCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
