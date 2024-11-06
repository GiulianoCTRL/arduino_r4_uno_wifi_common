#[doc = "Register `DTCST` reader"]
pub type R = crate::R<DTCST_SPEC>;
#[doc = "Register `DTCST` writer"]
pub type W = crate::W<DTCST_SPEC>;
#[doc = "Field `DTCST` reader - DTC Module Start"]
pub type DTCST_R = crate::BitReader<DTCST_A>;
#[doc = "DTC Module Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTCST_A {
    #[doc = "0: DTC module stop"]
    _0 = 0,
    #[doc = "1: DTC module start"]
    _1 = 1,
}
impl From<DTCST_A> for bool {
    #[inline(always)]
    fn from(variant: DTCST_A) -> Self {
        variant as u8 != 0
    }
}
impl DTCST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTCST_A {
        match self.bits {
            false => DTCST_A::_0,
            true => DTCST_A::_1,
        }
    }
    #[doc = "DTC module stop"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTCST_A::_0
    }
    #[doc = "DTC module start"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTCST_A::_1
    }
}
#[doc = "Field `DTCST` writer - DTC Module Start"]
pub type DTCST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DTCST_A>;
impl<'a, REG, const O: u8> DTCST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DTC module stop"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DTCST_A::_0)
    }
    #[doc = "DTC module start"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DTCST_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - DTC Module Start"]
    #[inline(always)]
    pub fn dtcst(&self) -> DTCST_R {
        DTCST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTC Module Start"]
    #[inline(always)]
    #[must_use]
    pub fn dtcst(&mut self) -> DTCST_W<DTCST_SPEC, 0> {
        DTCST_W::new(self)
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
#[doc = "DTC Module Start Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtcst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtcst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTCST_SPEC;
impl crate::RegisterSpec for DTCST_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dtcst::R`](R) reader structure"]
impl crate::Readable for DTCST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dtcst::W`](W) writer structure"]
impl crate::Writable for DTCST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTCST to value 0"]
impl crate::Resettable for DTCST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
