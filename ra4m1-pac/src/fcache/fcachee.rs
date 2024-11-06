#[doc = "Register `FCACHEE` reader"]
pub type R = crate::R<FCACHEE_SPEC>;
#[doc = "Register `FCACHEE` writer"]
pub type W = crate::W<FCACHEE_SPEC>;
#[doc = "Field `FCACHEEN` reader - FCACHE Enable"]
pub type FCACHEEN_R = crate::BitReader<FCACHEEN_A>;
#[doc = "FCACHE Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCACHEEN_A {
    #[doc = "0: FCACHE is disabled"]
    _0 = 0,
    #[doc = "1: FCACHE is enabled"]
    _1 = 1,
}
impl From<FCACHEEN_A> for bool {
    #[inline(always)]
    fn from(variant: FCACHEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FCACHEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FCACHEEN_A {
        match self.bits {
            false => FCACHEEN_A::_0,
            true => FCACHEEN_A::_1,
        }
    }
    #[doc = "FCACHE is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FCACHEEN_A::_0
    }
    #[doc = "FCACHE is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FCACHEEN_A::_1
    }
}
#[doc = "Field `FCACHEEN` writer - FCACHE Enable"]
pub type FCACHEEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FCACHEEN_A>;
impl<'a, REG, const O: u8> FCACHEEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FCACHE is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FCACHEEN_A::_0)
    }
    #[doc = "FCACHE is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FCACHEEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - FCACHE Enable"]
    #[inline(always)]
    pub fn fcacheen(&self) -> FCACHEEN_R {
        FCACHEEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FCACHE Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fcacheen(&mut self) -> FCACHEEN_W<FCACHEE_SPEC, 0> {
        FCACHEEN_W::new(self)
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
#[doc = "Flash Cache Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcachee::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcachee::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCACHEE_SPEC;
impl crate::RegisterSpec for FCACHEE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fcachee::R`](R) reader structure"]
impl crate::Readable for FCACHEE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fcachee::W`](W) writer structure"]
impl crate::Writable for FCACHEE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCACHEE to value 0"]
impl crate::Resettable for FCACHEE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
