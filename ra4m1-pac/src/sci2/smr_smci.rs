#[doc = "Register `SMR_SMCI` reader"]
pub type R = crate::R<SMR_SMCI_SPEC>;
#[doc = "Register `SMR_SMCI` writer"]
pub type W = crate::W<SMR_SMCI_SPEC>;
#[doc = "Field `CKS` reader - Clock Select"]
pub type CKS_R = crate::FieldReader<CKS_A>;
#[doc = "Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKS_A {
    #[doc = "0: PCLK clock"]
    _00 = 0,
    #[doc = "1: PCLK/4 clock"]
    _01 = 1,
    #[doc = "2: PCLK/16 clock"]
    _10 = 2,
    #[doc = "3: PCLK/64 clock"]
    _11 = 3,
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
            0 => CKS_A::_00,
            1 => CKS_A::_01,
            2 => CKS_A::_10,
            3 => CKS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLK clock"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CKS_A::_00
    }
    #[doc = "PCLK/4 clock"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CKS_A::_01
    }
    #[doc = "PCLK/16 clock"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CKS_A::_10
    }
    #[doc = "PCLK/64 clock"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CKS_A::_11
    }
}
#[doc = "Field `CKS` writer - Clock Select"]
pub type CKS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, CKS_A>;
impl<'a, REG, const O: u8> CKS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK clock"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_00)
    }
    #[doc = "PCLK/4 clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_01)
    }
    #[doc = "PCLK/16 clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_10)
    }
    #[doc = "PCLK/64 clock"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_11)
    }
}
#[doc = "Field `BCP` reader - Base Clock Pulse (Valid only in asynchronous mode)"]
pub type BCP_R = crate::FieldReader<BCP_A>;
#[doc = "Base Clock Pulse (Valid only in asynchronous mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BCP_A {
    #[doc = "0: 93 clock cycles(S=93) (SCMR.BCP2=0) / 32 clock cycles(S=32) (SCMR.BCP2=1)"]
    _00 = 0,
    #[doc = "1: 128 clock cycles(S=128) (SCMR.BCP2=0) / 64 clock cycles(S=64) (SCMR.BCP2=1)"]
    _01 = 1,
    #[doc = "2: 186 clock cycles(S=186) (SCMR.BCP2=0) / 372 clock cycles(S=372) (SCMR.BCP2=1)"]
    _10 = 2,
    #[doc = "3: 512 clock cycles(S=512) (SCMR.BCP2=0) / 256 clock cycles(S=256) (SCMR.BCP2=1)"]
    _11 = 3,
}
impl From<BCP_A> for u8 {
    #[inline(always)]
    fn from(variant: BCP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BCP_A {
    type Ux = u8;
}
impl BCP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BCP_A {
        match self.bits {
            0 => BCP_A::_00,
            1 => BCP_A::_01,
            2 => BCP_A::_10,
            3 => BCP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "93 clock cycles(S=93) (SCMR.BCP2=0) / 32 clock cycles(S=32) (SCMR.BCP2=1)"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == BCP_A::_00
    }
    #[doc = "128 clock cycles(S=128) (SCMR.BCP2=0) / 64 clock cycles(S=64) (SCMR.BCP2=1)"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == BCP_A::_01
    }
    #[doc = "186 clock cycles(S=186) (SCMR.BCP2=0) / 372 clock cycles(S=372) (SCMR.BCP2=1)"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == BCP_A::_10
    }
    #[doc = "512 clock cycles(S=512) (SCMR.BCP2=0) / 256 clock cycles(S=256) (SCMR.BCP2=1)"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == BCP_A::_11
    }
}
#[doc = "Field `BCP` writer - Base Clock Pulse (Valid only in asynchronous mode)"]
pub type BCP_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, BCP_A>;
impl<'a, REG, const O: u8> BCP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "93 clock cycles(S=93) (SCMR.BCP2=0) / 32 clock cycles(S=32) (SCMR.BCP2=1)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(BCP_A::_00)
    }
    #[doc = "128 clock cycles(S=128) (SCMR.BCP2=0) / 64 clock cycles(S=64) (SCMR.BCP2=1)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(BCP_A::_01)
    }
    #[doc = "186 clock cycles(S=186) (SCMR.BCP2=0) / 372 clock cycles(S=372) (SCMR.BCP2=1)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(BCP_A::_10)
    }
    #[doc = "512 clock cycles(S=512) (SCMR.BCP2=0) / 256 clock cycles(S=256) (SCMR.BCP2=1)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(BCP_A::_11)
    }
}
#[doc = "Field `PM` reader - Parity Mode (Valid only when the PE bit is 1)"]
pub type PM_R = crate::BitReader<PM_A>;
#[doc = "Parity Mode (Valid only when the PE bit is 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PM_A {
    #[doc = "0: Selects even parity"]
    _0 = 0,
    #[doc = "1: Selects odd parity"]
    _1 = 1,
}
impl From<PM_A> for bool {
    #[inline(always)]
    fn from(variant: PM_A) -> Self {
        variant as u8 != 0
    }
}
impl PM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PM_A {
        match self.bits {
            false => PM_A::_0,
            true => PM_A::_1,
        }
    }
    #[doc = "Selects even parity"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PM_A::_0
    }
    #[doc = "Selects odd parity"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PM_A::_1
    }
}
#[doc = "Field `PM` writer - Parity Mode (Valid only when the PE bit is 1)"]
pub type PM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PM_A>;
impl<'a, REG, const O: u8> PM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selects even parity"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PM_A::_0)
    }
    #[doc = "Selects odd parity"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PM_A::_1)
    }
}
#[doc = "Field `PE` reader - Parity Enable (Valid only in asynchronous mode)"]
pub type PE_R = crate::BitReader<PE_A>;
#[doc = "Parity Enable (Valid only in asynchronous mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PE_A {
    #[doc = "0: Setting Prohibited"]
    _0 = 0,
    #[doc = "1: Set this bit to 1 in smart card interface mode."]
    _1 = 1,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        variant as u8 != 0
    }
}
impl PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PE_A {
        match self.bits {
            false => PE_A::_0,
            true => PE_A::_1,
        }
    }
    #[doc = "Setting Prohibited"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PE_A::_0
    }
    #[doc = "Set this bit to 1 in smart card interface mode."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PE_A::_1
    }
}
#[doc = "Field `PE` writer - Parity Enable (Valid only in asynchronous mode)"]
pub type PE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PE_A>;
impl<'a, REG, const O: u8> PE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Setting Prohibited"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PE_A::_0)
    }
    #[doc = "Set this bit to 1 in smart card interface mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PE_A::_1)
    }
}
#[doc = "Field `BLK` reader - Block Transfer Mode"]
pub type BLK_R = crate::BitReader<BLK_A>;
#[doc = "Block Transfer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK_A {
    #[doc = "0: Normal mode operation"]
    _0 = 0,
    #[doc = "1: Block transfer mode operation"]
    _1 = 1,
}
impl From<BLK_A> for bool {
    #[inline(always)]
    fn from(variant: BLK_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BLK_A {
        match self.bits {
            false => BLK_A::_0,
            true => BLK_A::_1,
        }
    }
    #[doc = "Normal mode operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BLK_A::_0
    }
    #[doc = "Block transfer mode operation"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BLK_A::_1
    }
}
#[doc = "Field `BLK` writer - Block Transfer Mode"]
pub type BLK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BLK_A>;
impl<'a, REG, const O: u8> BLK_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BLK_A::_0)
    }
    #[doc = "Block transfer mode operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BLK_A::_1)
    }
}
#[doc = "Field `GM` reader - GSM Mode"]
pub type GM_R = crate::BitReader<GM_A>;
#[doc = "GSM Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GM_A {
    #[doc = "0: Normal mode operation"]
    _0 = 0,
    #[doc = "1: GSM mode operation"]
    _1 = 1,
}
impl From<GM_A> for bool {
    #[inline(always)]
    fn from(variant: GM_A) -> Self {
        variant as u8 != 0
    }
}
impl GM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GM_A {
        match self.bits {
            false => GM_A::_0,
            true => GM_A::_1,
        }
    }
    #[doc = "Normal mode operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GM_A::_0
    }
    #[doc = "GSM mode operation"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GM_A::_1
    }
}
#[doc = "Field `GM` writer - GSM Mode"]
pub type GM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, GM_A>;
impl<'a, REG, const O: u8> GM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(GM_A::_0)
    }
    #[doc = "GSM mode operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(GM_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock Select"]
    #[inline(always)]
    pub fn cks(&self) -> CKS_R {
        CKS_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Base Clock Pulse (Valid only in asynchronous mode)"]
    #[inline(always)]
    pub fn bcp(&self) -> BCP_R {
        BCP_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 4 - Parity Mode (Valid only when the PE bit is 1)"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Parity Enable (Valid only in asynchronous mode)"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Block Transfer Mode"]
    #[inline(always)]
    pub fn blk(&self) -> BLK_R {
        BLK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GSM Mode"]
    #[inline(always)]
    pub fn gm(&self) -> GM_R {
        GM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn cks(&mut self) -> CKS_W<SMR_SMCI_SPEC, 0> {
        CKS_W::new(self)
    }
    #[doc = "Bits 2:3 - Base Clock Pulse (Valid only in asynchronous mode)"]
    #[inline(always)]
    #[must_use]
    pub fn bcp(&mut self) -> BCP_W<SMR_SMCI_SPEC, 2> {
        BCP_W::new(self)
    }
    #[doc = "Bit 4 - Parity Mode (Valid only when the PE bit is 1)"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<SMR_SMCI_SPEC, 4> {
        PM_W::new(self)
    }
    #[doc = "Bit 5 - Parity Enable (Valid only in asynchronous mode)"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<SMR_SMCI_SPEC, 5> {
        PE_W::new(self)
    }
    #[doc = "Bit 6 - Block Transfer Mode"]
    #[inline(always)]
    #[must_use]
    pub fn blk(&mut self) -> BLK_W<SMR_SMCI_SPEC, 6> {
        BLK_W::new(self)
    }
    #[doc = "Bit 7 - GSM Mode"]
    #[inline(always)]
    #[must_use]
    pub fn gm(&mut self) -> GM_W<SMR_SMCI_SPEC, 7> {
        GM_W::new(self)
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
#[doc = "Serial mode register (SCMR.SMIF = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smr_smci::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smr_smci::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMR_SMCI_SPEC;
impl crate::RegisterSpec for SMR_SMCI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smr_smci::R`](R) reader structure"]
impl crate::Readable for SMR_SMCI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smr_smci::W`](W) writer structure"]
impl crate::Writable for SMR_SMCI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMR_SMCI to value 0"]
impl crate::Resettable for SMR_SMCI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
