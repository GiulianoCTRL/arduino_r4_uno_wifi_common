#[doc = "Register `DADPR` reader"]
pub type R = crate::R<DADPR_SPEC>;
#[doc = "Register `DADPR` writer"]
pub type W = crate::W<DADPR_SPEC>;
#[doc = "Field `DPSEL` reader - DADRm Format Select"]
pub type DPSEL_R = crate::BitReader<DPSEL_A>;
#[doc = "DADRm Format Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPSEL_A {
    #[doc = "0: Right justified format."]
    _0 = 0,
    #[doc = "1: Left justified format."]
    _1 = 1,
}
impl From<DPSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DPSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl DPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DPSEL_A {
        match self.bits {
            false => DPSEL_A::_0,
            true => DPSEL_A::_1,
        }
    }
    #[doc = "Right justified format."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPSEL_A::_0
    }
    #[doc = "Left justified format."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPSEL_A::_1
    }
}
#[doc = "Field `DPSEL` writer - DADRm Format Select"]
pub type DPSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DPSEL_A>;
impl<'a, REG, const O: u8> DPSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Right justified format."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DPSEL_A::_0)
    }
    #[doc = "Left justified format."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DPSEL_A::_1)
    }
}
impl R {
    #[doc = "Bit 7 - DADRm Format Select"]
    #[inline(always)]
    pub fn dpsel(&self) -> DPSEL_R {
        DPSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - DADRm Format Select"]
    #[inline(always)]
    #[must_use]
    pub fn dpsel(&mut self) -> DPSEL_W<DADPR_SPEC, 7> {
        DPSEL_W::new(self)
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
#[doc = "DADR0 Format Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dadpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dadpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DADPR_SPEC;
impl crate::RegisterSpec for DADPR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dadpr::R`](R) reader structure"]
impl crate::Readable for DADPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dadpr::W`](W) writer structure"]
impl crate::Writable for DADPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DADPR to value 0"]
impl crate::Resettable for DADPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
