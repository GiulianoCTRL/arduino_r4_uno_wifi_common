#[doc = "Register `PWPR` reader"]
pub type R = crate::R<PWPR_SPEC>;
#[doc = "Register `PWPR` writer"]
pub type W = crate::W<PWPR_SPEC>;
#[doc = "Field `PFSWE` reader - PFS Register Write Enable"]
pub type PFSWE_R = crate::BitReader<PFSWE_A>;
#[doc = "PFS Register Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFSWE_A {
    #[doc = "0: Writing to the PFS register is disabled"]
    _0 = 0,
    #[doc = "1: Writing to the PFS register is enabled"]
    _1 = 1,
}
impl From<PFSWE_A> for bool {
    #[inline(always)]
    fn from(variant: PFSWE_A) -> Self {
        variant as u8 != 0
    }
}
impl PFSWE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PFSWE_A {
        match self.bits {
            false => PFSWE_A::_0,
            true => PFSWE_A::_1,
        }
    }
    #[doc = "Writing to the PFS register is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PFSWE_A::_0
    }
    #[doc = "Writing to the PFS register is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PFSWE_A::_1
    }
}
#[doc = "Field `PFSWE` writer - PFS Register Write Enable"]
pub type PFSWE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PFSWE_A>;
impl<'a, REG, const O: u8> PFSWE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing to the PFS register is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PFSWE_A::_0)
    }
    #[doc = "Writing to the PFS register is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PFSWE_A::_1)
    }
}
#[doc = "Field `B0WI` reader - PFSWE Bit Write Disable"]
pub type B0WI_R = crate::BitReader<B0WI_A>;
#[doc = "PFSWE Bit Write Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B0WI_A {
    #[doc = "0: Writing to the PFSWE bit is enabled"]
    _0 = 0,
    #[doc = "1: Writing to the PFSWE bit is disabled"]
    _1 = 1,
}
impl From<B0WI_A> for bool {
    #[inline(always)]
    fn from(variant: B0WI_A) -> Self {
        variant as u8 != 0
    }
}
impl B0WI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> B0WI_A {
        match self.bits {
            false => B0WI_A::_0,
            true => B0WI_A::_1,
        }
    }
    #[doc = "Writing to the PFSWE bit is enabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B0WI_A::_0
    }
    #[doc = "Writing to the PFSWE bit is disabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B0WI_A::_1
    }
}
#[doc = "Field `B0WI` writer - PFSWE Bit Write Disable"]
pub type B0WI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, B0WI_A>;
impl<'a, REG, const O: u8> B0WI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing to the PFSWE bit is enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(B0WI_A::_0)
    }
    #[doc = "Writing to the PFSWE bit is disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(B0WI_A::_1)
    }
}
impl R {
    #[doc = "Bit 6 - PFS Register Write Enable"]
    #[inline(always)]
    pub fn pfswe(&self) -> PFSWE_R {
        PFSWE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PFSWE Bit Write Disable"]
    #[inline(always)]
    pub fn b0wi(&self) -> B0WI_R {
        B0WI_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - PFS Register Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pfswe(&mut self) -> PFSWE_W<PWPR_SPEC, 6> {
        PFSWE_W::new(self)
    }
    #[doc = "Bit 7 - PFSWE Bit Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn b0wi(&mut self) -> B0WI_W<PWPR_SPEC, 7> {
        B0WI_W::new(self)
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
#[doc = "Write-Protect Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWPR_SPEC;
impl crate::RegisterSpec for PWPR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pwpr::R`](R) reader structure"]
impl crate::Readable for PWPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwpr::W`](W) writer structure"]
impl crate::Writable for PWPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWPR to value 0x80"]
impl crate::Resettable for PWPR_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
