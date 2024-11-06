#[doc = "Register `FLWT` reader"]
pub type R = crate::R<FLWT_SPEC>;
#[doc = "Register `FLWT` writer"]
pub type W = crate::W<FLWT_SPEC>;
#[doc = "Field `FLWT` reader - These bits represent the ratio of the CPU clock period to the Flash memory access time."]
pub type FLWT_R = crate::FieldReader<FLWT_A>;
#[doc = "These bits represent the ratio of the CPU clock period to the Flash memory access time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLWT_A {
    #[doc = "0: zero wait"]
    _000 = 0,
}
impl From<FLWT_A> for u8 {
    #[inline(always)]
    fn from(variant: FLWT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FLWT_A {
    type Ux = u8;
}
impl FLWT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FLWT_A> {
        match self.bits {
            0 => Some(FLWT_A::_000),
            _ => None,
        }
    }
    #[doc = "zero wait"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == FLWT_A::_000
    }
}
#[doc = "Field `FLWT` writer - These bits represent the ratio of the CPU clock period to the Flash memory access time."]
pub type FLWT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, FLWT_A>;
impl<'a, REG, const O: u8> FLWT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "zero wait"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(FLWT_A::_000)
    }
}
impl R {
    #[doc = "Bits 0:2 - These bits represent the ratio of the CPU clock period to the Flash memory access time."]
    #[inline(always)]
    pub fn flwt(&self) -> FLWT_R {
        FLWT_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - These bits represent the ratio of the CPU clock period to the Flash memory access time."]
    #[inline(always)]
    #[must_use]
    pub fn flwt(&mut self) -> FLWT_W<FLWT_SPEC, 0> {
        FLWT_W::new(self)
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
#[doc = "Flash Wait Cycle Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flwt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flwt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLWT_SPEC;
impl crate::RegisterSpec for FLWT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`flwt::R`](R) reader structure"]
impl crate::Readable for FLWT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flwt::W`](W) writer structure"]
impl crate::Writable for FLWT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLWT to value 0"]
impl crate::Resettable for FLWT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
