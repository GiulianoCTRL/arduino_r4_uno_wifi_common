#[doc = "Register `DMREQ` reader"]
pub type R = crate::R<DMREQ_SPEC>;
#[doc = "Register `DMREQ` writer"]
pub type W = crate::W<DMREQ_SPEC>;
#[doc = "Field `SWREQ` reader - DMA Software Start\n\nThe field is **modified** in some way after a read operation."]
pub type SWREQ_R = crate::BitReader<SWREQ_A>;
#[doc = "DMA Software Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWREQ_A {
    #[doc = "0: DMA transfer is not requested."]
    _0 = 0,
    #[doc = "1: DMA transfer is requested."]
    _1 = 1,
}
impl From<SWREQ_A> for bool {
    #[inline(always)]
    fn from(variant: SWREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl SWREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWREQ_A {
        match self.bits {
            false => SWREQ_A::_0,
            true => SWREQ_A::_1,
        }
    }
    #[doc = "DMA transfer is not requested."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWREQ_A::_0
    }
    #[doc = "DMA transfer is requested."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWREQ_A::_1
    }
}
#[doc = "Field `SWREQ` writer - DMA Software Start"]
pub type SWREQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SWREQ_A>;
impl<'a, REG, const O: u8> SWREQ_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA transfer is not requested."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SWREQ_A::_0)
    }
    #[doc = "DMA transfer is requested."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SWREQ_A::_1)
    }
}
#[doc = "Field `CLRS` reader - DMA Software Start Bit Auto Clear Select"]
pub type CLRS_R = crate::BitReader<CLRS_A>;
#[doc = "DMA Software Start Bit Auto Clear Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRS_A {
    #[doc = "0: SWREQ bit is cleared after DMA transfer is started by software."]
    _0 = 0,
    #[doc = "1: SWREQ bit is not cleared after DMA transfer is started by software."]
    _1 = 1,
}
impl From<CLRS_A> for bool {
    #[inline(always)]
    fn from(variant: CLRS_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLRS_A {
        match self.bits {
            false => CLRS_A::_0,
            true => CLRS_A::_1,
        }
    }
    #[doc = "SWREQ bit is cleared after DMA transfer is started by software."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLRS_A::_0
    }
    #[doc = "SWREQ bit is not cleared after DMA transfer is started by software."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLRS_A::_1
    }
}
#[doc = "Field `CLRS` writer - DMA Software Start Bit Auto Clear Select"]
pub type CLRS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CLRS_A>;
impl<'a, REG, const O: u8> CLRS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SWREQ bit is cleared after DMA transfer is started by software."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CLRS_A::_0)
    }
    #[doc = "SWREQ bit is not cleared after DMA transfer is started by software."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CLRS_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - DMA Software Start"]
    #[inline(always)]
    pub fn swreq(&self) -> SWREQ_R {
        SWREQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Software Start Bit Auto Clear Select"]
    #[inline(always)]
    pub fn clrs(&self) -> CLRS_R {
        CLRS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Software Start"]
    #[inline(always)]
    #[must_use]
    pub fn swreq(&mut self) -> SWREQ_W<DMREQ_SPEC, 0> {
        SWREQ_W::new(self)
    }
    #[doc = "Bit 4 - DMA Software Start Bit Auto Clear Select"]
    #[inline(always)]
    #[must_use]
    pub fn clrs(&mut self) -> CLRS_W<DMREQ_SPEC, 4> {
        CLRS_W::new(self)
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
#[doc = "DMA Software Start Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmreq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmreq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMREQ_SPEC;
impl crate::RegisterSpec for DMREQ_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dmreq::R`](R) reader structure"]
impl crate::Readable for DMREQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmreq::W`](W) writer structure"]
impl crate::Writable for DMREQ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMREQ to value 0"]
impl crate::Resettable for DMREQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
