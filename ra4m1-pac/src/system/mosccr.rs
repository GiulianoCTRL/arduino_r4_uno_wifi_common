#[doc = "Register `MOSCCR` reader"]
pub type R = crate::R<MOSCCR_SPEC>;
#[doc = "Register `MOSCCR` writer"]
pub type W = crate::W<MOSCCR_SPEC>;
#[doc = "Field `MOSTP` reader - Main Clock Oscillator Stop Note: MOMCR register must be set before setting MOSTP to 0."]
pub type MOSTP_R = crate::BitReader<MOSTP_A>;
#[doc = "Main Clock Oscillator Stop Note: MOMCR register must be set before setting MOSTP to 0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOSTP_A {
    #[doc = "0: Main clock oscillator is operating."]
    _0 = 0,
    #[doc = "1: Main clock oscillator is stopped."]
    _1 = 1,
}
impl From<MOSTP_A> for bool {
    #[inline(always)]
    fn from(variant: MOSTP_A) -> Self {
        variant as u8 != 0
    }
}
impl MOSTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MOSTP_A {
        match self.bits {
            false => MOSTP_A::_0,
            true => MOSTP_A::_1,
        }
    }
    #[doc = "Main clock oscillator is operating."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MOSTP_A::_0
    }
    #[doc = "Main clock oscillator is stopped."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MOSTP_A::_1
    }
}
#[doc = "Field `MOSTP` writer - Main Clock Oscillator Stop Note: MOMCR register must be set before setting MOSTP to 0."]
pub type MOSTP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MOSTP_A>;
impl<'a, REG, const O: u8> MOSTP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Main clock oscillator is operating."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MOSTP_A::_0)
    }
    #[doc = "Main clock oscillator is stopped."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MOSTP_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Main Clock Oscillator Stop Note: MOMCR register must be set before setting MOSTP to 0."]
    #[inline(always)]
    pub fn mostp(&self) -> MOSTP_R {
        MOSTP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Main Clock Oscillator Stop Note: MOMCR register must be set before setting MOSTP to 0."]
    #[inline(always)]
    #[must_use]
    pub fn mostp(&mut self) -> MOSTP_W<MOSCCR_SPEC, 0> {
        MOSTP_W::new(self)
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
#[doc = "Main Clock Oscillator Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mosccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mosccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MOSCCR_SPEC;
impl crate::RegisterSpec for MOSCCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mosccr::R`](R) reader structure"]
impl crate::Readable for MOSCCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mosccr::W`](W) writer structure"]
impl crate::Writable for MOSCCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MOSCCR to value 0x01"]
impl crate::Resettable for MOSCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
