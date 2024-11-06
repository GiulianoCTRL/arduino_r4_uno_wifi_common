#[doc = "Register `GTCR` reader"]
pub type R = crate::R<GTCR_SPEC>;
#[doc = "Register `GTCR` writer"]
pub type W = crate::W<GTCR_SPEC>;
#[doc = "Field `CST` reader - Count Start"]
pub type CST_R = crate::BitReader<CST_A>;
#[doc = "Count Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CST_A {
    #[doc = "0: Count operation is stopped"]
    _0 = 0,
    #[doc = "1: Count operation is performed"]
    _1 = 1,
}
impl From<CST_A> for bool {
    #[inline(always)]
    fn from(variant: CST_A) -> Self {
        variant as u8 != 0
    }
}
impl CST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CST_A {
        match self.bits {
            false => CST_A::_0,
            true => CST_A::_1,
        }
    }
    #[doc = "Count operation is stopped"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CST_A::_0
    }
    #[doc = "Count operation is performed"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CST_A::_1
    }
}
#[doc = "Field `CST` writer - Count Start"]
pub type CST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CST_A>;
impl<'a, REG, const O: u8> CST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Count operation is stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CST_A::_0)
    }
    #[doc = "Count operation is performed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CST_A::_1)
    }
}
#[doc = "Field `MD` reader - Mode Select"]
pub type MD_R = crate::FieldReader<MD_A>;
#[doc = "Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MD_A {
    #[doc = "0: Saw-wave PWM mode (single buffer or double buffer possible)"]
    _000 = 0,
    #[doc = "1: Saw-wave one-shot pulse mode (fixed buffer operation)"]
    _001 = 1,
    #[doc = "2: Setting prohibited"]
    _010 = 2,
    #[doc = "3: Setting prohibited"]
    _011 = 3,
    #[doc = "4: Triangle-wave PWM mode 1 (16-bit transfer at crest) (single buffer or double buffer possible)"]
    _100 = 4,
    #[doc = "5: Triangle-wave PWM mode 2 (16-bit transfer at crest and trough) (single buffer or double buffer possible)"]
    _101 = 5,
    #[doc = "6: Triangle-wave PWM mode 3 (32-bit transfer at trough) fixed buffer operation)"]
    _110 = 6,
    #[doc = "7: Setting prohibited"]
    _111 = 7,
}
impl From<MD_A> for u8 {
    #[inline(always)]
    fn from(variant: MD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MD_A {
    type Ux = u8;
}
impl MD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MD_A {
        match self.bits {
            0 => MD_A::_000,
            1 => MD_A::_001,
            2 => MD_A::_010,
            3 => MD_A::_011,
            4 => MD_A::_100,
            5 => MD_A::_101,
            6 => MD_A::_110,
            7 => MD_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Saw-wave PWM mode (single buffer or double buffer possible)"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == MD_A::_000
    }
    #[doc = "Saw-wave one-shot pulse mode (fixed buffer operation)"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == MD_A::_001
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == MD_A::_010
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == MD_A::_011
    }
    #[doc = "Triangle-wave PWM mode 1 (16-bit transfer at crest) (single buffer or double buffer possible)"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == MD_A::_100
    }
    #[doc = "Triangle-wave PWM mode 2 (16-bit transfer at crest and trough) (single buffer or double buffer possible)"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == MD_A::_101
    }
    #[doc = "Triangle-wave PWM mode 3 (32-bit transfer at trough) fixed buffer operation)"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == MD_A::_110
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == MD_A::_111
    }
}
#[doc = "Field `MD` writer - Mode Select"]
pub type MD_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, MD_A>;
impl<'a, REG, const O: u8> MD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Saw-wave PWM mode (single buffer or double buffer possible)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(MD_A::_000)
    }
    #[doc = "Saw-wave one-shot pulse mode (fixed buffer operation)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(MD_A::_001)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(MD_A::_010)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(MD_A::_011)
    }
    #[doc = "Triangle-wave PWM mode 1 (16-bit transfer at crest) (single buffer or double buffer possible)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(MD_A::_100)
    }
    #[doc = "Triangle-wave PWM mode 2 (16-bit transfer at crest and trough) (single buffer or double buffer possible)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(MD_A::_101)
    }
    #[doc = "Triangle-wave PWM mode 3 (32-bit transfer at trough) fixed buffer operation)"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(MD_A::_110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(MD_A::_111)
    }
}
#[doc = "Field `TPCS` reader - Timer Prescaler Select"]
pub type TPCS_R = crate::FieldReader<TPCS_A>;
#[doc = "Timer Prescaler Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TPCS_A {
    #[doc = "0: PCLK/1"]
    _000 = 0,
    #[doc = "1: PCLK/4"]
    _001 = 1,
    #[doc = "2: PCLK/16"]
    _010 = 2,
    #[doc = "3: PCLK/64"]
    _011 = 3,
    #[doc = "4: PCLK/256"]
    _100 = 4,
    #[doc = "5: PCLK/1024"]
    _101 = 5,
}
impl From<TPCS_A> for u8 {
    #[inline(always)]
    fn from(variant: TPCS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TPCS_A {
    type Ux = u8;
}
impl TPCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TPCS_A> {
        match self.bits {
            0 => Some(TPCS_A::_000),
            1 => Some(TPCS_A::_001),
            2 => Some(TPCS_A::_010),
            3 => Some(TPCS_A::_011),
            4 => Some(TPCS_A::_100),
            5 => Some(TPCS_A::_101),
            _ => None,
        }
    }
    #[doc = "PCLK/1"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == TPCS_A::_000
    }
    #[doc = "PCLK/4"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == TPCS_A::_001
    }
    #[doc = "PCLK/16"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == TPCS_A::_010
    }
    #[doc = "PCLK/64"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == TPCS_A::_011
    }
    #[doc = "PCLK/256"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == TPCS_A::_100
    }
    #[doc = "PCLK/1024"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == TPCS_A::_101
    }
}
#[doc = "Field `TPCS` writer - Timer Prescaler Select"]
pub type TPCS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, TPCS_A>;
impl<'a, REG, const O: u8> TPCS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(TPCS_A::_000)
    }
    #[doc = "PCLK/4"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(TPCS_A::_001)
    }
    #[doc = "PCLK/16"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(TPCS_A::_010)
    }
    #[doc = "PCLK/64"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(TPCS_A::_011)
    }
    #[doc = "PCLK/256"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(TPCS_A::_100)
    }
    #[doc = "PCLK/1024"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(TPCS_A::_101)
    }
}
impl R {
    #[doc = "Bit 0 - Count Start"]
    #[inline(always)]
    pub fn cst(&self) -> CST_R {
        CST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:18 - Mode Select"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Timer Prescaler Select"]
    #[inline(always)]
    pub fn tpcs(&self) -> TPCS_R {
        TPCS_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Count Start"]
    #[inline(always)]
    #[must_use]
    pub fn cst(&mut self) -> CST_W<GTCR_SPEC, 0> {
        CST_W::new(self)
    }
    #[doc = "Bits 16:18 - Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn md(&mut self) -> MD_W<GTCR_SPEC, 16> {
        MD_W::new(self)
    }
    #[doc = "Bits 24:26 - Timer Prescaler Select"]
    #[inline(always)]
    #[must_use]
    pub fn tpcs(&mut self) -> TPCS_W<GTCR_SPEC, 24> {
        TPCS_W::new(self)
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
#[doc = "General PWM Timer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTCR_SPEC;
impl crate::RegisterSpec for GTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtcr::R`](R) reader structure"]
impl crate::Readable for GTCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gtcr::W`](W) writer structure"]
impl crate::Writable for GTCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTCR to value 0"]
impl crate::Resettable for GTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
