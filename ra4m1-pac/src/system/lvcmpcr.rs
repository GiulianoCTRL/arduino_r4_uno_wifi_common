#[doc = "Register `LVCMPCR` reader"]
pub type R = crate::R<LVCMPCR_SPEC>;
#[doc = "Register `LVCMPCR` writer"]
pub type W = crate::W<LVCMPCR_SPEC>;
#[doc = "Field `LVD1E` reader - Voltage Detection 1 Enable"]
pub type LVD1E_R = crate::BitReader<LVD1E_A>;
#[doc = "Voltage Detection 1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD1E_A {
    #[doc = "0: Voltage detection 1 circuit disabled"]
    _0 = 0,
    #[doc = "1: Voltage detection 1 circuit enabled"]
    _1 = 1,
}
impl From<LVD1E_A> for bool {
    #[inline(always)]
    fn from(variant: LVD1E_A) -> Self {
        variant as u8 != 0
    }
}
impl LVD1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LVD1E_A {
        match self.bits {
            false => LVD1E_A::_0,
            true => LVD1E_A::_1,
        }
    }
    #[doc = "Voltage detection 1 circuit disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD1E_A::_0
    }
    #[doc = "Voltage detection 1 circuit enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD1E_A::_1
    }
}
#[doc = "Field `LVD1E` writer - Voltage Detection 1 Enable"]
pub type LVD1E_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LVD1E_A>;
impl<'a, REG, const O: u8> LVD1E_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Voltage detection 1 circuit disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1E_A::_0)
    }
    #[doc = "Voltage detection 1 circuit enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1E_A::_1)
    }
}
#[doc = "Field `LVD2E` reader - Voltage Detection 2 Enable"]
pub type LVD2E_R = crate::BitReader<LVD2E_A>;
#[doc = "Voltage Detection 2 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD2E_A {
    #[doc = "0: Voltage detection 2 circuit disabled"]
    _0 = 0,
    #[doc = "1: Voltage detection 2 circuit enabled"]
    _1 = 1,
}
impl From<LVD2E_A> for bool {
    #[inline(always)]
    fn from(variant: LVD2E_A) -> Self {
        variant as u8 != 0
    }
}
impl LVD2E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LVD2E_A {
        match self.bits {
            false => LVD2E_A::_0,
            true => LVD2E_A::_1,
        }
    }
    #[doc = "Voltage detection 2 circuit disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD2E_A::_0
    }
    #[doc = "Voltage detection 2 circuit enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD2E_A::_1
    }
}
#[doc = "Field `LVD2E` writer - Voltage Detection 2 Enable"]
pub type LVD2E_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LVD2E_A>;
impl<'a, REG, const O: u8> LVD2E_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Voltage detection 2 circuit disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LVD2E_A::_0)
    }
    #[doc = "Voltage detection 2 circuit enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LVD2E_A::_1)
    }
}
impl R {
    #[doc = "Bit 5 - Voltage Detection 1 Enable"]
    #[inline(always)]
    pub fn lvd1e(&self) -> LVD1E_R {
        LVD1E_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Voltage Detection 2 Enable"]
    #[inline(always)]
    pub fn lvd2e(&self) -> LVD2E_R {
        LVD2E_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Voltage Detection 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvd1e(&mut self) -> LVD1E_W<LVCMPCR_SPEC, 5> {
        LVD1E_W::new(self)
    }
    #[doc = "Bit 6 - Voltage Detection 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvd2e(&mut self) -> LVD2E_W<LVCMPCR_SPEC, 6> {
        LVD2E_W::new(self)
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
#[doc = "Voltage Monitor Circuit Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lvcmpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lvcmpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LVCMPCR_SPEC;
impl crate::RegisterSpec for LVCMPCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lvcmpcr::R`](R) reader structure"]
impl crate::Readable for LVCMPCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lvcmpcr::W`](W) writer structure"]
impl crate::Writable for LVCMPCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LVCMPCR to value 0"]
impl crate::Resettable for LVCMPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
