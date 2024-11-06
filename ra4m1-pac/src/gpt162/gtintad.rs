#[doc = "Register `GTINTAD` reader"]
pub type R = crate::R<GTINTAD_SPEC>;
#[doc = "Register `GTINTAD` writer"]
pub type W = crate::W<GTINTAD_SPEC>;
#[doc = "Field `GRP` reader - Output Disable Source Select"]
pub type GRP_R = crate::FieldReader<GRP_A>;
#[doc = "Output Disable Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GRP_A {
    #[doc = "0: Group A output disable request"]
    _00 = 0,
    #[doc = "1: Group B output disable request"]
    _01 = 1,
}
impl From<GRP_A> for u8 {
    #[inline(always)]
    fn from(variant: GRP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GRP_A {
    type Ux = u8;
}
impl GRP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GRP_A> {
        match self.bits {
            0 => Some(GRP_A::_00),
            1 => Some(GRP_A::_01),
            _ => None,
        }
    }
    #[doc = "Group A output disable request"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == GRP_A::_00
    }
    #[doc = "Group B output disable request"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == GRP_A::_01
    }
}
#[doc = "Field `GRP` writer - Output Disable Source Select"]
pub type GRP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, GRP_A>;
impl<'a, REG, const O: u8> GRP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Group A output disable request"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(GRP_A::_00)
    }
    #[doc = "Group B output disable request"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(GRP_A::_01)
    }
}
#[doc = "Field `GRPABH` reader - Same Time Output Level High Disable Request Enable"]
pub type GRPABH_R = crate::BitReader<GRPABH_A>;
#[doc = "Same Time Output Level High Disable Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GRPABH_A {
    #[doc = "0: Same time output level high disable request is disabled."]
    _0 = 0,
    #[doc = "1: Same time output level high disable request is enabled."]
    _1 = 1,
}
impl From<GRPABH_A> for bool {
    #[inline(always)]
    fn from(variant: GRPABH_A) -> Self {
        variant as u8 != 0
    }
}
impl GRPABH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GRPABH_A {
        match self.bits {
            false => GRPABH_A::_0,
            true => GRPABH_A::_1,
        }
    }
    #[doc = "Same time output level high disable request is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GRPABH_A::_0
    }
    #[doc = "Same time output level high disable request is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GRPABH_A::_1
    }
}
#[doc = "Field `GRPABH` writer - Same Time Output Level High Disable Request Enable"]
pub type GRPABH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, GRPABH_A>;
impl<'a, REG, const O: u8> GRPABH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Same time output level high disable request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(GRPABH_A::_0)
    }
    #[doc = "Same time output level high disable request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(GRPABH_A::_1)
    }
}
#[doc = "Field `GRPABL` reader - Same Time Output Level Low Disable Request Enable"]
pub type GRPABL_R = crate::BitReader<GRPABL_A>;
#[doc = "Same Time Output Level Low Disable Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GRPABL_A {
    #[doc = "0: Same time output level low disable request is disabled."]
    _0 = 0,
    #[doc = "1: Same time output level low disable request is enabled."]
    _1 = 1,
}
impl From<GRPABL_A> for bool {
    #[inline(always)]
    fn from(variant: GRPABL_A) -> Self {
        variant as u8 != 0
    }
}
impl GRPABL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GRPABL_A {
        match self.bits {
            false => GRPABL_A::_0,
            true => GRPABL_A::_1,
        }
    }
    #[doc = "Same time output level low disable request is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GRPABL_A::_0
    }
    #[doc = "Same time output level low disable request is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GRPABL_A::_1
    }
}
#[doc = "Field `GRPABL` writer - Same Time Output Level Low Disable Request Enable"]
pub type GRPABL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, GRPABL_A>;
impl<'a, REG, const O: u8> GRPABL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Same time output level low disable request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(GRPABL_A::_0)
    }
    #[doc = "Same time output level low disable request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(GRPABL_A::_1)
    }
}
impl R {
    #[doc = "Bits 24:25 - Output Disable Source Select"]
    #[inline(always)]
    pub fn grp(&self) -> GRP_R {
        GRP_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 29 - Same Time Output Level High Disable Request Enable"]
    #[inline(always)]
    pub fn grpabh(&self) -> GRPABH_R {
        GRPABH_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Same Time Output Level Low Disable Request Enable"]
    #[inline(always)]
    pub fn grpabl(&self) -> GRPABL_R {
        GRPABL_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 24:25 - Output Disable Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn grp(&mut self) -> GRP_W<GTINTAD_SPEC, 24> {
        GRP_W::new(self)
    }
    #[doc = "Bit 29 - Same Time Output Level High Disable Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn grpabh(&mut self) -> GRPABH_W<GTINTAD_SPEC, 29> {
        GRPABH_W::new(self)
    }
    #[doc = "Bit 30 - Same Time Output Level Low Disable Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn grpabl(&mut self) -> GRPABL_W<GTINTAD_SPEC, 30> {
        GRPABL_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "General PWM Timer Interrupt Output Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtintad::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtintad::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTINTAD_SPEC;
impl crate::RegisterSpec for GTINTAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtintad::R`](R) reader structure"]
impl crate::Readable for GTINTAD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gtintad::W`](W) writer structure"]
impl crate::Writable for GTINTAD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTINTAD to value 0"]
impl crate::Resettable for GTINTAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
