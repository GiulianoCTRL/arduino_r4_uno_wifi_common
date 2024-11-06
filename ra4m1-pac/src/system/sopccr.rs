#[doc = "Register `SOPCCR` reader"]
pub type R = crate::R<SOPCCR_SPEC>;
#[doc = "Register `SOPCCR` writer"]
pub type W = crate::W<SOPCCR_SPEC>;
#[doc = "Field `SOPCM` reader - Sub Operating Power Control Mode Select"]
pub type SOPCM_R = crate::BitReader<SOPCM_A>;
#[doc = "Sub Operating Power Control Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOPCM_A {
    #[doc = "0: Other than Subosc-speed mode"]
    _0 = 0,
    #[doc = "1: Subosc-speed mode"]
    _1 = 1,
}
impl From<SOPCM_A> for bool {
    #[inline(always)]
    fn from(variant: SOPCM_A) -> Self {
        variant as u8 != 0
    }
}
impl SOPCM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SOPCM_A {
        match self.bits {
            false => SOPCM_A::_0,
            true => SOPCM_A::_1,
        }
    }
    #[doc = "Other than Subosc-speed mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOPCM_A::_0
    }
    #[doc = "Subosc-speed mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOPCM_A::_1
    }
}
#[doc = "Field `SOPCM` writer - Sub Operating Power Control Mode Select"]
pub type SOPCM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SOPCM_A>;
impl<'a, REG, const O: u8> SOPCM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Other than Subosc-speed mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SOPCM_A::_0)
    }
    #[doc = "Subosc-speed mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SOPCM_A::_1)
    }
}
#[doc = "Field `SOPCMTSF` reader - Sub Operating Power Control Mode Transition Status Flag"]
pub type SOPCMTSF_R = crate::BitReader<SOPCMTSF_A>;
#[doc = "Sub Operating Power Control Mode Transition Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOPCMTSF_A {
    #[doc = "0: Transition completed"]
    _0 = 0,
    #[doc = "1: During transition"]
    _1 = 1,
}
impl From<SOPCMTSF_A> for bool {
    #[inline(always)]
    fn from(variant: SOPCMTSF_A) -> Self {
        variant as u8 != 0
    }
}
impl SOPCMTSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SOPCMTSF_A {
        match self.bits {
            false => SOPCMTSF_A::_0,
            true => SOPCMTSF_A::_1,
        }
    }
    #[doc = "Transition completed"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOPCMTSF_A::_0
    }
    #[doc = "During transition"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOPCMTSF_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Sub Operating Power Control Mode Select"]
    #[inline(always)]
    pub fn sopcm(&self) -> SOPCM_R {
        SOPCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Sub Operating Power Control Mode Transition Status Flag"]
    #[inline(always)]
    pub fn sopcmtsf(&self) -> SOPCMTSF_R {
        SOPCMTSF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sub Operating Power Control Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn sopcm(&mut self) -> SOPCM_W<SOPCCR_SPEC, 0> {
        SOPCM_W::new(self)
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
#[doc = "Sub Operating Power Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sopccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sopccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOPCCR_SPEC;
impl crate::RegisterSpec for SOPCCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sopccr::R`](R) reader structure"]
impl crate::Readable for SOPCCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sopccr::W`](W) writer structure"]
impl crate::Writable for SOPCCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOPCCR to value 0"]
impl crate::Resettable for SOPCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
