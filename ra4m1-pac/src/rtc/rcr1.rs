#[doc = "Register `RCR1` reader"]
pub type R = crate::R<RCR1_SPEC>;
#[doc = "Register `RCR1` writer"]
pub type W = crate::W<RCR1_SPEC>;
#[doc = "Field `AIE` reader - Alarm Interrupt Enable"]
pub type AIE_R = crate::BitReader<AIE_A>;
#[doc = "Alarm Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AIE_A {
    #[doc = "0: An alarm interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: An alarm interrupt request is enabled."]
    _1 = 1,
}
impl From<AIE_A> for bool {
    #[inline(always)]
    fn from(variant: AIE_A) -> Self {
        variant as u8 != 0
    }
}
impl AIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AIE_A {
        match self.bits {
            false => AIE_A::_0,
            true => AIE_A::_1,
        }
    }
    #[doc = "An alarm interrupt request is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AIE_A::_0
    }
    #[doc = "An alarm interrupt request is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AIE_A::_1
    }
}
#[doc = "Field `AIE` writer - Alarm Interrupt Enable"]
pub type AIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AIE_A>;
impl<'a, REG, const O: u8> AIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An alarm interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AIE_A::_0)
    }
    #[doc = "An alarm interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AIE_A::_1)
    }
}
#[doc = "Field `CIE` reader - Carry Interrupt Enable"]
pub type CIE_R = crate::BitReader<CIE_A>;
#[doc = "Carry Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIE_A {
    #[doc = "0: A carry interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: A carry interrupt request is enabled."]
    _1 = 1,
}
impl From<CIE_A> for bool {
    #[inline(always)]
    fn from(variant: CIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CIE_A {
        match self.bits {
            false => CIE_A::_0,
            true => CIE_A::_1,
        }
    }
    #[doc = "A carry interrupt request is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CIE_A::_0
    }
    #[doc = "A carry interrupt request is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CIE_A::_1
    }
}
#[doc = "Field `CIE` writer - Carry Interrupt Enable"]
pub type CIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CIE_A>;
impl<'a, REG, const O: u8> CIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A carry interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CIE_A::_0)
    }
    #[doc = "A carry interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CIE_A::_1)
    }
}
#[doc = "Field `PIE` reader - Periodic Interrupt Enable"]
pub type PIE_R = crate::BitReader<PIE_A>;
#[doc = "Periodic Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIE_A {
    #[doc = "0: A periodic interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: A periodic interrupt request is enabled."]
    _1 = 1,
}
impl From<PIE_A> for bool {
    #[inline(always)]
    fn from(variant: PIE_A) -> Self {
        variant as u8 != 0
    }
}
impl PIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIE_A {
        match self.bits {
            false => PIE_A::_0,
            true => PIE_A::_1,
        }
    }
    #[doc = "A periodic interrupt request is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIE_A::_0
    }
    #[doc = "A periodic interrupt request is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIE_A::_1
    }
}
#[doc = "Field `PIE` writer - Periodic Interrupt Enable"]
pub type PIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PIE_A>;
impl<'a, REG, const O: u8> PIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A periodic interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIE_A::_0)
    }
    #[doc = "A periodic interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIE_A::_1)
    }
}
#[doc = "Field `RTCOS` reader - RTCOUT Output Select"]
pub type RTCOS_R = crate::BitReader<RTCOS_A>;
#[doc = "RTCOUT Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCOS_A {
    #[doc = "0: RTCOUT outputs 1 Hz."]
    _0 = 0,
    #[doc = "1: RTCOUT outputs 64 Hz."]
    _1 = 1,
}
impl From<RTCOS_A> for bool {
    #[inline(always)]
    fn from(variant: RTCOS_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTCOS_A {
        match self.bits {
            false => RTCOS_A::_0,
            true => RTCOS_A::_1,
        }
    }
    #[doc = "RTCOUT outputs 1 Hz."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTCOS_A::_0
    }
    #[doc = "RTCOUT outputs 64 Hz."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTCOS_A::_1
    }
}
#[doc = "Field `RTCOS` writer - RTCOUT Output Select"]
pub type RTCOS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RTCOS_A>;
impl<'a, REG, const O: u8> RTCOS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTCOUT outputs 1 Hz."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RTCOS_A::_0)
    }
    #[doc = "RTCOUT outputs 64 Hz."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RTCOS_A::_1)
    }
}
#[doc = "Field `PES` reader - Periodic Interrupt Select"]
pub type PES_R = crate::FieldReader<PES_A>;
#[doc = "Periodic Interrupt Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PES_A {
    #[doc = "6: A periodic interrupt is generated every 1/256 second((RCR4.RCKSEL = 0)./A periodic interrupt is generated every 1/128 second((RCR4.RCKSEL = 1)."]
    _0110 = 6,
    #[doc = "7: A periodic interrupt is generated every 1/128 second."]
    _0111 = 7,
    #[doc = "8: A periodic interrupt is generated every 1/64 second."]
    _1000 = 8,
    #[doc = "9: A periodic interrupt is generated every 1/32 second."]
    _1001 = 9,
    #[doc = "10: A periodic interrupt is generated every 1/16 second."]
    _1010 = 10,
    #[doc = "11: A periodic interrupt is generated every 1/8 second."]
    _1011 = 11,
    #[doc = "12: A periodic interrupt is generated every 1/4 second."]
    _1100 = 12,
    #[doc = "13: A periodic interrupt is generated every 1/2 second."]
    _1101 = 13,
    #[doc = "14: A periodic interrupt is generated every 1 second."]
    _1110 = 14,
    #[doc = "15: A periodic interrupt is generated every 2 seconds."]
    _1111 = 15,
}
impl From<PES_A> for u8 {
    #[inline(always)]
    fn from(variant: PES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PES_A {
    type Ux = u8;
}
impl PES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PES_A> {
        match self.bits {
            6 => Some(PES_A::_0110),
            7 => Some(PES_A::_0111),
            8 => Some(PES_A::_1000),
            9 => Some(PES_A::_1001),
            10 => Some(PES_A::_1010),
            11 => Some(PES_A::_1011),
            12 => Some(PES_A::_1100),
            13 => Some(PES_A::_1101),
            14 => Some(PES_A::_1110),
            15 => Some(PES_A::_1111),
            _ => None,
        }
    }
    #[doc = "A periodic interrupt is generated every 1/256 second((RCR4.RCKSEL = 0)./A periodic interrupt is generated every 1/128 second((RCR4.RCKSEL = 1)."]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == PES_A::_0110
    }
    #[doc = "A periodic interrupt is generated every 1/128 second."]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == PES_A::_0111
    }
    #[doc = "A periodic interrupt is generated every 1/64 second."]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == PES_A::_1000
    }
    #[doc = "A periodic interrupt is generated every 1/32 second."]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == PES_A::_1001
    }
    #[doc = "A periodic interrupt is generated every 1/16 second."]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == PES_A::_1010
    }
    #[doc = "A periodic interrupt is generated every 1/8 second."]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == PES_A::_1011
    }
    #[doc = "A periodic interrupt is generated every 1/4 second."]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == PES_A::_1100
    }
    #[doc = "A periodic interrupt is generated every 1/2 second."]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == PES_A::_1101
    }
    #[doc = "A periodic interrupt is generated every 1 second."]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == PES_A::_1110
    }
    #[doc = "A periodic interrupt is generated every 2 seconds."]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == PES_A::_1111
    }
}
#[doc = "Field `PES` writer - Periodic Interrupt Select"]
pub type PES_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, PES_A>;
impl<'a, REG, const O: u8> PES_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A periodic interrupt is generated every 1/256 second((RCR4.RCKSEL = 0)./A periodic interrupt is generated every 1/128 second((RCR4.RCKSEL = 1)."]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(PES_A::_0110)
    }
    #[doc = "A periodic interrupt is generated every 1/128 second."]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(PES_A::_0111)
    }
    #[doc = "A periodic interrupt is generated every 1/64 second."]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(PES_A::_1000)
    }
    #[doc = "A periodic interrupt is generated every 1/32 second."]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(PES_A::_1001)
    }
    #[doc = "A periodic interrupt is generated every 1/16 second."]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut crate::W<REG> {
        self.variant(PES_A::_1010)
    }
    #[doc = "A periodic interrupt is generated every 1/8 second."]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut crate::W<REG> {
        self.variant(PES_A::_1011)
    }
    #[doc = "A periodic interrupt is generated every 1/4 second."]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut crate::W<REG> {
        self.variant(PES_A::_1100)
    }
    #[doc = "A periodic interrupt is generated every 1/2 second."]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut crate::W<REG> {
        self.variant(PES_A::_1101)
    }
    #[doc = "A periodic interrupt is generated every 1 second."]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut crate::W<REG> {
        self.variant(PES_A::_1110)
    }
    #[doc = "A periodic interrupt is generated every 2 seconds."]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut crate::W<REG> {
        self.variant(PES_A::_1111)
    }
}
impl R {
    #[doc = "Bit 0 - Alarm Interrupt Enable"]
    #[inline(always)]
    pub fn aie(&self) -> AIE_R {
        AIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Carry Interrupt Enable"]
    #[inline(always)]
    pub fn cie(&self) -> CIE_R {
        CIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Periodic Interrupt Enable"]
    #[inline(always)]
    pub fn pie(&self) -> PIE_R {
        PIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTCOUT Output Select"]
    #[inline(always)]
    pub fn rtcos(&self) -> RTCOS_R {
        RTCOS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Periodic Interrupt Select"]
    #[inline(always)]
    pub fn pes(&self) -> PES_R {
        PES_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bit 0 - Alarm Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aie(&mut self) -> AIE_W<RCR1_SPEC, 0> {
        AIE_W::new(self)
    }
    #[doc = "Bit 1 - Carry Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cie(&mut self) -> CIE_W<RCR1_SPEC, 1> {
        CIE_W::new(self)
    }
    #[doc = "Bit 2 - Periodic Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pie(&mut self) -> PIE_W<RCR1_SPEC, 2> {
        PIE_W::new(self)
    }
    #[doc = "Bit 3 - RTCOUT Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn rtcos(&mut self) -> RTCOS_W<RCR1_SPEC, 3> {
        RTCOS_W::new(self)
    }
    #[doc = "Bits 4:7 - Periodic Interrupt Select"]
    #[inline(always)]
    #[must_use]
    pub fn pes(&mut self) -> PES_W<RCR1_SPEC, 4> {
        PES_W::new(self)
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
#[doc = "RTC Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCR1_SPEC;
impl crate::RegisterSpec for RCR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rcr1::R`](R) reader structure"]
impl crate::Readable for RCR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rcr1::W`](W) writer structure"]
impl crate::Writable for RCR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCR1 to value 0"]
impl crate::Resettable for RCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
