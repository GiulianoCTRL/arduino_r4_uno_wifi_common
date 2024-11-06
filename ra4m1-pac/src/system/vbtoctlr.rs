#[doc = "Register `VBTOCTLR` reader"]
pub type R = crate::R<VBTOCTLR_SPEC>;
#[doc = "Register `VBTOCTLR` writer"]
pub type W = crate::W<VBTOCTLR_SPEC>;
#[doc = "Field `VCH0OEN` reader - VBATT Wakeup I/O 0 Output Enable"]
pub type VCH0OEN_R = crate::BitReader<VCH0OEN_A>;
#[doc = "VBATT Wakeup I/O 0 Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH0OEN_A {
    #[doc = "0: VBATWIO0 output disabled"]
    _0 = 0,
    #[doc = "1: VBATWIO0 output enabled"]
    _1 = 1,
}
impl From<VCH0OEN_A> for bool {
    #[inline(always)]
    fn from(variant: VCH0OEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VCH0OEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VCH0OEN_A {
        match self.bits {
            false => VCH0OEN_A::_0,
            true => VCH0OEN_A::_1,
        }
    }
    #[doc = "VBATWIO0 output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH0OEN_A::_0
    }
    #[doc = "VBATWIO0 output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH0OEN_A::_1
    }
}
#[doc = "Field `VCH0OEN` writer - VBATT Wakeup I/O 0 Output Enable"]
pub type VCH0OEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VCH0OEN_A>;
impl<'a, REG, const O: u8> VCH0OEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATWIO0 output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VCH0OEN_A::_0)
    }
    #[doc = "VBATWIO0 output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VCH0OEN_A::_1)
    }
}
#[doc = "Field `VCH1OEN` reader - VBATT Wakeup I/O 1 Output Enable"]
pub type VCH1OEN_R = crate::BitReader<VCH1OEN_A>;
#[doc = "VBATT Wakeup I/O 1 Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH1OEN_A {
    #[doc = "0: VBATWIO1 output disabled"]
    _0 = 0,
    #[doc = "1: VBATWIO1 output enabled"]
    _1 = 1,
}
impl From<VCH1OEN_A> for bool {
    #[inline(always)]
    fn from(variant: VCH1OEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VCH1OEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VCH1OEN_A {
        match self.bits {
            false => VCH1OEN_A::_0,
            true => VCH1OEN_A::_1,
        }
    }
    #[doc = "VBATWIO1 output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH1OEN_A::_0
    }
    #[doc = "VBATWIO1 output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH1OEN_A::_1
    }
}
#[doc = "Field `VCH1OEN` writer - VBATT Wakeup I/O 1 Output Enable"]
pub type VCH1OEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VCH1OEN_A>;
impl<'a, REG, const O: u8> VCH1OEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATWIO1 output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VCH1OEN_A::_0)
    }
    #[doc = "VBATWIO1 output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VCH1OEN_A::_1)
    }
}
#[doc = "Field `VCH2OEN` reader - VBATT Wakeup I/O 2 Output Enable"]
pub type VCH2OEN_R = crate::BitReader<VCH2OEN_A>;
#[doc = "VBATT Wakeup I/O 2 Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH2OEN_A {
    #[doc = "0: VBATWIO2 output disabled"]
    _0 = 0,
    #[doc = "1: VBATWIO2 output enabled"]
    _1 = 1,
}
impl From<VCH2OEN_A> for bool {
    #[inline(always)]
    fn from(variant: VCH2OEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VCH2OEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VCH2OEN_A {
        match self.bits {
            false => VCH2OEN_A::_0,
            true => VCH2OEN_A::_1,
        }
    }
    #[doc = "VBATWIO2 output disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH2OEN_A::_0
    }
    #[doc = "VBATWIO2 output enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH2OEN_A::_1
    }
}
#[doc = "Field `VCH2OEN` writer - VBATT Wakeup I/O 2 Output Enable"]
pub type VCH2OEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VCH2OEN_A>;
impl<'a, REG, const O: u8> VCH2OEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATWIO2 output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VCH2OEN_A::_0)
    }
    #[doc = "VBATWIO2 output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VCH2OEN_A::_1)
    }
}
#[doc = "Field `VOUT0LSEL` reader - VBATT Wakeup I/O 0 Output Level Selection"]
pub type VOUT0LSEL_R = crate::BitReader<VOUT0LSEL_A>;
#[doc = "VBATT Wakeup I/O 0 Output Level Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VOUT0LSEL_A {
    #[doc = "0: Output L before VBATT wakeup trigger"]
    _0 = 0,
    #[doc = "1: Output H before VBATT wakeup trigger"]
    _1 = 1,
}
impl From<VOUT0LSEL_A> for bool {
    #[inline(always)]
    fn from(variant: VOUT0LSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl VOUT0LSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VOUT0LSEL_A {
        match self.bits {
            false => VOUT0LSEL_A::_0,
            true => VOUT0LSEL_A::_1,
        }
    }
    #[doc = "Output L before VBATT wakeup trigger"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VOUT0LSEL_A::_0
    }
    #[doc = "Output H before VBATT wakeup trigger"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VOUT0LSEL_A::_1
    }
}
#[doc = "Field `VOUT0LSEL` writer - VBATT Wakeup I/O 0 Output Level Selection"]
pub type VOUT0LSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VOUT0LSEL_A>;
impl<'a, REG, const O: u8> VOUT0LSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output L before VBATT wakeup trigger"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VOUT0LSEL_A::_0)
    }
    #[doc = "Output H before VBATT wakeup trigger"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VOUT0LSEL_A::_1)
    }
}
#[doc = "Field `VOUT1LSEL` reader - VBATT Wakeup I/O 1 Output Level Selection"]
pub type VOUT1LSEL_R = crate::BitReader<VOUT1LSEL_A>;
#[doc = "VBATT Wakeup I/O 1 Output Level Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VOUT1LSEL_A {
    #[doc = "0: Output L before VBATT wake up trigger"]
    _0 = 0,
    #[doc = "1: Output H before VBATT wake up trigger"]
    _1 = 1,
}
impl From<VOUT1LSEL_A> for bool {
    #[inline(always)]
    fn from(variant: VOUT1LSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl VOUT1LSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VOUT1LSEL_A {
        match self.bits {
            false => VOUT1LSEL_A::_0,
            true => VOUT1LSEL_A::_1,
        }
    }
    #[doc = "Output L before VBATT wake up trigger"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VOUT1LSEL_A::_0
    }
    #[doc = "Output H before VBATT wake up trigger"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VOUT1LSEL_A::_1
    }
}
#[doc = "Field `VOUT1LSEL` writer - VBATT Wakeup I/O 1 Output Level Selection"]
pub type VOUT1LSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VOUT1LSEL_A>;
impl<'a, REG, const O: u8> VOUT1LSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output L before VBATT wake up trigger"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VOUT1LSEL_A::_0)
    }
    #[doc = "Output H before VBATT wake up trigger"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VOUT1LSEL_A::_1)
    }
}
#[doc = "Field `VOUT2LSEL` reader - VBATT Wakeup I/O 2 Output Level Selection"]
pub type VOUT2LSEL_R = crate::BitReader<VOUT2LSEL_A>;
#[doc = "VBATT Wakeup I/O 2 Output Level Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VOUT2LSEL_A {
    #[doc = "0: Output L before VBATT wake up trigger"]
    _0 = 0,
    #[doc = "1: Output H before VBATT wake up trigger"]
    _1 = 1,
}
impl From<VOUT2LSEL_A> for bool {
    #[inline(always)]
    fn from(variant: VOUT2LSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl VOUT2LSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VOUT2LSEL_A {
        match self.bits {
            false => VOUT2LSEL_A::_0,
            true => VOUT2LSEL_A::_1,
        }
    }
    #[doc = "Output L before VBATT wake up trigger"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VOUT2LSEL_A::_0
    }
    #[doc = "Output H before VBATT wake up trigger"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VOUT2LSEL_A::_1
    }
}
#[doc = "Field `VOUT2LSEL` writer - VBATT Wakeup I/O 2 Output Level Selection"]
pub type VOUT2LSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VOUT2LSEL_A>;
impl<'a, REG, const O: u8> VOUT2LSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output L before VBATT wake up trigger"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VOUT2LSEL_A::_0)
    }
    #[doc = "Output H before VBATT wake up trigger"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VOUT2LSEL_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - VBATT Wakeup I/O 0 Output Enable"]
    #[inline(always)]
    pub fn vch0oen(&self) -> VCH0OEN_R {
        VCH0OEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBATT Wakeup I/O 1 Output Enable"]
    #[inline(always)]
    pub fn vch1oen(&self) -> VCH1OEN_R {
        VCH1OEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VBATT Wakeup I/O 2 Output Enable"]
    #[inline(always)]
    pub fn vch2oen(&self) -> VCH2OEN_R {
        VCH2OEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VBATT Wakeup I/O 0 Output Level Selection"]
    #[inline(always)]
    pub fn vout0lsel(&self) -> VOUT0LSEL_R {
        VOUT0LSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VBATT Wakeup I/O 1 Output Level Selection"]
    #[inline(always)]
    pub fn vout1lsel(&self) -> VOUT1LSEL_R {
        VOUT1LSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VBATT Wakeup I/O 2 Output Level Selection"]
    #[inline(always)]
    pub fn vout2lsel(&self) -> VOUT2LSEL_R {
        VOUT2LSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBATT Wakeup I/O 0 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vch0oen(&mut self) -> VCH0OEN_W<VBTOCTLR_SPEC, 0> {
        VCH0OEN_W::new(self)
    }
    #[doc = "Bit 1 - VBATT Wakeup I/O 1 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vch1oen(&mut self) -> VCH1OEN_W<VBTOCTLR_SPEC, 1> {
        VCH1OEN_W::new(self)
    }
    #[doc = "Bit 2 - VBATT Wakeup I/O 2 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vch2oen(&mut self) -> VCH2OEN_W<VBTOCTLR_SPEC, 2> {
        VCH2OEN_W::new(self)
    }
    #[doc = "Bit 3 - VBATT Wakeup I/O 0 Output Level Selection"]
    #[inline(always)]
    #[must_use]
    pub fn vout0lsel(&mut self) -> VOUT0LSEL_W<VBTOCTLR_SPEC, 3> {
        VOUT0LSEL_W::new(self)
    }
    #[doc = "Bit 4 - VBATT Wakeup I/O 1 Output Level Selection"]
    #[inline(always)]
    #[must_use]
    pub fn vout1lsel(&mut self) -> VOUT1LSEL_W<VBTOCTLR_SPEC, 4> {
        VOUT1LSEL_W::new(self)
    }
    #[doc = "Bit 5 - VBATT Wakeup I/O 2 Output Level Selection"]
    #[inline(always)]
    #[must_use]
    pub fn vout2lsel(&mut self) -> VOUT2LSEL_W<VBTOCTLR_SPEC, 5> {
        VOUT2LSEL_W::new(self)
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
#[doc = "VBATT Output Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbtoctlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbtoctlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VBTOCTLR_SPEC;
impl crate::RegisterSpec for VBTOCTLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vbtoctlr::R`](R) reader structure"]
impl crate::Readable for VBTOCTLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vbtoctlr::W`](W) writer structure"]
impl crate::Writable for VBTOCTLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VBTOCTLR to value 0"]
impl crate::Resettable for VBTOCTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
