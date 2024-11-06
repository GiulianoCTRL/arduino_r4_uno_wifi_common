#[doc = "Register `ECCETST` reader"]
pub type R = crate::R<ECCETST_SPEC>;
#[doc = "Register `ECCETST` writer"]
pub type W = crate::W<ECCETST_SPEC>;
#[doc = "Field `TSTBYP` reader - ECC Bypass Select"]
pub type TSTBYP_R = crate::BitReader<TSTBYP_A>;
#[doc = "ECC Bypass Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSTBYP_A {
    #[doc = "0: ECC bypass disabled."]
    _0 = 0,
    #[doc = "1: ECC bypass enabled."]
    _1 = 1,
}
impl From<TSTBYP_A> for bool {
    #[inline(always)]
    fn from(variant: TSTBYP_A) -> Self {
        variant as u8 != 0
    }
}
impl TSTBYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSTBYP_A {
        match self.bits {
            false => TSTBYP_A::_0,
            true => TSTBYP_A::_1,
        }
    }
    #[doc = "ECC bypass disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSTBYP_A::_0
    }
    #[doc = "ECC bypass enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSTBYP_A::_1
    }
}
#[doc = "Field `TSTBYP` writer - ECC Bypass Select"]
pub type TSTBYP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TSTBYP_A>;
impl<'a, REG, const O: u8> TSTBYP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ECC bypass disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TSTBYP_A::_0)
    }
    #[doc = "ECC bypass enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TSTBYP_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - ECC Bypass Select"]
    #[inline(always)]
    pub fn tstbyp(&self) -> TSTBYP_R {
        TSTBYP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC Bypass Select"]
    #[inline(always)]
    #[must_use]
    pub fn tstbyp(&mut self) -> TSTBYP_W<ECCETST_SPEC, 0> {
        TSTBYP_W::new(self)
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
#[doc = "ECC Test Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccetst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccetst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECCETST_SPEC;
impl crate::RegisterSpec for ECCETST_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`eccetst::R`](R) reader structure"]
impl crate::Readable for ECCETST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eccetst::W`](W) writer structure"]
impl crate::Writable for ECCETST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECCETST to value 0"]
impl crate::Resettable for ECCETST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
