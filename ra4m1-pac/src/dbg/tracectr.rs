#[doc = "Register `TRACECTR` reader"]
pub type R = crate::R<TRACECTR_SPEC>;
#[doc = "Register `TRACECTR` writer"]
pub type W = crate::W<TRACECTR_SPEC>;
#[doc = "Field `ENETBFULL` reader - Enable bit for halt request by ETB full"]
pub type ENETBFULL_R = crate::BitReader<ENETBFULL_A>;
#[doc = "Enable bit for halt request by ETB full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENETBFULL_A {
    #[doc = "0: ETB full does not cause CPU halt"]
    _0 = 0,
    #[doc = "1: ETB full cause CPU halt"]
    _1 = 1,
}
impl From<ENETBFULL_A> for bool {
    #[inline(always)]
    fn from(variant: ENETBFULL_A) -> Self {
        variant as u8 != 0
    }
}
impl ENETBFULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENETBFULL_A {
        match self.bits {
            false => ENETBFULL_A::_0,
            true => ENETBFULL_A::_1,
        }
    }
    #[doc = "ETB full does not cause CPU halt"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENETBFULL_A::_0
    }
    #[doc = "ETB full cause CPU halt"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENETBFULL_A::_1
    }
}
#[doc = "Field `ENETBFULL` writer - Enable bit for halt request by ETB full"]
pub type ENETBFULL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ENETBFULL_A>;
impl<'a, REG, const O: u8> ENETBFULL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ETB full does not cause CPU halt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ENETBFULL_A::_0)
    }
    #[doc = "ETB full cause CPU halt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ENETBFULL_A::_1)
    }
}
impl R {
    #[doc = "Bit 31 - Enable bit for halt request by ETB full"]
    #[inline(always)]
    pub fn enetbfull(&self) -> ENETBFULL_R {
        ENETBFULL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Enable bit for halt request by ETB full"]
    #[inline(always)]
    #[must_use]
    pub fn enetbfull(&mut self) -> ENETBFULL_W<TRACECTR_SPEC, 31> {
        ENETBFULL_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Trace Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tracectr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tracectr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRACECTR_SPEC;
impl crate::RegisterSpec for TRACECTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tracectr::R`](R) reader structure"]
impl crate::Readable for TRACECTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tracectr::W`](W) writer structure"]
impl crate::Writable for TRACECTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRACECTR to value 0"]
impl crate::Resettable for TRACECTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
