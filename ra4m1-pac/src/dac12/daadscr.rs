#[doc = "Register `DAADSCR` reader"]
pub type R = crate::R<DAADSCR_SPEC>;
#[doc = "Register `DAADSCR` writer"]
pub type W = crate::W<DAADSCR_SPEC>;
#[doc = "Field `DAADST` reader - D/A-A/D Synchronous Conversion"]
pub type DAADST_R = crate::BitReader<DAADST_A>;
#[doc = "D/A-A/D Synchronous Conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAADST_A {
    #[doc = "0: D/A converter operation does not synchronize with A/D converter operation (unit 1) (countermeasure against interference between D/A and A/D conversions is disabled)."]
    _0 = 0,
    #[doc = "1: D/A converter operation synchronizes with A/D converter operation (unit 1) (countermeasure against interference between D/A and A/D conversions is enabled)."]
    _1 = 1,
}
impl From<DAADST_A> for bool {
    #[inline(always)]
    fn from(variant: DAADST_A) -> Self {
        variant as u8 != 0
    }
}
impl DAADST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DAADST_A {
        match self.bits {
            false => DAADST_A::_0,
            true => DAADST_A::_1,
        }
    }
    #[doc = "D/A converter operation does not synchronize with A/D converter operation (unit 1) (countermeasure against interference between D/A and A/D conversions is disabled)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAADST_A::_0
    }
    #[doc = "D/A converter operation synchronizes with A/D converter operation (unit 1) (countermeasure against interference between D/A and A/D conversions is enabled)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAADST_A::_1
    }
}
#[doc = "Field `DAADST` writer - D/A-A/D Synchronous Conversion"]
pub type DAADST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DAADST_A>;
impl<'a, REG, const O: u8> DAADST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "D/A converter operation does not synchronize with A/D converter operation (unit 1) (countermeasure against interference between D/A and A/D conversions is disabled)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DAADST_A::_0)
    }
    #[doc = "D/A converter operation synchronizes with A/D converter operation (unit 1) (countermeasure against interference between D/A and A/D conversions is enabled)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DAADST_A::_1)
    }
}
impl R {
    #[doc = "Bit 7 - D/A-A/D Synchronous Conversion"]
    #[inline(always)]
    pub fn daadst(&self) -> DAADST_R {
        DAADST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - D/A-A/D Synchronous Conversion"]
    #[inline(always)]
    #[must_use]
    pub fn daadst(&mut self) -> DAADST_W<DAADSCR_SPEC, 7> {
        DAADST_W::new(self)
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
#[doc = "D/A-A/D Synchronous Start Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daadscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daadscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAADSCR_SPEC;
impl crate::RegisterSpec for DAADSCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`daadscr::R`](R) reader structure"]
impl crate::Readable for DAADSCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`daadscr::W`](W) writer structure"]
impl crate::Writable for DAADSCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAADSCR to value 0"]
impl crate::Resettable for DAADSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
