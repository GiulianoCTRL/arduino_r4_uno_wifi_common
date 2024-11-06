#[doc = "Register `DMTMD` reader"]
pub type R = crate::R<DMTMD_SPEC>;
#[doc = "Register `DMTMD` writer"]
pub type W = crate::W<DMTMD_SPEC>;
#[doc = "Field `DCTG` reader - Transfer Request Source Select"]
pub type DCTG_R = crate::FieldReader<DCTG_A>;
#[doc = "Transfer Request Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DCTG_A {
    #[doc = "0: Software"]
    _00 = 0,
    #[doc = "1: Interrupts*1 from peripheral modules or external interrupt input pins"]
    _01 = 1,
    #[doc = "2: Setting prohibited"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<DCTG_A> for u8 {
    #[inline(always)]
    fn from(variant: DCTG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DCTG_A {
    type Ux = u8;
}
impl DCTG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCTG_A {
        match self.bits {
            0 => DCTG_A::_00,
            1 => DCTG_A::_01,
            2 => DCTG_A::_10,
            3 => DCTG_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Software"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DCTG_A::_00
    }
    #[doc = "Interrupts*1 from peripheral modules or external interrupt input pins"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DCTG_A::_01
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DCTG_A::_10
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DCTG_A::_11
    }
}
#[doc = "Field `DCTG` writer - Transfer Request Source Select"]
pub type DCTG_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, DCTG_A>;
impl<'a, REG, const O: u8> DCTG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(DCTG_A::_00)
    }
    #[doc = "Interrupts*1 from peripheral modules or external interrupt input pins"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(DCTG_A::_01)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(DCTG_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(DCTG_A::_11)
    }
}
#[doc = "Field `SZ` reader - Transfer Data Size Select"]
pub type SZ_R = crate::FieldReader<SZ_A>;
#[doc = "Transfer Data Size Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SZ_A {
    #[doc = "0: 8 bits"]
    _00 = 0,
    #[doc = "1: 16 bits"]
    _01 = 1,
    #[doc = "2: 32 bits"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<SZ_A> for u8 {
    #[inline(always)]
    fn from(variant: SZ_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SZ_A {
    type Ux = u8;
}
impl SZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SZ_A {
        match self.bits {
            0 => SZ_A::_00,
            1 => SZ_A::_01,
            2 => SZ_A::_10,
            3 => SZ_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SZ_A::_00
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SZ_A::_01
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SZ_A::_10
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SZ_A::_11
    }
}
#[doc = "Field `SZ` writer - Transfer Data Size Select"]
pub type SZ_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, SZ_A>;
impl<'a, REG, const O: u8> SZ_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(SZ_A::_00)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(SZ_A::_01)
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(SZ_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(SZ_A::_11)
    }
}
#[doc = "Field `DTS` reader - Repeat Area Select"]
pub type DTS_R = crate::FieldReader<DTS_A>;
#[doc = "Repeat Area Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTS_A {
    #[doc = "0: The destination is specified as the repeat area or block area."]
    _00 = 0,
    #[doc = "1: The source is specified as the repeat area or block area."]
    _01 = 1,
    #[doc = "2: The repeat area or block area is not specified."]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<DTS_A> for u8 {
    #[inline(always)]
    fn from(variant: DTS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DTS_A {
    type Ux = u8;
}
impl DTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTS_A {
        match self.bits {
            0 => DTS_A::_00,
            1 => DTS_A::_01,
            2 => DTS_A::_10,
            3 => DTS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "The destination is specified as the repeat area or block area."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DTS_A::_00
    }
    #[doc = "The source is specified as the repeat area or block area."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DTS_A::_01
    }
    #[doc = "The repeat area or block area is not specified."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DTS_A::_10
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DTS_A::_11
    }
}
#[doc = "Field `DTS` writer - Repeat Area Select"]
pub type DTS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, DTS_A>;
impl<'a, REG, const O: u8> DTS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The destination is specified as the repeat area or block area."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(DTS_A::_00)
    }
    #[doc = "The source is specified as the repeat area or block area."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(DTS_A::_01)
    }
    #[doc = "The repeat area or block area is not specified."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(DTS_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(DTS_A::_11)
    }
}
#[doc = "Field `MD` reader - Transfer Mode Select"]
pub type MD_R = crate::FieldReader<MD_A>;
#[doc = "Transfer Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MD_A {
    #[doc = "0: Normal transfer"]
    _00 = 0,
    #[doc = "1: Repeat transfer"]
    _01 = 1,
    #[doc = "2: Block transfer"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
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
            0 => MD_A::_00,
            1 => MD_A::_01,
            2 => MD_A::_10,
            3 => MD_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal transfer"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == MD_A::_00
    }
    #[doc = "Repeat transfer"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == MD_A::_01
    }
    #[doc = "Block transfer"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == MD_A::_10
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == MD_A::_11
    }
}
#[doc = "Field `MD` writer - Transfer Mode Select"]
pub type MD_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, MD_A>;
impl<'a, REG, const O: u8> MD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal transfer"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(MD_A::_00)
    }
    #[doc = "Repeat transfer"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(MD_A::_01)
    }
    #[doc = "Block transfer"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(MD_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(MD_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Transfer Request Source Select"]
    #[inline(always)]
    pub fn dctg(&self) -> DCTG_R {
        DCTG_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Transfer Data Size Select"]
    #[inline(always)]
    pub fn sz(&self) -> SZ_R {
        SZ_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Repeat Area Select"]
    #[inline(always)]
    pub fn dts(&self) -> DTS_R {
        DTS_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Transfer Mode Select"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Transfer Request Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn dctg(&mut self) -> DCTG_W<DMTMD_SPEC, 0> {
        DCTG_W::new(self)
    }
    #[doc = "Bits 8:9 - Transfer Data Size Select"]
    #[inline(always)]
    #[must_use]
    pub fn sz(&mut self) -> SZ_W<DMTMD_SPEC, 8> {
        SZ_W::new(self)
    }
    #[doc = "Bits 12:13 - Repeat Area Select"]
    #[inline(always)]
    #[must_use]
    pub fn dts(&mut self) -> DTS_W<DMTMD_SPEC, 12> {
        DTS_W::new(self)
    }
    #[doc = "Bits 14:15 - Transfer Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn md(&mut self) -> MD_W<DMTMD_SPEC, 14> {
        MD_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA Transfer Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmtmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmtmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMTMD_SPEC;
impl crate::RegisterSpec for DMTMD_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dmtmd::R`](R) reader structure"]
impl crate::Readable for DMTMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmtmd::W`](W) writer structure"]
impl crate::Writable for DMTMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMTMD to value 0"]
impl crate::Resettable for DMTMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
