#[doc = "Register `ELCR` reader"]
pub type R = crate::R<ELCR_SPEC>;
#[doc = "Register `ELCR` writer"]
pub type W = crate::W<ELCR_SPEC>;
#[doc = "Field `ELCON` reader - All Event Link Enable"]
pub type ELCON_R = crate::BitReader<ELCON_A>;
#[doc = "All Event Link Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELCON_A {
    #[doc = "0: ELC function is disabled."]
    _0 = 0,
    #[doc = "1: ELC function is enabled."]
    _1 = 1,
}
impl From<ELCON_A> for bool {
    #[inline(always)]
    fn from(variant: ELCON_A) -> Self {
        variant as u8 != 0
    }
}
impl ELCON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ELCON_A {
        match self.bits {
            false => ELCON_A::_0,
            true => ELCON_A::_1,
        }
    }
    #[doc = "ELC function is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELCON_A::_0
    }
    #[doc = "ELC function is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELCON_A::_1
    }
}
#[doc = "Field `ELCON` writer - All Event Link Enable"]
pub type ELCON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ELCON_A>;
impl<'a, REG, const O: u8> ELCON_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ELC function is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ELCON_A::_0)
    }
    #[doc = "ELC function is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ELCON_A::_1)
    }
}
impl R {
    #[doc = "Bit 7 - All Event Link Enable"]
    #[inline(always)]
    pub fn elcon(&self) -> ELCON_R {
        ELCON_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - All Event Link Enable"]
    #[inline(always)]
    #[must_use]
    pub fn elcon(&mut self) -> ELCON_W<ELCR_SPEC, 7> {
        ELCON_W::new(self)
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
#[doc = "Event Link Controller Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`elcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`elcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ELCR_SPEC;
impl crate::RegisterSpec for ELCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`elcr::R`](R) reader structure"]
impl crate::Readable for ELCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`elcr::W`](W) writer structure"]
impl crate::Writable for ELCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ELCR to value 0"]
impl crate::Resettable for ELCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
