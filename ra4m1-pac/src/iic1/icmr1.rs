#[doc = "Register `ICMR1` reader"]
pub type R = crate::R<ICMR1_SPEC>;
#[doc = "Register `ICMR1` writer"]
pub type W = crate::W<ICMR1_SPEC>;
#[doc = "Field `BC` reader - Bit Counter"]
pub type BC_R = crate::FieldReader<BC_A>;
#[doc = "Bit Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BC_A {
    #[doc = "0: 9 bits"]
    _000 = 0,
    #[doc = "1: 2 bits"]
    _001 = 1,
    #[doc = "2: 3 bits"]
    _010 = 2,
    #[doc = "3: 4 bits"]
    _011 = 3,
    #[doc = "4: 5 bits"]
    _100 = 4,
    #[doc = "5: 6 bits"]
    _101 = 5,
    #[doc = "6: 7 bits"]
    _110 = 6,
    #[doc = "7: 8 bits"]
    _111 = 7,
}
impl From<BC_A> for u8 {
    #[inline(always)]
    fn from(variant: BC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BC_A {
    type Ux = u8;
}
impl BC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BC_A {
        match self.bits {
            0 => BC_A::_000,
            1 => BC_A::_001,
            2 => BC_A::_010,
            3 => BC_A::_011,
            4 => BC_A::_100,
            5 => BC_A::_101,
            6 => BC_A::_110,
            7 => BC_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "9 bits"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == BC_A::_000
    }
    #[doc = "2 bits"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == BC_A::_001
    }
    #[doc = "3 bits"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == BC_A::_010
    }
    #[doc = "4 bits"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == BC_A::_011
    }
    #[doc = "5 bits"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == BC_A::_100
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == BC_A::_101
    }
    #[doc = "7 bits"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == BC_A::_110
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == BC_A::_111
    }
}
#[doc = "Field `BC` writer - Bit Counter"]
pub type BC_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, BC_A>;
impl<'a, REG, const O: u8> BC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "9 bits"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(BC_A::_000)
    }
    #[doc = "2 bits"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(BC_A::_001)
    }
    #[doc = "3 bits"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(BC_A::_010)
    }
    #[doc = "4 bits"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(BC_A::_011)
    }
    #[doc = "5 bits"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(BC_A::_100)
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(BC_A::_101)
    }
    #[doc = "7 bits"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(BC_A::_110)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(BC_A::_111)
    }
}
#[doc = "BC Write Protect (This bit is read as 1.)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCWP_AW {
    #[doc = "0: Enables a value to be written in the BC\\[2:0\\]
bits."]
    _0 = 0,
    #[doc = "1: Disables a value to be written in the BC\\[2:0\\]
