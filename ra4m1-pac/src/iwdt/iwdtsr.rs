#[doc = "Register `IWDTSR` reader"]
pub type R = crate::R<IWDTSR_SPEC>;
#[doc = "Register `IWDTSR` writer"]
pub type W = crate::W<IWDTSR_SPEC>;
#[doc = "Field `CNTVAL` reader - Counter Value Value counted by the counter"]
pub type CNTVAL_R = crate::FieldReader<u16>;
#[doc = "Field `UNDFF` reader - Underflow Flag\n\nThe field is **modified** in some way after a read operation."]
pub type UNDFF_R = crate::BitReader<UNDFF_A>;
#[doc = "Underflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UNDFF_A {
    #[doc = "0: Underflow not occurred"]
    _0 = 0,
    #[doc = "1: Underflow occurred"]
    _1 = 1,
}
impl From<UNDFF_A> for bool {
    #[inline(always)]
    fn from(variant: UNDFF_A) -> Self {
        variant as u8 != 0
    }
}
impl UNDFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UNDFF_A {
        match self.bits {
            false => UNDFF_A::_0,
            true => UNDFF_A::_1,
        }
    }
    #[doc = "Underflow not occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UNDFF_A::_0
    }
    #[doc = "Underflow occurred"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UNDFF_A::_1
    }
}
#[doc = "Field `UNDFF` writer - Underflow Flag"]
pub type UNDFF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, UNDFF_A>;
impl<'a, REG, const O: u8> UNDFF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Underflow not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(UNDFF_A::_0)
    }
    #[doc = "Underflow occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(UNDFF_A::_1)
    }
}
#[doc = "Field `REFEF` reader - Refresh Error Flag\n\nThe field is **modified** in some way after a read operation."]
pub type REFEF_R = crate::BitReader<REFEF_A>;
#[doc = "Refresh Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFEF_A {
    #[doc = "0: Refresh error not occurred"]
    _0 = 0,
    #[doc = "1: Refresh error occurred"]
    _1 = 1,
}
impl From<REFEF_A> for bool {
    #[inline(always)]
    fn from(variant: REFEF_A) -> Self {
        variant as u8 != 0
    }
}
impl REFEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REFEF_A {
        match self.bits {
            false => REFEF_A::_0,
            true => REFEF_A::_1,
        }
    }
    #[doc = "Refresh error not occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == REFEF_A::_0
    }
    #[doc = "Refresh error occurred"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == REFEF_A::_1
    }
}
#[doc = "Field `REFEF` writer - Refresh Error Flag"]
pub type REFEF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, REFEF_A>;
impl<'a, REG, const O: u8> REFEF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Refresh error not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(REFEF_A::_0)
    }
    #[doc = "Refresh error occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(REFEF_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:13 - Counter Value Value counted by the counter"]
    #[inline(always)]
    pub fn cntval(&self) -> CNTVAL_R {
        CNTVAL_R::new(self.bits & 0x3fff)
    }
    #[doc = "Bit 14 - Underflow Flag"]
    #[inline(always)]
    pub fn undff(&self) -> UNDFF_R {
        UNDFF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Refresh Error Flag"]
    #[inline(always)]
    pub fn refef(&self) -> REFEF_R {
        REFEF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Underflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn undff(&mut self) -> UNDFF_W<IWDTSR_SPEC, 14> {
        UNDFF_W::new(self)
    }
    #[doc = "Bit 15 - Refresh Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn refef(&mut self) -> REFEF_W<IWDTSR_SPEC, 15> {
        REFEF_W::new(self)
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
#[doc = "IWDT Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdtsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwdtsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IWDTSR_SPEC;
impl crate::RegisterSpec for IWDTSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`iwdtsr::R`](R) reader structure"]
impl crate::Readable for IWDTSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iwdtsr::W`](W) writer structure"]
impl crate::Writable for IWDTSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xc000;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IWDTSR to value 0"]
impl crate::Resettable for IWDTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
