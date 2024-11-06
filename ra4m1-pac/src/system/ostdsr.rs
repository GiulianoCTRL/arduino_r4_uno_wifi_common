#[doc = "Register `OSTDSR` reader"]
pub type R = crate::R<OSTDSR_SPEC>;
#[doc = "Register `OSTDSR` writer"]
pub type W = crate::W<OSTDSR_SPEC>;
#[doc = "Field `OSTDF` reader - Oscillation Stop Detection Flag\n\nThe field is **modified** in some way after a read operation."]
pub type OSTDF_R = crate::BitReader<OSTDF_A>;
#[doc = "Oscillation Stop Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSTDF_A {
    #[doc = "0: The main clock oscillation stop has not been detected."]
    _0 = 0,
    #[doc = "1: The main clock oscillation stop has been detected."]
    _1 = 1,
}
impl From<OSTDF_A> for bool {
    #[inline(always)]
    fn from(variant: OSTDF_A) -> Self {
        variant as u8 != 0
    }
}
impl OSTDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSTDF_A {
        match self.bits {
            false => OSTDF_A::_0,
            true => OSTDF_A::_1,
        }
    }
    #[doc = "The main clock oscillation stop has not been detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OSTDF_A::_0
    }
    #[doc = "The main clock oscillation stop has been detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OSTDF_A::_1
    }
}
#[doc = "Field `OSTDF` writer - Oscillation Stop Detection Flag"]
pub type OSTDF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, OSTDF_A>;
impl<'a, REG, const O: u8> OSTDF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The main clock oscillation stop has not been detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OSTDF_A::_0)
    }
    #[doc = "The main clock oscillation stop has been detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OSTDF_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Oscillation Stop Detection Flag"]
    #[inline(always)]
    pub fn ostdf(&self) -> OSTDF_R {
        OSTDF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Oscillation Stop Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ostdf(&mut self) -> OSTDF_W<OSTDSR_SPEC, 0> {
        OSTDF_W::new(self)
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
#[doc = "Oscillation Stop Detection Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ostdsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ostdsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSTDSR_SPEC;
impl crate::RegisterSpec for OSTDSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ostdsr::R`](R) reader structure"]
impl crate::Readable for OSTDSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ostdsr::W`](W) writer structure"]
impl crate::Writable for OSTDSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSTDSR to value 0"]
impl crate::Resettable for OSTDSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
