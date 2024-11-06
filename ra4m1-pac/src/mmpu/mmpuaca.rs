#[doc = "Register `MMPUACA%s` reader"]
pub type R = crate::R<MMPUACA_SPEC>;
#[doc = "Register `MMPUACA%s` writer"]
pub type W = crate::W<MMPUACA_SPEC>;
#[doc = "Field `ENABLE` reader - Region enable"]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
#[doc = "Region enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_A {
    #[doc = "0: Group m Region n unit is disabled"]
    _0 = 0,
    #[doc = "1: Group m Region n unit is enabled"]
    _1 = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::_0,
            true => ENABLE_A::_1,
        }
    }
    #[doc = "Group m Region n unit is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENABLE_A::_0
    }
    #[doc = "Group m Region n unit is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENABLE_A::_1
    }
}
#[doc = "Field `ENABLE` writer - Region enable"]
pub type ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ENABLE_A>;
impl<'a, REG, const O: u8> ENABLE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Group m Region n unit is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE_A::_0)
    }
    #[doc = "Group m Region n unit is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE_A::_1)
    }
}
#[doc = "Field `RP` reader - Read protection"]
pub type RP_R = crate::BitReader<RP_A>;
#[doc = "Read protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RP_A {
    #[doc = "0: Read permission"]
    _0 = 0,
    #[doc = "1: Read protection"]
    _1 = 1,
}
impl From<RP_A> for bool {
    #[inline(always)]
    fn from(variant: RP_A) -> Self {
        variant as u8 != 0
    }
}
impl RP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RP_A {
        match self.bits {
            false => RP_A::_0,
            true => RP_A::_1,
        }
    }
    #[doc = "Read permission"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RP_A::_0
    }
    #[doc = "Read protection"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RP_A::_1
    }
}
#[doc = "Field `RP` writer - Read protection"]
pub type RP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RP_A>;
impl<'a, REG, const O: u8> RP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read permission"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RP_A::_0)
    }
    #[doc = "Read protection"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RP_A::_1)
    }
}
#[doc = "Field `WP` reader - Write protection"]
pub type WP_R = crate::BitReader<WP_A>;
#[doc = "Write protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP_A {
    #[doc = "0: Write permission"]
    _0 = 0,
    #[doc = "1: Write protection"]
    _1 = 1,
}
impl From<WP_A> for bool {
    #[inline(always)]
    fn from(variant: WP_A) -> Self {
        variant as u8 != 0
    }
}
impl WP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WP_A {
        match self.bits {
            false => WP_A::_0,
            true => WP_A::_1,
        }
    }
    #[doc = "Write permission"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WP_A::_0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WP_A::_1
    }
}
#[doc = "Field `WP` writer - Write protection"]
pub type WP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WP_A>;
impl<'a, REG, const O: u8> WP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write permission"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WP_A::_0)
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WP_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Region enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read protection"]
    #[inline(always)]
    pub fn rp(&self) -> RP_R {
        RP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write protection"]
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Region enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<MMPUACA_SPEC, 0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Read protection"]
    #[inline(always)]
    #[must_use]
    pub fn rp(&mut self) -> RP_W<MMPUACA_SPEC, 1> {
        RP_W::new(self)
    }
    #[doc = "Bit 2 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn wp(&mut self) -> WP_W<MMPUACA_SPEC, 2> {
        WP_W::new(self)
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
#[doc = "Group A Region %s Access Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmpuaca::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmpuaca::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMPUACA_SPEC;
impl crate::RegisterSpec for MMPUACA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mmpuaca::R`](R) reader structure"]
impl crate::Readable for MMPUACA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmpuaca::W`](W) writer structure"]
impl crate::Writable for MMPUACA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMPUACA%s to value 0"]
impl crate::Resettable for MMPUACA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
