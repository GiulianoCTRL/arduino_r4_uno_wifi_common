#[doc = "Register `ECC2STS` reader"]
pub type R = crate::R<ECC2STS_SPEC>;
#[doc = "Register `ECC2STS` writer"]
pub type W = crate::W<ECC2STS_SPEC>;
#[doc = "Field `ECC2ERR` reader - ECC 2-Bit Error Status\n\nThe field is **modified** in some way after a read operation."]
pub type ECC2ERR_R = crate::BitReader<ECC2ERR_A>;
#[doc = "ECC 2-Bit Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECC2ERR_A {
    #[doc = "0: No 2-bit ECC error occurred"]
    _0 = 0,
    #[doc = "1: 2-bit ECC error occurred."]
    _1 = 1,
}
impl From<ECC2ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ECC2ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl ECC2ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ECC2ERR_A {
        match self.bits {
            false => ECC2ERR_A::_0,
            true => ECC2ERR_A::_1,
        }
    }
    #[doc = "No 2-bit ECC error occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECC2ERR_A::_0
    }
    #[doc = "2-bit ECC error occurred."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECC2ERR_A::_1
    }
}
#[doc = "Field `ECC2ERR` writer - ECC 2-Bit Error Status"]
pub type ECC2ERR_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, ECC2ERR_A>;
impl<'a, REG, const O: u8> ECC2ERR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No 2-bit ECC error occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ECC2ERR_A::_0)
    }
    #[doc = "2-bit ECC error occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ECC2ERR_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - ECC 2-Bit Error Status"]
    #[inline(always)]
    pub fn ecc2err(&self) -> ECC2ERR_R {
        ECC2ERR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC 2-Bit Error Status"]
    #[inline(always)]
    #[must_use]
    pub fn ecc2err(&mut self) -> ECC2ERR_W<ECC2STS_SPEC, 0> {
        ECC2ERR_W::new(self)
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
#[doc = "ECC 2-Bit Error Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc2sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc2sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECC2STS_SPEC;
impl crate::RegisterSpec for ECC2STS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ecc2sts::R`](R) reader structure"]
impl crate::Readable for ECC2STS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ecc2sts::W`](W) writer structure"]
impl crate::Writable for ECC2STS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECC2STS to value 0"]
impl crate::Resettable for ECC2STS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
