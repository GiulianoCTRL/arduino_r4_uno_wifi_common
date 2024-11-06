#[doc = "Register `SMPUMBIU` reader"]
pub type R = crate::R<SMPUMBIU_SPEC>;
#[doc = "Register `SMPUMBIU` writer"]
pub type W = crate::W<SMPUMBIU_SPEC>;
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
    #[doc = "Bit 2 - Master Group A Read protection"]
    #[inline(always)]
    #[must_use]
    pub fn rpgrpa(&mut self) -> RPGRPA_W<SMPUMBIU_SPEC, 2> {
        RPGRPA_W::new(self)
    }
    #[doc = "Bit 3 - Master Group A Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn wpgrpa(&mut self) -> WPGRPA_W<SMPUMBIU_SPEC, 3> {
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
#[doc = "Access Control Register for MBIU\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpumbiu::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpumbiu::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMPUMBIU_SPEC;
impl crate::RegisterSpec for SMPUMBIU_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`smpumbiu::R`](R) reader structure"]
impl crate::Readable for SMPUMBIU_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smpumbiu::W`](W) writer structure"]
impl crate::Writable for SMPUMBIU_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMPUMBIU to value 0"]
impl crate::Resettable for SMPUMBIU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
