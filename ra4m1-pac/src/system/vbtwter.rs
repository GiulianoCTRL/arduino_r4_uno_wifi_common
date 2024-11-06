#[doc = "Register `VBTWTER` reader"]
pub type R = crate::R<VBTWTER_SPEC>;
#[doc = "Register `VBTWTER` writer"]
pub type W = crate::W<VBTWTER_SPEC>;
#[doc = "Field `VCH0E` reader - VBATWIO0 Pin Enable"]
pub type VCH0E_R = crate::BitReader<VCH0E_A>;
#[doc = "VBATWIO0 Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH0E_A {
    #[doc = "0: VBATT wakeup triggered by the VBATWIO0 pin is disabled"]
    _0 = 0,
    #[doc = "1: VBATT wakeup triggered by the VBATWIO0 pin is enabled."]
    _1 = 1,
}
impl From<VCH0E_A> for bool {
    #[inline(always)]
    fn from(variant: VCH0E_A) -> Self {
        variant as u8 != 0
    }
}
impl VCH0E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VCH0E_A {
        match self.bits {
            false => VCH0E_A::_0,
            true => VCH0E_A::_1,
        }
    }
    #[doc = "VBATT wakeup triggered by the VBATWIO0 pin is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH0E_A::_0
    }
    #[doc = "VBATT wakeup triggered by the VBATWIO0 pin is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH0E_A::_1
    }
}
#[doc = "Field `VCH0E` writer - VBATWIO0 Pin Enable"]
pub type VCH0E_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VCH0E_A>;
impl<'a, REG, const O: u8> VCH0E_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT wakeup triggered by the VBATWIO0 pin is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VCH0E_A::_0)
    }
    #[doc = "VBATT wakeup triggered by the VBATWIO0 pin is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VCH0E_A::_1)
    }
}
#[doc = "Field `VCH1E` reader - VBATWIO1 Pin Enable"]
pub type VCH1E_R = crate::BitReader<VCH1E_A>;
#[doc = "VBATWIO1 Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH1E_A {
    #[doc = "0: VBATT wakeup triggered by the VBATWIO1 pin is disabled"]
    _0 = 0,
    #[doc = "1: VBATT wakeup triggered by the VBATWIO1 pin is enabled."]
    _1 = 1,
}
impl From<VCH1E_A> for bool {
    #[inline(always)]
    fn from(variant: VCH1E_A) -> Self {
        variant as u8 != 0
    }
}
impl VCH1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VCH1E_A {
        match self.bits {
            false => VCH1E_A::_0,
            true => VCH1E_A::_1,
        }
    }
    #[doc = "VBATT wakeup triggered by the VBATWIO1 pin is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH1E_A::_0
    }
    #[doc = "VBATT wakeup triggered by the VBATWIO1 pin is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH1E_A::_1
    }
}
#[doc = "Field `VCH1E` writer - VBATWIO1 Pin Enable"]
pub type VCH1E_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VCH1E_A>;
impl<'a, REG, const O: u8> VCH1E_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT wakeup triggered by the VBATWIO1 pin is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VCH1E_A::_0)
    }
    #[doc = "VBATT wakeup triggered by the VBATWIO1 pin is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VCH1E_A::_1)
    }
}
#[doc = "Field `VCH2E` reader - VBATWIO2 Pin Enable"]
pub type VCH2E_R = crate::BitReader<VCH2E_A>;
#[doc = "VBATWIO2 Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH2E_A {
    #[doc = "0: VBATT wakeup triggered by the VBATWIO2 pin is disabled"]
    _0 = 0,
    #[doc = "1: VBATT wakeup triggered by the VBATWIO2 pin is enabled."]
    _1 = 1,
}
impl From<VCH2E_A> for bool {
    #[inline(always)]
    fn from(variant: VCH2E_A) -> Self {
        variant as u8 != 0
    }
}
impl VCH2E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VCH2E_A {
        match self.bits {
            false => VCH2E_A::_0,
            true => VCH2E_A::_1,
        }
    }
    #[doc = "VBATT wakeup triggered by the VBATWIO2 pin is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH2E_A::_0
    }
    #[doc = "VBATT wakeup triggered by the VBATWIO2 pin is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH2E_A::_1
    }
}
#[doc = "Field `VCH2E` writer - VBATWIO2 Pin Enable"]
pub type VCH2E_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VCH2E_A>;
impl<'a, REG, const O: u8> VCH2E_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT wakeup triggered by the VBATWIO2 pin is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VCH2E_A::_0)
    }
    #[doc = "VBATT wakeup triggered by the VBATWIO2 pin is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VCH2E_A::_1)
    }
}
#[doc = "Field `VRTCIE` reader - RTC Periodic Signal Enable"]
pub type VRTCIE_R = crate::BitReader<VRTCIE_A>;
#[doc = "RTC Periodic Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VRTCIE_A {
    #[doc = "0: VBATT wakeup triggered by RTC periodic signal is disabled"]
    _0 = 0,
    #[doc = "1: VBATT wakeup triggered by RTC periodic signal is enabled."]
    _1 = 1,
}
impl From<VRTCIE_A> for bool {
    #[inline(always)]
    fn from(variant: VRTCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl VRTCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VRTCIE_A {
        match self.bits {
            false => VRTCIE_A::_0,
            true => VRTCIE_A::_1,
        }
    }
    #[doc = "VBATT wakeup triggered by RTC periodic signal is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VRTCIE_A::_0
    }
    #[doc = "VBATT wakeup triggered by RTC periodic signal is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VRTCIE_A::_1
    }
}
#[doc = "Field `VRTCIE` writer - RTC Periodic Signal Enable"]
pub type VRTCIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VRTCIE_A>;
impl<'a, REG, const O: u8> VRTCIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT wakeup triggered by RTC periodic signal is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VRTCIE_A::_0)
    }
    #[doc = "VBATT wakeup triggered by RTC periodic signal is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VRTCIE_A::_1)
    }
}
#[doc = "Field `VRTCAE` reader - RTC Alarm Signal Enable"]
pub type VRTCAE_R = crate::BitReader<VRTCAE_A>;
#[doc = "RTC Alarm Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VRTCAE_A {
    #[doc = "0: VBATT wakeup triggered by RTC alarm signal is disabled"]
    _0 = 0,
    #[doc = "1: VBATT wakeup triggered by RTC alarm signal is enabled."]
    _1 = 1,
}
impl From<VRTCAE_A> for bool {
    #[inline(always)]
    fn from(variant: VRTCAE_A) -> Self {
        variant as u8 != 0
    }
}
impl VRTCAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VRTCAE_A {
        match self.bits {
            false => VRTCAE_A::_0,
            true => VRTCAE_A::_1,
        }
    }
    #[doc = "VBATT wakeup triggered by RTC alarm signal is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VRTCAE_A::_0
    }
    #[doc = "VBATT wakeup triggered by RTC alarm signal is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VRTCAE_A::_1
    }
}
#[doc = "Field `VRTCAE` writer - RTC Alarm Signal Enable"]
pub type VRTCAE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VRTCAE_A>;
impl<'a, REG, const O: u8> VRTCAE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT wakeup triggered by RTC alarm signal is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VRTCAE_A::_0)
    }
    #[doc = "VBATT wakeup triggered by RTC alarm signal is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VRTCAE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - VBATWIO0 Pin Enable"]
    #[inline(always)]
    pub fn vch0e(&self) -> VCH0E_R {
        VCH0E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBATWIO1 Pin Enable"]
    #[inline(always)]
    pub fn vch1e(&self) -> VCH1E_R {
        VCH1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VBATWIO2 Pin Enable"]
    #[inline(always)]
    pub fn vch2e(&self) -> VCH2E_R {
        VCH2E_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC Periodic Signal Enable"]
    #[inline(always)]
    pub fn vrtcie(&self) -> VRTCIE_R {
        VRTCIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC Alarm Signal Enable"]
    #[inline(always)]
    pub fn vrtcae(&self) -> VRTCAE_R {
        VRTCAE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBATWIO0 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vch0e(&mut self) -> VCH0E_W<VBTWTER_SPEC, 0> {
        VCH0E_W::new(self)
    }
    #[doc = "Bit 1 - VBATWIO1 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vch1e(&mut self) -> VCH1E_W<VBTWTER_SPEC, 1> {
        VCH1E_W::new(self)
    }
    #[doc = "Bit 2 - VBATWIO2 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vch2e(&mut self) -> VCH2E_W<VBTWTER_SPEC, 2> {
        VCH2E_W::new(self)
    }
    #[doc = "Bit 3 - RTC Periodic Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vrtcie(&mut self) -> VRTCIE_W<VBTWTER_SPEC, 3> {
        VRTCIE_W::new(self)
    }
    #[doc = "Bit 4 - RTC Alarm Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vrtcae(&mut self) -> VRTCAE_W<VBTWTER_SPEC, 4> {
        VRTCAE_W::new(self)
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
#[doc = "VBATT Wakeup Trigger source Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbtwter::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbtwter::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VBTWTER_SPEC;
impl crate::RegisterSpec for VBTWTER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vbtwter::R`](R) reader structure"]
impl crate::Readable for VBTWTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vbtwter::W`](W) writer structure"]
impl crate::Writable for VBTWTER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VBTWTER to value 0"]
impl crate::Resettable for VBTWTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
