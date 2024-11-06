#[doc = "Register `FCACHEIV` reader"]
pub type R = crate::R<FCACHEIV_SPEC>;
#[doc = "Register `FCACHEIV` writer"]
pub type W = crate::W<FCACHEIV_SPEC>;
#[doc = "Field `FCACHEIV` reader - FCACHE Invalidation\n\nThe field is **modified** in some way after a read operation."]
pub type FCACHEIV_R = crate::BitReader<FCACHEIV_A>;
#[doc = "FCACHE Invalidation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCACHEIV_A {
    #[doc = "0: (Read)not in progress / (Write) no effect."]
    _0 = 0,
    #[doc = "1: (Read)in progress /(Write) Starting Cache Invalidation"]
    _1 = 1,
}
impl From<FCACHEIV_A> for bool {
    #[inline(always)]
    fn from(variant: FCACHEIV_A) -> Self {
        variant as u8 != 0
    }
}
impl FCACHEIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FCACHEIV_A {
        match self.bits {
            false => FCACHEIV_A::_0,
            true => FCACHEIV_A::_1,
        }
    }
    #[doc = "(Read)not in progress / (Write) no effect."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FCACHEIV_A::_0
    }
    #[doc = "(Read)in progress /(Write) Starting Cache Invalidation"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FCACHEIV_A::_1
    }
}
#[doc = "Field `FCACHEIV` writer - FCACHE Invalidation"]
pub type FCACHEIV_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O, FCACHEIV_A>;
impl<'a, REG, const O: u8> FCACHEIV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "(Read)not in progress / (Write) no effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FCACHEIV_A::_0)
    }
    #[doc = "(Read)in progress /(Write) Starting Cache Invalidation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FCACHEIV_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - FCACHE Invalidation"]
    #[inline(always)]
    pub fn fcacheiv(&self) -> FCACHEIV_R {
        FCACHEIV_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FCACHE Invalidation"]
    #[inline(always)]
    #[must_use]
    pub fn fcacheiv(&mut self) -> FCACHEIV_W<FCACHEIV_SPEC, 0> {
        FCACHEIV_W::new(self)
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
#[doc = "Flash Cache Invalidate Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcacheiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcacheiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCACHEIV_SPEC;
impl crate::RegisterSpec for FCACHEIV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fcacheiv::R`](R) reader structure"]
impl crate::Readable for FCACHEIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fcacheiv::W`](W) writer structure"]
impl crate::Writable for FCACHEIV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
}
#[doc = "`reset()` method sets FCACHEIV to value 0"]
impl crate::Resettable for FCACHEIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
