#[doc = "Register `INTENB0` reader"]
pub type R = crate::R<INTENB0_SPEC>;
#[doc = "Register `INTENB0` writer"]
pub type W = crate::W<INTENB0_SPEC>;
#[doc = "Field `BRDYE` reader - Buffer Ready Interrupt Enable"]
pub type BRDYE_R = crate::BitReader<BRDYE_A>;
#[doc = "Buffer Ready Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRDYE_A {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<BRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: BRDYE_A) -> Self {
        variant as u8 != 0
    }
}
impl BRDYE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BRDYE_A {
        match self.bits {
            false => BRDYE_A::_0,
            true => BRDYE_A::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BRDYE_A::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BRDYE_A::_1
    }
}
#[doc = "Field `BRDYE` writer - Buffer Ready Interrupt Enable"]
pub type BRDYE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BRDYE_A>;
impl<'a, REG, const O: u8> BRDYE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BRDYE_A::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BRDYE_A::_1)
    }
}
#[doc = "Field `NRDYE` reader - Buffer Not Ready Response Interrupt Enable"]
pub type NRDYE_R = crate::BitReader<NRDYE_A>;
#[doc = "Buffer Not Ready Response Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NRDYE_A {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<NRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: NRDYE_A) -> Self {
        variant as u8 != 0
    }
}
impl NRDYE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NRDYE_A {
        match self.bits {
            false => NRDYE_A::_0,
            true => NRDYE_A::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NRDYE_A::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NRDYE_A::_1
    }
}
#[doc = "Field `NRDYE` writer - Buffer Not Ready Response Interrupt Enable"]
pub type NRDYE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, NRDYE_A>;
impl<'a, REG, const O: u8> NRDYE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(NRDYE_A::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(NRDYE_A::_1)
    }
}
#[doc = "Field `BEMPE` reader - Buffer Empty Interrupt Enable"]
pub type BEMPE_R = crate::BitReader<BEMPE_A>;
#[doc = "Buffer Empty Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BEMPE_A {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<BEMPE_A> for bool {
    #[inline(always)]
    fn from(variant: BEMPE_A) -> Self {
        variant as u8 != 0
    }
}
impl BEMPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BEMPE_A {
        match self.bits {
            false => BEMPE_A::_0,
            true => BEMPE_A::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BEMPE_A::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BEMPE_A::_1
    }
}
#[doc = "Field `BEMPE` writer - Buffer Empty Interrupt Enable"]
pub type BEMPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BEMPE_A>;
impl<'a, REG, const O: u8> BEMPE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BEMPE_A::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BEMPE_A::_1)
    }
}
#[doc = "Field `CTRE` reader - Control Transfer Stage Transition Interrupt Enable"]
pub type CTRE_R = crate::BitReader<CTRE_A>;
#[doc = "Control Transfer Stage Transition Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTRE_A {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<CTRE_A> for bool {
    #[inline(always)]
    fn from(variant: CTRE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CTRE_A {
        match self.bits {
            false => CTRE_A::_0,
            true => CTRE_A::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTRE_A::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTRE_A::_1
    }
}
#[doc = "Field `CTRE` writer - Control Transfer Stage Transition Interrupt Enable"]
pub type CTRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CTRE_A>;
impl<'a, REG, const O: u8> CTRE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CTRE_A::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CTRE_A::_1)
    }
}
#[doc = "Field `DVSE` reader - Device State Transition Interrupt Enable"]
pub type DVSE_R = crate::BitReader<DVSE_A>;
#[doc = "Device State Transition Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DVSE_A {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<DVSE_A> for bool {
    #[inline(always)]
    fn from(variant: DVSE_A) -> Self {
        variant as u8 != 0
    }
}
impl DVSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DVSE_A {
        match self.bits {
            false => DVSE_A::_0,
            true => DVSE_A::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DVSE_A::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DVSE_A::_1
    }
}
#[doc = "Field `DVSE` writer - Device State Transition Interrupt Enable"]
pub type DVSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DVSE_A>;
impl<'a, REG, const O: u8> DVSE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DVSE_A::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DVSE_A::_1)
    }
}
#[doc = "Field `SOFE` reader - Frame Number Update Interrupt Enable"]
pub type SOFE_R = crate::BitReader<SOFE_A>;
#[doc = "Frame Number Update Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFE_A {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<SOFE_A> for bool {
    #[inline(always)]
    fn from(variant: SOFE_A) -> Self {
        variant as u8 != 0
    }
}
impl SOFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SOFE_A {
        match self.bits {
            false => SOFE_A::_0,
            true => SOFE_A::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOFE_A::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOFE_A::_1
    }
}
#[doc = "Field `SOFE` writer - Frame Number Update Interrupt Enable"]
pub type SOFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SOFE_A>;
impl<'a, REG, const O: u8> SOFE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SOFE_A::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SOFE_A::_1)
    }
}
#[doc = "Field `RSME` reader - Resume Interrupt Enable"]
pub type RSME_R = crate::BitReader<RSME_A>;
#[doc = "Resume Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSME_A {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<RSME_A> for bool {
    #[inline(always)]
    fn from(variant: RSME_A) -> Self {
        variant as u8 != 0
    }
}
impl RSME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RSME_A {
        match self.bits {
            false => RSME_A::_0,
            true => RSME_A::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSME_A::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSME_A::_1
    }
}
#[doc = "Field `RSME` writer - Resume Interrupt Enable"]
pub type RSME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RSME_A>;
impl<'a, REG, const O: u8> RSME_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RSME_A::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RSME_A::_1)
    }
}
#[doc = "Field `VBSE` reader - VBUS Interrupt Enable"]
pub type VBSE_R = crate::BitReader<VBSE_A>;
#[doc = "VBUS Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBSE_A {
    #[doc = "0: Interrupt output disabled"]
    _0 = 0,
    #[doc = "1: Interrupt output enabled"]
    _1 = 1,
}
impl From<VBSE_A> for bool {
    #[inline(always)]
    fn from(variant: VBSE_A) -> Self {
        variant as u8 != 0
    }
}
impl VBSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBSE_A {
        match self.bits {
            false => VBSE_A::_0,
            true => VBSE_A::_1,
        }
    }
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBSE_A::_0
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBSE_A::_1
    }
}
#[doc = "Field `VBSE` writer - VBUS Interrupt Enable"]
pub type VBSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VBSE_A>;
impl<'a, REG, const O: u8> VBSE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VBSE_A::_0)
    }
    #[doc = "Interrupt output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VBSE_A::_1)
    }
}
impl R {
    #[doc = "Bit 8 - Buffer Ready Interrupt Enable"]
    #[inline(always)]
    pub fn brdye(&self) -> BRDYE_R {
        BRDYE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Buffer Not Ready Response Interrupt Enable"]
    #[inline(always)]
    pub fn nrdye(&self) -> NRDYE_R {
        NRDYE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn bempe(&self) -> BEMPE_R {
        BEMPE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Control Transfer Stage Transition Interrupt Enable"]
    #[inline(always)]
    pub fn ctre(&self) -> CTRE_R {
        CTRE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Device State Transition Interrupt Enable"]
    #[inline(always)]
    pub fn dvse(&self) -> DVSE_R {
        DVSE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Frame Number Update Interrupt Enable"]
    #[inline(always)]
    pub fn sofe(&self) -> SOFE_R {
        SOFE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Resume Interrupt Enable"]
    #[inline(always)]
    pub fn rsme(&self) -> RSME_R {
        RSME_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VBUS Interrupt Enable"]
    #[inline(always)]
    pub fn vbse(&self) -> VBSE_R {
        VBSE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Buffer Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn brdye(&mut self) -> BRDYE_W<INTENB0_SPEC, 8> {
        BRDYE_W::new(self)
    }
    #[doc = "Bit 9 - Buffer Not Ready Response Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nrdye(&mut self) -> NRDYE_W<INTENB0_SPEC, 9> {
        NRDYE_W::new(self)
    }
    #[doc = "Bit 10 - Buffer Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bempe(&mut self) -> BEMPE_W<INTENB0_SPEC, 10> {
        BEMPE_W::new(self)
    }
    #[doc = "Bit 11 - Control Transfer Stage Transition Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctre(&mut self) -> CTRE_W<INTENB0_SPEC, 11> {
        CTRE_W::new(self)
    }
    #[doc = "Bit 12 - Device State Transition Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dvse(&mut self) -> DVSE_W<INTENB0_SPEC, 12> {
        DVSE_W::new(self)
    }
    #[doc = "Bit 13 - Frame Number Update Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sofe(&mut self) -> SOFE_W<INTENB0_SPEC, 13> {
        SOFE_W::new(self)
    }
    #[doc = "Bit 14 - Resume Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rsme(&mut self) -> RSME_W<INTENB0_SPEC, 14> {
        RSME_W::new(self)
    }
    #[doc = "Bit 15 - VBUS Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbse(&mut self) -> VBSE_W<INTENB0_SPEC, 15> {
        VBSE_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Enable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenb0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenb0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTENB0_SPEC;
impl crate::RegisterSpec for INTENB0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`intenb0::R`](R) reader structure"]
impl crate::Readable for INTENB0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intenb0::W`](W) writer structure"]
impl crate::Writable for INTENB0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENB0 to value 0"]
impl crate::Resettable for INTENB0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
