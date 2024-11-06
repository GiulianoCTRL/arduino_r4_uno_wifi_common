#[doc = "Register `AMPTRM` reader"]
pub type R = crate::R<AMPTRM_SPEC>;
#[doc = "Register `AMPTRM` writer"]
pub type W = crate::W<AMPTRM_SPEC>;
#[doc = "Field `AMPTRM00` reader - Operational amplifier function activation/stop trigger control"]
pub type AMPTRM00_R = crate::BitReader<AMPTRM00_A>;
#[doc = "Operational amplifier function activation/stop trigger control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPTRM00_A {
    #[doc = "0: Software trigger mode(AMPTRM01=0)/Setting prohibited(AMPTRM01=1)."]
    _0 = 0,
    #[doc = "1: An activation trigger mode(AMPTRM01=0)/An activation and A/D trigger mode(AMPTRM01=1)."]
    _1 = 1,
}
impl From<AMPTRM00_A> for bool {
    #[inline(always)]
    fn from(variant: AMPTRM00_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPTRM00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AMPTRM00_A {
        match self.bits {
            false => AMPTRM00_A::_0,
            true => AMPTRM00_A::_1,
        }
    }
    #[doc = "Software trigger mode(AMPTRM01=0)/Setting prohibited(AMPTRM01=1)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPTRM00_A::_0
    }
    #[doc = "An activation trigger mode(AMPTRM01=0)/An activation and A/D trigger mode(AMPTRM01=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPTRM00_A::_1
    }
}
#[doc = "Field `AMPTRM00` writer - Operational amplifier function activation/stop trigger control"]
pub type AMPTRM00_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AMPTRM00_A>;
impl<'a, REG, const O: u8> AMPTRM00_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software trigger mode(AMPTRM01=0)/Setting prohibited(AMPTRM01=1)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM00_A::_0)
    }
    #[doc = "An activation trigger mode(AMPTRM01=0)/An activation and A/D trigger mode(AMPTRM01=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM00_A::_1)
    }
}
#[doc = "Field `AMPTRM01` reader - Operational amplifier function activation/stop trigger control"]
pub type AMPTRM01_R = crate::BitReader<AMPTRM01_A>;
#[doc = "Operational amplifier function activation/stop trigger control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPTRM01_A {
    #[doc = "0: Software trigger mode(AMPTRM00=0)/An activation trigger mode(AMPTRM00=1)."]
    _0 = 0,
    #[doc = "1: Setting prohibited(AMPTRM00=0)/An activation and A/D trigger mode(AMPTRM00=1)."]
    _1 = 1,
}
impl From<AMPTRM01_A> for bool {
    #[inline(always)]
    fn from(variant: AMPTRM01_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPTRM01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AMPTRM01_A {
        match self.bits {
            false => AMPTRM01_A::_0,
            true => AMPTRM01_A::_1,
        }
    }
    #[doc = "Software trigger mode(AMPTRM00=0)/An activation trigger mode(AMPTRM00=1)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPTRM01_A::_0
    }
    #[doc = "Setting prohibited(AMPTRM00=0)/An activation and A/D trigger mode(AMPTRM00=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPTRM01_A::_1
    }
}
#[doc = "Field `AMPTRM01` writer - Operational amplifier function activation/stop trigger control"]
pub type AMPTRM01_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AMPTRM01_A>;
impl<'a, REG, const O: u8> AMPTRM01_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software trigger mode(AMPTRM00=0)/An activation trigger mode(AMPTRM00=1)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM01_A::_0)
    }
    #[doc = "Setting prohibited(AMPTRM00=0)/An activation and A/D trigger mode(AMPTRM00=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM01_A::_1)
    }
}
#[doc = "Field `AMPTRM10` reader - Operational amplifier function activation/stop trigger control"]
pub type AMPTRM10_R = crate::BitReader<AMPTRM10_A>;
#[doc = "Operational amplifier function activation/stop trigger control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPTRM10_A {
    #[doc = "0: Software trigger mode(AMPTRM11=0)/Setting prohibited(AMPTRM11=1)."]
    _0 = 0,
    #[doc = "1: An activation trigger mode(AMPTRM11=0)/An activation and A/D trigger mode(AMPTRM11=1)."]
    _1 = 1,
}
impl From<AMPTRM10_A> for bool {
    #[inline(always)]
    fn from(variant: AMPTRM10_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPTRM10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AMPTRM10_A {
        match self.bits {
            false => AMPTRM10_A::_0,
            true => AMPTRM10_A::_1,
        }
    }
    #[doc = "Software trigger mode(AMPTRM11=0)/Setting prohibited(AMPTRM11=1)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPTRM10_A::_0
    }
    #[doc = "An activation trigger mode(AMPTRM11=0)/An activation and A/D trigger mode(AMPTRM11=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPTRM10_A::_1
    }
}
#[doc = "Field `AMPTRM10` writer - Operational amplifier function activation/stop trigger control"]
pub type AMPTRM10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AMPTRM10_A>;
impl<'a, REG, const O: u8> AMPTRM10_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software trigger mode(AMPTRM11=0)/Setting prohibited(AMPTRM11=1)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM10_A::_0)
    }
    #[doc = "An activation trigger mode(AMPTRM11=0)/An activation and A/D trigger mode(AMPTRM11=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM10_A::_1)
    }
}
#[doc = "Field `AMPTRM11` reader - Operational amplifier function activation/stop trigger control"]
pub type AMPTRM11_R = crate::BitReader<AMPTRM11_A>;
#[doc = "Operational amplifier function activation/stop trigger control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPTRM11_A {
    #[doc = "0: Software trigger mode(AMPTRM10=0)/An activation trigger mode(AMPTRM10=1)."]
    _0 = 0,
    #[doc = "1: Setting prohibited(AMPTRM10=0)/An activation and A/D trigger mode(AMPTRM10=1)."]
    _1 = 1,
}
impl From<AMPTRM11_A> for bool {
    #[inline(always)]
    fn from(variant: AMPTRM11_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPTRM11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AMPTRM11_A {
        match self.bits {
            false => AMPTRM11_A::_0,
            true => AMPTRM11_A::_1,
        }
    }
    #[doc = "Software trigger mode(AMPTRM10=0)/An activation trigger mode(AMPTRM10=1)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPTRM11_A::_0
    }
    #[doc = "Setting prohibited(AMPTRM10=0)/An activation and A/D trigger mode(AMPTRM10=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPTRM11_A::_1
    }
}
#[doc = "Field `AMPTRM11` writer - Operational amplifier function activation/stop trigger control"]
pub type AMPTRM11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AMPTRM11_A>;
impl<'a, REG, const O: u8> AMPTRM11_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software trigger mode(AMPTRM10=0)/An activation trigger mode(AMPTRM10=1)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM11_A::_0)
    }
    #[doc = "Setting prohibited(AMPTRM10=0)/An activation and A/D trigger mode(AMPTRM10=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM11_A::_1)
    }
}
#[doc = "Field `AMPTRM20` reader - Operational amplifier function activation/stop trigger control"]
pub type AMPTRM20_R = crate::BitReader<AMPTRM20_A>;
#[doc = "Operational amplifier function activation/stop trigger control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPTRM20_A {
    #[doc = "0: Software trigger mode(AMPTRM21=0)/Setting prohibited(AMPTRM21=1)."]
    _0 = 0,
    #[doc = "1: An activation trigger mode(AMPTRM21=0)/An activation and A/D trigger mode(AMPTRM21=1)."]
    _1 = 1,
}
impl From<AMPTRM20_A> for bool {
    #[inline(always)]
    fn from(variant: AMPTRM20_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPTRM20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AMPTRM20_A {
        match self.bits {
            false => AMPTRM20_A::_0,
            true => AMPTRM20_A::_1,
        }
    }
    #[doc = "Software trigger mode(AMPTRM21=0)/Setting prohibited(AMPTRM21=1)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPTRM20_A::_0
    }
    #[doc = "An activation trigger mode(AMPTRM21=0)/An activation and A/D trigger mode(AMPTRM21=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPTRM20_A::_1
    }
}
#[doc = "Field `AMPTRM20` writer - Operational amplifier function activation/stop trigger control"]
pub type AMPTRM20_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AMPTRM20_A>;
impl<'a, REG, const O: u8> AMPTRM20_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software trigger mode(AMPTRM21=0)/Setting prohibited(AMPTRM21=1)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM20_A::_0)
    }
    #[doc = "An activation trigger mode(AMPTRM21=0)/An activation and A/D trigger mode(AMPTRM21=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM20_A::_1)
    }
}
#[doc = "Field `AMPTRM21` reader - Operational amplifier function activation/stop trigger control"]
pub type AMPTRM21_R = crate::BitReader<AMPTRM21_A>;
#[doc = "Operational amplifier function activation/stop trigger control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPTRM21_A {
    #[doc = "0: Software trigger mode(AMPTRM20=0)/An activation trigger mode(AMPTRM20=1)."]
    _0 = 0,
    #[doc = "1: Setting prohibited(AMPTRM20=0)/An activation and A/D trigger mode(AMPTRM20=1)."]
    _1 = 1,
}
impl From<AMPTRM21_A> for bool {
    #[inline(always)]
    fn from(variant: AMPTRM21_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPTRM21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AMPTRM21_A {
        match self.bits {
            false => AMPTRM21_A::_0,
            true => AMPTRM21_A::_1,
        }
    }
    #[doc = "Software trigger mode(AMPTRM20=0)/An activation trigger mode(AMPTRM20=1)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPTRM21_A::_0
    }
    #[doc = "Setting prohibited(AMPTRM20=0)/An activation and A/D trigger mode(AMPTRM20=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPTRM21_A::_1
    }
}
#[doc = "Field `AMPTRM21` writer - Operational amplifier function activation/stop trigger control"]
pub type AMPTRM21_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AMPTRM21_A>;
impl<'a, REG, const O: u8> AMPTRM21_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software trigger mode(AMPTRM20=0)/An activation trigger mode(AMPTRM20=1)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM21_A::_0)
    }
    #[doc = "Setting prohibited(AMPTRM20=0)/An activation and A/D trigger mode(AMPTRM20=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM21_A::_1)
    }
}
#[doc = "Field `AMPTRM30` reader - Operational amplifier function activation/stop trigger control"]
pub type AMPTRM30_R = crate::BitReader<AMPTRM30_A>;
#[doc = "Operational amplifier function activation/stop trigger control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPTRM30_A {
    #[doc = "0: Software trigger mode(AMPTRM31=0)/Setting prohibited(AMPTRM31=1)."]
    _0 = 0,
    #[doc = "1: An activation trigger mode(AMPTRM31=0)/An activation and A/D trigger mode(AMPTRM31=1)."]
    _1 = 1,
}
impl From<AMPTRM30_A> for bool {
    #[inline(always)]
    fn from(variant: AMPTRM30_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPTRM30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AMPTRM30_A {
        match self.bits {
            false => AMPTRM30_A::_0,
            true => AMPTRM30_A::_1,
        }
    }
    #[doc = "Software trigger mode(AMPTRM31=0)/Setting prohibited(AMPTRM31=1)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPTRM30_A::_0
    }
    #[doc = "An activation trigger mode(AMPTRM31=0)/An activation and A/D trigger mode(AMPTRM31=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPTRM30_A::_1
    }
}
#[doc = "Field `AMPTRM30` writer - Operational amplifier function activation/stop trigger control"]
pub type AMPTRM30_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AMPTRM30_A>;
impl<'a, REG, const O: u8> AMPTRM30_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software trigger mode(AMPTRM31=0)/Setting prohibited(AMPTRM31=1)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM30_A::_0)
    }
    #[doc = "An activation trigger mode(AMPTRM31=0)/An activation and A/D trigger mode(AMPTRM31=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM30_A::_1)
    }
}
#[doc = "Field `AMPTRM31` reader - Operational amplifier function activation/stop trigger control"]
pub type AMPTRM31_R = crate::BitReader<AMPTRM31_A>;
#[doc = "Operational amplifier function activation/stop trigger control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPTRM31_A {
    #[doc = "0: Software trigger mode(AMPTRM30=0)/An activation trigger mode(AMPTRM30=1)."]
    _0 = 0,
    #[doc = "1: Setting prohibited(AMPTRM30=0)/An activation and A/D trigger mode(AMPTRM30=1)."]
    _1 = 1,
}
impl From<AMPTRM31_A> for bool {
    #[inline(always)]
    fn from(variant: AMPTRM31_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPTRM31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AMPTRM31_A {
        match self.bits {
            false => AMPTRM31_A::_0,
            true => AMPTRM31_A::_1,
        }
    }
    #[doc = "Software trigger mode(AMPTRM30=0)/An activation trigger mode(AMPTRM30=1)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPTRM31_A::_0
    }
    #[doc = "Setting prohibited(AMPTRM30=0)/An activation and A/D trigger mode(AMPTRM30=1)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPTRM31_A::_1
    }
}
#[doc = "Field `AMPTRM31` writer - Operational amplifier function activation/stop trigger control"]
pub type AMPTRM31_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AMPTRM31_A>;
impl<'a, REG, const O: u8> AMPTRM31_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software trigger mode(AMPTRM30=0)/An activation trigger mode(AMPTRM30=1)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM31_A::_0)
    }
    #[doc = "Setting prohibited(AMPTRM30=0)/An activation and A/D trigger mode(AMPTRM30=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM31_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm00(&self) -> AMPTRM00_R {
        AMPTRM00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm01(&self) -> AMPTRM01_R {
        AMPTRM01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm10(&self) -> AMPTRM10_R {
        AMPTRM10_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm11(&self) -> AMPTRM11_R {
        AMPTRM11_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm20(&self) -> AMPTRM20_R {
        AMPTRM20_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm21(&self) -> AMPTRM21_R {
        AMPTRM21_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm30(&self) -> AMPTRM30_R {
        AMPTRM30_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm31(&self) -> AMPTRM31_R {
        AMPTRM31_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    #[must_use]
    pub fn amptrm00(&mut self) -> AMPTRM00_W<AMPTRM_SPEC, 0> {
        AMPTRM00_W::new(self)
    }
    #[doc = "Bit 1 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    #[must_use]
    pub fn amptrm01(&mut self) -> AMPTRM01_W<AMPTRM_SPEC, 1> {
        AMPTRM01_W::new(self)
    }
    #[doc = "Bit 2 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    #[must_use]
    pub fn amptrm10(&mut self) -> AMPTRM10_W<AMPTRM_SPEC, 2> {
        AMPTRM10_W::new(self)
    }
    #[doc = "Bit 3 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    #[must_use]
    pub fn amptrm11(&mut self) -> AMPTRM11_W<AMPTRM_SPEC, 3> {
        AMPTRM11_W::new(self)
    }
    #[doc = "Bit 4 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    #[must_use]
    pub fn amptrm20(&mut self) -> AMPTRM20_W<AMPTRM_SPEC, 4> {
        AMPTRM20_W::new(self)
    }
    #[doc = "Bit 5 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    #[must_use]
    pub fn amptrm21(&mut self) -> AMPTRM21_W<AMPTRM_SPEC, 5> {
        AMPTRM21_W::new(self)
    }
    #[doc = "Bit 6 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    #[must_use]
    pub fn amptrm30(&mut self) -> AMPTRM30_W<AMPTRM_SPEC, 6> {
        AMPTRM30_W::new(self)
    }
    #[doc = "Bit 7 - Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    #[must_use]
    pub fn amptrm31(&mut self) -> AMPTRM31_W<AMPTRM_SPEC, 7> {
        AMPTRM31_W::new(self)
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
#[doc = "Operational amplifier trigger mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`amptrm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`amptrm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AMPTRM_SPEC;
impl crate::RegisterSpec for AMPTRM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`amptrm::R`](R) reader structure"]
impl crate::Readable for AMPTRM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`amptrm::W`](W) writer structure"]
impl crate::Writable for AMPTRM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AMPTRM to value 0"]
impl crate::Resettable for AMPTRM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
