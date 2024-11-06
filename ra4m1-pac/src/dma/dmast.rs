#[doc = "Register `DMAST` reader"]
pub type R = crate::R<DMAST_SPEC>;
#[doc = "Register `DMAST` writer"]
pub type W = crate::W<DMAST_SPEC>;
#[doc = "Field `DMST` reader - DMAC Operation Enable"]
pub type DMST_R = crate::BitReader<DMST_A>;
#[doc = "DMAC Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMST_A {
    #[doc = "0: Disabled."]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<DMST_A> for bool {
    #[inline(always)]
    fn from(variant: DMST_A) -> Self {
        variant as u8 != 0
    }
}
impl DMST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMST_A {
        match self.bits {
            false => DMST_A::_0,
            true => DMST_A::_1,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMST_A::_0
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMST_A::_1
    }
}
#[doc = "Field `DMST` writer - DMAC Operation Enable"]
pub type DMST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMST_A>;
impl<'a, REG, const O: u8> DMST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DMST_A::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DMST_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - DMAC Operation Enable"]
    #[inline(always)]
    pub fn dmst(&self) -> DMST_R {
        DMST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMAC Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmst(&mut self) -> DMST_W<DMAST_SPEC, 0> {
        DMST_W::new(self)
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
#[doc = "DMAC Module Activation Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmast::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmast::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAST_SPEC;
impl crate::RegisterSpec for DMAST_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dmast::R`](R) reader structure"]
impl crate::Readable for DMAST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmast::W`](W) writer structure"]
impl crate::Writable for DMAST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAST to value 0"]
impl crate::Resettable for DMAST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
