#[doc = "Register `BUSMCNT%s` reader"]
pub type R = crate::R<BUSMCNT_SPEC>;
#[doc = "Register `BUSMCNT%s` writer"]
pub type W = crate::W<BUSMCNT_SPEC>;
#[doc = "Field `IERES` reader - Ignore Error Responses"]
pub type IERES_R = crate::BitReader<IERES_A>;
#[doc = "Ignore Error Responses\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IERES_A {
    #[doc = "0: A bus error is reported"]
    _0 = 0,
    #[doc = "1: A bus error is not reported."]
    _1 = 1,
}
impl From<IERES_A> for bool {
    #[inline(always)]
    fn from(variant: IERES_A) -> Self {
        variant as u8 != 0
    }
}
impl IERES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IERES_A {
        match self.bits {
            false => IERES_A::_0,
            true => IERES_A::_1,
        }
    }
    #[doc = "A bus error is reported"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IERES_A::_0
    }
    #[doc = "A bus error is not reported."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IERES_A::_1
    }
}
#[doc = "Field `IERES` writer - Ignore Error Responses"]
pub type IERES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IERES_A>;
impl<'a, REG, const O: u8> IERES_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A bus error is reported"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IERES_A::_0)
    }
    #[doc = "A bus error is not reported."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IERES_A::_1)
    }
}
impl R {
    #[doc = "Bit 15 - Ignore Error Responses"]
    #[inline(always)]
    pub fn ieres(&self) -> IERES_R {
        IERES_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Ignore Error Responses"]
    #[inline(always)]
    #[must_use]
    pub fn ieres(&mut self) -> IERES_W<BUSMCNT_SPEC, 15> {
        IERES_W::new(self)
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
#[doc = "Master Bus Control Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busmcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`busmcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUSMCNT_SPEC;
impl crate::RegisterSpec for BUSMCNT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`busmcnt::R`](R) reader structure"]
impl crate::Readable for BUSMCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`busmcnt::W`](W) writer structure"]
impl crate::Writable for BUSMCNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUSMCNT%s to value 0"]
impl crate::Resettable for BUSMCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
