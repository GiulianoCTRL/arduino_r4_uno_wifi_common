#[doc = "Register `PIPE%sTRE` reader"]
pub type R = crate::R<PIPETRE_SPEC>;
#[doc = "Register `PIPE%sTRE` writer"]
pub type W = crate::W<PIPETRE_SPEC>;
#[doc = "Field `TRCLR` reader - Transaction Counter Clear"]
pub type TRCLR_R = crate::BitReader<TRCLR_A>;
#[doc = "Transaction Counter Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRCLR_A {
    #[doc = "0: Invalid"]
    _0 = 0,
    #[doc = "1: The current counter value is cleared."]
    _1 = 1,
}
impl From<TRCLR_A> for bool {
    #[inline(always)]
    fn from(variant: TRCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl TRCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRCLR_A {
        match self.bits {
            false => TRCLR_A::_0,
            true => TRCLR_A::_1,
        }
    }
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRCLR_A::_0
    }
    #[doc = "The current counter value is cleared."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRCLR_A::_1
    }
}
#[doc = "Field `TRCLR` writer - Transaction Counter Clear"]
pub type TRCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TRCLR_A>;
impl<'a, REG, const O: u8> TRCLR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TRCLR_A::_0)
    }
    #[doc = "The current counter value is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TRCLR_A::_1)
    }
}
#[doc = "Field `TRENB` reader - Transaction Counter Enable"]
pub type TRENB_R = crate::BitReader<TRENB_A>;
#[doc = "Transaction Counter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRENB_A {
    #[doc = "0: Transaction counter is disabled."]
    _0 = 0,
    #[doc = "1: Transaction counter is enabled."]
    _1 = 1,
}
impl From<TRENB_A> for bool {
    #[inline(always)]
    fn from(variant: TRENB_A) -> Self {
        variant as u8 != 0
    }
}
impl TRENB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRENB_A {
        match self.bits {
            false => TRENB_A::_0,
            true => TRENB_A::_1,
        }
    }
    #[doc = "Transaction counter is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRENB_A::_0
    }
    #[doc = "Transaction counter is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRENB_A::_1
    }
}
#[doc = "Field `TRENB` writer - Transaction Counter Enable"]
pub type TRENB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TRENB_A>;
impl<'a, REG, const O: u8> TRENB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transaction counter is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TRENB_A::_0)
    }
    #[doc = "Transaction counter is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TRENB_A::_1)
    }
}
impl R {
    #[doc = "Bit 8 - Transaction Counter Clear"]
    #[inline(always)]
    pub fn trclr(&self) -> TRCLR_R {
        TRCLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transaction Counter Enable"]
    #[inline(always)]
    pub fn trenb(&self) -> TRENB_R {
        TRENB_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Transaction Counter Clear"]
    #[inline(always)]
    #[must_use]
    pub fn trclr(&mut self) -> TRCLR_W<PIPETRE_SPEC, 8> {
        TRCLR_W::new(self)
    }
    #[doc = "Bit 9 - Transaction Counter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trenb(&mut self) -> TRENB_W<PIPETRE_SPEC, 9> {
        TRENB_W::new(self)
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
#[doc = "Pipe %s Transaction Counter Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pipetre::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pipetre::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIPETRE_SPEC;
impl crate::RegisterSpec for PIPETRE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pipetre::R`](R) reader structure"]
impl crate::Readable for PIPETRE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pipetre::W`](W) writer structure"]
impl crate::Writable for PIPETRE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIPE%sTRE to value 0"]
impl crate::Resettable for PIPETRE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
