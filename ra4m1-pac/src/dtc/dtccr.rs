#[doc = "Register `DTCCR` reader"]
pub type R = crate::R<DTCCR_SPEC>;
#[doc = "Register `DTCCR` writer"]
pub type W = crate::W<DTCCR_SPEC>;
#[doc = "Field `RRS` reader - DTC Transfer Information Read Skip Enable."]
pub type RRS_R = crate::BitReader<RRS_A>;
#[doc = "DTC Transfer Information Read Skip Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRS_A {
    #[doc = "0: Do not skip transfer information read"]
    _0 = 0,
    #[doc = "1: Skip transfer information read when vector numbers match"]
    _1 = 1,
}
impl From<RRS_A> for bool {
    #[inline(always)]
    fn from(variant: RRS_A) -> Self {
        variant as u8 != 0
    }
}
impl RRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RRS_A {
        match self.bits {
            false => RRS_A::_0,
            true => RRS_A::_1,
        }
    }
    #[doc = "Do not skip transfer information read"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RRS_A::_0
    }
    #[doc = "Skip transfer information read when vector numbers match"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RRS_A::_1
    }
}
#[doc = "Field `RRS` writer - DTC Transfer Information Read Skip Enable."]
pub type RRS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RRS_A>;
impl<'a, REG, const O: u8> RRS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not skip transfer information read"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RRS_A::_0)
    }
    #[doc = "Skip transfer information read when vector numbers match"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RRS_A::_1)
    }
}
impl R {
    #[doc = "Bit 4 - DTC Transfer Information Read Skip Enable."]
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - DTC Transfer Information Read Skip Enable."]
    #[inline(always)]
    #[must_use]
    pub fn rrs(&mut self) -> RRS_W<DTCCR_SPEC, 4> {
        RRS_W::new(self)
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
#[doc = "DTC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTCCR_SPEC;
impl crate::RegisterSpec for DTCCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dtccr::R`](R) reader structure"]
impl crate::Readable for DTCCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dtccr::W`](W) writer structure"]
impl crate::Writable for DTCCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTCCR to value 0x08"]
impl crate::Resettable for DTCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
