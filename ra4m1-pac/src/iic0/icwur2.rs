#[doc = "Register `ICWUR2` reader"]
pub type R = crate::R<ICWUR2_SPEC>;
#[doc = "Register `ICWUR2` writer"]
pub type W = crate::W<ICWUR2_SPEC>;
#[doc = "Field `WUSEN` reader - Wake-up Function Synchronous Enable"]
pub type WUSEN_R = crate::BitReader<WUSEN_A>;
#[doc = "Wake-up Function Synchronous Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUSEN_A {
    #[doc = "0: IIC asynchronous circuit enable"]
    _0 = 0,
    #[doc = "1: IIC synchronous circuit enable"]
    _1 = 1,
}
impl From<WUSEN_A> for bool {
    #[inline(always)]
    fn from(variant: WUSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl WUSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUSEN_A {
        match self.bits {
            false => WUSEN_A::_0,
            true => WUSEN_A::_1,
        }
    }
    #[doc = "IIC asynchronous circuit enable"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUSEN_A::_0
    }
    #[doc = "IIC synchronous circuit enable"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUSEN_A::_1
    }
}
#[doc = "Field `WUSEN` writer - Wake-up Function Synchronous Enable"]
pub type WUSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WUSEN_A>;
impl<'a, REG, const O: u8> WUSEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IIC asynchronous circuit enable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WUSEN_A::_0)
    }
    #[doc = "IIC synchronous circuit enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WUSEN_A::_1)
    }
}
#[doc = "Field `WUASYF` reader - Wake-up Function Asynchronous Operation Status Flag"]
pub type WUASYF_R = crate::BitReader<WUASYF_A>;
#[doc = "Wake-up Function Asynchronous Operation Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUASYF_A {
    #[doc = "0: IIC synchronous circuit enable condition"]
    _0 = 0,
    #[doc = "1: IIC asynchronous circuit enable condition."]
    _1 = 1,
}
impl From<WUASYF_A> for bool {
    #[inline(always)]
    fn from(variant: WUASYF_A) -> Self {
        variant as u8 != 0
    }
}
impl WUASYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUASYF_A {
        match self.bits {
            false => WUASYF_A::_0,
            true => WUASYF_A::_1,
        }
    }
    #[doc = "IIC synchronous circuit enable condition"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUASYF_A::_0
    }
    #[doc = "IIC asynchronous circuit enable condition."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUASYF_A::_1
    }
}
#[doc = "Field `WUSYF` reader - Wake-up Function Synchronous Operation Status Flag"]
pub type WUSYF_R = crate::BitReader<WUSYF_A>;
#[doc = "Wake-up Function Synchronous Operation Status Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUSYF_A {
    #[doc = "0: IIC asynchronous circuit enable condition"]
    _0 = 0,
    #[doc = "1: IIC synchronous circuit enable condition."]
    _1 = 1,
}
impl From<WUSYF_A> for bool {
    #[inline(always)]
    fn from(variant: WUSYF_A) -> Self {
        variant as u8 != 0
    }
}
impl WUSYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUSYF_A {
        match self.bits {
            false => WUSYF_A::_0,
            true => WUSYF_A::_1,
        }
    }
    #[doc = "IIC asynchronous circuit enable condition"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUSYF_A::_0
    }
    #[doc = "IIC synchronous circuit enable condition."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUSYF_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Wake-up Function Synchronous Enable"]
    #[inline(always)]
    pub fn wusen(&self) -> WUSEN_R {
        WUSEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake-up Function Asynchronous Operation Status Flag"]
    #[inline(always)]
    pub fn wuasyf(&self) -> WUASYF_R {
        WUASYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake-up Function Synchronous Operation Status Flag"]
    #[inline(always)]
    pub fn wusyf(&self) -> WUSYF_R {
        WUSYF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-up Function Synchronous Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wusen(&mut self) -> WUSEN_W<ICWUR2_SPEC, 0> {
        WUSEN_W::new(self)
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
#[doc = "I2C Bus Wake up Unit Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icwur2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icwur2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICWUR2_SPEC;
impl crate::RegisterSpec for ICWUR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`icwur2::R`](R) reader structure"]
impl crate::Readable for ICWUR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icwur2::W`](W) writer structure"]
impl crate::Writable for ICWUR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICWUR2 to value 0xfd"]
impl crate::Resettable for ICWUR2_SPEC {
    const RESET_VALUE: Self::Ux = 0xfd;
}
