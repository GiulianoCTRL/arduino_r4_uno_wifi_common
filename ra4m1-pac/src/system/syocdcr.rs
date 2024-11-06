#[doc = "Register `SYOCDCR` reader"]
pub type R = crate::R<SYOCDCR_SPEC>;
#[doc = "Register `SYOCDCR` writer"]
pub type W = crate::W<SYOCDCR_SPEC>;
#[doc = "Field `DBGEN` reader - Debugger Enable bit"]
pub type DBGEN_R = crate::BitReader<DBGEN_A>;
#[doc = "Debugger Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGEN_A {
    #[doc = "0: On-chip debugger is disabled"]
    _0 = 0,
    #[doc = "1: On-chip debugger is enabled"]
    _1 = 1,
}
impl From<DBGEN_A> for bool {
    #[inline(always)]
    fn from(variant: DBGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DBGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBGEN_A {
        match self.bits {
            false => DBGEN_A::_0,
            true => DBGEN_A::_1,
        }
    }
    #[doc = "On-chip debugger is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBGEN_A::_0
    }
    #[doc = "On-chip debugger is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBGEN_A::_1
    }
}
#[doc = "Field `DBGEN` writer - Debugger Enable bit"]
pub type DBGEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DBGEN_A>;
impl<'a, REG, const O: u8> DBGEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "On-chip debugger is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DBGEN_A::_0)
    }
    #[doc = "On-chip debugger is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DBGEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 7 - Debugger Enable bit"]
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Debugger Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn dbgen(&mut self) -> DBGEN_W<SYOCDCR_SPEC, 7> {
        DBGEN_W::new(self)
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
#[doc = "System Control OCD Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syocdcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syocdcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYOCDCR_SPEC;
impl crate::RegisterSpec for SYOCDCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`syocdcr::R`](R) reader structure"]
impl crate::Readable for SYOCDCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`syocdcr::W`](W) writer structure"]
impl crate::Writable for SYOCDCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYOCDCR to value 0"]
impl crate::Resettable for SYOCDCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
