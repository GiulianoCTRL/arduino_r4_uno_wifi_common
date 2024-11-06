#[doc = "Register `HOCOWTCR` reader"]
pub type R = crate::R<HOCOWTCR_SPEC>;
#[doc = "Register `HOCOWTCR` writer"]
pub type W = crate::W<HOCOWTCR_SPEC>;
#[doc = "Field `HSTS` reader - HOCO wait time setting"]
pub type HSTS_R = crate::FieldReader<HSTS_A>;
#[doc = "HOCO wait time setting\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HSTS_A {
    #[doc = "5: If HOCO frequency is other than 64MHz, should set the value to 101b."]
    _101 = 5,
    #[doc = "6: If HOCO frequency = 64MHz, should set the value to 110b."]
    _110 = 6,
}
impl From<HSTS_A> for u8 {
    #[inline(always)]
    fn from(variant: HSTS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HSTS_A {
    type Ux = u8;
}
impl HSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HSTS_A> {
        match self.bits {
            5 => Some(HSTS_A::_101),
            6 => Some(HSTS_A::_110),
            _ => None,
        }
    }
    #[doc = "If HOCO frequency is other than 64MHz, should set the value to 101b."]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == HSTS_A::_101
    }
    #[doc = "If HOCO frequency = 64MHz, should set the value to 110b."]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == HSTS_A::_110
    }
}
#[doc = "Field `HSTS` writer - HOCO wait time setting"]
pub type HSTS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, HSTS_A>;
impl<'a, REG, const O: u8> HSTS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "If HOCO frequency is other than 64MHz, should set the value to 101b."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(HSTS_A::_101)
    }
    #[doc = "If HOCO frequency = 64MHz, should set the value to 110b."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(HSTS_A::_110)
    }
}
impl R {
    #[doc = "Bits 0:2 - HOCO wait time setting"]
    #[inline(always)]
    pub fn hsts(&self) -> HSTS_R {
        HSTS_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - HOCO wait time setting"]
    #[inline(always)]
    #[must_use]
    pub fn hsts(&mut self) -> HSTS_W<HOCOWTCR_SPEC, 0> {
        HSTS_W::new(self)
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
#[doc = "High-Speed On-Chip Oscillator Wait Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hocowtcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hocowtcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOCOWTCR_SPEC;
impl crate::RegisterSpec for HOCOWTCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hocowtcr::R`](R) reader structure"]
impl crate::Readable for HOCOWTCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hocowtcr::W`](W) writer structure"]
impl crate::Writable for HOCOWTCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOCOWTCR to value 0x05"]
impl crate::Resettable for HOCOWTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
