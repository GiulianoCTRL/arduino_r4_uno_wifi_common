#[doc = "Register `DAM` reader"]
pub type R = crate::R<DAM_SPEC>;
#[doc = "Register `DAM` writer"]
pub type W = crate::W<DAM_SPEC>;
#[doc = "Field `DACE0` reader - D/A Operation Enable 0"]
pub type DACE0_R = crate::BitReader<DACE0_A>;
#[doc = "D/A Operation Enable 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACE0_A {
    #[doc = "0: D/A conversion disabled for channel 0"]
    _0 = 0,
    #[doc = "1: D/A conversion enabled for channel 0."]
    _1 = 1,
}
impl From<DACE0_A> for bool {
    #[inline(always)]
    fn from(variant: DACE0_A) -> Self {
        variant as u8 != 0
    }
}
impl DACE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DACE0_A {
        match self.bits {
            false => DACE0_A::_0,
            true => DACE0_A::_1,
        }
    }
    #[doc = "D/A conversion disabled for channel 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACE0_A::_0
    }
    #[doc = "D/A conversion enabled for channel 0."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACE0_A::_1
    }
}
#[doc = "Field `DACE0` writer - D/A Operation Enable 0"]
pub type DACE0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DACE0_A>;
impl<'a, REG, const O: u8> DACE0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "D/A conversion disabled for channel 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DACE0_A::_0)
    }
    #[doc = "D/A conversion enabled for channel 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DACE0_A::_1)
    }
}
#[doc = "Field `DACE1` reader - D/A Operation Enable 1"]
pub type DACE1_R = crate::BitReader<DACE1_A>;
#[doc = "D/A Operation Enable 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACE1_A {
    #[doc = "0: D/A conversion disabled for channel 1"]
    _0 = 0,
    #[doc = "1: D/A conversion enabled for channel 1"]
    _1 = 1,
}
impl From<DACE1_A> for bool {
    #[inline(always)]
    fn from(variant: DACE1_A) -> Self {
        variant as u8 != 0
    }
}
impl DACE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DACE1_A {
        match self.bits {
            false => DACE1_A::_0,
            true => DACE1_A::_1,
        }
    }
    #[doc = "D/A conversion disabled for channel 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACE1_A::_0
    }
    #[doc = "D/A conversion enabled for channel 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACE1_A::_1
    }
}
#[doc = "Field `DACE1` writer - D/A Operation Enable 1"]
pub type DACE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DACE1_A>;
impl<'a, REG, const O: u8> DACE1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "D/A conversion disabled for channel 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DACE1_A::_0)
    }
    #[doc = "D/A conversion enabled for channel 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DACE1_A::_1)
    }
}
impl R {
    #[doc = "Bit 4 - D/A Operation Enable 0"]
    #[inline(always)]
    pub fn dace0(&self) -> DACE0_R {
        DACE0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - D/A Operation Enable 1"]
    #[inline(always)]
    pub fn dace1(&self) -> DACE1_R {
        DACE1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - D/A Operation Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn dace0(&mut self) -> DACE0_W<DAM_SPEC, 4> {
        DACE0_W::new(self)
    }
    #[doc = "Bit 5 - D/A Operation Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn dace1(&mut self) -> DACE1_W<DAM_SPEC, 5> {
        DACE1_W::new(self)
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
#[doc = "D/A Converter Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dam::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dam::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAM_SPEC;
impl crate::RegisterSpec for DAM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dam::R`](R) reader structure"]
impl crate::Readable for DAM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dam::W`](W) writer structure"]
impl crate::Writable for DAM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAM to value 0"]
impl crate::Resettable for DAM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
