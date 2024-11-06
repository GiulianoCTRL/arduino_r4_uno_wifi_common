#[doc = "Register `GTCLR` writer"]
pub type W = crate::W<GTCLR_SPEC>;
#[doc = "Channel 0 GTCNT Count Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR0_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: GPT320.GTCNT counter clears"]
    _1 = 1,
}
impl From<CCLR0_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR0` writer - Channel 0 GTCNT Count Clear"]
pub type CCLR0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CCLR0_AW>;
impl<'a, REG, const O: u8> CCLR0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR0_AW::_0)
    }
    #[doc = "GPT320.GTCNT counter clears"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR0_AW::_1)
    }
}
#[doc = "Channel 1 GTCNT Count Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR1_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: GPT321.GTCNT counter clears"]
    _1 = 1,
}
impl From<CCLR1_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR1` writer - Channel 1 GTCNT Count Clear"]
pub type CCLR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CCLR1_AW>;
impl<'a, REG, const O: u8> CCLR1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR1_AW::_0)
    }
    #[doc = "GPT321.GTCNT counter clears"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR1_AW::_1)
    }
}
#[doc = "Channel 2 GTCNT Count Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR2_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: GPT322.GTCNT counter clears"]
    _1 = 1,
}
impl From<CCLR2_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR2` writer - Channel 2 GTCNT Count Clear"]
pub type CCLR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CCLR2_AW>;
impl<'a, REG, const O: u8> CCLR2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR2_AW::_0)
    }
    #[doc = "GPT322.GTCNT counter clears"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR2_AW::_1)
    }
}
#[doc = "Channel 3 GTCNT Count Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR3_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: GPT323.GTCNT counter clears"]
    _1 = 1,
}
impl From<CCLR3_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR3` writer - Channel 3 GTCNT Count Clear"]
pub type CCLR3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CCLR3_AW>;
impl<'a, REG, const O: u8> CCLR3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR3_AW::_0)
    }
    #[doc = "GPT323.GTCNT counter clears"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR3_AW::_1)
    }
}
#[doc = "Channel 4 GTCNT Count Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR4_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: GPT164.GTCNT counter clears"]
    _1 = 1,
}
impl From<CCLR4_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR4` writer - Channel 4 GTCNT Count Clear"]
pub type CCLR4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CCLR4_AW>;
impl<'a, REG, const O: u8> CCLR4_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR4_AW::_0)
    }
    #[doc = "GPT164.GTCNT counter clears"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR4_AW::_1)
    }
}
#[doc = "Channel 5 GTCNT Count Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR5_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: GPT165.GTCNT counter clears"]
    _1 = 1,
}
impl From<CCLR5_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR5` writer - Channel 5 GTCNT Count Clear"]
pub type CCLR5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CCLR5_AW>;
impl<'a, REG, const O: u8> CCLR5_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR5_AW::_0)
    }
    #[doc = "GPT165.GTCNT counter clears"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR5_AW::_1)
    }
}
#[doc = "Channel 6 GTCNT Count Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR6_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: GPT166.GTCNT counter clears"]
    _1 = 1,
}
impl From<CCLR6_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR6` writer - Channel 6 GTCNT Count Clear"]
pub type CCLR6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CCLR6_AW>;
impl<'a, REG, const O: u8> CCLR6_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR6_AW::_0)
    }
    #[doc = "GPT166.GTCNT counter clears"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR6_AW::_1)
    }
}
#[doc = "Channel 7 GTCNT Count Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR7_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: GPT167.GTCNT counter clears"]
    _1 = 1,
}
impl From<CCLR7_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLR7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLR7` writer - Channel 7 GTCNT Count Clear"]
pub type CCLR7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CCLR7_AW>;
impl<'a, REG, const O: u8> CCLR7_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR7_AW::_0)
    }
    #[doc = "GPT167.GTCNT counter clears"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR7_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 GTCNT Count Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cclr0(&mut self) -> CCLR0_W<GTCLR_SPEC, 0> {
        CCLR0_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 GTCNT Count Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cclr1(&mut self) -> CCLR1_W<GTCLR_SPEC, 1> {
        CCLR1_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 GTCNT Count Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cclr2(&mut self) -> CCLR2_W<GTCLR_SPEC, 2> {
        CCLR2_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 GTCNT Count Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cclr3(&mut self) -> CCLR3_W<GTCLR_SPEC, 3> {
        CCLR3_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 GTCNT Count Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cclr4(&mut self) -> CCLR4_W<GTCLR_SPEC, 4> {
        CCLR4_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 GTCNT Count Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cclr5(&mut self) -> CCLR5_W<GTCLR_SPEC, 5> {
        CCLR5_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 GTCNT Count Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cclr6(&mut self) -> CCLR6_W<GTCLR_SPEC, 6> {
        CCLR6_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 GTCNT Count Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cclr7(&mut self) -> CCLR7_W<GTCLR_SPEC, 7> {
        CCLR7_W::new(self)
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
#[doc = "General PWM Timer Software Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTCLR_SPEC;
impl crate::RegisterSpec for GTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gtclr::W`](W) writer structure"]
impl crate::Writable for GTCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTCLR to value 0"]
impl crate::Resettable for GTCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
