#[doc = "Register `SMPUP%sBIU` reader"]
pub type R = crate::R<SMPUPBIU_SPEC>;
#[doc = "Register `SMPUP%sBIU` writer"]
pub type W = crate::W<SMPUPBIU_SPEC>;
#[doc = "Field `RPCPU` reader - CPU Read protection"]
pub type RPCPU_R = crate::BitReader<RPCPU_A>;
#[doc = "CPU Read protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPCPU_A {
    #[doc = "0: CPU read of memory protection disabled."]
    _0 = 0,
    #[doc = "1: CPU read of memory protection enabled."]
    _1 = 1,
}
impl From<RPCPU_A> for bool {
    #[inline(always)]
    fn from(variant: RPCPU_A) -> Self {
        variant as u8 != 0
    }
}
impl RPCPU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPCPU_A {
        match self.bits {
            false => RPCPU_A::_0,
            true => RPCPU_A::_1,
        }
    }
    #[doc = "CPU read of memory protection disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPCPU_A::_0
    }
    #[doc = "CPU read of memory protection enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPCPU_A::_1
    }
}
#[doc = "Field `RPCPU` writer - CPU Read protection"]
pub type RPCPU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RPCPU_A>;
impl<'a, REG, const O: u8> RPCPU_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPU read of memory protection disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RPCPU_A::_0)
    }
    #[doc = "CPU read of memory protection enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RPCPU_A::_1)
    }
}
#[doc = "Field `WPCPU` reader - CPU Write protection"]
pub type WPCPU_R = crate::BitReader<WPCPU_A>;
#[doc = "CPU Write protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPCPU_A {
    #[doc = "0: CPU write of memory protection disabled."]
    _0 = 0,
    #[doc = "1: CPU write of memory protection enabled."]
    _1 = 1,
}
impl From<WPCPU_A> for bool {
    #[inline(always)]
    fn from(variant: WPCPU_A) -> Self {
        variant as u8 != 0
    }
}
impl WPCPU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WPCPU_A {
        match self.bits {
            false => WPCPU_A::_0,
            true => WPCPU_A::_1,
        }
    }
    #[doc = "CPU write of memory protection disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WPCPU_A::_0
    }
    #[doc = "CPU write of memory protection enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WPCPU_A::_1
    }
}
#[doc = "Field `WPCPU` writer - CPU Write protection"]
pub type WPCPU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WPCPU_A>;
impl<'a, REG, const O: u8> WPCPU_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPU write of memory protection disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WPCPU_A::_0)
    }
    #[doc = "CPU write of memory protection enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WPCPU_A::_1)
    }
}
#[doc = "Field `RPGRPA` reader - Master Group A Read protection"]
pub type RPGRPA_R = crate::BitReader<RPGRPA_A>;
#[doc = "Master Group A Read protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPGRPA_A {
    #[doc = "0: Master group A read of memory protection disabled."]
    _0 = 0,
    #[doc = "1: Master group A read of memory protection enabled."]
    _1 = 1,
}
impl From<RPGRPA_A> for bool {
    #[inline(always)]
    fn from(variant: RPGRPA_A) -> Self {
        variant as u8 != 0
    }
}
impl RPGRPA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPGRPA_A {
        match self.bits {
            false => RPGRPA_A::_0,
            true => RPGRPA_A::_1,
        }
    }
    #[doc = "Master group A read of memory protection disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPGRPA_A::_0
    }
    #[doc = "Master group A read of memory protection enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPGRPA_A::_1
    }
}
#[doc = "Field `RPGRPA` writer - Master Group A Read protection"]
pub type RPGRPA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RPGRPA_A>;
impl<'a, REG, const O: u8> RPGRPA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master group A read of memory protection disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RPGRPA_A::_0)
    }
    #[doc = "Master group A read of memory protection enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RPGRPA_A::_1)
    }
}
#[doc = "Field `WPGRPA` reader - Master Group A Write protection"]
pub type WPGRPA_R = crate::BitReader<WPGRPA_A>;
#[doc = "Master Group A Write protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPGRPA_A {
    #[doc = "0: Master group A write of memory protection disabled."]
    _0 = 0,
    #[doc = "1: Master group A write of memory protection enabled."]
    _1 = 1,
}
impl From<WPGRPA_A> for bool {
    #[inline(always)]
    fn from(variant: WPGRPA_A) -> Self {
        variant as u8 != 0
    }
}
impl WPGRPA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WPGRPA_A {
        match self.bits {
            false => WPGRPA_A::_0,
            true => WPGRPA_A::_1,
        }
    }
    #[doc = "Master group A write of memory protection disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WPGRPA_A::_0
    }
    #[doc = "Master group A write of memory protection enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WPGRPA_A::_1
    }
}
#[doc = "Field `WPGRPA` writer - Master Group A Write protection"]
pub type WPGRPA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WPGRPA_A>;
impl<'a, REG, const O: u8> WPGRPA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master group A write of memory protection disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WPGRPA_A::_0)
    }
    #[doc = "Master group A write of memory protection enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WPGRPA_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - CPU Read protection"]
    #[inline(always)]
    pub fn rpcpu(&self) -> RPCPU_R {
        RPCPU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU Write protection"]
    #[inline(always)]
    pub fn wpcpu(&self) -> WPCPU_R {
        WPCPU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master Group A Read protection"]
    #[inline(always)]
    pub fn rpgrpa(&self) -> RPGRPA_R {
        RPGRPA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master Group A Write protection"]
    #[inline(always)]
    pub fn wpgrpa(&self) -> WPGRPA_R {
        WPGRPA_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU Read protection"]
    #[inline(always)]
    #[must_use]
    pub fn rpcpu(&mut self) -> RPCPU_W<SMPUPBIU_SPEC, 0> {
        RPCPU_W::new(self)
    }
    #[doc = "Bit 1 - CPU Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn wpcpu(&mut self) -> WPCPU_W<SMPUPBIU_SPEC, 1> {
        WPCPU_W::new(self)
    }
    #[doc = "Bit 2 - Master Group A Read protection"]
    #[inline(always)]
    #[must_use]
    pub fn rpgrpa(&mut self) -> RPGRPA_W<SMPUPBIU_SPEC, 2> {
        RPGRPA_W::new(self)
    }
    #[doc = "Bit 3 - Master Group A Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn wpgrpa(&mut self) -> WPGRPA_W<SMPUPBIU_SPEC, 3> {
        WPGRPA_W::new(self)
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
#[doc = "Access Control Register for P%sBIU\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpupbiu::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpupbiu::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMPUPBIU_SPEC;
impl crate::RegisterSpec for SMPUPBIU_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`smpupbiu::R`](R) reader structure"]
impl crate::Readable for SMPUPBIU_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smpupbiu::W`](W) writer structure"]
impl crate::Writable for SMPUPBIU_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMPUP%sBIU to value 0"]
impl crate::Resettable for SMPUPBIU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
