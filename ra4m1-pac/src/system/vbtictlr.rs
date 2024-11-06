#[doc = "Register `VBTICTLR` reader"]
pub type R = crate::R<VBTICTLR_SPEC>;
#[doc = "Register `VBTICTLR` writer"]
pub type W = crate::W<VBTICTLR_SPEC>;
#[doc = "Field `VCH0INEN` reader - VBATT Wakeup I/O 0 Input Enable"]
pub type VCH0INEN_R = crate::BitReader<VCH0INEN_A>;
#[doc = "VBATT Wakeup I/O 0 Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH0INEN_A {
    #[doc = "0: VBATWIO0, RTCIC0 inputs disabled"]
    _0 = 0,
    #[doc = "1: VBATWIO0, RTCIC0 inputs enabled."]
    _1 = 1,
}
impl From<VCH0INEN_A> for bool {
    #[inline(always)]
    fn from(variant: VCH0INEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VCH0INEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VCH0INEN_A {
        match self.bits {
            false => VCH0INEN_A::_0,
            true => VCH0INEN_A::_1,
        }
    }
    #[doc = "VBATWIO0, RTCIC0 inputs disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH0INEN_A::_0
    }
    #[doc = "VBATWIO0, RTCIC0 inputs enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH0INEN_A::_1
    }
}
#[doc = "Field `VCH0INEN` writer - VBATT Wakeup I/O 0 Input Enable"]
pub type VCH0INEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VCH0INEN_A>;
impl<'a, REG, const O: u8> VCH0INEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATWIO0, RTCIC0 inputs disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VCH0INEN_A::_0)
    }
    #[doc = "VBATWIO0, RTCIC0 inputs enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VCH0INEN_A::_1)
    }
}
#[doc = "Field `VCH1INEN` reader - VBATT Wakeup I/O 1 Input Enable"]
pub type VCH1INEN_R = crate::BitReader<VCH1INEN_A>;
#[doc = "VBATT Wakeup I/O 1 Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH1INEN_A {
    #[doc = "0: VBATWIO1, RTCIC1 inputs disabled"]
    _0 = 0,
    #[doc = "1: VBATWIO1, RTCIC1 inputs enabled."]
    _1 = 1,
}
impl From<VCH1INEN_A> for bool {
    #[inline(always)]
    fn from(variant: VCH1INEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VCH1INEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VCH1INEN_A {
        match self.bits {
            false => VCH1INEN_A::_0,
            true => VCH1INEN_A::_1,
        }
    }
    #[doc = "VBATWIO1, RTCIC1 inputs disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH1INEN_A::_0
    }
    #[doc = "VBATWIO1, RTCIC1 inputs enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH1INEN_A::_1
    }
}
#[doc = "Field `VCH1INEN` writer - VBATT Wakeup I/O 1 Input Enable"]
pub type VCH1INEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VCH1INEN_A>;
impl<'a, REG, const O: u8> VCH1INEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATWIO1, RTCIC1 inputs disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VCH1INEN_A::_0)
    }
    #[doc = "VBATWIO1, RTCIC1 inputs enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VCH1INEN_A::_1)
    }
}
#[doc = "Field `VCH2INEN` reader - VBATT Wakeup I/O 2 Input Enable"]
pub type VCH2INEN_R = crate::BitReader<VCH2INEN_A>;
#[doc = "VBATT Wakeup I/O 2 Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH2INEN_A {
    #[doc = "0: VBATWIO2 and RTCIC2 inputs disabled"]
    _0 = 0,
    #[doc = "1: VBATWIO2 and RTCIC2 inputs enabled."]
    _1 = 1,
}
impl From<VCH2INEN_A> for bool {
    #[inline(always)]
    fn from(variant: VCH2INEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VCH2INEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VCH2INEN_A {
        match self.bits {
            false => VCH2INEN_A::_0,
            true => VCH2INEN_A::_1,
        }
    }
    #[doc = "VBATWIO2 and RTCIC2 inputs disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH2INEN_A::_0
    }
    #[doc = "VBATWIO2 and RTCIC2 inputs enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH2INEN_A::_1
    }
}
#[doc = "Field `VCH2INEN` writer - VBATT Wakeup I/O 2 Input Enable"]
pub type VCH2INEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VCH2INEN_A>;
impl<'a, REG, const O: u8> VCH2INEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATWIO2 and RTCIC2 inputs disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VCH2INEN_A::_0)
    }
    #[doc = "VBATWIO2 and RTCIC2 inputs enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VCH2INEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - VBATT Wakeup I/O 0 Input Enable"]
    #[inline(always)]
    pub fn vch0inen(&self) -> VCH0INEN_R {
        VCH0INEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBATT Wakeup I/O 1 Input Enable"]
    #[inline(always)]
    pub fn vch1inen(&self) -> VCH1INEN_R {
        VCH1INEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VBATT Wakeup I/O 2 Input Enable"]
    #[inline(always)]
    pub fn vch2inen(&self) -> VCH2INEN_R {
        VCH2INEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBATT Wakeup I/O 0 Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vch0inen(&mut self) -> VCH0INEN_W<VBTICTLR_SPEC, 0> {
        VCH0INEN_W::new(self)
    }
    #[doc = "Bit 1 - VBATT Wakeup I/O 1 Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vch1inen(&mut self) -> VCH1INEN_W<VBTICTLR_SPEC, 1> {
        VCH1INEN_W::new(self)
    }
    #[doc = "Bit 2 - VBATT Wakeup I/O 2 Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vch2inen(&mut self) -> VCH2INEN_W<VBTICTLR_SPEC, 2> {
        VCH2INEN_W::new(self)
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
#[doc = "VBATT Input Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbtictlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbtictlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VBTICTLR_SPEC;
impl crate::RegisterSpec for VBTICTLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vbtictlr::R`](R) reader structure"]
impl crate::Readable for VBTICTLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vbtictlr::W`](W) writer structure"]
impl crate::Writable for VBTICTLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VBTICTLR to value 0"]
impl crate::Resettable for VBTICTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
