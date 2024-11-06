#[doc = "Register `ICMR2` reader"]
pub type R = crate::R<ICMR2_SPEC>;
#[doc = "Register `ICMR2` writer"]
pub type W = crate::W<ICMR2_SPEC>;
#[doc = "Field `TMOS` reader - Timeout Detection Time Select"]
pub type TMOS_R = crate::BitReader<TMOS_A>;
#[doc = "Timeout Detection Time Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMOS_A {
    #[doc = "0: Long mode is selected."]
    _0 = 0,
    #[doc = "1: Short mode is selected."]
    _1 = 1,
}
impl From<TMOS_A> for bool {
    #[inline(always)]
    fn from(variant: TMOS_A) -> Self {
        variant as u8 != 0
    }
}
impl TMOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TMOS_A {
        match self.bits {
            false => TMOS_A::_0,
            true => TMOS_A::_1,
        }
    }
    #[doc = "Long mode is selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMOS_A::_0
    }
    #[doc = "Short mode is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMOS_A::_1
    }
}
#[doc = "Field `TMOS` writer - Timeout Detection Time Select"]
pub type TMOS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TMOS_A>;
impl<'a, REG, const O: u8> TMOS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Long mode is selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TMOS_A::_0)
    }
    #[doc = "Short mode is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TMOS_A::_1)
    }
}
#[doc = "Field `TMOL` reader - Timeout L Count Control"]
pub type TMOL_R = crate::BitReader<TMOL_A>;
#[doc = "Timeout L Count Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMOL_A {
    #[doc = "0: Count is disabled while the SCLn line is at a low level."]
    _0 = 0,
    #[doc = "1: Count is enabled while the SCLn line is at a low level."]
    _1 = 1,
}
impl From<TMOL_A> for bool {
    #[inline(always)]
    fn from(variant: TMOL_A) -> Self {
        variant as u8 != 0
    }
}
impl TMOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TMOL_A {
        match self.bits {
            false => TMOL_A::_0,
            true => TMOL_A::_1,
        }
    }
    #[doc = "Count is disabled while the SCLn line is at a low level."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMOL_A::_0
    }
    #[doc = "Count is enabled while the SCLn line is at a low level."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMOL_A::_1
    }
}
#[doc = "Field `TMOL` writer - Timeout L Count Control"]
pub type TMOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TMOL_A>;
impl<'a, REG, const O: u8> TMOL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Count is disabled while the SCLn line is at a low level."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TMOL_A::_0)
    }
    #[doc = "Count is enabled while the SCLn line is at a low level."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TMOL_A::_1)
    }
}
#[doc = "Field `TMOH` reader - Timeout H Count Control"]
pub type TMOH_R = crate::BitReader<TMOH_A>;
#[doc = "Timeout H Count Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMOH_A {
    #[doc = "0: Count is disabled while the SCLn line is at a high level."]
    _0 = 0,
    #[doc = "1: Count is enabled while the SCLn line is at a high level."]
    _1 = 1,
}
impl From<TMOH_A> for bool {
    #[inline(always)]
    fn from(variant: TMOH_A) -> Self {
        variant as u8 != 0
    }
}
impl TMOH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TMOH_A {
        match self.bits {
            false => TMOH_A::_0,
            true => TMOH_A::_1,
        }
    }
    #[doc = "Count is disabled while the SCLn line is at a high level."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMOH_A::_0
    }
    #[doc = "Count is enabled while the SCLn line is at a high level."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMOH_A::_1
    }
}
#[doc = "Field `TMOH` writer - Timeout H Count Control"]
pub type TMOH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TMOH_A>;
impl<'a, REG, const O: u8> TMOH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Count is disabled while the SCLn line is at a high level."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TMOH_A::_0)
    }
    #[doc = "Count is enabled while the SCLn line is at a high level."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TMOH_A::_1)
    }
}
#[doc = "Field `SDDL` reader - SDA Output Delay Counter"]
pub type SDDL_R = crate::FieldReader<SDDL_A>;
#[doc = "SDA Output Delay Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDDL_A {
    #[doc = "0: No output delay"]
    _000 = 0,
    #[doc = "1: 1 fIIC cycle (ICMR2.DLCS=0) / 1 or 2 fIIC cycles (ICMR2.DLCS=1)"]
    _001 = 1,
    #[doc = "2: 2 fIIC cycles (ICMR2.DLCS=0) / 3 or 4 fIIC cycles (ICMR2.DLCS=1)"]
    _010 = 2,
    #[doc = "3: 3 fIIC cycles (ICMR2.DLCS=0) / 5 or 6 fIIC cycles (ICMR2.DLCS=1)"]
    _011 = 3,
    #[doc = "4: 4 fIIC cycles (ICMR2.DLCS=0) / 7 or 8 fIIC cycles (ICMR2.DLCS=1)"]
    _100 = 4,
    #[doc = "5: 5 fIIC cycles (ICMR2.DLCS=0) / 9 or 10 fIIC cycles (ICMR2.DLCS=1)"]
    _101 = 5,
    #[doc = "6: 6 fIIC cycles (ICMR2.DLCS=0) / 11 or 12 fIIC cycles (ICMR2.DLCS=1)"]
    _110 = 6,
    #[doc = "7: 7 fIIC cycles (ICMR2.DLCS=0) / 13 or 14 fIIC cycles (ICMR2.DLCS=1)"]
    _111 = 7,
}
impl From<SDDL_A> for u8 {
    #[inline(always)]
    fn from(variant: SDDL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SDDL_A {
    type Ux = u8;
}
impl SDDL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDDL_A {
        match self.bits {
            0 => SDDL_A::_000,
            1 => SDDL_A::_001,
            2 => SDDL_A::_010,
            3 => SDDL_A::_011,
            4 => SDDL_A::_100,
            5 => SDDL_A::_101,
            6 => SDDL_A::_110,
            7 => SDDL_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "No output delay"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SDDL_A::_000
    }
    #[doc = "1 fIIC cycle (ICMR2.DLCS=0) / 1 or 2 fIIC cycles (ICMR2.DLCS=1)"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SDDL_A::_001
    }
    #[doc = "2 fIIC cycles (ICMR2.DLCS=0) / 3 or 4 fIIC cycles (ICMR2.DLCS=1)"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SDDL_A::_010
    }
    #[doc = "3 fIIC cycles (ICMR2.DLCS=0) / 5 or 6 fIIC cycles (ICMR2.DLCS=1)"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SDDL_A::_011
    }
    #[doc = "4 fIIC cycles (ICMR2.DLCS=0) / 7 or 8 fIIC cycles (ICMR2.DLCS=1)"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SDDL_A::_100
    }
    #[doc = "5 fIIC cycles (ICMR2.DLCS=0) / 9 or 10 fIIC cycles (ICMR2.DLCS=1)"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SDDL_A::_101
    }
    #[doc = "6 fIIC cycles (ICMR2.DLCS=0) / 11 or 12 fIIC cycles (ICMR2.DLCS=1)"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SDDL_A::_110
    }
    #[doc = "7 fIIC cycles (ICMR2.DLCS=0) / 13 or 14 fIIC cycles (ICMR2.DLCS=1)"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SDDL_A::_111
    }
}
#[doc = "Field `SDDL` writer - SDA Output Delay Counter"]
pub type SDDL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, SDDL_A>;
impl<'a, REG, const O: u8> SDDL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No output delay"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(SDDL_A::_000)
    }
    #[doc = "1 fIIC cycle (ICMR2.DLCS=0) / 1 or 2 fIIC cycles (ICMR2.DLCS=1)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(SDDL_A::_001)
    }
    #[doc = "2 fIIC cycles (ICMR2.DLCS=0) / 3 or 4 fIIC cycles (ICMR2.DLCS=1)"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(SDDL_A::_010)
    }
    #[doc = "3 fIIC cycles (ICMR2.DLCS=0) / 5 or 6 fIIC cycles (ICMR2.DLCS=1)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(SDDL_A::_011)
    }
    #[doc = "4 fIIC cycles (ICMR2.DLCS=0) / 7 or 8 fIIC cycles (ICMR2.DLCS=1)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(SDDL_A::_100)
    }
    #[doc = "5 fIIC cycles (ICMR2.DLCS=0) / 9 or 10 fIIC cycles (ICMR2.DLCS=1)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(SDDL_A::_101)
    }
    #[doc = "6 fIIC cycles (ICMR2.DLCS=0) / 11 or 12 fIIC cycles (ICMR2.DLCS=1)"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(SDDL_A::_110)
    }
    #[doc = "7 fIIC cycles (ICMR2.DLCS=0) / 13 or 14 fIIC cycles (ICMR2.DLCS=1)"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(SDDL_A::_111)
    }
}
#[doc = "Field `DLCS` reader - SDA Output Delay Clock Source Select"]
pub type DLCS_R = crate::BitReader<DLCS_A>;
#[doc = "SDA Output Delay Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLCS_A {
    #[doc = "0: The internal reference clock (fIIC) is selected as the clock source of the SDA output delay counter."]
    _0 = 0,
    #[doc = "1: The internal reference clock divided by 2 (fIIC/2) is selected as the clock source of the SDA output delay counter."]
    _1 = 1,
}
impl From<DLCS_A> for bool {
    #[inline(always)]
    fn from(variant: DLCS_A) -> Self {
        variant as u8 != 0
    }
}
impl DLCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DLCS_A {
        match self.bits {
            false => DLCS_A::_0,
            true => DLCS_A::_1,
        }
    }
    #[doc = "The internal reference clock (fIIC) is selected as the clock source of the SDA output delay counter."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLCS_A::_0
    }
    #[doc = "The internal reference clock divided by 2 (fIIC/2) is selected as the clock source of the SDA output delay counter."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLCS_A::_1
    }
}
#[doc = "Field `DLCS` writer - SDA Output Delay Clock Source Select"]
pub type DLCS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DLCS_A>;
impl<'a, REG, const O: u8> DLCS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The internal reference clock (fIIC) is selected as the clock source of the SDA output delay counter."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DLCS_A::_0)
    }
    #[doc = "The internal reference clock divided by 2 (fIIC/2) is selected as the clock source of the SDA output delay counter."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DLCS_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Timeout Detection Time Select"]
    #[inline(always)]
    pub fn tmos(&self) -> TMOS_R {
        TMOS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timeout L Count Control"]
    #[inline(always)]
    pub fn tmol(&self) -> TMOL_R {
        TMOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timeout H Count Control"]
    #[inline(always)]
    pub fn tmoh(&self) -> TMOH_R {
        TMOH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SDA Output Delay Counter"]
    #[inline(always)]
    pub fn sddl(&self) -> SDDL_R {
        SDDL_R::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7 - SDA Output Delay Clock Source Select"]
    #[inline(always)]
    pub fn dlcs(&self) -> DLCS_R {
        DLCS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timeout Detection Time Select"]
    #[inline(always)]
    #[must_use]
    pub fn tmos(&mut self) -> TMOS_W<ICMR2_SPEC, 0> {
        TMOS_W::new(self)
    }
    #[doc = "Bit 1 - Timeout L Count Control"]
    #[inline(always)]
    #[must_use]
    pub fn tmol(&mut self) -> TMOL_W<ICMR2_SPEC, 1> {
        TMOL_W::new(self)
    }
    #[doc = "Bit 2 - Timeout H Count Control"]
    #[inline(always)]
    #[must_use]
    pub fn tmoh(&mut self) -> TMOH_W<ICMR2_SPEC, 2> {
        TMOH_W::new(self)
    }
    #[doc = "Bits 4:6 - SDA Output Delay Counter"]
    #[inline(always)]
    #[must_use]
    pub fn sddl(&mut self) -> SDDL_W<ICMR2_SPEC, 4> {
        SDDL_W::new(self)
    }
    #[doc = "Bit 7 - SDA Output Delay Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn dlcs(&mut self) -> DLCS_W<ICMR2_SPEC, 7> {
        DLCS_W::new(self)
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
#[doc = "I2C Bus Mode Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icmr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icmr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICMR2_SPEC;
impl crate::RegisterSpec for ICMR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`icmr2::R`](R) reader structure"]
impl crate::Readable for ICMR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icmr2::W`](W) writer structure"]
impl crate::Writable for ICMR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICMR2 to value 0x06"]
impl crate::Resettable for ICMR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x06;
}
