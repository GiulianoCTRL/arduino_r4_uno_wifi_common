#[doc = "Register `ELSEGR%s` reader"]
pub type R = crate::R<ELSEGR_SPEC>;
#[doc = "Register `ELSEGR%s` writer"]
pub type W = crate::W<ELSEGR_SPEC>;
#[doc = "Software Event Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEG_AW {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: Software event is generated."]
    _1 = 1,
}
impl From<SEG_AW> for bool {
    #[inline(always)]
    fn from(variant: SEG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEG` writer - Software Event Generation"]
pub type SEG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SEG_AW>;
impl<'a, REG, const O: u8> SEG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SEG_AW::_0)
    }
    #[doc = "Software event is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SEG_AW::_1)
    }
}
#[doc = "Field `WE` reader - SEG Bit Write Enable"]
pub type WE_R = crate::BitReader<WE_A>;
#[doc = "SEG Bit Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_A {
    #[doc = "0: Write to SEG bit is disabled."]
    _0 = 0,
    #[doc = "1: Write to SEG bit is enabled."]
    _1 = 1,
}
impl From<WE_A> for bool {
    #[inline(always)]
    fn from(variant: WE_A) -> Self {
        variant as u8 != 0
    }
}
impl WE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WE_A {
        match self.bits {
            false => WE_A::_0,
            true => WE_A::_1,
        }
    }
    #[doc = "Write to SEG bit is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WE_A::_0
    }
    #[doc = "Write to SEG bit is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WE_A::_1
    }
}
#[doc = "Field `WE` writer - SEG Bit Write Enable"]
pub type WE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WE_A>;
impl<'a, REG, const O: u8> WE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write to SEG bit is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WE_A::_0)
    }
    #[doc = "Write to SEG bit is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WE_A::_1)
    }
}
#[doc = "ELSEGR Register Write Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WI_AW {
    #[doc = "0: Write to ELSEGR register is enabled."]
    _0 = 0,
    #[doc = "1: Write to ELSEGR register is disabled."]
    _1 = 1,
}
impl From<WI_AW> for bool {
    #[inline(always)]
    fn from(variant: WI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WI` writer - ELSEGR Register Write Disable"]
pub type WI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WI_AW>;
impl<'a, REG, const O: u8> WI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write to ELSEGR register is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WI_AW::_0)
    }
    #[doc = "Write to ELSEGR register is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WI_AW::_1)
    }
}
impl R {
    #[doc = "Bit 6 - SEG Bit Write Enable"]
    #[inline(always)]
    pub fn we(&self) -> WE_R {
        WE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Event Generation"]
    #[inline(always)]
    #[must_use]
    pub fn seg(&mut self) -> SEG_W<ELSEGR_SPEC, 0> {
        SEG_W::new(self)
    }
    #[doc = "Bit 6 - SEG Bit Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn we(&mut self) -> WE_W<ELSEGR_SPEC, 6> {
        WE_W::new(self)
    }
    #[doc = "Bit 7 - ELSEGR Register Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wi(&mut self) -> WI_W<ELSEGR_SPEC, 7> {
        WI_W::new(self)
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
#[doc = "Event Link Software Event Generation Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`elsegr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`elsegr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ELSEGR_SPEC;
impl crate::RegisterSpec for ELSEGR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`elsegr::R`](R) reader structure"]
impl crate::Readable for ELSEGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`elsegr::W`](W) writer structure"]
impl crate::Writable for ELSEGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ELSEGR%s to value 0x80"]
impl crate::Resettable for ELSEGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
