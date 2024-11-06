#[doc = "Register `MSTPCRA` reader"]
pub type R = crate::R<MSTPCRA_SPEC>;
#[doc = "Register `MSTPCRA` writer"]
pub type W = crate::W<MSTPCRA_SPEC>;
#[doc = "Field `MSTPA0` reader - RAM0 Module Stop"]
pub type MSTPA0_R = crate::BitReader<MSTPA0_A>;
#[doc = "RAM0 Module Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPA0_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPA0_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPA0_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSTPA0_A {
        match self.bits {
            false => MSTPA0_A::_0,
            true => MSTPA0_A::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPA0_A::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPA0_A::_1
    }
}
#[doc = "Field `MSTPA0` writer - RAM0 Module Stop"]
pub type MSTPA0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MSTPA0_A>;
impl<'a, REG, const O: u8> MSTPA0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPA0_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPA0_A::_1)
    }
}
#[doc = "Field `MSTPA6` reader - ECCRAM Module Stop"]
pub type MSTPA6_R = crate::BitReader<MSTPA6_A>;
#[doc = "ECCRAM Module Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPA6_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPA6_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPA6_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPA6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSTPA6_A {
        match self.bits {
            false => MSTPA6_A::_0,
            true => MSTPA6_A::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPA6_A::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPA6_A::_1
    }
}
#[doc = "Field `MSTPA6` writer - ECCRAM Module Stop"]
pub type MSTPA6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MSTPA6_A>;
impl<'a, REG, const O: u8> MSTPA6_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPA6_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPA6_A::_1)
    }
}
#[doc = "Field `MSTPA22` reader - DMA Controller/Data Transfer Controller Module Stop"]
pub type MSTPA22_R = crate::BitReader<MSTPA22_A>;
#[doc = "DMA Controller/Data Transfer Controller Module Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPA22_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPA22_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPA22_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPA22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSTPA22_A {
        match self.bits {
            false => MSTPA22_A::_0,
            true => MSTPA22_A::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPA22_A::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPA22_A::_1
    }
}
#[doc = "Field `MSTPA22` writer - DMA Controller/Data Transfer Controller Module Stop"]
pub type MSTPA22_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MSTPA22_A>;
impl<'a, REG, const O: u8> MSTPA22_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPA22_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPA22_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - RAM0 Module Stop"]
    #[inline(always)]
    pub fn mstpa0(&self) -> MSTPA0_R {
        MSTPA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 6 - ECCRAM Module Stop"]
    #[inline(always)]
    pub fn mstpa6(&self) -> MSTPA6_R {
        MSTPA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA Controller/Data Transfer Controller Module Stop"]
    #[inline(always)]
    pub fn mstpa22(&self) -> MSTPA22_R {
        MSTPA22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAM0 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpa0(&mut self) -> MSTPA0_W<MSTPCRA_SPEC, 0> {
        MSTPA0_W::new(self)
    }
    #[doc = "Bit 6 - ECCRAM Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpa6(&mut self) -> MSTPA6_W<MSTPCRA_SPEC, 6> {
        MSTPA6_W::new(self)
    }
    #[doc = "Bit 22 - DMA Controller/Data Transfer Controller Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpa22(&mut self) -> MSTPA22_W<MSTPCRA_SPEC, 22> {
        MSTPA22_W::new(self)
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
#[doc = "Module Stop Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mstpcra::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mstpcra::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSTPCRA_SPEC;
impl crate::RegisterSpec for MSTPCRA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mstpcra::R`](R) reader structure"]
impl crate::Readable for MSTPCRA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mstpcra::W`](W) writer structure"]
impl crate::Writable for MSTPCRA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSTPCRA to value 0xffbf_ffbe"]
impl crate::Resettable for MSTPCRA_SPEC {
    const RESET_VALUE: Self::Ux = 0xffbf_ffbe;
}
