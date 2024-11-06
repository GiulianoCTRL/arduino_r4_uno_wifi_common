#[doc = "Register `CACR2` reader"]
pub type R = crate::R<CACR2_SPEC>;
#[doc = "Register `CACR2` writer"]
pub type W = crate::W<CACR2_SPEC>;
#[doc = "Field `RPS` reader - Reference Signal Select"]
pub type RPS_R = crate::BitReader<RPS_A>;
#[doc = "Reference Signal Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPS_A {
    #[doc = "0: CACREF pin input"]
    _0 = 0,
    #[doc = "1: Internal clock (internally generated signal)"]
    _1 = 1,
}
impl From<RPS_A> for bool {
    #[inline(always)]
    fn from(variant: RPS_A) -> Self {
        variant as u8 != 0
    }
}
impl RPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPS_A {
        match self.bits {
            false => RPS_A::_0,
            true => RPS_A::_1,
        }
    }
    #[doc = "CACREF pin input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPS_A::_0
    }
    #[doc = "Internal clock (internally generated signal)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPS_A::_1
    }
}
#[doc = "Field `RPS` writer - Reference Signal Select"]
pub type RPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RPS_A>;
impl<'a, REG, const O: u8> RPS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CACREF pin input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RPS_A::_0)
    }
    #[doc = "Internal clock (internally generated signal)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RPS_A::_1)
    }
}
#[doc = "Field `RSCS` reader - Measurement Reference Clock Select"]
pub type RSCS_R = crate::FieldReader<RSCS_A>;
#[doc = "Measurement Reference Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RSCS_A {
    #[doc = "0: Main clock"]
    _000 = 0,
    #[doc = "1: Sub-clock"]
    _001 = 1,
    #[doc = "2: HOCO clock"]
    _010 = 2,
    #[doc = "3: MOCO clock"]
    _011 = 3,
    #[doc = "4: LOCO clock"]
    _100 = 4,
    #[doc = "5: Peripheral module clock(PCLKB)"]
    _101 = 5,
    #[doc = "6: IWDTCLK clock"]
    _110 = 6,
    #[doc = "7: Setting prohibited"]
    _111 = 7,
}
impl From<RSCS_A> for u8 {
    #[inline(always)]
    fn from(variant: RSCS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RSCS_A {
    type Ux = u8;
}
impl RSCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RSCS_A {
        match self.bits {
            0 => RSCS_A::_000,
            1 => RSCS_A::_001,
            2 => RSCS_A::_010,
            3 => RSCS_A::_011,
            4 => RSCS_A::_100,
            5 => RSCS_A::_101,
            6 => RSCS_A::_110,
            7 => RSCS_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Main clock"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == RSCS_A::_000
    }
    #[doc = "Sub-clock"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == RSCS_A::_001
    }
    #[doc = "HOCO clock"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == RSCS_A::_010
    }
    #[doc = "MOCO clock"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == RSCS_A::_011
    }
    #[doc = "LOCO clock"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == RSCS_A::_100
    }
    #[doc = "Peripheral module clock(PCLKB)"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == RSCS_A::_101
    }
    #[doc = "IWDTCLK clock"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == RSCS_A::_110
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == RSCS_A::_111
    }
}
#[doc = "Field `RSCS` writer - Measurement Reference Clock Select"]
pub type RSCS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, RSCS_A>;
impl<'a, REG, const O: u8> RSCS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Main clock"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(RSCS_A::_000)
    }
    #[doc = "Sub-clock"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(RSCS_A::_001)
    }
    #[doc = "HOCO clock"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(RSCS_A::_010)
    }
    #[doc = "MOCO clock"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(RSCS_A::_011)
    }
    #[doc = "LOCO clock"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(RSCS_A::_100)
    }
    #[doc = "Peripheral module clock(PCLKB)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(RSCS_A::_101)
    }
    #[doc = "IWDTCLK clock"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(RSCS_A::_110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(RSCS_A::_111)
    }
}
#[doc = "Field `RCDS` reader - Measurement Reference Clock Frequency Division Ratio Select"]
pub type RCDS_R = crate::FieldReader<RCDS_A>;
#[doc = "Measurement Reference Clock Frequency Division Ratio Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RCDS_A {
    #[doc = "0: 1/32 clock"]
    _00 = 0,
    #[doc = "1: 1/128 clock"]
    _01 = 1,
    #[doc = "2: 1/1024 clock"]
    _10 = 2,
    #[doc = "3: 1/8192 clock"]
    _11 = 3,
}
impl From<RCDS_A> for u8 {
    #[inline(always)]
    fn from(variant: RCDS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RCDS_A {
    type Ux = u8;
}
impl RCDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RCDS_A {
        match self.bits {
            0 => RCDS_A::_00,
            1 => RCDS_A::_01,
            2 => RCDS_A::_10,
            3 => RCDS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "1/32 clock"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RCDS_A::_00
    }
    #[doc = "1/128 clock"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RCDS_A::_01
    }
    #[doc = "1/1024 clock"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RCDS_A::_10
    }
    #[doc = "1/8192 clock"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == RCDS_A::_11
    }
}
#[doc = "Field `RCDS` writer - Measurement Reference Clock Frequency Division Ratio Select"]
pub type RCDS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, RCDS_A>;
impl<'a, REG, const O: u8> RCDS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1/32 clock"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(RCDS_A::_00)
    }
    #[doc = "1/128 clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(RCDS_A::_01)
    }
    #[doc = "1/1024 clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(RCDS_A::_10)
    }
    #[doc = "1/8192 clock"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(RCDS_A::_11)
    }
}
#[doc = "Field `DFS` reader - Digital Filter Selection"]
pub type DFS_R = crate::FieldReader<DFS_A>;
#[doc = "Digital Filter Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DFS_A {
    #[doc = "0: Digital filtering is disabled."]
    _00 = 0,
    #[doc = "1: The sampling clock for the digital filter is the frequency measuring clock."]
    _01 = 1,
    #[doc = "2: The sampling clock for the digital filter is the frequency measuring clock divided by 4."]
    _10 = 2,
    #[doc = "3: The sampling clock for the digital filter is the frequency measuring clock divided by 16."]
    _11 = 3,
}
impl From<DFS_A> for u8 {
    #[inline(always)]
    fn from(variant: DFS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DFS_A {
    type Ux = u8;
}
impl DFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DFS_A {
        match self.bits {
            0 => DFS_A::_00,
            1 => DFS_A::_01,
            2 => DFS_A::_10,
            3 => DFS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Digital filtering is disabled."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DFS_A::_00
    }
    #[doc = "The sampling clock for the digital filter is the frequency measuring clock."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DFS_A::_01
    }
    #[doc = "The sampling clock for the digital filter is the frequency measuring clock divided by 4."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DFS_A::_10
    }
    #[doc = "The sampling clock for the digital filter is the frequency measuring clock divided by 16."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DFS_A::_11
    }
}
#[doc = "Field `DFS` writer - Digital Filter Selection"]
pub type DFS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, DFS_A>;
impl<'a, REG, const O: u8> DFS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Digital filtering is disabled."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(DFS_A::_00)
    }
    #[doc = "The sampling clock for the digital filter is the frequency measuring clock."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(DFS_A::_01)
    }
    #[doc = "The sampling clock for the digital filter is the frequency measuring clock divided by 4."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(DFS_A::_10)
    }
    #[doc = "The sampling clock for the digital filter is the frequency measuring clock divided by 16."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(DFS_A::_11)
    }
}
impl R {
    #[doc = "Bit 0 - Reference Signal Select"]
    #[inline(always)]
    pub fn rps(&self) -> RPS_R {
        RPS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Measurement Reference Clock Select"]
    #[inline(always)]
    pub fn rscs(&self) -> RSCS_R {
        RSCS_R::new((self.bits >> 1) & 7)
    }
    #[doc = "Bits 4:5 - Measurement Reference Clock Frequency Division Ratio Select"]
    #[inline(always)]
    pub fn rcds(&self) -> RCDS_R {
        RCDS_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Digital Filter Selection"]
    #[inline(always)]
    pub fn dfs(&self) -> DFS_R {
        DFS_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Reference Signal Select"]
    #[inline(always)]
    #[must_use]
    pub fn rps(&mut self) -> RPS_W<CACR2_SPEC, 0> {
        RPS_W::new(self)
    }
    #[doc = "Bits 1:3 - Measurement Reference Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn rscs(&mut self) -> RSCS_W<CACR2_SPEC, 1> {
        RSCS_W::new(self)
    }
    #[doc = "Bits 4:5 - Measurement Reference Clock Frequency Division Ratio Select"]
    #[inline(always)]
    #[must_use]
    pub fn rcds(&mut self) -> RCDS_W<CACR2_SPEC, 4> {
        RCDS_W::new(self)
    }
    #[doc = "Bits 6:7 - Digital Filter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn dfs(&mut self) -> DFS_W<CACR2_SPEC, 6> {
        DFS_W::new(self)
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
#[doc = "CAC Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cacr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cacr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACR2_SPEC;
impl crate::RegisterSpec for CACR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cacr2::R`](R) reader structure"]
impl crate::Readable for CACR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cacr2::W`](W) writer structure"]
impl crate::Writable for CACR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACR2 to value 0"]
impl crate::Resettable for CACR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
