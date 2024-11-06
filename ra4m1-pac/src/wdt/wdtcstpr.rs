#[doc = "Register `WDTCSTPR` reader"]
pub type R = crate::R<WDTCSTPR_SPEC>;
#[doc = "Register `WDTCSTPR` writer"]
pub type W = crate::W<WDTCSTPR_SPEC>;
#[doc = "Field `SLCSTP` reader - Sleep-Mode Count Stop Control"]
pub type SLCSTP_R = crate::BitReader<SLCSTP_A>;
#[doc = "Sleep-Mode Count Stop Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLCSTP_A {
    #[doc = "0: Count stop is disabled."]
    _0 = 0,
    #[doc = "1: Count is stopped at a transition to sleep mode."]
    _1 = 1,
}
impl From<SLCSTP_A> for bool {
    #[inline(always)]
    fn from(variant: SLCSTP_A) -> Self {
        variant as u8 != 0
    }
}
impl SLCSTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SLCSTP_A {
        match self.bits {
            false => SLCSTP_A::_0,
            true => SLCSTP_A::_1,
        }
    }
    #[doc = "Count stop is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLCSTP_A::_0
    }
    #[doc = "Count is stopped at a transition to sleep mode."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLCSTP_A::_1
    }
}
#[doc = "Field `SLCSTP` writer - Sleep-Mode Count Stop Control"]
pub type SLCSTP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SLCSTP_A>;
impl<'a, REG, const O: u8> SLCSTP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Count stop is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SLCSTP_A::_0)
    }
    #[doc = "Count is stopped at a transition to sleep mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SLCSTP_A::_1)
    }
}
impl R {
    #[doc = "Bit 7 - Sleep-Mode Count Stop Control"]
    #[inline(always)]
    pub fn slcstp(&self) -> SLCSTP_R {
        SLCSTP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Sleep-Mode Count Stop Control"]
    #[inline(always)]
    #[must_use]
    pub fn slcstp(&mut self) -> SLCSTP_W<WDTCSTPR_SPEC, 7> {
        SLCSTP_W::new(self)
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
#[doc = "WDT Count Stop Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtcstpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtcstpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDTCSTPR_SPEC;
impl crate::RegisterSpec for WDTCSTPR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wdtcstpr::R`](R) reader structure"]
impl crate::Readable for WDTCSTPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdtcstpr::W`](W) writer structure"]
impl crate::Writable for WDTCSTPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTCSTPR to value 0x80"]
impl crate::Resettable for WDTCSTPR_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
