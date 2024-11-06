#[doc = "Register `SMR` reader"]
pub type R = crate::R<SMR_SPEC>;
#[doc = "Register `SMR` writer"]
pub type W = crate::W<SMR_SPEC>;
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
#[doc = "Field `MP` reader - Multi-Processor Mode (Valid only in asynchronous mode)"]
pub type MP_R = crate::BitReader<MP_A>;
#[doc = "Multi-Processor Mode (Valid only in asynchronous mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MP_A {
    #[doc = "0: Multi-processor communications function is disabled"]
    _0 = 0,
    #[doc = "1: Multi-processor communications function is enabled"]
    _1 = 1,
}
impl From<MP_A> for bool {
    #[inline(always)]
    fn from(variant: MP_A) -> Self {
        variant as u8 != 0
    }
}
impl MP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MP_A {
        match self.bits {
            false => MP_A::_0,
            true => MP_A::_1,
        }
    }
    #[doc = "Multi-processor communications function is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MP_A::_0
    }
    #[doc = "Multi-processor communications function is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MP_A::_1
    }
}
#[doc = "Field `MP` writer - Multi-Processor Mode (Valid only in asynchronous mode)"]
pub type MP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MP_A>;
impl<'a, REG, const O: u8> MP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multi-processor communications function is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MP_A::_0)
    }
    #[doc = "Multi-processor communications function is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MP_A::_1)
    }
}
#[doc = "Field `STOP` reader - Stop Bit Length (Valid only in asynchronous mode)"]
pub type STOP_R = crate::BitReader<STOP_A>;
#[doc = "Stop Bit Length (Valid only in asynchronous mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOP_A {
    #[doc = "0: 1 stop bit"]
    _0 = 0,
    #[doc = "1: 2 stop bits"]
    _1 = 1,
}
impl From<STOP_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOP_A {
        match self.bits {
            false => STOP_A::_0,
            true => STOP_A::_1,
        }
    }
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STOP_A::_0
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STOP_A::_1
    }
}
#[doc = "Field `STOP` writer - Stop Bit Length (Valid only in asynchronous mode)"]
pub type STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, STOP_A>;
impl<'a, REG, const O: u8> STOP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_A::_0)
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_A::_1)
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
    #[doc = "0: Parity bit addition is not performed (transmitting) / Parity bit checking is not performed ( receiving )"]
    _0 = 0,
    #[doc = "1: The parity bit is added (transmitting) / The parity bit is checked (receiving)"]
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
    #[doc = "Parity bit addition is not performed (transmitting) / Parity bit checking is not performed ( receiving )"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PE_A::_0
    }
    #[doc = "The parity bit is added (transmitting) / The parity bit is checked (receiving)"]
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
    #[doc = "Parity bit addition is not performed (transmitting) / Parity bit checking is not performed ( receiving )"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PE_A::_0)
    }
    #[doc = "The parity bit is added (transmitting) / The parity bit is checked (receiving)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PE_A::_1)
    }
}
#[doc = "Field `CHR` reader - Character Length (Valid only in asynchronous mode)"]
pub type CHR_R = crate::BitReader<CHR_A>;
#[doc = "Character Length (Valid only in asynchronous mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHR_A {
    #[doc = "0: Transmit/receive in 9-bit data length(SCMR.CHR1=0) / in 8bit data length(SCMR.CHR1=1)"]
    _0 = 0,
    #[doc = "1: Transmit/receive in 9-bit data length(SCMR.CHR1=0) / in 7bit data length(SCMR.CHR1=1)"]
    _1 = 1,
}
impl From<CHR_A> for bool {
    #[inline(always)]
    fn from(variant: CHR_A) -> Self {
        variant as u8 != 0
    }
}
impl CHR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHR_A {
        match self.bits {
            false => CHR_A::_0,
            true => CHR_A::_1,
        }
    }
    #[doc = "Transmit/receive in 9-bit data length(SCMR.CHR1=0) / in 8bit data length(SCMR.CHR1=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHR_A::_0
    }
    #[doc = "Transmit/receive in 9-bit data length(SCMR.CHR1=0) / in 7bit data length(SCMR.CHR1=1)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHR_A::_1
    }
}
#[doc = "Field `CHR` writer - Character Length (Valid only in asynchronous mode)"]
pub type CHR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CHR_A>;
impl<'a, REG, const O: u8> CHR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit/receive in 9-bit data length(SCMR.CHR1=0) / in 8bit data length(SCMR.CHR1=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CHR_A::_0)
    }
    #[doc = "Transmit/receive in 9-bit data length(SCMR.CHR1=0) / in 7bit data length(SCMR.CHR1=1)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CHR_A::_1)
    }
}
#[doc = "Field `CM` reader - Communication Mode"]
pub type CM_R = crate::BitReader<CM_A>;
#[doc = "Communication Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CM_A {
    #[doc = "0: Asynchronous mode or simple I2C mode"]
    _0 = 0,
    #[doc = "1: Clock synchronous mode"]
    _1 = 1,
}
impl From<CM_A> for bool {
    #[inline(always)]
    fn from(variant: CM_A) -> Self {
        variant as u8 != 0
    }
}
impl CM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CM_A {
        match self.bits {
            false => CM_A::_0,
            true => CM_A::_1,
        }
    }
    #[doc = "Asynchronous mode or simple I2C mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CM_A::_0
    }
    #[doc = "Clock synchronous mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CM_A::_1
    }
}
#[doc = "Field `CM` writer - Communication Mode"]
pub type CM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CM_A>;
impl<'a, REG, const O: u8> CM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Asynchronous mode or simple I2C mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CM_A::_0)
    }
    #[doc = "Clock synchronous mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CM_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock Select"]
    #[inline(always)]
    pub fn cks(&self) -> CKS_R {
        CKS_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Multi-Processor Mode (Valid only in asynchronous mode)"]
    #[inline(always)]
    pub fn mp(&self) -> MP_R {
        MP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stop Bit Length (Valid only in asynchronous mode)"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - Character Length (Valid only in asynchronous mode)"]
    #[inline(always)]
    pub fn chr(&self) -> CHR_R {
        CHR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Communication Mode"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn cks(&mut self) -> CKS_W<SMR_SPEC, 0> {
        CKS_W::new(self)
    }
    #[doc = "Bit 2 - Multi-Processor Mode (Valid only in asynchronous mode)"]
    #[inline(always)]
    #[must_use]
    pub fn mp(&mut self) -> MP_W<SMR_SPEC, 2> {
        MP_W::new(self)
    }
    #[doc = "Bit 3 - Stop Bit Length (Valid only in asynchronous mode)"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<SMR_SPEC, 3> {
        STOP_W::new(self)
    }
    #[doc = "Bit 4 - Parity Mode (Valid only when the PE bit is 1)"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<SMR_SPEC, 4> {
        PM_W::new(self)
    }
    #[doc = "Bit 5 - Parity Enable (Valid only in asynchronous mode)"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<SMR_SPEC, 5> {
        PE_W::new(self)
    }
    #[doc = "Bit 6 - Character Length (Valid only in asynchronous mode)"]
    #[inline(always)]
    #[must_use]
    pub fn chr(&mut self) -> CHR_W<SMR_SPEC, 6> {
        CHR_W::new(self)
    }
    #[doc = "Bit 7 - Communication Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<SMR_SPEC, 7> {
        CM_W::new(self)
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
#[doc = "Serial Mode Register (SCMR.SMIF = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMR_SPEC;
impl crate::RegisterSpec for SMR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smr::R`](R) reader structure"]
impl crate::Readable for SMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smr::W`](W) writer structure"]
impl crate::Writable for SMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMR to value 0"]
impl crate::Resettable for SMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
