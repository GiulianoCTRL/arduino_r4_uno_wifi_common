#[doc = "Register `CACR1` reader"]
pub type R = crate::R<CACR1_SPEC>;
#[doc = "Register `CACR1` writer"]
pub type W = crate::W<CACR1_SPEC>;
#[doc = "Field `CACREFE` reader - CACREF Pin Input Enable"]
pub type CACREFE_R = crate::BitReader<CACREFE_A>;
#[doc = "CACREF Pin Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CACREFE_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<CACREFE_A> for bool {
    #[inline(always)]
    fn from(variant: CACREFE_A) -> Self {
        variant as u8 != 0
    }
}
impl CACREFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CACREFE_A {
        match self.bits {
            false => CACREFE_A::_0,
            true => CACREFE_A::_1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CACREFE_A::_0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CACREFE_A::_1
    }
}
#[doc = "Field `CACREFE` writer - CACREF Pin Input Enable"]
pub type CACREFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CACREFE_A>;
impl<'a, REG, const O: u8> CACREFE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CACREFE_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CACREFE_A::_1)
    }
}
#[doc = "Field `FMCS` reader - Measurement Target Clock Select"]
pub type FMCS_R = crate::FieldReader<FMCS_A>;
#[doc = "Measurement Target Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FMCS_A {
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
impl From<FMCS_A> for u8 {
    #[inline(always)]
    fn from(variant: FMCS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FMCS_A {
    type Ux = u8;
}
impl FMCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FMCS_A {
        match self.bits {
            0 => FMCS_A::_000,
            1 => FMCS_A::_001,
            2 => FMCS_A::_010,
            3 => FMCS_A::_011,
            4 => FMCS_A::_100,
            5 => FMCS_A::_101,
            6 => FMCS_A::_110,
            7 => FMCS_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Main clock"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == FMCS_A::_000
    }
    #[doc = "Sub-clock"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == FMCS_A::_001
    }
    #[doc = "HOCO clock"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == FMCS_A::_010
    }
    #[doc = "MOCO clock"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == FMCS_A::_011
    }
    #[doc = "LOCO clock"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == FMCS_A::_100
    }
    #[doc = "Peripheral module clock(PCLKB)"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == FMCS_A::_101
    }
    #[doc = "IWDTCLK clock"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == FMCS_A::_110
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == FMCS_A::_111
    }
}
#[doc = "Field `FMCS` writer - Measurement Target Clock Select"]
pub type FMCS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, FMCS_A>;
impl<'a, REG, const O: u8> FMCS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Main clock"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(FMCS_A::_000)
    }
    #[doc = "Sub-clock"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(FMCS_A::_001)
    }
    #[doc = "HOCO clock"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(FMCS_A::_010)
    }
    #[doc = "MOCO clock"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(FMCS_A::_011)
    }
    #[doc = "LOCO clock"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(FMCS_A::_100)
    }
    #[doc = "Peripheral module clock(PCLKB)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(FMCS_A::_101)
    }
    #[doc = "IWDTCLK clock"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(FMCS_A::_110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(FMCS_A::_111)
    }
}
#[doc = "Field `TCSS` reader - Measurement Target Clock Frequency Division Ratio Select"]
pub type TCSS_R = crate::FieldReader<TCSS_A>;
#[doc = "Measurement Target Clock Frequency Division Ratio Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCSS_A {
    #[doc = "0: No division"]
    _00 = 0,
    #[doc = "1: x 1/4 clock"]
    _01 = 1,
    #[doc = "2: x 1/8 clock"]
    _10 = 2,
    #[doc = "3: x 1/32 clock"]
    _11 = 3,
}
impl From<TCSS_A> for u8 {
    #[inline(always)]
    fn from(variant: TCSS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TCSS_A {
    type Ux = u8;
}
impl TCSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCSS_A {
        match self.bits {
            0 => TCSS_A::_00,
            1 => TCSS_A::_01,
            2 => TCSS_A::_10,
            3 => TCSS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "No division"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TCSS_A::_00
    }
    #[doc = "x 1/4 clock"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TCSS_A::_01
    }
    #[doc = "x 1/8 clock"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TCSS_A::_10
    }
    #[doc = "x 1/32 clock"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TCSS_A::_11
    }
}
#[doc = "Field `TCSS` writer - Measurement Target Clock Frequency Division Ratio Select"]
pub type TCSS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, TCSS_A>;
impl<'a, REG, const O: u8> TCSS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No division"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(TCSS_A::_00)
    }
    #[doc = "x 1/4 clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(TCSS_A::_01)
    }
    #[doc = "x 1/8 clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(TCSS_A::_10)
    }
    #[doc = "x 1/32 clock"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(TCSS_A::_11)
    }
}
#[doc = "Field `EDGES` reader - Valid Edge Select"]
pub type EDGES_R = crate::FieldReader<EDGES_A>;
#[doc = "Valid Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDGES_A {
    #[doc = "0: Rising edge"]
    _00 = 0,
    #[doc = "1: Falling edge"]
    _01 = 1,
    #[doc = "2: Both rising and falling edges"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<EDGES_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EDGES_A {
    type Ux = u8;
}
impl EDGES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EDGES_A {
        match self.bits {
            0 => EDGES_A::_00,
            1 => EDGES_A::_01,
            2 => EDGES_A::_10,
            3 => EDGES_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == EDGES_A::_00
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == EDGES_A::_01
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == EDGES_A::_10
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == EDGES_A::_11
    }
}
#[doc = "Field `EDGES` writer - Valid Edge Select"]
pub type EDGES_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, EDGES_A>;
impl<'a, REG, const O: u8> EDGES_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(EDGES_A::_00)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(EDGES_A::_01)
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(EDGES_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(EDGES_A::_11)
    }
}
impl R {
    #[doc = "Bit 0 - CACREF Pin Input Enable"]
    #[inline(always)]
    pub fn cacrefe(&self) -> CACREFE_R {
        CACREFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Measurement Target Clock Select"]
    #[inline(always)]
    pub fn fmcs(&self) -> FMCS_R {
        FMCS_R::new((self.bits >> 1) & 7)
    }
    #[doc = "Bits 4:5 - Measurement Target Clock Frequency Division Ratio Select"]
    #[inline(always)]
    pub fn tcss(&self) -> TCSS_R {
        TCSS_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Valid Edge Select"]
    #[inline(always)]
    pub fn edges(&self) -> EDGES_R {
        EDGES_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - CACREF Pin Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cacrefe(&mut self) -> CACREFE_W<CACR1_SPEC, 0> {
        CACREFE_W::new(self)
    }
    #[doc = "Bits 1:3 - Measurement Target Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn fmcs(&mut self) -> FMCS_W<CACR1_SPEC, 1> {
        FMCS_W::new(self)
    }
    #[doc = "Bits 4:5 - Measurement Target Clock Frequency Division Ratio Select"]
    #[inline(always)]
    #[must_use]
    pub fn tcss(&mut self) -> TCSS_W<CACR1_SPEC, 4> {
        TCSS_W::new(self)
    }
    #[doc = "Bits 6:7 - Valid Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn edges(&mut self) -> EDGES_W<CACR1_SPEC, 6> {
        EDGES_W::new(self)
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
#[doc = "CAC Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cacr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cacr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACR1_SPEC;
impl crate::RegisterSpec for CACR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cacr1::R`](R) reader structure"]
impl crate::Readable for CACR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cacr1::W`](W) writer structure"]
impl crate::Writable for CACR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACR1 to value 0"]
impl crate::Resettable for CACR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
