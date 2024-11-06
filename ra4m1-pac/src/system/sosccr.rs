#[doc = "Register `SOSCCR` reader"]
pub type R = crate::R<SOSCCR_SPEC>;
#[doc = "Register `SOSCCR` writer"]
pub type W = crate::W<SOSCCR_SPEC>;
#[doc = "Field `SOSTP` reader - Sub-Clock Oscillator Stop"]
pub type SOSTP_R = crate::BitReader<SOSTP_A>;
#[doc = "Sub-Clock Oscillator Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOSTP_A {
    #[doc = "0: Sub-clock oscillator is operating."]
    _0 = 0,
    #[doc = "1: Sub-clock oscillator is stopped."]
    _1 = 1,
}
impl From<SOSTP_A> for bool {
    #[inline(always)]
    fn from(variant: SOSTP_A) -> Self {
        variant as u8 != 0
    }
}
impl SOSTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SOSTP_A {
        match self.bits {
            false => SOSTP_A::_0,
            true => SOSTP_A::_1,
        }
    }
    #[doc = "Sub-clock oscillator is operating."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOSTP_A::_0
    }
    #[doc = "Sub-clock oscillator is stopped."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOSTP_A::_1
    }
}
#[doc = "Field `SOSTP` writer - Sub-Clock Oscillator Stop"]
pub type SOSTP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SOSTP_A>;
impl<'a, REG, const O: u8> SOSTP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sub-clock oscillator is operating."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SOSTP_A::_0)
    }
    #[doc = "Sub-clock oscillator is stopped."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SOSTP_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Sub-Clock Oscillator Stop"]
    #[inline(always)]
    pub fn sostp(&self) -> SOSTP_R {
        SOSTP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sub-Clock Oscillator Stop"]
    #[inline(always)]
    #[must_use]
    pub fn sostp(&mut self) -> SOSTP_W<SOSCCR_SPEC, 0> {
        SOSTP_W::new(self)
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
#[doc = "Sub-Clock Oscillator Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sosccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sosccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOSCCR_SPEC;
impl crate::RegisterSpec for SOSCCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sosccr::R`](R) reader structure"]
impl crate::Readable for SOSCCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sosccr::W`](W) writer structure"]
impl crate::Writable for SOSCCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOSCCR to value 0x01"]
impl crate::Resettable for SOSCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