bits."]
    _1 = 1,
}
impl From<BCWP_AW> for bool {
    #[inline(always)]
    fn from(variant: BCWP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCWP` writer - BC Write Protect (This bit is read as 1.)"]
pub type BCWP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BCWP_AW>;
impl<'a, REG, const O: u8> BCWP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enables a value to be written in the BC\\[2:0\\]
bits."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BCWP_AW::_0)
    }
    #[doc = "Disables a value to be written in the BC\\[2:0\\]
bits."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BCWP_AW::_1)
    }
}
#[doc = "Field `CKS` reader - Internal Reference Clock (fIIC) Selection ( fIIC = PCLKB / 2^CKS )"]
pub type CKS_R = crate::FieldReader<CKS_A>;
#[doc = "Internal Reference Clock (fIIC) Selection ( fIIC = PCLKB / 2^CKS )\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKS_A {
    #[doc = "0: PCLKB/1 clock"]
    _000 = 0,
    #[doc = "1: PCLKB/2 clock"]
    _001 = 1,
    #[doc = "2: PCLKB/4 clock"]
    _010 = 2,
    #[doc = "3: PCLKB/8 clock"]
    _011 = 3,
    #[doc = "4: PCLKB/16 clock"]
    _100 = 4,
    #[doc = "5: PCLKB/32 clock"]
    _101 = 5,
    #[doc = "6: PCLKB/64 clock"]
    _110 = 6,
    #[doc = "7: PCLKB/128 clock"]
    _111 = 7,
}
impl From<CKS_A> for u8 {
    #[inline(always)]
    fn from(variant: CKS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKS_A {
    type Ux = u8;
}
impl CKS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CKS_A {
        match self.bits {
            0 => CKS_A::_000,
            1 => CKS_A::_001,
            2 => CKS_A::_010,
            3 => CKS_A::_011,
            4 => CKS_A::_100,
            5 => CKS_A::_101,
            6 => CKS_A::_110,
            7 => CKS_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLKB/1 clock"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CKS_A::_000
    }
    #[doc = "PCLKB/2 clock"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CKS_A::_001
    }
    #[doc = "PCLKB/4 clock"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CKS_A::_010
    }
    #[doc = "PCLKB/8 clock"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CKS_A::_011
    }
    #[doc = "PCLKB/16 clock"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CKS_A::_100
    }
    #[doc = "PCLKB/32 clock"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == CKS_A::_101
    }
    #[doc = "PCLKB/64 clock"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == CKS_A::_110
    }
    #[doc = "PCLKB/128 clock"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == CKS_A::_111
    }
}
#[doc = "Field `CKS` writer - Internal Reference Clock (fIIC) Selection ( fIIC = PCLKB / 2^CKS )"]
pub type CKS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, CKS_A>;
impl<'a, REG, const O: u8> CKS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLKB/1 clock"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_000)
    }
    #[doc = "PCLKB/2 clock"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_001)
    }
    #[doc = "PCLKB/4 clock"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_010)
    }
    #[doc = "PCLKB/8 clock"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_011)
    }
    #[doc = "PCLKB/16 clock"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_100)
    }
    #[doc = "PCLKB/32 clock"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_101)
    }
    #[doc = "PCLKB/64 clock"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_110)
    }
    #[doc = "PCLKB/128 clock"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_111)
    }
}
#[doc = "Field `MTWP` reader - MST/TRS Write Protect"]
pub type MTWP_R = crate::BitReader<MTWP_A>;
#[doc = "MST/TRS Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTWP_A {
    #[doc = "0: Disables writing to the MST and TRS bits in ICCR2."]
    _0 = 0,
    #[doc = "1: Enables writing to the MST and TRS bits in ICCR2."]
    _1 = 1,
}
impl From<MTWP_A> for bool {
    #[inline(always)]
    fn from(variant: MTWP_A) -> Self {
        variant as u8 != 0
    }
}
impl MTWP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MTWP_A {
        match self.bits {
            false => MTWP_A::_0,
            true => MTWP_A::_1,
        }
    }
    #[doc = "Disables writing to the MST and TRS bits in ICCR2."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTWP_A::_0
    }
    #[doc = "Enables writing to the MST and TRS bits in ICCR2."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTWP_A::_1
    }
}
#[doc = "Field `MTWP` writer - MST/TRS Write Protect"]
pub type MTWP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MTWP_A>;
impl<'a, REG, const O: u8> MTWP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables writing to the MST and TRS bits in ICCR2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MTWP_A::_0)
    }
    #[doc = "Enables writing to the MST and TRS bits in ICCR2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MTWP_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Bit Counter"]
    #[inline(always)]
    pub fn bc(&self) -> BC_R {
        BC_R::new(self.bits & 7)
    }
    #[doc = "Bits 4:6 - Internal Reference Clock (fIIC) Selection ( fIIC = PCLKB / 2^CKS )"]
    #[inline(always)]
    pub fn cks(&self) -> CKS_R {
        CKS_R::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7 - MST/TRS Write Protect"]
    #[inline(always)]
    pub fn mtwp(&self) -> MTWP_R {
        MTWP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Bit Counter"]
    #[inline(always)]
    #[must_use]
    pub fn bc(&mut self) -> BC_W<ICMR1_SPEC, 0> {
        BC_W::new(self)
    }
    #[doc = "Bit 3 - BC Write Protect (This bit is read as 1.)"]
    #[inline(always)]
    #[must_use]
    pub fn bcwp(&mut self) -> BCWP_W<ICMR1_SPEC, 3> {
        BCWP_W::new(self)
    }
    #[doc = "Bits 4:6 - Internal Reference Clock (fIIC) Selection ( fIIC = PCLKB / 2^CKS )"]
    #[inline(always)]
    #[must_use]
    pub fn cks(&mut self) -> CKS_W<ICMR1_SPEC, 4> {
        CKS_W::new(self)
    }
    #[doc = "Bit 7 - MST/TRS Write Protect"]
    #[inline(always)]
    #[must_use]
    pub fn mtwp(&mut self) -> MTWP_W<ICMR1_SPEC, 7> {
        MTWP_W::new(self)
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
#[doc = "I2C Bus Mode Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icmr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icmr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICMR1_SPEC;
impl crate::RegisterSpec for ICMR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`icmr1::R`](R) reader structure"]
impl crate::Readable for ICMR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icmr1::W`](W) writer structure"]
impl crate::Writable for ICMR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICMR1 to value 0x08"]
impl crate::Resettable for ICMR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
