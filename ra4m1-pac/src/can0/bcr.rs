#[doc = "Register `BCR` reader"]
pub type R = crate::R<BCR_SPEC>;
#[doc = "Register `BCR` writer"]
pub type W = crate::W<BCR_SPEC>;
#[doc = "Field `CCLKS` reader - CAN Clock Source Selection"]
pub type CCLKS_R = crate::BitReader<CCLKS_A>;
#[doc = "CAN Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLKS_A {
    #[doc = "0: PCLK (generated by the PLL clock)"]
    _0 = 0,
    #[doc = "1: CANMCLK (generated by the main clock)"]
    _1 = 1,
}
impl From<CCLKS_A> for bool {
    #[inline(always)]
    fn from(variant: CCLKS_A) -> Self {
        variant as u8 != 0
    }
}
impl CCLKS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCLKS_A {
        match self.bits {
            false => CCLKS_A::_0,
            true => CCLKS_A::_1,
        }
    }
    #[doc = "PCLK (generated by the PLL clock)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCLKS_A::_0
    }
    #[doc = "CANMCLK (generated by the main clock)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCLKS_A::_1
    }
}
#[doc = "Field `CCLKS` writer - CAN Clock Source Selection"]
pub type CCLKS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CCLKS_A>;
impl<'a, REG, const O: u8> CCLKS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PCLK (generated by the PLL clock)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CCLKS_A::_0)
    }
    #[doc = "CANMCLK (generated by the main clock)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CCLKS_A::_1)
    }
}
#[doc = "Field `TSEG2` reader - Time Segment 2 Control"]
pub type TSEG2_R = crate::FieldReader<TSEG2_A>;
#[doc = "Time Segment 2 Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSEG2_A {
    #[doc = "0: Setting prohibited"]
    _000 = 0,
    #[doc = "1: 2 Tq"]
    _001 = 1,
    #[doc = "2: 3 Tq"]
    _010 = 2,
    #[doc = "3: 4 Tq"]
    _011 = 3,
    #[doc = "4: 5 Tq"]
    _100 = 4,
    #[doc = "5: 6 Tq"]
    _101 = 5,
    #[doc = "6: 7 Tq"]
    _110 = 6,
    #[doc = "7: 8 Tq"]
    _111 = 7,
}
impl From<TSEG2_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEG2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSEG2_A {
    type Ux = u8;
}
impl TSEG2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSEG2_A {
        match self.bits {
            0 => TSEG2_A::_000,
            1 => TSEG2_A::_001,
            2 => TSEG2_A::_010,
            3 => TSEG2_A::_011,
            4 => TSEG2_A::_100,
            5 => TSEG2_A::_101,
            6 => TSEG2_A::_110,
            7 => TSEG2_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == TSEG2_A::_000
    }
    #[doc = "2 Tq"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == TSEG2_A::_001
    }
    #[doc = "3 Tq"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == TSEG2_A::_010
    }
    #[doc = "4 Tq"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == TSEG2_A::_011
    }
    #[doc = "5 Tq"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == TSEG2_A::_100
    }
    #[doc = "6 Tq"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == TSEG2_A::_101
    }
    #[doc = "7 Tq"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == TSEG2_A::_110
    }
    #[doc = "8 Tq"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == TSEG2_A::_111
    }
}
#[doc = "Field `TSEG2` writer - Time Segment 2 Control"]
pub type TSEG2_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, TSEG2_A>;
impl<'a, REG, const O: u8> TSEG2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(TSEG2_A::_000)
    }
    #[doc = "2 Tq"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(TSEG2_A::_001)
    }
    #[doc = "3 Tq"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(TSEG2_A::_010)
    }
    #[doc = "4 Tq"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(TSEG2_A::_011)
    }
    #[doc = "5 Tq"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(TSEG2_A::_100)
    }
    #[doc = "6 Tq"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(TSEG2_A::_101)
    }
    #[doc = "7 Tq"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(TSEG2_A::_110)
    }
    #[doc = "8 Tq"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(TSEG2_A::_111)
    }
}
#[doc = "Field `SJW` reader - Resynchronization Jump Width Control"]
pub type SJW_R = crate::FieldReader<SJW_A>;
#[doc = "Resynchronization Jump Width Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SJW_A {
    #[doc = "0: 1 Tq"]
    _00 = 0,
    #[doc = "1: 2 Tq"]
    _01 = 1,
    #[doc = "2: 3 Tq"]
    _10 = 2,
    #[doc = "3: 4 Tq"]
    _11 = 3,
}
impl From<SJW_A> for u8 {
    #[inline(always)]
    fn from(variant: SJW_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SJW_A {
    type Ux = u8;
}
impl SJW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SJW_A {
        match self.bits {
            0 => SJW_A::_00,
            1 => SJW_A::_01,
            2 => SJW_A::_10,
            3 => SJW_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "1 Tq"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SJW_A::_00
    }
    #[doc = "2 Tq"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SJW_A::_01
    }
    #[doc = "3 Tq"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SJW_A::_10
    }
    #[doc = "4 Tq"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SJW_A::_11
    }
}
#[doc = "Field `SJW` writer - Resynchronization Jump Width Control"]
pub type SJW_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, SJW_A>;
impl<'a, REG, const O: u8> SJW_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 Tq"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(SJW_A::_00)
    }
    #[doc = "2 Tq"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(SJW_A::_01)
    }
    #[doc = "3 Tq"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(SJW_A::_10)
    }
    #[doc = "4 Tq"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(SJW_A::_11)
    }
}
#[doc = "Field `BRP` reader - Prescaler Division Ratio Select . These bits set the frequency of the CAN communication clock (fCANCLK)."]
pub type BRP_R = crate::FieldReader<u16>;
#[doc = "Field `BRP` writer - Prescaler Division Ratio Select . These bits set the frequency of the CAN communication clock (fCANCLK)."]
pub type BRP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `TSEG1` reader - Time Segment 1 Control"]
pub type TSEG1_R = crate::FieldReader<TSEG1_A>;
#[doc = "Time Segment 1 Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSEG1_A {
    #[doc = "3: 4 Tq"]
    _0011 = 3,
    #[doc = "4: 5 Tq"]
    _0100 = 4,
    #[doc = "5: 6 Tq"]
    _0101 = 5,
    #[doc = "6: 7 Tq"]
    _0110 = 6,
    #[doc = "7: 8 Tq"]
    _0111 = 7,
    #[doc = "8: 9 Tq"]
    _1000 = 8,
    #[doc = "9: 10 Tq"]
    _1001 = 9,
    #[doc = "10: 11 Tq"]
    _1010 = 10,
    #[doc = "11: 12 Tq"]
    _1011 = 11,
    #[doc = "12: 13 Tq"]
    _1100 = 12,
    #[doc = "13: 14 Tq"]
    _1101 = 13,
    #[doc = "14: 15 Tq"]
    _1110 = 14,
    #[doc = "15: 16 Tq"]
    _1111 = 15,
}
impl From<TSEG1_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEG1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSEG1_A {
    type Ux = u8;
}
impl TSEG1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSEG1_A> {
        match self.bits {
            3 => Some(TSEG1_A::_0011),
            4 => Some(TSEG1_A::_0100),
            5 => Some(TSEG1_A::_0101),
            6 => Some(TSEG1_A::_0110),
            7 => Some(TSEG1_A::_0111),
            8 => Some(TSEG1_A::_1000),
            9 => Some(TSEG1_A::_1001),
            10 => Some(TSEG1_A::_1010),
            11 => Some(TSEG1_A::_1011),
            12 => Some(TSEG1_A::_1100),
            13 => Some(TSEG1_A::_1101),
            14 => Some(TSEG1_A::_1110),
            15 => Some(TSEG1_A::_1111),
            _ => None,
        }
    }
    #[doc = "4 Tq"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == TSEG1_A::_0011
    }
    #[doc = "5 Tq"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == TSEG1_A::_0100
    }
    #[doc = "6 Tq"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == TSEG1_A::_0101
    }
    #[doc = "7 Tq"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == TSEG1_A::_0110
    }
    #[doc = "8 Tq"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == TSEG1_A::_0111
    }
    #[doc = "9 Tq"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == TSEG1_A::_1000
    }
    #[doc = "10 Tq"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == TSEG1_A::_1001
    }
    #[doc = "11 Tq"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == TSEG1_A::_1010
    }
    #[doc = "12 Tq"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == TSEG1_A::_1011
    }
    #[doc = "13 Tq"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == TSEG1_A::_1100
    }
    #[doc = "14 Tq"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == TSEG1_A::_1101
    }
    #[doc = "15 Tq"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == TSEG1_A::_1110
    }
    #[doc = "16 Tq"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == TSEG1_A::_1111
    }
}
#[doc = "Field `TSEG1` writer - Time Segment 1 Control"]
pub type TSEG1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, TSEG1_A>;
impl<'a, REG, const O: u8> TSEG1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4 Tq"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(TSEG1_A::_0011)
    }
    #[doc = "5 Tq"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(TSEG1_A::_0100)
    }
    #[doc = "6 Tq"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(TSEG1_A::_0101)
    }
    #[doc = "7 Tq"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(TSEG1_A::_0110)
    }
    #[doc = "8 Tq"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(TSEG1_A::_0111)
    }
    #[doc = "9 Tq"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(TSEG1_A::_1000)
    }
    #[doc = "10 Tq"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(TSEG1_A::_1001)
    }
    #[doc = "11 Tq"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut crate::W<REG> {
        self.variant(TSEG1_A::_1010)
    }
    #[doc = "12 Tq"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut crate::W<REG> {
        self.variant(TSEG1_A::_1011)
    }
    #[doc = "13 Tq"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut crate::W<REG> {
        self.variant(TSEG1_A::_1100)
    }
    #[doc = "14 Tq"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut crate::W<REG> {
        self.variant(TSEG1_A::_1101)
    }
    #[doc = "15 Tq"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut crate::W<REG> {
        self.variant(TSEG1_A::_1110)
    }
    #[doc = "16 Tq"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut crate::W<REG> {
        self.variant(TSEG1_A::_1111)
    }
}
impl R {
    #[doc = "Bit 0 - CAN Clock Source Selection"]
    #[inline(always)]
    pub fn cclks(&self) -> CCLKS_R {
        CCLKS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:10 - Time Segment 2 Control"]
    #[inline(always)]
    pub fn tseg2(&self) -> TSEG2_R {
        TSEG2_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13 - Resynchronization Jump Width Control"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:25 - Prescaler Division Ratio Select . These bits set the frequency of the CAN communication clock (fCANCLK)."]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 28:31 - Time Segment 1 Control"]
    #[inline(always)]
    pub fn tseg1(&self) -> TSEG1_R {
        TSEG1_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CAN Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cclks(&mut self) -> CCLKS_W<BCR_SPEC, 0> {
        CCLKS_W::new(self)
    }
    #[doc = "Bits 8:10 - Time Segment 2 Control"]
    #[inline(always)]
    #[must_use]
    pub fn tseg2(&mut self) -> TSEG2_W<BCR_SPEC, 8> {
        TSEG2_W::new(self)
    }
    #[doc = "Bits 12:13 - Resynchronization Jump Width Control"]
    #[inline(always)]
    #[must_use]
    pub fn sjw(&mut self) -> SJW_W<BCR_SPEC, 12> {
        SJW_W::new(self)
    }
    #[doc = "Bits 16:25 - Prescaler Division Ratio Select . These bits set the frequency of the CAN communication clock (fCANCLK)."]
    #[inline(always)]
    #[must_use]
    pub fn brp(&mut self) -> BRP_W<BCR_SPEC, 16> {
        BRP_W::new(self)
    }
    #[doc = "Bits 28:31 - Time Segment 1 Control"]
    #[inline(always)]
    #[must_use]
    pub fn tseg1(&mut self) -> TSEG1_W<BCR_SPEC, 28> {
        TSEG1_W::new(self)
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
#[doc = "Bit Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BCR_SPEC;
impl crate::RegisterSpec for BCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcr::R`](R) reader structure"]
impl crate::Readable for BCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bcr::W`](W) writer structure"]
impl crate::Writable for BCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCR to value 0"]
impl crate::Resettable for BCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
