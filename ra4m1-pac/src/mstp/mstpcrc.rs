#[doc = "Register `MSTPCRC` reader"]
pub type R = crate::R<MSTPCRC_SPEC>;
#[doc = "Register `MSTPCRC` writer"]
pub type W = crate::W<MSTPCRC_SPEC>;
#[doc = "Field `MSTPC0` reader - Clock Frequency Accuracy Measurement Circuit Module Stop"]
pub type MSTPC0_R = crate::BitReader<MSTPC0_A>;
#[doc = "Clock Frequency Accuracy Measurement Circuit Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC0_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPC0_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC0_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC0_A {
        match self.bits {
            false => MSTPC0_A::_0,
            true => MSTPC0_A::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC0_A::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC0_A::_1
    }
}
#[doc = "Field `MSTPC0` writer - Clock Frequency Accuracy Measurement Circuit Module Stop"]
pub type MSTPC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MSTPC0_A>;
impl<'a, REG, const O: u8> MSTPC0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC0_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC0_A::_1)
    }
}
#[doc = "Field `MSTPC1` reader - Cyclic Redundancy Check Calculator Module Stop"]
pub type MSTPC1_R = crate::BitReader<MSTPC1_A>;
#[doc = "Cyclic Redundancy Check Calculator Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC1_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPC1_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC1_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC1_A {
        match self.bits {
            false => MSTPC1_A::_0,
            true => MSTPC1_A::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC1_A::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC1_A::_1
    }
}
#[doc = "Field `MSTPC1` writer - Cyclic Redundancy Check Calculator Module Stop"]
pub type MSTPC1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MSTPC1_A>;
impl<'a, REG, const O: u8> MSTPC1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC1_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC1_A::_1)
    }
}
#[doc = "Field `MSTPC3` reader - Capacitive Touch Sensing Unit Module Stop"]
pub type MSTPC3_R = crate::BitReader<MSTPC3_A>;
#[doc = "Capacitive Touch Sensing Unit Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC3_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPC3_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC3_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPC3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC3_A {
        match self.bits {
            false => MSTPC3_A::_0,
            true => MSTPC3_A::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC3_A::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC3_A::_1
    }
}
#[doc = "Field `MSTPC3` writer - Capacitive Touch Sensing Unit Module Stop"]
pub type MSTPC3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MSTPC3_A>;
impl<'a, REG, const O: u8> MSTPC3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC3_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC3_A::_1)
    }
}
#[doc = "Field `MSTPC4` reader - Segment LCD Controller Module Stop"]
pub type MSTPC4_R = crate::BitReader<MSTPC4_A>;
#[doc = "Segment LCD Controller Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC4_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPC4_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC4_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPC4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC4_A {
        match self.bits {
            false => MSTPC4_A::_0,
            true => MSTPC4_A::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC4_A::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC4_A::_1
    }
}
#[doc = "Field `MSTPC4` writer - Segment LCD Controller Module Stop"]
pub type MSTPC4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MSTPC4_A>;
impl<'a, REG, const O: u8> MSTPC4_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC4_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC4_A::_1)
    }
}
#[doc = "Field `MSTPC8` reader - Synchronous Serial Interface 0 Module Stop"]
pub type MSTPC8_R = crate::BitReader<MSTPC8_A>;
#[doc = "Synchronous Serial Interface 0 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC8_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPC8_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC8_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPC8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC8_A {
        match self.bits {
            false => MSTPC8_A::_0,
            true => MSTPC8_A::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC8_A::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC8_A::_1
    }
}
#[doc = "Field `MSTPC8` writer - Synchronous Serial Interface 0 Module Stop"]
pub type MSTPC8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MSTPC8_A>;
impl<'a, REG, const O: u8> MSTPC8_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC8_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC8_A::_1)
    }
}
#[doc = "Field `MSTPC13` reader - Data Operation Circuit Module Stop"]
pub type MSTPC13_R = crate::BitReader<MSTPC13_A>;
#[doc = "Data Operation Circuit Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC13_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPC13_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC13_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPC13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC13_A {
        match self.bits {
            false => MSTPC13_A::_0,
            true => MSTPC13_A::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC13_A::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC13_A::_1
    }
}
#[doc = "Field `MSTPC13` writer - Data Operation Circuit Module Stop"]
pub type MSTPC13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MSTPC13_A>;
impl<'a, REG, const O: u8> MSTPC13_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC13_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC13_A::_1)
    }
}
#[doc = "Field `MSTPC14` reader - Event Link Controller Module Stop"]
pub type MSTPC14_R = crate::BitReader<MSTPC14_A>;
#[doc = "Event Link Controller Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC14_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPC14_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC14_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPC14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC14_A {
        match self.bits {
            false => MSTPC14_A::_0,
            true => MSTPC14_A::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC14_A::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC14_A::_1
    }
}
#[doc = "Field `MSTPC14` writer - Event Link Controller Module Stop"]
pub type MSTPC14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MSTPC14_A>;
impl<'a, REG, const O: u8> MSTPC14_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC14_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC14_A::_1)
    }
}
#[doc = "Field `MSTPC31` reader - SCE5 Module Stop"]
pub type MSTPC31_R = crate::BitReader<MSTPC31_A>;
#[doc = "SCE5 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC31_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPC31_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC31_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPC31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC31_A {
        match self.bits {
            false => MSTPC31_A::_0,
            true => MSTPC31_A::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC31_A::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC31_A::_1
    }
}
#[doc = "Field `MSTPC31` writer - SCE5 Module Stop"]
pub type MSTPC31_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MSTPC31_A>;
impl<'a, REG, const O: u8> MSTPC31_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC31_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC31_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Clock Frequency Accuracy Measurement Circuit Module Stop"]
    #[inline(always)]
    pub fn mstpc0(&self) -> MSTPC0_R {
        MSTPC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Cyclic Redundancy Check Calculator Module Stop"]
    #[inline(always)]
    pub fn mstpc1(&self) -> MSTPC1_R {
        MSTPC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Capacitive Touch Sensing Unit Module Stop"]
    #[inline(always)]
    pub fn mstpc3(&self) -> MSTPC3_R {
        MSTPC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Segment LCD Controller Module Stop"]
    #[inline(always)]
    pub fn mstpc4(&self) -> MSTPC4_R {
        MSTPC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Synchronous Serial Interface 0 Module Stop"]
    #[inline(always)]
    pub fn mstpc8(&self) -> MSTPC8_R {
        MSTPC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 13 - Data Operation Circuit Module Stop"]
    #[inline(always)]
    pub fn mstpc13(&self) -> MSTPC13_R {
        MSTPC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Event Link Controller Module Stop"]
    #[inline(always)]
    pub fn mstpc14(&self) -> MSTPC14_R {
        MSTPC14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 31 - SCE5 Module Stop"]
    #[inline(always)]
    pub fn mstpc31(&self) -> MSTPC31_R {
        MSTPC31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Frequency Accuracy Measurement Circuit Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpc0(&mut self) -> MSTPC0_W<MSTPCRC_SPEC, 0> {
        MSTPC0_W::new(self)
    }
    #[doc = "Bit 1 - Cyclic Redundancy Check Calculator Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpc1(&mut self) -> MSTPC1_W<MSTPCRC_SPEC, 1> {
        MSTPC1_W::new(self)
    }
    #[doc = "Bit 3 - Capacitive Touch Sensing Unit Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpc3(&mut self) -> MSTPC3_W<MSTPCRC_SPEC, 3> {
        MSTPC3_W::new(self)
    }
    #[doc = "Bit 4 - Segment LCD Controller Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpc4(&mut self) -> MSTPC4_W<MSTPCRC_SPEC, 4> {
        MSTPC4_W::new(self)
    }
    #[doc = "Bit 8 - Synchronous Serial Interface 0 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpc8(&mut self) -> MSTPC8_W<MSTPCRC_SPEC, 8> {
        MSTPC8_W::new(self)
    }
    #[doc = "Bit 13 - Data Operation Circuit Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpc13(&mut self) -> MSTPC13_W<MSTPCRC_SPEC, 13> {
        MSTPC13_W::new(self)
    }
    #[doc = "Bit 14 - Event Link Controller Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpc14(&mut self) -> MSTPC14_W<MSTPCRC_SPEC, 14> {
        MSTPC14_W::new(self)
    }
    #[doc = "Bit 31 - SCE5 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpc31(&mut self) -> MSTPC31_W<MSTPCRC_SPEC, 31> {
        MSTPC31_W::new(self)
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
#[doc = "Module Stop Control Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mstpcrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mstpcrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSTPCRC_SPEC;
impl crate::RegisterSpec for MSTPCRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mstpcrc::R`](R) reader structure"]
impl crate::Readable for MSTPCRC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mstpcrc::W`](W) writer structure"]
impl crate::Writable for MSTPCRC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSTPCRC to value 0xffff_ffff"]
impl crate::Resettable for MSTPCRC_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
