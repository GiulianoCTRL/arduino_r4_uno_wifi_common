#[doc = "Register `DMCRB` reader"]
pub type R = crate::R<DMCRB_SPEC>;
#[doc = "Register `DMCRB` writer"]
pub type W = crate::W<DMCRB_SPEC>;
#[doc = "Field `DMCRB` reader - Specifies the number of block transfer operations or repeat transfer operations."]
pub type DMCRB_R = crate::FieldReader<DMCRB_A>;
#[doc = "Specifies the number of block transfer operations or repeat transfer operations.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum DMCRB_A {
    #[doc = "0: 65,536 blocks"]
    _0000 = 0,
}
impl From<DMCRB_A> for u16 {
    #[inline(always)]
    fn from(variant: DMCRB_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMCRB_A {
    type Ux = u16;
}
impl DMCRB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DMCRB_A> {
        match self.bits {
            0 => Some(DMCRB_A::_0000),
            _ => None,
        }
    }
    #[doc = "65,536 blocks"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == DMCRB_A::_0000
    }
}
#[doc = "Field `DMCRB` writer - Specifies the number of block transfer operations or repeat transfer operations."]
pub type DMCRB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, DMCRB_A>;
impl<'a, REG, const O: u8> DMCRB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "65,536 blocks"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(DMCRB_A::_0000)
    }
}
impl R {
    #[doc = "Bits 0:15 - Specifies the number of block transfer operations or repeat transfer operations."]
    #[inline(always)]
    pub fn dmcrb(&self) -> DMCRB_R {
        DMCRB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specifies the number of block transfer operations or repeat transfer operations."]
    #[inline(always)]
    #[must_use]
    pub fn dmcrb(&mut self) -> DMCRB_W<DMCRB_SPEC, 0> {
        DMCRB_W::new(self)
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
#[doc = "DMA Block Transfer Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmcrb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmcrb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMCRB_SPEC;
impl crate::RegisterSpec for DMCRB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dmcrb::R`](R) reader structure"]
impl crate::Readable for DMCRB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmcrb::W`](W) writer structure"]
impl crate::Writable for DMCRB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMCRB to value 0"]
impl crate::Resettable for DMCRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
