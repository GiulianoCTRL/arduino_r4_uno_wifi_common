#[doc = "Register `VBTCMPCR` reader"]
pub type R = crate::R<VBTCMPCR_SPEC>;
#[doc = "Register `VBTCMPCR` writer"]
pub type W = crate::W<VBTCMPCR_SPEC>;
#[doc = "Field `VBTCMPE` reader - VBATT pin low voltage detect circuit output enable"]
pub type VBTCMPE_R = crate::BitReader<VBTCMPE_A>;
#[doc = "VBATT pin low voltage detect circuit output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBTCMPE_A {
    #[doc = "0: VBATT pin low voltage detect circuit output disabled"]
    _0 = 0,
    #[doc = "1: VBATT pin low voltage detect circuit output enabled"]
    _1 = 1,
}
impl From<VBTCMPE_A> for bool {
    #[inline(always)]
    fn from(variant: VBTCMPE_A) -> Self {
        variant as u8 != 0
    }
}
impl VBTCMPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBTCMPE_A {
        match self.bits {
            false => VBTCMPE_A::_0,
            true => VBTCMPE_A::_1,
        }
    }
    #[doc = "VBATT pin low voltage detect circuit output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBTCMPE_A::_0
    }
    #[doc = "VBATT pin low voltage detect circuit output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBTCMPE_A::_1
    }
}
#[doc = "Field `VBTCMPE` writer - VBATT pin low voltage detect circuit output enable"]
pub type VBTCMPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VBTCMPE_A>;
impl<'a, REG, const O: u8> VBTCMPE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT pin low voltage detect circuit output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VBTCMPE_A::_0)
    }
    #[doc = "VBATT pin low voltage detect circuit output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VBTCMPE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - VBATT pin low voltage detect circuit output enable"]
    #[inline(always)]
    pub fn vbtcmpe(&self) -> VBTCMPE_R {
        VBTCMPE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBATT pin low voltage detect circuit output enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbtcmpe(&mut self) -> VBTCMPE_W<VBTCMPCR_SPEC, 0> {
        VBTCMPE_W::new(self)
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
#[doc = "VBATT Comparator Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbtcmpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbtcmpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VBTCMPCR_SPEC;
impl crate::RegisterSpec for VBTCMPCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vbtcmpcr::R`](R) reader structure"]
impl crate::Readable for VBTCMPCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vbtcmpcr::W`](W) writer structure"]
impl crate::Writable for VBTCMPCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VBTCMPCR to value 0"]
impl crate::Resettable for VBTCMPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
