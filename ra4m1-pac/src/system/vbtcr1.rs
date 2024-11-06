#[doc = "Register `VBTCR1` reader"]
pub type R = crate::R<VBTCR1_SPEC>;
#[doc = "Register `VBTCR1` writer"]
pub type W = crate::W<VBTCR1_SPEC>;
#[doc = "Field `BPWSWSTP` reader - Battery Power supply Switch Stop"]
pub type BPWSWSTP_R = crate::BitReader<BPWSWSTP_A>;
#[doc = "Battery Power supply Switch Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BPWSWSTP_A {
    #[doc = "0: Battery Power supply Switch Enable"]
    _0 = 0,
    #[doc = "1: Battery Power supply Switch stop"]
    _1 = 1,
}
impl From<BPWSWSTP_A> for bool {
    #[inline(always)]
    fn from(variant: BPWSWSTP_A) -> Self {
        variant as u8 != 0
    }
}
impl BPWSWSTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BPWSWSTP_A {
        match self.bits {
            false => BPWSWSTP_A::_0,
            true => BPWSWSTP_A::_1,
        }
    }
    #[doc = "Battery Power supply Switch Enable"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPWSWSTP_A::_0
    }
    #[doc = "Battery Power supply Switch stop"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPWSWSTP_A::_1
    }
}
#[doc = "Field `BPWSWSTP` writer - Battery Power supply Switch Stop"]
pub type BPWSWSTP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BPWSWSTP_A>;
impl<'a, REG, const O: u8> BPWSWSTP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Battery Power supply Switch Enable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BPWSWSTP_A::_0)
    }
    #[doc = "Battery Power supply Switch stop"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BPWSWSTP_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Battery Power supply Switch Stop"]
    #[inline(always)]
    pub fn bpwswstp(&self) -> BPWSWSTP_R {
        BPWSWSTP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Battery Power supply Switch Stop"]
    #[inline(always)]
    #[must_use]
    pub fn bpwswstp(&mut self) -> BPWSWSTP_W<VBTCR1_SPEC, 0> {
        BPWSWSTP_W::new(self)
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
#[doc = "VBATT Control Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbtcr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbtcr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VBTCR1_SPEC;
impl crate::RegisterSpec for VBTCR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vbtcr1::R`](R) reader structure"]
impl crate::Readable for VBTCR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vbtcr1::W`](W) writer structure"]
impl crate::Writable for VBTCR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VBTCR1 to value 0"]
impl crate::Resettable for VBTCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
