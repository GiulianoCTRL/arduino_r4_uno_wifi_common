#[doc = "Register `SCKSCR` reader"]
pub type R = crate::R<SCKSCR_SPEC>;
#[doc = "Register `SCKSCR` writer"]
pub type W = crate::W<SCKSCR_SPEC>;
#[doc = "Field `CKSEL` reader - Clock Source Select Selecting the system clock source faster than 32MHz(system clock source > 32MHz ) is prohibit when SCKDIVCR.ICK\\[2:0\\]
bits select the division-by-1 and MEMWAIT.MEMWAIT =0."]
pub type CKSEL_R = crate::FieldReader<CKSEL_A>;
#[doc = "Clock Source Select Selecting the system clock source faster than 32MHz(system clock source > 32MHz ) is prohibit when SCKDIVCR.ICK\\[2:0\\]
bits select the division-by-1 and MEMWAIT.MEMWAIT =0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKSEL_A {
    #[doc = "0: HOCO"]
    _000 = 0,
    #[doc = "1: MOCO"]
    _001 = 1,
    #[doc = "2: LOCO"]
    _010 = 2,
    #[doc = "3: Main clock oscillator"]
    _011 = 3,
    #[doc = "4: Sub-clock oscillator"]
    _100 = 4,
    #[doc = "5: PLL"]
    _101 = 5,
}
impl From<CKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKSEL_A {
    type Ux = u8;
}
impl CKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CKSEL_A> {
        match self.bits {
            0 => Some(CKSEL_A::_000),
            1 => Some(CKSEL_A::_001),
            2 => Some(CKSEL_A::_010),
            3 => Some(CKSEL_A::_011),
            4 => Some(CKSEL_A::_100),
            5 => Some(CKSEL_A::_101),
            _ => None,
        }
    }
    #[doc = "HOCO"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CKSEL_A::_000
    }
    #[doc = "MOCO"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CKSEL_A::_001
    }
    #[doc = "LOCO"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CKSEL_A::_010
    }
    #[doc = "Main clock oscillator"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CKSEL_A::_011
    }
    #[doc = "Sub-clock oscillator"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CKSEL_A::_100
    }
    #[doc = "PLL"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == CKSEL_A::_101
    }
}
#[doc = "Field `CKSEL` writer - Clock Source Select Selecting the system clock source faster than 32MHz(system clock source > 32MHz ) is prohibit when SCKDIVCR.ICK\\[2:0\\]
bits select the division-by-1 and MEMWAIT.MEMWAIT =0."]
pub type CKSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, CKSEL_A>;
impl<'a, REG, const O: u8> CKSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HOCO"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(CKSEL_A::_000)
    }
    #[doc = "MOCO"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(CKSEL_A::_001)
    }
    #[doc = "LOCO"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(CKSEL_A::_010)
    }
    #[doc = "Main clock oscillator"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(CKSEL_A::_011)
    }
    #[doc = "Sub-clock oscillator"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(CKSEL_A::_100)
    }
    #[doc = "PLL"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(CKSEL_A::_101)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Source Select Selecting the system clock source faster than 32MHz(system clock source > 32MHz ) is prohibit when SCKDIVCR.ICK\\[2:0\\]
bits select the division-by-1 and MEMWAIT.MEMWAIT =0."]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Source Select Selecting the system clock source faster than 32MHz(system clock source > 32MHz ) is prohibit when SCKDIVCR.ICK\\[2:0\\]
bits select the division-by-1 and MEMWAIT.MEMWAIT =0."]
    #[inline(always)]
    #[must_use]
    pub fn cksel(&mut self) -> CKSEL_W<SCKSCR_SPEC, 0> {
        CKSEL_W::new(self)
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
#[doc = "System Clock Source Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sckscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sckscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCKSCR_SPEC;
impl crate::RegisterSpec for SCKSCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sckscr::R`](R) reader structure"]
impl crate::Readable for SCKSCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sckscr::W`](W) writer structure"]
impl crate::Writable for SCKSCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCKSCR to value 0x01"]
impl crate::Resettable for SCKSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
