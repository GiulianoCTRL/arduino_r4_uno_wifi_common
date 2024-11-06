#[doc = "Register `OPCCR` reader"]
pub type R = crate::R<OPCCR_SPEC>;
#[doc = "Register `OPCCR` writer"]
pub type W = crate::W<OPCCR_SPEC>;
#[doc = "Field `OPCM` reader - Operating Power Control Mode Select"]
pub type OPCM_R = crate::FieldReader<OPCM_A>;
#[doc = "Operating Power Control Mode Select\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPCM_A {
    #[doc = "0: High-speed mode"]
    _00 = 0,
    #[doc = "1: Middle-speed mode"]
    _01 = 1,
    #[doc = "2: Low-voltage mode"]
    _10 = 2,
    #[doc = "3: Low-speed mode"]
    _11 = 3,
}
impl From<OPCM_A> for u8 {
    #[inline(always)]
    fn from(variant: OPCM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OPCM_A {
    type Ux = u8;
}
impl OPCM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OPCM_A {
        match self.bits {
            0 => OPCM_A::_00,
            1 => OPCM_A::_01,
            2 => OPCM_A::_10,
            3 => OPCM_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "High-speed mode"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OPCM_A::_00
    }
    #[doc = "Middle-speed mode"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OPCM_A::_01
    }
    #[doc = "Low-voltage mode"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OPCM_A::_10
    }
    #[doc = "Low-speed mode"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == OPCM_A::_11
    }
}
#[doc = "Field `OPCM` writer - Operating Power Control Mode Select"]
pub type OPCM_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, OPCM_A>;
impl<'a, REG, const O: u8> OPCM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "High-speed mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(OPCM_A::_00)
    }
    #[doc = "Middle-speed mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(OPCM_A::_01)
    }
    #[doc = "Low-voltage mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(OPCM_A::_10)
    }
    #[doc = "Low-speed mode"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(OPCM_A::_11)
    }
}
#[doc = "Field `OPCMTSF` reader - Operating Power Control Mode Transition Status Flag"]
pub type OPCMTSF_R = crate::BitReader<OPCMTSF_A>;
#[doc = "Operating Power Control Mode Transition Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPCMTSF_A {
    #[doc = "0: Transition completed"]
    _0 = 0,
    #[doc = "1: During transition"]
    _1 = 1,
}
impl From<OPCMTSF_A> for bool {
    #[inline(always)]
    fn from(variant: OPCMTSF_A) -> Self {
        variant as u8 != 0
    }
}
impl OPCMTSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OPCMTSF_A {
        match self.bits {
            false => OPCMTSF_A::_0,
            true => OPCMTSF_A::_1,
        }
    }
    #[doc = "Transition completed"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OPCMTSF_A::_0
    }
    #[doc = "During transition"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OPCMTSF_A::_1
    }
}
impl R {
    #[doc = "Bits 0:1 - Operating Power Control Mode Select"]
    #[inline(always)]
    pub fn opcm(&self) -> OPCM_R {
        OPCM_R::new(self.bits & 3)
    }
    #[doc = "Bit 4 - Operating Power Control Mode Transition Status Flag"]
    #[inline(always)]
    pub fn opcmtsf(&self) -> OPCMTSF_R {
        OPCMTSF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Operating Power Control Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn opcm(&mut self) -> OPCM_W<OPCCR_SPEC, 0> {
        OPCM_W::new(self)
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
#[doc = "Operating Power Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPCCR_SPEC;
impl crate::RegisterSpec for OPCCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`opccr::R`](R) reader structure"]
impl crate::Readable for OPCCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`opccr::W`](W) writer structure"]
impl crate::Writable for OPCCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPCCR to value 0x02"]
impl crate::Resettable for OPCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
