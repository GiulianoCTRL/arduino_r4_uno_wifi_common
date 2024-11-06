#[doc = "Register `WDTRCR` reader"]
pub type R = crate::R<WDTRCR_SPEC>;
#[doc = "Register `WDTRCR` writer"]
pub type W = crate::W<WDTRCR_SPEC>;
#[doc = "Field `RSTIRQS` reader - Reset Interrupt Request Selection"]
pub type RSTIRQS_R = crate::BitReader<RSTIRQS_A>;
#[doc = "Reset Interrupt Request Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTIRQS_A {
    #[doc = "0: Non-maskable interrupt request or interrupt request output is enabled"]
    _0 = 0,
    #[doc = "1: Reset output is enabled."]
    _1 = 1,
}
impl From<RSTIRQS_A> for bool {
    #[inline(always)]
    fn from(variant: RSTIRQS_A) -> Self {
        variant as u8 != 0
    }
}
impl RSTIRQS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RSTIRQS_A {
        match self.bits {
            false => RSTIRQS_A::_0,
            true => RSTIRQS_A::_1,
        }
    }
    #[doc = "Non-maskable interrupt request or interrupt request output is enabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSTIRQS_A::_0
    }
    #[doc = "Reset output is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSTIRQS_A::_1
    }
}
#[doc = "Field `RSTIRQS` writer - Reset Interrupt Request Selection"]
pub type RSTIRQS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RSTIRQS_A>;
impl<'a, REG, const O: u8> RSTIRQS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Non-maskable interrupt request or interrupt request output is enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RSTIRQS_A::_0)
    }
    #[doc = "Reset output is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RSTIRQS_A::_1)
    }
}
impl R {
    #[doc = "Bit 7 - Reset Interrupt Request Selection"]
    #[inline(always)]
    pub fn rstirqs(&self) -> RSTIRQS_R {
        RSTIRQS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Reset Interrupt Request Selection"]
    #[inline(always)]
    #[must_use]
    pub fn rstirqs(&mut self) -> RSTIRQS_W<WDTRCR_SPEC, 7> {
        RSTIRQS_W::new(self)
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
#[doc = "WDT Reset Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtrcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtrcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDTRCR_SPEC;
impl crate::RegisterSpec for WDTRCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wdtrcr::R`](R) reader structure"]
impl crate::Readable for WDTRCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdtrcr::W`](W) writer structure"]
impl crate::Writable for WDTRCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTRCR to value 0x80"]
impl crate::Resettable for WDTRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
