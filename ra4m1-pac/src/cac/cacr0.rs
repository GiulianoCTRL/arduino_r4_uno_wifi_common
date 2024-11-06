#[doc = "Register `CACR0` reader"]
pub type R = crate::R<CACR0_SPEC>;
#[doc = "Register `CACR0` writer"]
pub type W = crate::W<CACR0_SPEC>;
#[doc = "Field `CFME` reader - Clock Frequency Measurement Enable."]
pub type CFME_R = crate::BitReader<CFME_A>;
#[doc = "Clock Frequency Measurement Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFME_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<CFME_A> for bool {
    #[inline(always)]
    fn from(variant: CFME_A) -> Self {
        variant as u8 != 0
    }
}
impl CFME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CFME_A {
        match self.bits {
            false => CFME_A::_0,
            true => CFME_A::_1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFME_A::_0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFME_A::_1
    }
}
#[doc = "Field `CFME` writer - Clock Frequency Measurement Enable."]
pub type CFME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CFME_A>;
impl<'a, REG, const O: u8> CFME_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CFME_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CFME_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Clock Frequency Measurement Enable."]
    #[inline(always)]
    pub fn cfme(&self) -> CFME_R {
        CFME_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Frequency Measurement Enable."]
    #[inline(always)]
    #[must_use]
    pub fn cfme(&mut self) -> CFME_W<CACR0_SPEC, 0> {
        CFME_W::new(self)
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
#[doc = "CAC Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cacr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cacr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACR0_SPEC;
impl crate::RegisterSpec for CACR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cacr0::R`](R) reader structure"]
impl crate::Readable for CACR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cacr0::W`](W) writer structure"]
impl crate::Writable for CACR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACR0 to value 0"]
impl crate::Resettable for CACR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
