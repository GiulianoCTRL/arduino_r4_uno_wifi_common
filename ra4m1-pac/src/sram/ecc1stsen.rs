#[doc = "Register `ECC1STSEN` reader"]
pub type R = crate::R<ECC1STSEN_SPEC>;
#[doc = "Register `ECC1STSEN` writer"]
pub type W = crate::W<ECC1STSEN_SPEC>;
#[doc = "Field `E1STSEN` reader - ECC 1-Bit Error Information Update Enable"]
pub type E1STSEN_R = crate::BitReader<E1STSEN_A>;
#[doc = "ECC 1-Bit Error Information Update Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E1STSEN_A {
    #[doc = "0: Disables updating of the 1-bit ECC error information."]
    _0 = 0,
    #[doc = "1: Enables updating of the 1-bit ECC error information."]
    _1 = 1,
}
impl From<E1STSEN_A> for bool {
    #[inline(always)]
    fn from(variant: E1STSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl E1STSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> E1STSEN_A {
        match self.bits {
            false => E1STSEN_A::_0,
            true => E1STSEN_A::_1,
        }
    }
    #[doc = "Disables updating of the 1-bit ECC error information."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == E1STSEN_A::_0
    }
    #[doc = "Enables updating of the 1-bit ECC error information."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == E1STSEN_A::_1
    }
}
#[doc = "Field `E1STSEN` writer - ECC 1-Bit Error Information Update Enable"]
pub type E1STSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, E1STSEN_A>;
impl<'a, REG, const O: u8> E1STSEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables updating of the 1-bit ECC error information."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(E1STSEN_A::_0)
    }
    #[doc = "Enables updating of the 1-bit ECC error information."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(E1STSEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - ECC 1-Bit Error Information Update Enable"]
    #[inline(always)]
    pub fn e1stsen(&self) -> E1STSEN_R {
        E1STSEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC 1-Bit Error Information Update Enable"]
    #[inline(always)]
    #[must_use]
    pub fn e1stsen(&mut self) -> E1STSEN_W<ECC1STSEN_SPEC, 0> {
        E1STSEN_W::new(self)
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
#[doc = "ECC 1-Bit Error Information Update Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc1stsen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc1stsen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECC1STSEN_SPEC;
impl crate::RegisterSpec for ECC1STSEN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ecc1stsen::R`](R) reader structure"]
impl crate::Readable for ECC1STSEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ecc1stsen::W`](W) writer structure"]
impl crate::Writable for ECC1STSEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECC1STSEN to value 0"]
impl crate::Resettable for ECC1STSEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
