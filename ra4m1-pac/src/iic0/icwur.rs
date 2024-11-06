#[doc = "Register `ICWUR` reader"]
pub type R = crate::R<ICWUR_SPEC>;
#[doc = "Register `ICWUR` writer"]
pub type W = crate::W<ICWUR_SPEC>;
#[doc = "Field `WUAFA` reader - Wakeup Analog Filter Additional Selection"]
pub type WUAFA_R = crate::BitReader<WUAFA_A>;
#[doc = "Wakeup Analog Filter Additional Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUAFA_A {
    #[doc = "0: Do not add the wakeup analog filter"]
    _0 = 0,
    #[doc = "1: Add the wakeup analog filter."]
    _1 = 1,
}
impl From<WUAFA_A> for bool {
    #[inline(always)]
    fn from(variant: WUAFA_A) -> Self {
        variant as u8 != 0
    }
}
impl WUAFA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUAFA_A {
        match self.bits {
            false => WUAFA_A::_0,
            true => WUAFA_A::_1,
        }
    }
    #[doc = "Do not add the wakeup analog filter"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUAFA_A::_0
    }
    #[doc = "Add the wakeup analog filter."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUAFA_A::_1
    }
}
#[doc = "Field `WUAFA` writer - Wakeup Analog Filter Additional Selection"]
pub type WUAFA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WUAFA_A>;
impl<'a, REG, const O: u8> WUAFA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not add the wakeup analog filter"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WUAFA_A::_0)
    }
    #[doc = "Add the wakeup analog filter."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WUAFA_A::_1)
    }
}
#[doc = "Field `WUACK` reader - ACK bit for Wakeup Mode"]
pub type WUACK_R = crate::BitReader<WUACK_A>;
#[doc = "ACK bit for Wakeup Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUACK_A {
    #[doc = "0: State of synchronous operation"]
    _0 = 0,
    #[doc = "1: State of asynchronous operation"]
    _1 = 1,
}
impl From<WUACK_A> for bool {
    #[inline(always)]
    fn from(variant: WUACK_A) -> Self {
        variant as u8 != 0
    }
}
impl WUACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUACK_A {
        match self.bits {
            false => WUACK_A::_0,
            true => WUACK_A::_1,
        }
    }
    #[doc = "State of synchronous operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUACK_A::_0
    }
    #[doc = "State of asynchronous operation"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUACK_A::_1
    }
}
#[doc = "Field `WUACK` writer - ACK bit for Wakeup Mode"]
pub type WUACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WUACK_A>;
impl<'a, REG, const O: u8> WUACK_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "State of synchronous operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WUACK_A::_0)
    }
    #[doc = "State of asynchronous operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WUACK_A::_1)
    }
}
#[doc = "Field `WUF` reader - Wakeup Event Occurrence Flag"]
pub type WUF_R = crate::BitReader<WUF_A>;
#[doc = "Wakeup Event Occurrence Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF_A {
    #[doc = "0: Slave address does not match during wakeup function"]
    _0 = 0,
    #[doc = "1: Slave address matches during wakeup function."]
    _1 = 1,
}
impl From<WUF_A> for bool {
    #[inline(always)]
    fn from(variant: WUF_A) -> Self {
        variant as u8 != 0
    }
}
impl WUF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUF_A {
        match self.bits {
            false => WUF_A::_0,
            true => WUF_A::_1,
        }
    }
    #[doc = "Slave address does not match during wakeup function"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF_A::_0
    }
    #[doc = "Slave address matches during wakeup function."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF_A::_1
    }
}
#[doc = "Field `WUF` writer - Wakeup Event Occurrence Flag"]
pub type WUF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WUF_A>;
impl<'a, REG, const O: u8> WUF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave address does not match during wakeup function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WUF_A::_0)
    }
    #[doc = "Slave address matches during wakeup function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WUF_A::_1)
    }
}
#[doc = "Field `WUIE` reader - Wakeup Interrupt Request Enable"]
pub type WUIE_R = crate::BitReader<WUIE_A>;
#[doc = "Wakeup Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUIE_A {
    #[doc = "0: Wakeup Interrupt Request (IIC0_WUI) disabled"]
    _0 = 0,
    #[doc = "1: Wakeup Interrupt Request (IIC0_WUI) enabled."]
    _1 = 1,
}
impl From<WUIE_A> for bool {
    #[inline(always)]
    fn from(variant: WUIE_A) -> Self {
        variant as u8 != 0
    }
}
impl WUIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUIE_A {
        match self.bits {
            false => WUIE_A::_0,
            true => WUIE_A::_1,
        }
    }
    #[doc = "Wakeup Interrupt Request (IIC0_WUI) disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUIE_A::_0
    }
    #[doc = "Wakeup Interrupt Request (IIC0_WUI) enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUIE_A::_1
    }
}
#[doc = "Field `WUIE` writer - Wakeup Interrupt Request Enable"]
pub type WUIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WUIE_A>;
impl<'a, REG, const O: u8> WUIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup Interrupt Request (IIC0_WUI) disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WUIE_A::_0)
    }
    #[doc = "Wakeup Interrupt Request (IIC0_WUI) enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WUIE_A::_1)
    }
}
#[doc = "Field `WUE` reader - Wakeup Function Enable"]
pub type WUE_R = crate::BitReader<WUE_A>;
#[doc = "Wakeup Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUE_A {
    #[doc = "0: Wakeup function disabled"]
    _0 = 0,
    #[doc = "1: Wakeup function enabled."]
    _1 = 1,
}
impl From<WUE_A> for bool {
    #[inline(always)]
    fn from(variant: WUE_A) -> Self {
        variant as u8 != 0
    }
}
impl WUE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUE_A {
        match self.bits {
            false => WUE_A::_0,
            true => WUE_A::_1,
        }
    }
    #[doc = "Wakeup function disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUE_A::_0
    }
    #[doc = "Wakeup function enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUE_A::_1
    }
}
#[doc = "Field `WUE` writer - Wakeup Function Enable"]
pub type WUE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WUE_A>;
impl<'a, REG, const O: u8> WUE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup function disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WUE_A::_0)
    }
    #[doc = "Wakeup function enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WUE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup Analog Filter Additional Selection"]
    #[inline(always)]
    pub fn wuafa(&self) -> WUAFA_R {
        WUAFA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - ACK bit for Wakeup Mode"]
    #[inline(always)]
    pub fn wuack(&self) -> WUACK_R {
        WUACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wakeup Event Occurrence Flag"]
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wakeup Interrupt Request Enable"]
    #[inline(always)]
    pub fn wuie(&self) -> WUIE_R {
        WUIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wakeup Function Enable"]
    #[inline(always)]
    pub fn wue(&self) -> WUE_R {
        WUE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Analog Filter Additional Selection"]
    #[inline(always)]
    #[must_use]
    pub fn wuafa(&mut self) -> WUAFA_W<ICWUR_SPEC, 0> {
        WUAFA_W::new(self)
    }
    #[doc = "Bit 4 - ACK bit for Wakeup Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wuack(&mut self) -> WUACK_W<ICWUR_SPEC, 4> {
        WUACK_W::new(self)
    }
    #[doc = "Bit 5 - Wakeup Event Occurrence Flag"]
    #[inline(always)]
    #[must_use]
    pub fn wuf(&mut self) -> WUF_W<ICWUR_SPEC, 5> {
        WUF_W::new(self)
    }
    #[doc = "Bit 6 - Wakeup Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wuie(&mut self) -> WUIE_W<ICWUR_SPEC, 6> {
        WUIE_W::new(self)
    }
    #[doc = "Bit 7 - Wakeup Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wue(&mut self) -> WUE_W<ICWUR_SPEC, 7> {
        WUE_W::new(self)
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
#[doc = "I2C Bus Wake Up Unit Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icwur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icwur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICWUR_SPEC;
impl crate::RegisterSpec for ICWUR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`icwur::R`](R) reader structure"]
impl crate::Readable for ICWUR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icwur::W`](W) writer structure"]
impl crate::Writable for ICWUR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICWUR to value 0x10"]
impl crate::Resettable for ICWUR_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
