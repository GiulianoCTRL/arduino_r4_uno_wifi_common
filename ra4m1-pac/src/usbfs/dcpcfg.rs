#[doc = "Register `DCPCFG` reader"]
pub type R = crate::R<DCPCFG_SPEC>;
#[doc = "Register `DCPCFG` writer"]
pub type W = crate::W<DCPCFG_SPEC>;
#[doc = "Field `DIR` reader - Transfer Direction"]
pub type DIR_R = crate::BitReader<DIR_A>;
#[doc = "Transfer Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR_A {
    #[doc = "0: Data receiving direction"]
    _0 = 0,
    #[doc = "1: Data transmitting direction"]
    _1 = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
impl DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::_0,
            true => DIR_A::_1,
        }
    }
    #[doc = "Data receiving direction"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIR_A::_0
    }
    #[doc = "Data transmitting direction"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIR_A::_1
    }
}
#[doc = "Field `DIR` writer - Transfer Direction"]
pub type DIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DIR_A>;
impl<'a, REG, const O: u8> DIR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data receiving direction"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIR_A::_0)
    }
    #[doc = "Data transmitting direction"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIR_A::_1)
    }
}
#[doc = "Field `SHTNAK` reader - Pipe Disabled at End of Transfer"]
pub type SHTNAK_R = crate::BitReader<SHTNAK_A>;
#[doc = "Pipe Disabled at End of Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHTNAK_A {
    #[doc = "0: Pipe continued at the end of transfer"]
    _0 = 0,
    #[doc = "1: Pipe disabled at the end of transfer"]
    _1 = 1,
}
impl From<SHTNAK_A> for bool {
    #[inline(always)]
    fn from(variant: SHTNAK_A) -> Self {
        variant as u8 != 0
    }
}
impl SHTNAK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SHTNAK_A {
        match self.bits {
            false => SHTNAK_A::_0,
            true => SHTNAK_A::_1,
        }
    }
    #[doc = "Pipe continued at the end of transfer"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHTNAK_A::_0
    }
    #[doc = "Pipe disabled at the end of transfer"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHTNAK_A::_1
    }
}
#[doc = "Field `SHTNAK` writer - Pipe Disabled at End of Transfer"]
pub type SHTNAK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SHTNAK_A>;
impl<'a, REG, const O: u8> SHTNAK_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pipe continued at the end of transfer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SHTNAK_A::_0)
    }
    #[doc = "Pipe disabled at the end of transfer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SHTNAK_A::_1)
    }
}
impl R {
    #[doc = "Bit 4 - Transfer Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Pipe Disabled at End of Transfer"]
    #[inline(always)]
    pub fn shtnak(&self) -> SHTNAK_R {
        SHTNAK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Transfer Direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<DCPCFG_SPEC, 4> {
        DIR_W::new(self)
    }
    #[doc = "Bit 7 - Pipe Disabled at End of Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn shtnak(&mut self) -> SHTNAK_W<DCPCFG_SPEC, 7> {
        SHTNAK_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DCP Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcpcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcpcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCPCFG_SPEC;
impl crate::RegisterSpec for DCPCFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dcpcfg::R`](R) reader structure"]
impl crate::Readable for DCPCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcpcfg::W`](W) writer structure"]
impl crate::Writable for DCPCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCPCFG to value 0"]
impl crate::Resettable for DCPCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
