#[doc = "Register `SOMCR` reader"]
pub type R = crate::R<SOMCR_SPEC>;
#[doc = "Register `SOMCR` writer"]
pub type W = crate::W<SOMCR_SPEC>;
#[doc = "Field `SODRV` reader - Sub-Clock Oscillator Drive Capability Switching"]
pub type SODRV_R = crate::FieldReader<SODRV_A>;
#[doc = "Sub-Clock Oscillator Drive Capability Switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SODRV_A {
    #[doc = "0: Normal mode"]
    _00 = 0,
    #[doc = "1: Low power mode 1"]
    _01 = 1,
    #[doc = "2: Low power mode 2"]
    _10 = 2,
    #[doc = "3: Low power mode 3."]
    _11 = 3,
}
impl From<SODRV_A> for u8 {
    #[inline(always)]
    fn from(variant: SODRV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SODRV_A {
    type Ux = u8;
}
impl SODRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SODRV_A {
        match self.bits {
            0 => SODRV_A::_00,
            1 => SODRV_A::_01,
            2 => SODRV_A::_10,
            3 => SODRV_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SODRV_A::_00
    }
    #[doc = "Low power mode 1"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SODRV_A::_01
    }
    #[doc = "Low power mode 2"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SODRV_A::_10
    }
    #[doc = "Low power mode 3."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SODRV_A::_11
    }
}
#[doc = "Field `SODRV` writer - Sub-Clock Oscillator Drive Capability Switching"]
pub type SODRV_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, SODRV_A>;
impl<'a, REG, const O: u8> SODRV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(SODRV_A::_00)
    }
    #[doc = "Low power mode 1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(SODRV_A::_01)
    }
    #[doc = "Low power mode 2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(SODRV_A::_10)
    }
    #[doc = "Low power mode 3."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(SODRV_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Sub-Clock Oscillator Drive Capability Switching"]
    #[inline(always)]
    pub fn sodrv(&self) -> SODRV_R {
        SODRV_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sub-Clock Oscillator Drive Capability Switching"]
    #[inline(always)]
    #[must_use]
    pub fn sodrv(&mut self) -> SODRV_W<SOMCR_SPEC, 0> {
        SODRV_W::new(self)
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
#[doc = "Sub Clock Oscillator Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`somcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`somcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOMCR_SPEC;
impl crate::RegisterSpec for SOMCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`somcr::R`](R) reader structure"]
impl crate::Readable for SOMCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`somcr::W`](W) writer structure"]
impl crate::Writable for SOMCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOMCR to value 0"]
impl crate::Resettable for SOMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
