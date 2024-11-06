#[doc = "Register `ECC1STS` reader"]
pub type R = crate::R<ECC1STS_SPEC>;
#[doc = "Register `ECC1STS` writer"]
pub type W = crate::W<ECC1STS_SPEC>;
#[doc = "Field `ECC1ERR` reader - ECC 1-Bit Error Status\n\nThe field is **modified** in some way after a read operation."]
pub type ECC1ERR_R = crate::BitReader<ECC1ERR_A>;
#[doc = "ECC 1-Bit Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECC1ERR_A {
    #[doc = "0: No 1-bit ECC error occurred"]
    _0 = 0,
    #[doc = "1: 1-bit ECC error occurred"]
    _1 = 1,
}
impl From<ECC1ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ECC1ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl ECC1ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ECC1ERR_A {
        match self.bits {
            false => ECC1ERR_A::_0,
            true => ECC1ERR_A::_1,
        }
    }
    #[doc = "No 1-bit ECC error occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECC1ERR_A::_0
    }
    #[doc = "1-bit ECC error occurred"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECC1ERR_A::_1
    }
}
#[doc = "Field `ECC1ERR` writer - ECC 1-Bit Error Status"]
pub type ECC1ERR_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, ECC1ERR_A>;
impl<'a, REG, const O: u8> ECC1ERR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No 1-bit ECC error occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ECC1ERR_A::_0)
    }
    #[doc = "1-bit ECC error occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ECC1ERR_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - ECC 1-Bit Error Status"]
    #[inline(always)]
    pub fn ecc1err(&self) -> ECC1ERR_R {
        ECC1ERR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC 1-Bit Error Status"]
    #[inline(always)]
    #[must_use]
    pub fn ecc1err(&mut self) -> ECC1ERR_W<ECC1STS_SPEC, 0> {
        ECC1ERR_W::new(self)
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
#[doc = "ECC 1-Bit Error Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc1sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc1sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECC1STS_SPEC;
impl crate::RegisterSpec for ECC1STS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ecc1sts::R`](R) reader structure"]
impl crate::Readable for ECC1STS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ecc1sts::W`](W) writer structure"]
impl crate::Writable for ECC1STS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECC1STS to value 0"]
impl crate::Resettable for ECC1STS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
