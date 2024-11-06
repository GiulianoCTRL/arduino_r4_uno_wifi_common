#[doc = "Register `DMCNT` reader"]
pub type R = crate::R<DMCNT_SPEC>;
#[doc = "Register `DMCNT` writer"]
pub type W = crate::W<DMCNT_SPEC>;
#[doc = "Field `DTE` reader - DMA Transfer Enable\n\nThe field is **modified** in some way after a read operation."]
pub type DTE_R = crate::BitReader<DTE_A>;
#[doc = "DMA Transfer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTE_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<DTE_A> for bool {
    #[inline(always)]
    fn from(variant: DTE_A) -> Self {
        variant as u8 != 0
    }
}
impl DTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTE_A {
        match self.bits {
            false => DTE_A::_0,
            true => DTE_A::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTE_A::_0
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTE_A::_1
    }
}
#[doc = "Field `DTE` writer - DMA Transfer Enable"]
pub type DTE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DTE_A>;
impl<'a, REG, const O: u8> DTE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DTE_A::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DTE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - DMA Transfer Enable"]
    #[inline(always)]
    pub fn dte(&self) -> DTE_R {
        DTE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Transfer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dte(&mut self) -> DTE_W<DMCNT_SPEC, 0> {
        DTE_W::new(self)
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
#[doc = "DMA Transfer Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMCNT_SPEC;
impl crate::RegisterSpec for DMCNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dmcnt::R`](R) reader structure"]
impl crate::Readable for DMCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmcnt::W`](W) writer structure"]
impl crate::Writable for DMCNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMCNT to value 0"]
impl crate::Resettable for DMCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
