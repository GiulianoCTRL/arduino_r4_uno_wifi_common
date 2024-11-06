#[doc = "Register `VBTWEGR` reader"]
pub type R = crate::R<VBTWEGR_SPEC>;
#[doc = "Register `VBTWEGR` writer"]
pub type W = crate::W<VBTWEGR_SPEC>;
#[doc = "Field `VCH0EG` reader - VBATWIO0 Wakeup Trigger Source Edge Select"]
pub type VCH0EG_R = crate::BitReader<VCH0EG_A>;
#[doc = "VBATWIO0 Wakeup Trigger Source Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH0EG_A {
    #[doc = "0: Wakeup trigger is generated at a falling edge"]
    _0 = 0,
    #[doc = "1: Wakeup trigger is generated at a rising edge."]
    _1 = 1,
}
impl From<VCH0EG_A> for bool {
    #[inline(always)]
    fn from(variant: VCH0EG_A) -> Self {
        variant as u8 != 0
    }
}
impl VCH0EG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VCH0EG_A {
        match self.bits {
            false => VCH0EG_A::_0,
            true => VCH0EG_A::_1,
        }
    }
    #[doc = "Wakeup trigger is generated at a falling edge"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH0EG_A::_0
    }
    #[doc = "Wakeup trigger is generated at a rising edge."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH0EG_A::_1
    }
}
#[doc = "Field `VCH0EG` writer - VBATWIO0 Wakeup Trigger Source Edge Select"]
pub type VCH0EG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VCH0EG_A>;
impl<'a, REG, const O: u8> VCH0EG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup trigger is generated at a falling edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VCH0EG_A::_0)
    }
    #[doc = "Wakeup trigger is generated at a rising edge."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VCH0EG_A::_1)
    }
}
#[doc = "Field `VCH1EG` reader - VBATWIO1 Wakeup Trigger Source Edge Select"]
pub type VCH1EG_R = crate::BitReader<VCH1EG_A>;
#[doc = "VBATWIO1 Wakeup Trigger Source Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH1EG_A {
    #[doc = "0: Wakeup trigger is generated at a falling edge"]
    _0 = 0,
    #[doc = "1: Wakeup trigger is generated at a rising edge."]
    _1 = 1,
}
impl From<VCH1EG_A> for bool {
    #[inline(always)]
    fn from(variant: VCH1EG_A) -> Self {
        variant as u8 != 0
    }
}
impl VCH1EG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VCH1EG_A {
        match self.bits {
            false => VCH1EG_A::_0,
            true => VCH1EG_A::_1,
        }
    }
    #[doc = "Wakeup trigger is generated at a falling edge"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH1EG_A::_0
    }
    #[doc = "Wakeup trigger is generated at a rising edge."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH1EG_A::_1
    }
}
#[doc = "Field `VCH1EG` writer - VBATWIO1 Wakeup Trigger Source Edge Select"]
pub type VCH1EG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VCH1EG_A>;
impl<'a, REG, const O: u8> VCH1EG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup trigger is generated at a falling edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VCH1EG_A::_0)
    }
    #[doc = "Wakeup trigger is generated at a rising edge."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VCH1EG_A::_1)
    }
}
#[doc = "Field `VCH2EG` reader - VBATWIO2 Wakeup Trigger Source Edge Select"]
pub type VCH2EG_R = crate::BitReader<VCH2EG_A>;
#[doc = "VBATWIO2 Wakeup Trigger Source Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH2EG_A {
    #[doc = "0: Wakeup trigger is generated at a falling edge"]
    _0 = 0,
    #[doc = "1: Wakeup trigger is generated at a rising edge."]
    _1 = 1,
}
impl From<VCH2EG_A> for bool {
    #[inline(always)]
    fn from(variant: VCH2EG_A) -> Self {
        variant as u8 != 0
    }
}
impl VCH2EG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VCH2EG_A {
        match self.bits {
            false => VCH2EG_A::_0,
            true => VCH2EG_A::_1,
        }
    }
    #[doc = "Wakeup trigger is generated at a falling edge"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH2EG_A::_0
    }
    #[doc = "Wakeup trigger is generated at a rising edge."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH2EG_A::_1
    }
}
#[doc = "Field `VCH2EG` writer - VBATWIO2 Wakeup Trigger Source Edge Select"]
pub type VCH2EG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VCH2EG_A>;
impl<'a, REG, const O: u8> VCH2EG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup trigger is generated at a falling edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VCH2EG_A::_0)
    }
    #[doc = "Wakeup trigger is generated at a rising edge."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VCH2EG_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - VBATWIO0 Wakeup Trigger Source Edge Select"]
    #[inline(always)]
    pub fn vch0eg(&self) -> VCH0EG_R {
        VCH0EG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBATWIO1 Wakeup Trigger Source Edge Select"]
    #[inline(always)]
    pub fn vch1eg(&self) -> VCH1EG_R {
        VCH1EG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VBATWIO2 Wakeup Trigger Source Edge Select"]
    #[inline(always)]
    pub fn vch2eg(&self) -> VCH2EG_R {
        VCH2EG_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBATWIO0 Wakeup Trigger Source Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn vch0eg(&mut self) -> VCH0EG_W<VBTWEGR_SPEC, 0> {
        VCH0EG_W::new(self)
    }
    #[doc = "Bit 1 - VBATWIO1 Wakeup Trigger Source Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn vch1eg(&mut self) -> VCH1EG_W<VBTWEGR_SPEC, 1> {
        VCH1EG_W::new(self)
    }
    #[doc = "Bit 2 - VBATWIO2 Wakeup Trigger Source Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn vch2eg(&mut self) -> VCH2EG_W<VBTWEGR_SPEC, 2> {
        VCH2EG_W::new(self)
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
#[doc = "VBATT Wakeup Trigger source Edge Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbtwegr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbtwegr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VBTWEGR_SPEC;
impl crate::RegisterSpec for VBTWEGR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vbtwegr::R`](R) reader structure"]
impl crate::Readable for VBTWEGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vbtwegr::W`](W) writer structure"]
impl crate::Writable for VBTWEGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VBTWEGR to value 0"]
impl crate::Resettable for VBTWEGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
