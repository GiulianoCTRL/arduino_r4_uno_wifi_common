#[doc = "Register `AGTCR` reader"]
pub type R = crate::R<AGTCR_SPEC>;
#[doc = "Register `AGTCR` writer"]
pub type W = crate::W<AGTCR_SPEC>;
#[doc = "Field `TSTART` reader - AGT count start"]
pub type TSTART_R = crate::BitReader<TSTART_A>;
#[doc = "AGT count start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSTART_A {
    #[doc = "0: Count stops"]
    _0 = 0,
    #[doc = "1: Count starts."]
    _1 = 1,
}
impl From<TSTART_A> for bool {
    #[inline(always)]
    fn from(variant: TSTART_A) -> Self {
        variant as u8 != 0
    }
}
impl TSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSTART_A {
        match self.bits {
            false => TSTART_A::_0,
            true => TSTART_A::_1,
        }
    }
    #[doc = "Count stops"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSTART_A::_0
    }
    #[doc = "Count starts."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSTART_A::_1
    }
}
#[doc = "Field `TSTART` writer - AGT count start"]
pub type TSTART_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TSTART_A>;
impl<'a, REG, const O: u8> TSTART_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Count stops"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TSTART_A::_0)
    }
    #[doc = "Count starts."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TSTART_A::_1)
    }
}
#[doc = "Field `TCSTF` reader - AGT count status flag"]
pub type TCSTF_R = crate::BitReader<TCSTF_A>;
#[doc = "AGT count status flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCSTF_A {
    #[doc = "0: Count stops"]
    _0 = 0,
    #[doc = "1: Count in progress."]
    _1 = 1,
}
impl From<TCSTF_A> for bool {
    #[inline(always)]
    fn from(variant: TCSTF_A) -> Self {
        variant as u8 != 0
    }
}
impl TCSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCSTF_A {
        match self.bits {
            false => TCSTF_A::_0,
            true => TCSTF_A::_1,
        }
    }
    #[doc = "Count stops"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCSTF_A::_0
    }
    #[doc = "Count in progress."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCSTF_A::_1
    }
}
#[doc = "AGT count forced stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSTOP_AW {
    #[doc = "0: Writing is invalid"]
    _0 = 0,
    #[doc = "1: The count is forcibly stopped."]
    _1 = 1,
}
impl From<TSTOP_AW> for bool {
    #[inline(always)]
    fn from(variant: TSTOP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTOP` writer - AGT count forced stop"]
pub type TSTOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TSTOP_AW>;
impl<'a, REG, const O: u8> TSTOP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing is invalid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TSTOP_AW::_0)
    }
    #[doc = "The count is forcibly stopped."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TSTOP_AW::_1)
    }
}
#[doc = "Field `TEDGF` reader - Active edge judgment flag\n\nThe field is **modified** in some way after a read operation."]
pub type TEDGF_R = crate::BitReader<TEDGF_A>;
#[doc = "Active edge judgment flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEDGF_A {
    #[doc = "0: No active edge received"]
    _0 = 0,
    #[doc = "1: Active edge received."]
    _1 = 1,
}
impl From<TEDGF_A> for bool {
    #[inline(always)]
    fn from(variant: TEDGF_A) -> Self {
        variant as u8 != 0
    }
}
impl TEDGF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEDGF_A {
        match self.bits {
            false => TEDGF_A::_0,
            true => TEDGF_A::_1,
        }
    }
    #[doc = "No active edge received"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TEDGF_A::_0
    }
    #[doc = "Active edge received."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TEDGF_A::_1
    }
}
#[doc = "Field `TEDGF` writer - Active edge judgment flag"]
pub type TEDGF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, TEDGF_A>;
impl<'a, REG, const O: u8> TEDGF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No active edge received"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TEDGF_A::_0)
    }
    #[doc = "Active edge received."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TEDGF_A::_1)
    }
}
#[doc = "Field `TUNDF` reader - Underflow flag\n\nThe field is **modified** in some way after a read operation."]
pub type TUNDF_R = crate::BitReader<TUNDF_A>;
#[doc = "Underflow flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TUNDF_A {
    #[doc = "0: No match"]
    _0 = 0,
    #[doc = "1: Match."]
    _1 = 1,
}
impl From<TUNDF_A> for bool {
    #[inline(always)]
    fn from(variant: TUNDF_A) -> Self {
        variant as u8 != 0
    }
}
impl TUNDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TUNDF_A {
        match self.bits {
            false => TUNDF_A::_0,
            true => TUNDF_A::_1,
        }
    }
    #[doc = "No match"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TUNDF_A::_0
    }
    #[doc = "Match."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TUNDF_A::_1
    }
}
#[doc = "Field `TUNDF` writer - Underflow flag"]
pub type TUNDF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, TUNDF_A>;
impl<'a, REG, const O: u8> TUNDF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No match"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TUNDF_A::_0)
    }
    #[doc = "Match."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TUNDF_A::_1)
    }
}
#[doc = "Field `TCMAF` reader - Compare match A flag\n\nThe field is **modified** in some way after a read operation."]
pub type TCMAF_R = crate::BitReader<TCMAF_A>;
#[doc = "Compare match A flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCMAF_A {
    #[doc = "0: No match"]
    _0 = 0,
    #[doc = "1: Match."]
    _1 = 1,
}
impl From<TCMAF_A> for bool {
    #[inline(always)]
    fn from(variant: TCMAF_A) -> Self {
        variant as u8 != 0
    }
}
impl TCMAF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCMAF_A {
        match self.bits {
            false => TCMAF_A::_0,
            true => TCMAF_A::_1,
        }
    }
    #[doc = "No match"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCMAF_A::_0
    }
    #[doc = "Match."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCMAF_A::_1
    }
}
#[doc = "Field `TCMAF` writer - Compare match A flag"]
pub type TCMAF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, TCMAF_A>;
impl<'a, REG, const O: u8> TCMAF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No match"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TCMAF_A::_0)
    }
    #[doc = "Match."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TCMAF_A::_1)
    }
}
#[doc = "Field `TCMBF` reader - Compare match B flag\n\nThe field is **modified** in some way after a read operation."]
pub type TCMBF_R = crate::BitReader<TCMBF_A>;
#[doc = "Compare match B flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCMBF_A {
    #[doc = "0: No match"]
    _0 = 0,
    #[doc = "1: Match."]
    _1 = 1,
}
impl From<TCMBF_A> for bool {
    #[inline(always)]
    fn from(variant: TCMBF_A) -> Self {
        variant as u8 != 0
    }
}
impl TCMBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCMBF_A {
        match self.bits {
            false => TCMBF_A::_0,
            true => TCMBF_A::_1,
        }
    }
    #[doc = "No match"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCMBF_A::_0
    }
    #[doc = "Match."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCMBF_A::_1
    }
}
#[doc = "Field `TCMBF` writer - Compare match B flag"]
pub type TCMBF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, TCMBF_A>;
impl<'a, REG, const O: u8> TCMBF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No match"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TCMBF_A::_0)
    }
    #[doc = "Match."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TCMBF_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AGT count start"]
    #[inline(always)]
    pub fn tstart(&self) -> TSTART_R {
        TSTART_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AGT count status flag"]
    #[inline(always)]
    pub fn tcstf(&self) -> TCSTF_R {
        TCSTF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Active edge judgment flag"]
    #[inline(always)]
    pub fn tedgf(&self) -> TEDGF_R {
        TEDGF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Underflow flag"]
    #[inline(always)]
    pub fn tundf(&self) -> TUNDF_R {
        TUNDF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Compare match A flag"]
    #[inline(always)]
    pub fn tcmaf(&self) -> TCMAF_R {
        TCMAF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Compare match B flag"]
    #[inline(always)]
    pub fn tcmbf(&self) -> TCMBF_R {
        TCMBF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AGT count start"]
    #[inline(always)]
    #[must_use]
    pub fn tstart(&mut self) -> TSTART_W<AGTCR_SPEC, 0> {
        TSTART_W::new(self)
    }
    #[doc = "Bit 2 - AGT count forced stop"]
    #[inline(always)]
    #[must_use]
    pub fn tstop(&mut self) -> TSTOP_W<AGTCR_SPEC, 2> {
        TSTOP_W::new(self)
    }
    #[doc = "Bit 4 - Active edge judgment flag"]
    #[inline(always)]
    #[must_use]
    pub fn tedgf(&mut self) -> TEDGF_W<AGTCR_SPEC, 4> {
        TEDGF_W::new(self)
    }
    #[doc = "Bit 5 - Underflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn tundf(&mut self) -> TUNDF_W<AGTCR_SPEC, 5> {
        TUNDF_W::new(self)
    }
    #[doc = "Bit 6 - Compare match A flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcmaf(&mut self) -> TCMAF_W<AGTCR_SPEC, 6> {
        TCMAF_W::new(self)
    }
    #[doc = "Bit 7 - Compare match B flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcmbf(&mut self) -> TCMBF_W<AGTCR_SPEC, 7> {
        TCMBF_W::new(self)
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
#[doc = "AGT Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`agtcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`agtcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AGTCR_SPEC;
impl crate::RegisterSpec for AGTCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`agtcr::R`](R) reader structure"]
impl crate::Readable for AGTCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`agtcr::W`](W) writer structure"]
impl crate::Writable for AGTCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xf0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AGTCR to value 0"]
impl crate::Resettable for AGTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
