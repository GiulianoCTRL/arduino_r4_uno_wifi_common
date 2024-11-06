#[doc = "Register `AGTISR` reader"]
pub type R = crate::R<AGTISR_SPEC>;
#[doc = "Register `AGTISR` writer"]
pub type W = crate::W<AGTISR_SPEC>;
#[doc = "Field `EEPS` reader - AGTEE polarty selection"]
pub type EEPS_R = crate::BitReader<EEPS_A>;
#[doc = "AGTEE polarty selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEPS_A {
    #[doc = "0: An event is counted during the low-level period"]
    _0 = 0,
    #[doc = "1: An event is counted during the high-level period"]
    _1 = 1,
}
impl From<EEPS_A> for bool {
    #[inline(always)]
    fn from(variant: EEPS_A) -> Self {
        variant as u8 != 0
    }
}
impl EEPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EEPS_A {
        match self.bits {
            false => EEPS_A::_0,
            true => EEPS_A::_1,
        }
    }
    #[doc = "An event is counted during the low-level period"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EEPS_A::_0
    }
    #[doc = "An event is counted during the high-level period"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EEPS_A::_1
    }
}
#[doc = "Field `EEPS` writer - AGTEE polarty selection"]
pub type EEPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EEPS_A>;
impl<'a, REG, const O: u8> EEPS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An event is counted during the low-level period"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EEPS_A::_0)
    }
    #[doc = "An event is counted during the high-level period"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EEPS_A::_1)
    }
}
impl R {
    #[doc = "Bit 2 - AGTEE polarty selection"]
    #[inline(always)]
    pub fn eeps(&self) -> EEPS_R {
        EEPS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - AGTEE polarty selection"]
    #[inline(always)]
    #[must_use]
    pub fn eeps(&mut self) -> EEPS_W<AGTISR_SPEC, 2> {
        EEPS_W::new(self)
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
#[doc = "AGT Event Pin Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`agtisr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`agtisr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AGTISR_SPEC;
impl crate::RegisterSpec for AGTISR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`agtisr::R`](R) reader structure"]
impl crate::Readable for AGTISR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`agtisr::W`](W) writer structure"]
impl crate::Writable for AGTISR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AGTISR to value 0"]
impl crate::Resettable for AGTISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
