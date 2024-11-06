#[doc = "Register `POEGG%s` reader"]
pub type R = crate::R<POEGG_SPEC>;
#[doc = "Register `POEGG%s` writer"]
pub type W = crate::W<POEGG_SPEC>;
#[doc = "Field `PIDF` reader - Port Input Detection Flag\n\nThe field is **modified** in some way after a read operation."]
pub type PIDF_R = crate::BitReader<PIDF_A>;
#[doc = "Port Input Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIDF_A {
    #[doc = "0: No output-disable request from the GTETRGn pin has occurred"]
    _0 = 0,
    #[doc = "1: Output-disable request from the GTETRGn pin occurred."]
    _1 = 1,
}
impl From<PIDF_A> for bool {
    #[inline(always)]
    fn from(variant: PIDF_A) -> Self {
        variant as u8 != 0
    }
}
impl PIDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIDF_A {
        match self.bits {
            false => PIDF_A::_0,
            true => PIDF_A::_1,
        }
    }
    #[doc = "No output-disable request from the GTETRGn pin has occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDF_A::_0
    }
    #[doc = "Output-disable request from the GTETRGn pin occurred."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDF_A::_1
    }
}
#[doc = "Field `PIDF` writer - Port Input Detection Flag"]
pub type PIDF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, PIDF_A>;
impl<'a, REG, const O: u8> PIDF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No output-disable request from the GTETRGn pin has occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIDF_A::_0)
    }
    #[doc = "Output-disable request from the GTETRGn pin occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIDF_A::_1)
    }
}
#[doc = "Field `IOCF` reader - Output-disable Request Detection Flag from GPT\n\nThe field is **modified** in some way after a read operation."]
pub type IOCF_R = crate::BitReader<IOCF_A>;
#[doc = "Output-disable Request Detection Flag from GPT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOCF_A {
    #[doc = "0: No output-disable request from the GPT disable request has occurred"]
    _0 = 0,
    #[doc = "1: Output-disable request from the GPT disable request occurred."]
    _1 = 1,
}
impl From<IOCF_A> for bool {
    #[inline(always)]
    fn from(variant: IOCF_A) -> Self {
        variant as u8 != 0
    }
}
impl IOCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IOCF_A {
        match self.bits {
            false => IOCF_A::_0,
            true => IOCF_A::_1,
        }
    }
    #[doc = "No output-disable request from the GPT disable request has occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IOCF_A::_0
    }
    #[doc = "Output-disable request from the GPT disable request occurred."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IOCF_A::_1
    }
}
#[doc = "Field `IOCF` writer - Output-disable Request Detection Flag from GPT"]
pub type IOCF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, IOCF_A>;
impl<'a, REG, const O: u8> IOCF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No output-disable request from the GPT disable request has occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IOCF_A::_0)
    }
    #[doc = "Output-disable request from the GPT disable request occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IOCF_A::_1)
    }
}
#[doc = "Field `OSTPF` reader - Oscillation Stop Detection Flag\n\nThe field is **modified** in some way after a read operation."]
pub type OSTPF_R = crate::BitReader<OSTPF_A>;
#[doc = "Oscillation Stop Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSTPF_A {
    #[doc = "0: No output-disable request from oscillation stop detection has occurred"]
    _0 = 0,
    #[doc = "1: Output-disable request from oscillation stop detection occurred."]
    _1 = 1,
}
impl From<OSTPF_A> for bool {
    #[inline(always)]
    fn from(variant: OSTPF_A) -> Self {
        variant as u8 != 0
    }
}
impl OSTPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSTPF_A {
        match self.bits {
            false => OSTPF_A::_0,
            true => OSTPF_A::_1,
        }
    }
    #[doc = "No output-disable request from oscillation stop detection has occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OSTPF_A::_0
    }
    #[doc = "Output-disable request from oscillation stop detection occurred."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OSTPF_A::_1
    }
}
#[doc = "Field `OSTPF` writer - Oscillation Stop Detection Flag"]
pub type OSTPF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, OSTPF_A>;
impl<'a, REG, const O: u8> OSTPF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No output-disable request from oscillation stop detection has occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OSTPF_A::_0)
    }
    #[doc = "Output-disable request from oscillation stop detection occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OSTPF_A::_1)
    }
}
#[doc = "Field `SSF` reader - Software Stop Flag"]
pub type SSF_R = crate::BitReader<SSF_A>;
#[doc = "Software Stop Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSF_A {
    #[doc = "0: No output-disable request from software has occurred"]
    _0 = 0,
    #[doc = "1: Output-disable request from software occurred."]
    _1 = 1,
}
impl From<SSF_A> for bool {
    #[inline(always)]
    fn from(variant: SSF_A) -> Self {
        variant as u8 != 0
    }
}
impl SSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSF_A {
        match self.bits {
            false => SSF_A::_0,
            true => SSF_A::_1,
        }
    }
    #[doc = "No output-disable request from software has occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSF_A::_0
    }
    #[doc = "Output-disable request from software occurred."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSF_A::_1
    }
}
#[doc = "Field `SSF` writer - Software Stop Flag"]
pub type SSF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SSF_A>;
impl<'a, REG, const O: u8> SSF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No output-disable request from software has occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSF_A::_0)
    }
    #[doc = "Output-disable request from software occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSF_A::_1)
    }
}
#[doc = "Field `PIDE` reader - Port Input Detection Enable Note: Can be modified only once after a reset."]
pub type PIDE_R = crate::BitReader<PIDE_A>;
#[doc = "Port Input Detection Enable Note: Can be modified only once after a reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIDE_A {
    #[doc = "0: Output-disable request from the GTETRG pins disabled"]
    _0 = 0,
    #[doc = "1: Output-disable request from the GTETRG pins enabled."]
    _1 = 1,
}
impl From<PIDE_A> for bool {
    #[inline(always)]
    fn from(variant: PIDE_A) -> Self {
        variant as u8 != 0
    }
}
impl PIDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIDE_A {
        match self.bits {
            false => PIDE_A::_0,
            true => PIDE_A::_1,
        }
    }
    #[doc = "Output-disable request from the GTETRG pins disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDE_A::_0
    }
    #[doc = "Output-disable request from the GTETRG pins enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDE_A::_1
    }
}
#[doc = "Field `PIDE` writer - Port Input Detection Enable Note: Can be modified only once after a reset."]
pub type PIDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PIDE_A>;
impl<'a, REG, const O: u8> PIDE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output-disable request from the GTETRG pins disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIDE_A::_0)
    }
    #[doc = "Output-disable request from the GTETRG pins enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIDE_A::_1)
    }
}
#[doc = "Field `IOCE` reader - Output-disable Request Enable from GPT Note: Can be modified only once after a reset."]
pub type IOCE_R = crate::BitReader<IOCE_A>;
#[doc = "Output-disable Request Enable from GPT Note: Can be modified only once after a reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOCE_A {
    #[doc = "0: Output-disable request from the GPT disable request disabled"]
    _0 = 0,
    #[doc = "1: Output-disable request from the GPT disable request enabled."]
    _1 = 1,
}
impl From<IOCE_A> for bool {
    #[inline(always)]
    fn from(variant: IOCE_A) -> Self {
        variant as u8 != 0
    }
}
impl IOCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IOCE_A {
        match self.bits {
            false => IOCE_A::_0,
            true => IOCE_A::_1,
        }
    }
    #[doc = "Output-disable request from the GPT disable request disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IOCE_A::_0
    }
    #[doc = "Output-disable request from the GPT disable request enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IOCE_A::_1
    }
}
#[doc = "Field `IOCE` writer - Output-disable Request Enable from GPT Note: Can be modified only once after a reset."]
pub type IOCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IOCE_A>;
impl<'a, REG, const O: u8> IOCE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output-disable request from the GPT disable request disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IOCE_A::_0)
    }
    #[doc = "Output-disable request from the GPT disable request enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IOCE_A::_1)
    }
}
#[doc = "Field `OSTPE` reader - Oscillation Stop Detection Enable Note: Can be modified only once after a reset."]
pub type OSTPE_R = crate::BitReader<OSTPE_A>;
#[doc = "Oscillation Stop Detection Enable Note: Can be modified only once after a reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSTPE_A {
    #[doc = "0: A output-disable request from the oscillation stop detection disabled."]
    _0 = 0,
    #[doc = "1: A output-disable request from the oscillation stop detection enabled."]
    _1 = 1,
}
impl From<OSTPE_A> for bool {
    #[inline(always)]
    fn from(variant: OSTPE_A) -> Self {
        variant as u8 != 0
    }
}
impl OSTPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSTPE_A {
        match self.bits {
            false => OSTPE_A::_0,
            true => OSTPE_A::_1,
        }
    }
    #[doc = "A output-disable request from the oscillation stop detection disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OSTPE_A::_0
    }
    #[doc = "A output-disable request from the oscillation stop detection enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OSTPE_A::_1
    }
}
#[doc = "Field `OSTPE` writer - Oscillation Stop Detection Enable Note: Can be modified only once after a reset."]
pub type OSTPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OSTPE_A>;
impl<'a, REG, const O: u8> OSTPE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A output-disable request from the oscillation stop detection disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OSTPE_A::_0)
    }
    #[doc = "A output-disable request from the oscillation stop detection enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OSTPE_A::_1)
    }
}
#[doc = "Field `ST` reader - GTETRG Input Status Flag"]
pub type ST_R = crate::BitReader<ST_A>;
#[doc = "GTETRG Input Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ST_A {
    #[doc = "0: GTETRG input after filtering is 0."]
    _0 = 0,
    #[doc = "1: GTETRG input after filtering is 1."]
    _1 = 1,
}
impl From<ST_A> for bool {
    #[inline(always)]
    fn from(variant: ST_A) -> Self {
        variant as u8 != 0
    }
}
impl ST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ST_A {
        match self.bits {
            false => ST_A::_0,
            true => ST_A::_1,
        }
    }
    #[doc = "GTETRG input after filtering is 0."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ST_A::_0
    }
    #[doc = "GTETRG input after filtering is 1."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ST_A::_1
    }
}
#[doc = "Field `INV` reader - GTETRG Input Reverse"]
pub type INV_R = crate::BitReader<INV_A>;
#[doc = "GTETRG Input Reverse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INV_A {
    #[doc = "0: GTETRG Input"]
    _0 = 0,
    #[doc = "1: GTETRG Input Reversed."]
    _1 = 1,
}
impl From<INV_A> for bool {
    #[inline(always)]
    fn from(variant: INV_A) -> Self {
        variant as u8 != 0
    }
}
impl INV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INV_A {
        match self.bits {
            false => INV_A::_0,
            true => INV_A::_1,
        }
    }
    #[doc = "GTETRG Input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INV_A::_0
    }
    #[doc = "GTETRG Input Reversed."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INV_A::_1
    }
}
#[doc = "Field `INV` writer - GTETRG Input Reverse"]
pub type INV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INV_A>;
impl<'a, REG, const O: u8> INV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTETRG Input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(INV_A::_0)
    }
    #[doc = "GTETRG Input Reversed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(INV_A::_1)
    }
}
#[doc = "Field `NFEN` reader - Noise Filter Enable"]
pub type NFEN_R = crate::BitReader<NFEN_A>;
#[doc = "Noise Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NFEN_A {
    #[doc = "0: Filtering noise disabled"]
    _0 = 0,
    #[doc = "1: Filtering noise enabled"]
    _1 = 1,
}
impl From<NFEN_A> for bool {
    #[inline(always)]
    fn from(variant: NFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl NFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NFEN_A {
        match self.bits {
            false => NFEN_A::_0,
            true => NFEN_A::_1,
        }
    }
    #[doc = "Filtering noise disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NFEN_A::_0
    }
    #[doc = "Filtering noise enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NFEN_A::_1
    }
}
#[doc = "Field `NFEN` writer - Noise Filter Enable"]
pub type NFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, NFEN_A>;
impl<'a, REG, const O: u8> NFEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Filtering noise disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(NFEN_A::_0)
    }
    #[doc = "Filtering noise enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(NFEN_A::_1)
    }
}
#[doc = "Field `NFCS` reader - Noise Filter Clock Select"]
pub type NFCS_R = crate::FieldReader<NFCS_A>;
#[doc = "Noise Filter Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NFCS_A {
    #[doc = "0: Sampling GTETRG pin input level for three times in every PCLKB."]
    _00 = 0,
    #[doc = "1: Sampling GTETRG pin input level for three times in every PCLKB /8."]
    _01 = 1,
    #[doc = "2: Sampling GTETRG pin input level for three times in every PCLKB /32."]
    _10 = 2,
    #[doc = "3: Sampling GTETRG pin input level for three times in every PCLKB /128."]
    _11 = 3,
}
impl From<NFCS_A> for u8 {
    #[inline(always)]
    fn from(variant: NFCS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NFCS_A {
    type Ux = u8;
}
impl NFCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NFCS_A {
        match self.bits {
            0 => NFCS_A::_00,
            1 => NFCS_A::_01,
            2 => NFCS_A::_10,
            3 => NFCS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Sampling GTETRG pin input level for three times in every PCLKB."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == NFCS_A::_00
    }
    #[doc = "Sampling GTETRG pin input level for three times in every PCLKB /8."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == NFCS_A::_01
    }
    #[doc = "Sampling GTETRG pin input level for three times in every PCLKB /32."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == NFCS_A::_10
    }
    #[doc = "Sampling GTETRG pin input level for three times in every PCLKB /128."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == NFCS_A::_11
    }
}
#[doc = "Field `NFCS` writer - Noise Filter Clock Select"]
pub type NFCS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, NFCS_A>;
impl<'a, REG, const O: u8> NFCS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sampling GTETRG pin input level for three times in every PCLKB."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(NFCS_A::_00)
    }
    #[doc = "Sampling GTETRG pin input level for three times in every PCLKB /8."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(NFCS_A::_01)
    }
    #[doc = "Sampling GTETRG pin input level for three times in every PCLKB /32."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(NFCS_A::_10)
    }
    #[doc = "Sampling GTETRG pin input level for three times in every PCLKB /128."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(NFCS_A::_11)
    }
}
impl R {
    #[doc = "Bit 0 - Port Input Detection Flag"]
    #[inline(always)]
    pub fn pidf(&self) -> PIDF_R {
        PIDF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output-disable Request Detection Flag from GPT"]
    #[inline(always)]
    pub fn iocf(&self) -> IOCF_R {
        IOCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Oscillation Stop Detection Flag"]
    #[inline(always)]
    pub fn ostpf(&self) -> OSTPF_R {
        OSTPF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software Stop Flag"]
    #[inline(always)]
    pub fn ssf(&self) -> SSF_R {
        SSF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port Input Detection Enable Note: Can be modified only once after a reset."]
    #[inline(always)]
    pub fn pide(&self) -> PIDE_R {
        PIDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Output-disable Request Enable from GPT Note: Can be modified only once after a reset."]
    #[inline(always)]
    pub fn ioce(&self) -> IOCE_R {
        IOCE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Oscillation Stop Detection Enable Note: Can be modified only once after a reset."]
    #[inline(always)]
    pub fn ostpe(&self) -> OSTPE_R {
        OSTPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - GTETRG Input Status Flag"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 28 - GTETRG Input Reverse"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Noise Filter Enable"]
    #[inline(always)]
    pub fn nfen(&self) -> NFEN_R {
        NFEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Noise Filter Clock Select"]
    #[inline(always)]
    pub fn nfcs(&self) -> NFCS_R {
        NFCS_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Port Input Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pidf(&mut self) -> PIDF_W<POEGG_SPEC, 0> {
        PIDF_W::new(self)
    }
    #[doc = "Bit 1 - Output-disable Request Detection Flag from GPT"]
    #[inline(always)]
    #[must_use]
    pub fn iocf(&mut self) -> IOCF_W<POEGG_SPEC, 1> {
        IOCF_W::new(self)
    }
    #[doc = "Bit 2 - Oscillation Stop Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ostpf(&mut self) -> OSTPF_W<POEGG_SPEC, 2> {
        OSTPF_W::new(self)
    }
    #[doc = "Bit 3 - Software Stop Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ssf(&mut self) -> SSF_W<POEGG_SPEC, 3> {
        SSF_W::new(self)
    }
    #[doc = "Bit 4 - Port Input Detection Enable Note: Can be modified only once after a reset."]
    #[inline(always)]
    #[must_use]
    pub fn pide(&mut self) -> PIDE_W<POEGG_SPEC, 4> {
        PIDE_W::new(self)
    }
    #[doc = "Bit 5 - Output-disable Request Enable from GPT Note: Can be modified only once after a reset."]
    #[inline(always)]
    #[must_use]
    pub fn ioce(&mut self) -> IOCE_W<POEGG_SPEC, 5> {
        IOCE_W::new(self)
    }
    #[doc = "Bit 6 - Oscillation Stop Detection Enable Note: Can be modified only once after a reset."]
    #[inline(always)]
    #[must_use]
    pub fn ostpe(&mut self) -> OSTPE_W<POEGG_SPEC, 6> {
        OSTPE_W::new(self)
    }
    #[doc = "Bit 28 - GTETRG Input Reverse"]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> INV_W<POEGG_SPEC, 28> {
        INV_W::new(self)
    }
    #[doc = "Bit 29 - Noise Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nfen(&mut self) -> NFEN_W<POEGG_SPEC, 29> {
        NFEN_W::new(self)
    }
    #[doc = "Bits 30:31 - Noise Filter Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn nfcs(&mut self) -> NFCS_W<POEGG_SPEC, 30> {
        NFCS_W::new(self)
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
#[doc = "POEG Group %s Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`poegg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`poegg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POEGG_SPEC;
impl crate::RegisterSpec for POEGG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`poegg::R`](R) reader structure"]
impl crate::Readable for POEGG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`poegg::W`](W) writer structure"]
impl crate::Writable for POEGG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x07;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POEGG%s to value 0"]
impl crate::Resettable for POEGG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
