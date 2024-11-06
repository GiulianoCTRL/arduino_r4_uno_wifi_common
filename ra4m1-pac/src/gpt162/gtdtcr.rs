#[doc = "Register `GTDTCR` reader"]
pub type R = crate::R<GTDTCR_SPEC>;
#[doc = "Register `GTDTCR` writer"]
pub type W = crate::W<GTDTCR_SPEC>;
#[doc = "Field `TDE` reader - Negative-Phase Waveform Setting"]
pub type TDE_R = crate::BitReader<TDE_A>;
#[doc = "Negative-Phase Waveform Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDE_A {
    #[doc = "0: GTCCRB is set without using GTDVU and GTDVD."]
    _0 = 0,
    #[doc = "1: GTDVU and GTDVD are used to set the compare match value for negative-phase waveform with dead time automatically in GTCCRB."]
    _1 = 1,
}
impl From<TDE_A> for bool {
    #[inline(always)]
    fn from(variant: TDE_A) -> Self {
        variant as u8 != 0
    }
}
impl TDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TDE_A {
        match self.bits {
            false => TDE_A::_0,
            true => TDE_A::_1,
        }
    }
    #[doc = "GTCCRB is set without using GTDVU and GTDVD."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDE_A::_0
    }
    #[doc = "GTDVU and GTDVD are used to set the compare match value for negative-phase waveform with dead time automatically in GTCCRB."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDE_A::_1
    }
}
#[doc = "Field `TDE` writer - Negative-Phase Waveform Setting"]
pub type TDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TDE_A>;
impl<'a, REG, const O: u8> TDE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCCRB is set without using GTDVU and GTDVD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TDE_A::_0)
    }
    #[doc = "GTDVU and GTDVD are used to set the compare match value for negative-phase waveform with dead time automatically in GTCCRB."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TDE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Negative-Phase Waveform Setting"]
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Negative-Phase Waveform Setting"]
    #[inline(always)]
    #[must_use]
    pub fn tde(&mut self) -> TDE_W<GTDTCR_SPEC, 0> {
        TDE_W::new(self)
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
#[doc = "General PWM Timer Dead Time Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtdtcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtdtcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTDTCR_SPEC;
impl crate::RegisterSpec for GTDTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtdtcr::R`](R) reader structure"]
impl crate::Readable for GTDTCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gtdtcr::W`](W) writer structure"]
impl crate::Writable for GTDTCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTDTCR to value 0"]
impl crate::Resettable for GTDTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
