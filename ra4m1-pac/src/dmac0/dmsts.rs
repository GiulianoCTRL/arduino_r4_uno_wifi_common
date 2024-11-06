#[doc = "Register `DMSTS` reader"]
pub type R = crate::R<DMSTS_SPEC>;
#[doc = "Register `DMSTS` writer"]
pub type W = crate::W<DMSTS_SPEC>;
#[doc = "Field `ESIF` reader - Transfer Escape End Interrupt Flag\n\nThe field is **modified** in some way after a read operation."]
pub type ESIF_R = crate::BitReader<ESIF_A>;
#[doc = "Transfer Escape End Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESIF_A {
    #[doc = "0: No interrupt"]
    _0 = 0,
    #[doc = "1: Interrupt occurred."]
    _1 = 1,
}
impl From<ESIF_A> for bool {
    #[inline(always)]
    fn from(variant: ESIF_A) -> Self {
        variant as u8 != 0
    }
}
impl ESIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ESIF_A {
        match self.bits {
            false => ESIF_A::_0,
            true => ESIF_A::_1,
        }
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ESIF_A::_0
    }
    #[doc = "Interrupt occurred."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ESIF_A::_1
    }
}
#[doc = "Field `ESIF` writer - Transfer Escape End Interrupt Flag"]
pub type ESIF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, ESIF_A>;
impl<'a, REG, const O: u8> ESIF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ESIF_A::_0)
    }
    #[doc = "Interrupt occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ESIF_A::_1)
    }
}
#[doc = "Field `DTIF` reader - Transfer End Interrupt Flag\n\nThe field is **modified** in some way after a read operation."]
pub type DTIF_R = crate::BitReader<DTIF_A>;
#[doc = "Transfer End Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTIF_A {
    #[doc = "0: No interrupt"]
    _0 = 0,
    #[doc = "1: Interrupt occurred."]
    _1 = 1,
}
impl From<DTIF_A> for bool {
    #[inline(always)]
    fn from(variant: DTIF_A) -> Self {
        variant as u8 != 0
    }
}
impl DTIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTIF_A {
        match self.bits {
            false => DTIF_A::_0,
            true => DTIF_A::_1,
        }
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTIF_A::_0
    }
    #[doc = "Interrupt occurred."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTIF_A::_1
    }
}
#[doc = "Field `DTIF` writer - Transfer End Interrupt Flag"]
pub type DTIF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, DTIF_A>;
impl<'a, REG, const O: u8> DTIF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DTIF_A::_0)
    }
    #[doc = "Interrupt occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DTIF_A::_1)
    }
}
#[doc = "Field `ACT` reader - DMA Active Flag"]
pub type ACT_R = crate::BitReader<ACT_A>;
#[doc = "DMA Active Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACT_A {
    #[doc = "0: DMAC operation suspended"]
    _0 = 0,
    #[doc = "1: DMAC operating."]
    _1 = 1,
}
impl From<ACT_A> for bool {
    #[inline(always)]
    fn from(variant: ACT_A) -> Self {
        variant as u8 != 0
    }
}
impl ACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ACT_A {
        match self.bits {
            false => ACT_A::_0,
            true => ACT_A::_1,
        }
    }
    #[doc = "DMAC operation suspended"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACT_A::_0
    }
    #[doc = "DMAC operating."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACT_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Transfer Escape End Interrupt Flag"]
    #[inline(always)]
    pub fn esif(&self) -> ESIF_R {
        ESIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Transfer End Interrupt Flag"]
    #[inline(always)]
    pub fn dtif(&self) -> DTIF_R {
        DTIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Active Flag"]
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Escape End Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn esif(&mut self) -> ESIF_W<DMSTS_SPEC, 0> {
        ESIF_W::new(self)
    }
    #[doc = "Bit 4 - Transfer End Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dtif(&mut self) -> DTIF_W<DMSTS_SPEC, 4> {
        DTIF_W::new(self)
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
#[doc = "DMA Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMSTS_SPEC;
impl crate::RegisterSpec for DMSTS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dmsts::R`](R) reader structure"]
impl crate::Readable for DMSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmsts::W`](W) writer structure"]
impl crate::Writable for DMSTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x11;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMSTS to value 0"]
impl crate::Resettable for DMSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
