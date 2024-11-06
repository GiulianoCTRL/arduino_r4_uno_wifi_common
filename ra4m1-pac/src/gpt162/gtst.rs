#[doc = "Register `GTST` reader"]
pub type R = crate::R<GTST_SPEC>;
#[doc = "Register `GTST` writer"]
pub type W = crate::W<GTST_SPEC>;
#[doc = "Field `TCFA` reader - Input Capture/Compare Match Flag A"]
pub type TCFA_R = crate::BitReader<TCFA_A>;
#[doc = "Input Capture/Compare Match Flag A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCFA_A {
    #[doc = "0: No input capture/compare match of GTCCRA is generated."]
    _0 = 0,
    #[doc = "1: An input capture/compare match of GTCCRA is generated."]
    _1 = 1,
}
impl From<TCFA_A> for bool {
    #[inline(always)]
    fn from(variant: TCFA_A) -> Self {
        variant as u8 != 0
    }
}
impl TCFA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCFA_A {
        match self.bits {
            false => TCFA_A::_0,
            true => TCFA_A::_1,
        }
    }
    #[doc = "No input capture/compare match of GTCCRA is generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCFA_A::_0
    }
    #[doc = "An input capture/compare match of GTCCRA is generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCFA_A::_1
    }
}
#[doc = "Field `TCFA` writer - Input Capture/Compare Match Flag A"]
pub type TCFA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TCFA_A>;
impl<'a, REG, const O: u8> TCFA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No input capture/compare match of GTCCRA is generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TCFA_A::_0)
    }
    #[doc = "An input capture/compare match of GTCCRA is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TCFA_A::_1)
    }
}
#[doc = "Field `TCFB` reader - Input Capture/Compare Match Flag B"]
pub type TCFB_R = crate::BitReader<TCFB_A>;
#[doc = "Input Capture/Compare Match Flag B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCFB_A {
    #[doc = "0: No input capture/compare match of GTCCRB is generated."]
    _0 = 0,
    #[doc = "1: An input capture/compare match of GTCCRB is generated."]
    _1 = 1,
}
impl From<TCFB_A> for bool {
    #[inline(always)]
    fn from(variant: TCFB_A) -> Self {
        variant as u8 != 0
    }
}
impl TCFB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCFB_A {
        match self.bits {
            false => TCFB_A::_0,
            true => TCFB_A::_1,
        }
    }
    #[doc = "No input capture/compare match of GTCCRB is generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCFB_A::_0
    }
    #[doc = "An input capture/compare match of GTCCRB is generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCFB_A::_1
    }
}
#[doc = "Field `TCFB` writer - Input Capture/Compare Match Flag B"]
pub type TCFB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TCFB_A>;
impl<'a, REG, const O: u8> TCFB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No input capture/compare match of GTCCRB is generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TCFB_A::_0)
    }
    #[doc = "An input capture/compare match of GTCCRB is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TCFB_A::_1)
    }
}
#[doc = "Field `TCFC` reader - Input Compare Match Flag C"]
pub type TCFC_R = crate::BitReader<TCFC_A>;
#[doc = "Input Compare Match Flag C\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCFC_A {
    #[doc = "0: No compare match of GTCCRC is generated."]
    _0 = 0,
    #[doc = "1: A compare match of GTCCRC is generated."]
    _1 = 1,
}
impl From<TCFC_A> for bool {
    #[inline(always)]
    fn from(variant: TCFC_A) -> Self {
        variant as u8 != 0
    }
}
impl TCFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCFC_A {
        match self.bits {
            false => TCFC_A::_0,
            true => TCFC_A::_1,
        }
    }
    #[doc = "No compare match of GTCCRC is generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCFC_A::_0
    }
    #[doc = "A compare match of GTCCRC is generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCFC_A::_1
    }
}
#[doc = "Field `TCFC` writer - Input Compare Match Flag C"]
pub type TCFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TCFC_A>;
impl<'a, REG, const O: u8> TCFC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No compare match of GTCCRC is generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TCFC_A::_0)
    }
    #[doc = "A compare match of GTCCRC is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TCFC_A::_1)
    }
}
#[doc = "Field `TCFD` reader - Input Compare Match Flag D"]
pub type TCFD_R = crate::BitReader<TCFD_A>;
#[doc = "Input Compare Match Flag D\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCFD_A {
    #[doc = "0: No compare match of GTCCRD is generated."]
    _0 = 0,
    #[doc = "1: A compare match of GTCCRD is generated."]
    _1 = 1,
}
impl From<TCFD_A> for bool {
    #[inline(always)]
    fn from(variant: TCFD_A) -> Self {
        variant as u8 != 0
    }
}
impl TCFD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCFD_A {
        match self.bits {
            false => TCFD_A::_0,
            true => TCFD_A::_1,
        }
    }
    #[doc = "No compare match of GTCCRD is generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCFD_A::_0
    }
    #[doc = "A compare match of GTCCRD is generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCFD_A::_1
    }
}
#[doc = "Field `TCFD` writer - Input Compare Match Flag D"]
pub type TCFD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TCFD_A>;
impl<'a, REG, const O: u8> TCFD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No compare match of GTCCRD is generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TCFD_A::_0)
    }
    #[doc = "A compare match of GTCCRD is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TCFD_A::_1)
    }
}
#[doc = "Field `TCFE` reader - Input Compare Match Flag E"]
pub type TCFE_R = crate::BitReader<TCFE_A>;
#[doc = "Input Compare Match Flag E\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCFE_A {
    #[doc = "0: No compare match of GTCCRE is generated."]
    _0 = 0,
    #[doc = "1: A compare match of GTCCRE is generated."]
    _1 = 1,
}
impl From<TCFE_A> for bool {
    #[inline(always)]
    fn from(variant: TCFE_A) -> Self {
        variant as u8 != 0
    }
}
impl TCFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCFE_A {
        match self.bits {
            false => TCFE_A::_0,
            true => TCFE_A::_1,
        }
    }
    #[doc = "No compare match of GTCCRE is generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCFE_A::_0
    }
    #[doc = "A compare match of GTCCRE is generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCFE_A::_1
    }
}
#[doc = "Field `TCFE` writer - Input Compare Match Flag E"]
pub type TCFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TCFE_A>;
impl<'a, REG, const O: u8> TCFE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No compare match of GTCCRE is generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TCFE_A::_0)
    }
    #[doc = "A compare match of GTCCRE is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TCFE_A::_1)
    }
}
#[doc = "Field `TCFF` reader - Input Compare Match Flag F"]
pub type TCFF_R = crate::BitReader<TCFF_A>;
#[doc = "Input Compare Match Flag F\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCFF_A {
    #[doc = "0: No compare match of GTCCRF is generated."]
    _0 = 0,
    #[doc = "1: A compare match of GTCCRF is generated."]
    _1 = 1,
}
impl From<TCFF_A> for bool {
    #[inline(always)]
    fn from(variant: TCFF_A) -> Self {
        variant as u8 != 0
    }
}
impl TCFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCFF_A {
        match self.bits {
            false => TCFF_A::_0,
            true => TCFF_A::_1,
        }
    }
    #[doc = "No compare match of GTCCRF is generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCFF_A::_0
    }
    #[doc = "A compare match of GTCCRF is generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCFF_A::_1
    }
}
#[doc = "Field `TCFF` writer - Input Compare Match Flag F"]
pub type TCFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TCFF_A>;
impl<'a, REG, const O: u8> TCFF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No compare match of GTCCRF is generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TCFF_A::_0)
    }
    #[doc = "A compare match of GTCCRF is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TCFF_A::_1)
    }
}
#[doc = "Field `TCFPO` reader - Overflow Flag"]
pub type TCFPO_R = crate::BitReader<TCFPO_A>;
#[doc = "Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCFPO_A {
    #[doc = "0: No overflow (crest) has occurred."]
    _0 = 0,
    #[doc = "1: An overflow (crest) has occurred."]
    _1 = 1,
}
impl From<TCFPO_A> for bool {
    #[inline(always)]
    fn from(variant: TCFPO_A) -> Self {
        variant as u8 != 0
    }
}
impl TCFPO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCFPO_A {
        match self.bits {
            false => TCFPO_A::_0,
            true => TCFPO_A::_1,
        }
    }
    #[doc = "No overflow (crest) has occurred."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCFPO_A::_0
    }
    #[doc = "An overflow (crest) has occurred."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCFPO_A::_1
    }
}
#[doc = "Field `TCFPO` writer - Overflow Flag"]
pub type TCFPO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TCFPO_A>;
impl<'a, REG, const O: u8> TCFPO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No overflow (crest) has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TCFPO_A::_0)
    }
    #[doc = "An overflow (crest) has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TCFPO_A::_1)
    }
}
#[doc = "Field `TCFPU` reader - Underflow Flag"]
pub type TCFPU_R = crate::BitReader<TCFPU_A>;
#[doc = "Underflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCFPU_A {
    #[doc = "0: No underflow (trough) has occurred."]
    _0 = 0,
    #[doc = "1: An underflow (trough) has occurred."]
    _1 = 1,
}
impl From<TCFPU_A> for bool {
    #[inline(always)]
    fn from(variant: TCFPU_A) -> Self {
        variant as u8 != 0
    }
}
impl TCFPU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCFPU_A {
        match self.bits {
            false => TCFPU_A::_0,
            true => TCFPU_A::_1,
        }
    }
    #[doc = "No underflow (trough) has occurred."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCFPU_A::_0
    }
    #[doc = "An underflow (trough) has occurred."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCFPU_A::_1
    }
}
#[doc = "Field `TCFPU` writer - Underflow Flag"]
pub type TCFPU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TCFPU_A>;
impl<'a, REG, const O: u8> TCFPU_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No underflow (trough) has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TCFPU_A::_0)
    }
    #[doc = "An underflow (trough) has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TCFPU_A::_1)
    }
}
#[doc = "Field `TUCF` reader - Count Direction Flag"]
pub type TUCF_R = crate::BitReader<TUCF_A>;
#[doc = "Count Direction Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TUCF_A {
    #[doc = "0: The GTCNT counter counts downward."]
    _0 = 0,
    #[doc = "1: The GTCNT counter counts upward."]
    _1 = 1,
}
impl From<TUCF_A> for bool {
    #[inline(always)]
    fn from(variant: TUCF_A) -> Self {
        variant as u8 != 0
    }
}
impl TUCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TUCF_A {
        match self.bits {
            false => TUCF_A::_0,
            true => TUCF_A::_1,
        }
    }
    #[doc = "The GTCNT counter counts downward."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TUCF_A::_0
    }
    #[doc = "The GTCNT counter counts upward."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TUCF_A::_1
    }
}
#[doc = "Field `ODF` reader - Output Disable Flag"]
pub type ODF_R = crate::BitReader<ODF_A>;
#[doc = "Output Disable Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODF_A {
    #[doc = "0: No output disable request is generated."]
    _0 = 0,
    #[doc = "1: An output disable request is generated."]
    _1 = 1,
}
impl From<ODF_A> for bool {
    #[inline(always)]
    fn from(variant: ODF_A) -> Self {
        variant as u8 != 0
    }
}
impl ODF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ODF_A {
        match self.bits {
            false => ODF_A::_0,
            true => ODF_A::_1,
        }
    }
    #[doc = "No output disable request is generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ODF_A::_0
    }
    #[doc = "An output disable request is generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ODF_A::_1
    }
}
#[doc = "Field `OABHF` reader - Same Time Output Level High Disable Request Enable"]
pub type OABHF_R = crate::BitReader<OABHF_A>;
#[doc = "Same Time Output Level High Disable Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OABHF_A {
    #[doc = "0: GTIOCA pin and GTIOCB pin don't output 1 at the same time."]
    _0 = 0,
    #[doc = "1: GTIOCA pin and GTIOCB pin output 1 at the same time."]
    _1 = 1,
}
impl From<OABHF_A> for bool {
    #[inline(always)]
    fn from(variant: OABHF_A) -> Self {
        variant as u8 != 0
    }
}
impl OABHF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OABHF_A {
        match self.bits {
            false => OABHF_A::_0,
            true => OABHF_A::_1,
        }
    }
    #[doc = "GTIOCA pin and GTIOCB pin don't output 1 at the same time."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OABHF_A::_0
    }
    #[doc = "GTIOCA pin and GTIOCB pin output 1 at the same time."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OABHF_A::_1
    }
}
#[doc = "Field `OABLF` reader - Same Time Output Level Low Disable Request Enable"]
pub type OABLF_R = crate::BitReader<OABLF_A>;
#[doc = "Same Time Output Level Low Disable Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OABLF_A {
    #[doc = "0: GTIOCA pin and GTIOCB pin don't output 0 at the same time."]
    _0 = 0,
    #[doc = "1: GTIOCA pin and GTIOCB pin output 0 at the same time."]
    _1 = 1,
}
impl From<OABLF_A> for bool {
    #[inline(always)]
    fn from(variant: OABLF_A) -> Self {
        variant as u8 != 0
    }
}
impl OABLF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OABLF_A {
        match self.bits {
            false => OABLF_A::_0,
            true => OABLF_A::_1,
        }
    }
    #[doc = "GTIOCA pin and GTIOCB pin don't output 0 at the same time."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OABLF_A::_0
    }
    #[doc = "GTIOCA pin and GTIOCB pin output 0 at the same time."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OABLF_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Input Capture/Compare Match Flag A"]
    #[inline(always)]
    pub fn tcfa(&self) -> TCFA_R {
        TCFA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input Capture/Compare Match Flag B"]
    #[inline(always)]
    pub fn tcfb(&self) -> TCFB_R {
        TCFB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input Compare Match Flag C"]
    #[inline(always)]
    pub fn tcfc(&self) -> TCFC_R {
        TCFC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Input Compare Match Flag D"]
    #[inline(always)]
    pub fn tcfd(&self) -> TCFD_R {
        TCFD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Input Compare Match Flag E"]
    #[inline(always)]
    pub fn tcfe(&self) -> TCFE_R {
        TCFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Input Compare Match Flag F"]
    #[inline(always)]
    pub fn tcff(&self) -> TCFF_R {
        TCFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overflow Flag"]
    #[inline(always)]
    pub fn tcfpo(&self) -> TCFPO_R {
        TCFPO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Underflow Flag"]
    #[inline(always)]
    pub fn tcfpu(&self) -> TCFPU_R {
        TCFPU_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - Count Direction Flag"]
    #[inline(always)]
    pub fn tucf(&self) -> TUCF_R {
        TUCF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - Output Disable Flag"]
    #[inline(always)]
    pub fn odf(&self) -> ODF_R {
        ODF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 29 - Same Time Output Level High Disable Request Enable"]
    #[inline(always)]
    pub fn oabhf(&self) -> OABHF_R {
        OABHF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Same Time Output Level Low Disable Request Enable"]
    #[inline(always)]
    pub fn oablf(&self) -> OABLF_R {
        OABLF_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Input Capture/Compare Match Flag A"]
    #[inline(always)]
    #[must_use]
    pub fn tcfa(&mut self) -> TCFA_W<GTST_SPEC, 0> {
        TCFA_W::new(self)
    }
    #[doc = "Bit 1 - Input Capture/Compare Match Flag B"]
    #[inline(always)]
    #[must_use]
    pub fn tcfb(&mut self) -> TCFB_W<GTST_SPEC, 1> {
        TCFB_W::new(self)
    }
    #[doc = "Bit 2 - Input Compare Match Flag C"]
    #[inline(always)]
    #[must_use]
    pub fn tcfc(&mut self) -> TCFC_W<GTST_SPEC, 2> {
        TCFC_W::new(self)
    }
    #[doc = "Bit 3 - Input Compare Match Flag D"]
    #[inline(always)]
    #[must_use]
    pub fn tcfd(&mut self) -> TCFD_W<GTST_SPEC, 3> {
        TCFD_W::new(self)
    }
    #[doc = "Bit 4 - Input Compare Match Flag E"]
    #[inline(always)]
    #[must_use]
    pub fn tcfe(&mut self) -> TCFE_W<GTST_SPEC, 4> {
        TCFE_W::new(self)
    }
    #[doc = "Bit 5 - Input Compare Match Flag F"]
    #[inline(always)]
    #[must_use]
    pub fn tcff(&mut self) -> TCFF_W<GTST_SPEC, 5> {
        TCFF_W::new(self)
    }
    #[doc = "Bit 6 - Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcfpo(&mut self) -> TCFPO_W<GTST_SPEC, 6> {
        TCFPO_W::new(self)
    }
    #[doc = "Bit 7 - Underflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcfpu(&mut self) -> TCFPU_W<GTST_SPEC, 7> {
        TCFPU_W::new(self)
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
#[doc = "General PWM Timer Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTST_SPEC;
impl crate::RegisterSpec for GTST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtst::R`](R) reader structure"]
impl crate::Readable for GTST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gtst::W`](W) writer structure"]
impl crate::Writable for GTST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTST to value 0x8000"]
impl crate::Resettable for GTST_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
