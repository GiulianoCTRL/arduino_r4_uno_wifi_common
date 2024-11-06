#[doc = "Register `CAICR` reader"]
pub type R = crate::R<CAICR_SPEC>;
#[doc = "Register `CAICR` writer"]
pub type W = crate::W<CAICR_SPEC>;
#[doc = "Field `FERRIE` reader - Frequency Error Interrupt Request Enable"]
pub type FERRIE_R = crate::BitReader<FERRIE_A>;
#[doc = "Frequency Error Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FERRIE_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<FERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: FERRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl FERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FERRIE_A {
        match self.bits {
            false => FERRIE_A::_0,
            true => FERRIE_A::_1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FERRIE_A::_0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FERRIE_A::_1
    }
}
#[doc = "Field `FERRIE` writer - Frequency Error Interrupt Request Enable"]
pub type FERRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FERRIE_A>;
impl<'a, REG, const O: u8> FERRIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FERRIE_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FERRIE_A::_1)
    }
}
#[doc = "Field `MENDIE` reader - Measurement End Interrupt Request Enable"]
pub type MENDIE_R = crate::BitReader<MENDIE_A>;
#[doc = "Measurement End Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MENDIE_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<MENDIE_A> for bool {
    #[inline(always)]
    fn from(variant: MENDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl MENDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MENDIE_A {
        match self.bits {
            false => MENDIE_A::_0,
            true => MENDIE_A::_1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MENDIE_A::_0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MENDIE_A::_1
    }
}
#[doc = "Field `MENDIE` writer - Measurement End Interrupt Request Enable"]
pub type MENDIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MENDIE_A>;
impl<'a, REG, const O: u8> MENDIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MENDIE_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MENDIE_A::_1)
    }
}
#[doc = "Field `OVFIE` reader - Overflow Interrupt Request Enable"]
pub type OVFIE_R = crate::BitReader<OVFIE_A>;
#[doc = "Overflow Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFIE_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<OVFIE_A> for bool {
    #[inline(always)]
    fn from(variant: OVFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl OVFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVFIE_A {
        match self.bits {
            false => OVFIE_A::_0,
            true => OVFIE_A::_1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVFIE_A::_0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVFIE_A::_1
    }
}
#[doc = "Field `OVFIE` writer - Overflow Interrupt Request Enable"]
pub type OVFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OVFIE_A>;
impl<'a, REG, const O: u8> OVFIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OVFIE_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OVFIE_A::_1)
    }
}
#[doc = "FERRF Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FERRFCL_AW {
    #[doc = "0: No effect on operations"]
    _0 = 0,
    #[doc = "1: Clears the FERRF flag"]
    _1 = 1,
}
impl From<FERRFCL_AW> for bool {
    #[inline(always)]
    fn from(variant: FERRFCL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FERRFCL` writer - FERRF Clear"]
pub type FERRFCL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FERRFCL_AW>;
impl<'a, REG, const O: u8> FERRFCL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on operations"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FERRFCL_AW::_0)
    }
    #[doc = "Clears the FERRF flag"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FERRFCL_AW::_1)
    }
}
#[doc = "MENDF Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MENDFCL_AW {
    #[doc = "0: No effect on operations"]
    _0 = 0,
    #[doc = "1: Clears the MENDF flag"]
    _1 = 1,
}
impl From<MENDFCL_AW> for bool {
    #[inline(always)]
    fn from(variant: MENDFCL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MENDFCL` writer - MENDF Clear"]
pub type MENDFCL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MENDFCL_AW>;
impl<'a, REG, const O: u8> MENDFCL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on operations"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MENDFCL_AW::_0)
    }
    #[doc = "Clears the MENDF flag"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MENDFCL_AW::_1)
    }
}
#[doc = "OVFF Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFFCL_AW {
    #[doc = "0: No effect on operations"]
    _0 = 0,
    #[doc = "1: Clears the OVFF flag"]
    _1 = 1,
}
impl From<OVFFCL_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFFCL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFFCL` writer - OVFF Clear"]
pub type OVFFCL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OVFFCL_AW>;
impl<'a, REG, const O: u8> OVFFCL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on operations"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OVFFCL_AW::_0)
    }
    #[doc = "Clears the OVFF flag"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OVFFCL_AW::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Frequency Error Interrupt Request Enable"]
    #[inline(always)]
    pub fn ferrie(&self) -> FERRIE_R {
        FERRIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Measurement End Interrupt Request Enable"]
    #[inline(always)]
    pub fn mendie(&self) -> MENDIE_R {
        MENDIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow Interrupt Request Enable"]
    #[inline(always)]
    pub fn ovfie(&self) -> OVFIE_R {
        OVFIE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frequency Error Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ferrie(&mut self) -> FERRIE_W<CAICR_SPEC, 0> {
        FERRIE_W::new(self)
    }
    #[doc = "Bit 1 - Measurement End Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mendie(&mut self) -> MENDIE_W<CAICR_SPEC, 1> {
        MENDIE_W::new(self)
    }
    #[doc = "Bit 2 - Overflow Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovfie(&mut self) -> OVFIE_W<CAICR_SPEC, 2> {
        OVFIE_W::new(self)
    }
    #[doc = "Bit 4 - FERRF Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ferrfcl(&mut self) -> FERRFCL_W<CAICR_SPEC, 4> {
        FERRFCL_W::new(self)
    }
    #[doc = "Bit 5 - MENDF Clear"]
    #[inline(always)]
    #[must_use]
    pub fn mendfcl(&mut self) -> MENDFCL_W<CAICR_SPEC, 5> {
        MENDFCL_W::new(self)
    }
    #[doc = "Bit 6 - OVFF Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovffcl(&mut self) -> OVFFCL_W<CAICR_SPEC, 6> {
        OVFFCL_W::new(self)
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
#[doc = "CAC Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`caicr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`caicr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAICR_SPEC;
impl crate::RegisterSpec for CAICR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`caicr::R`](R) reader structure"]
impl crate::Readable for CAICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`caicr::W`](W) writer structure"]
impl crate::Writable for CAICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAICR to value 0"]
impl crate::Resettable for CAICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
