#[doc = "Register `SIMR3` reader"]
pub type R = crate::R<SIMR3_SPEC>;
#[doc = "Register `SIMR3` writer"]
pub type W = crate::W<SIMR3_SPEC>;
#[doc = "Field `IICSTAREQ` reader - Start Condition Generation"]
pub type IICSTAREQ_R = crate::BitReader<IICSTAREQ_A>;
#[doc = "Start Condition Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICSTAREQ_A {
    #[doc = "0: A start condition is not generated."]
    _0 = 0,
    #[doc = "1: A start condition is generated."]
    _1 = 1,
}
impl From<IICSTAREQ_A> for bool {
    #[inline(always)]
    fn from(variant: IICSTAREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl IICSTAREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IICSTAREQ_A {
        match self.bits {
            false => IICSTAREQ_A::_0,
            true => IICSTAREQ_A::_1,
        }
    }
    #[doc = "A start condition is not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICSTAREQ_A::_0
    }
    #[doc = "A start condition is generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICSTAREQ_A::_1
    }
}
#[doc = "Field `IICSTAREQ` writer - Start Condition Generation"]
pub type IICSTAREQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IICSTAREQ_A>;
impl<'a, REG, const O: u8> IICSTAREQ_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A start condition is not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IICSTAREQ_A::_0)
    }
    #[doc = "A start condition is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IICSTAREQ_A::_1)
    }
}
#[doc = "Field `IICRSTAREQ` reader - Restart Condition Generation"]
pub type IICRSTAREQ_R = crate::BitReader<IICRSTAREQ_A>;
#[doc = "Restart Condition Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICRSTAREQ_A {
    #[doc = "0: A restart condition is not generated."]
    _0 = 0,
    #[doc = "1: A restart condition is generated."]
    _1 = 1,
}
impl From<IICRSTAREQ_A> for bool {
    #[inline(always)]
    fn from(variant: IICRSTAREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl IICRSTAREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IICRSTAREQ_A {
        match self.bits {
            false => IICRSTAREQ_A::_0,
            true => IICRSTAREQ_A::_1,
        }
    }
    #[doc = "A restart condition is not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICRSTAREQ_A::_0
    }
    #[doc = "A restart condition is generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICRSTAREQ_A::_1
    }
}
#[doc = "Field `IICRSTAREQ` writer - Restart Condition Generation"]
pub type IICRSTAREQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IICRSTAREQ_A>;
impl<'a, REG, const O: u8> IICRSTAREQ_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A restart condition is not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IICRSTAREQ_A::_0)
    }
    #[doc = "A restart condition is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IICRSTAREQ_A::_1)
    }
}
#[doc = "Field `IICSTPREQ` reader - Stop Condition Generation"]
pub type IICSTPREQ_R = crate::BitReader<IICSTPREQ_A>;
#[doc = "Stop Condition Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICSTPREQ_A {
    #[doc = "0: A stop condition is not generated."]
    _0 = 0,
    #[doc = "1: A stop condition is generated."]
    _1 = 1,
}
impl From<IICSTPREQ_A> for bool {
    #[inline(always)]
    fn from(variant: IICSTPREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl IICSTPREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IICSTPREQ_A {
        match self.bits {
            false => IICSTPREQ_A::_0,
            true => IICSTPREQ_A::_1,
        }
    }
    #[doc = "A stop condition is not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICSTPREQ_A::_0
    }
    #[doc = "A stop condition is generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICSTPREQ_A::_1
    }
}
#[doc = "Field `IICSTPREQ` writer - Stop Condition Generation"]
pub type IICSTPREQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IICSTPREQ_A>;
impl<'a, REG, const O: u8> IICSTPREQ_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A stop condition is not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IICSTPREQ_A::_0)
    }
    #[doc = "A stop condition is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IICSTPREQ_A::_1)
    }
}
#[doc = "Field `IICSTIF` reader - Issuing of Start, Restart, or Stop Condition Completed Flag (When 0 is written to IICSTIF, it is cleared to 0.)\n\nThe field is **modified** in some way after a read operation."]
pub type IICSTIF_R = crate::BitReader<IICSTIF_A>;
#[doc = "Issuing of Start, Restart, or Stop Condition Completed Flag (When 0 is written to IICSTIF, it is cleared to 0.)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICSTIF_A {
    #[doc = "0: There are no requests for generating conditions or a condition is being generated."]
    _0 = 0,
    #[doc = "1: A start, restart, or stop condition is completely generated."]
    _1 = 1,
}
impl From<IICSTIF_A> for bool {
    #[inline(always)]
    fn from(variant: IICSTIF_A) -> Self {
        variant as u8 != 0
    }
}
impl IICSTIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IICSTIF_A {
        match self.bits {
            false => IICSTIF_A::_0,
            true => IICSTIF_A::_1,
        }
    }
    #[doc = "There are no requests for generating conditions or a condition is being generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICSTIF_A::_0
    }
    #[doc = "A start, restart, or stop condition is completely generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICSTIF_A::_1
    }
}
#[doc = "Field `IICSTIF` writer - Issuing of Start, Restart, or Stop Condition Completed Flag (When 0 is written to IICSTIF, it is cleared to 0.)"]
pub type IICSTIF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, IICSTIF_A>;
impl<'a, REG, const O: u8> IICSTIF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "There are no requests for generating conditions or a condition is being generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IICSTIF_A::_0)
    }
    #[doc = "A start, restart, or stop condition is completely generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IICSTIF_A::_1)
    }
}
#[doc = "Field `IICSDAS` reader - SDA Output Select"]
pub type IICSDAS_R = crate::FieldReader<IICSDAS_A>;
#[doc = "SDA Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IICSDAS_A {
    #[doc = "0: Serial data output"]
    _00 = 0,
    #[doc = "1: Generate a start, restart, or stop condition."]
    _01 = 1,
    #[doc = "2: Output the low level on the SSDAn pin."]
    _10 = 2,
    #[doc = "3: Place the SSDAn pin in the high-impedance state."]
    _11 = 3,
}
impl From<IICSDAS_A> for u8 {
    #[inline(always)]
    fn from(variant: IICSDAS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IICSDAS_A {
    type Ux = u8;
}
impl IICSDAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IICSDAS_A {
        match self.bits {
            0 => IICSDAS_A::_00,
            1 => IICSDAS_A::_01,
            2 => IICSDAS_A::_10,
            3 => IICSDAS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Serial data output"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == IICSDAS_A::_00
    }
    #[doc = "Generate a start, restart, or stop condition."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == IICSDAS_A::_01
    }
    #[doc = "Output the low level on the SSDAn pin."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == IICSDAS_A::_10
    }
    #[doc = "Place the SSDAn pin in the high-impedance state."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == IICSDAS_A::_11
    }
}
#[doc = "Field `IICSDAS` writer - SDA Output Select"]
pub type IICSDAS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, IICSDAS_A>;
impl<'a, REG, const O: u8> IICSDAS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Serial data output"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(IICSDAS_A::_00)
    }
    #[doc = "Generate a start, restart, or stop condition."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(IICSDAS_A::_01)
    }
    #[doc = "Output the low level on the SSDAn pin."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(IICSDAS_A::_10)
    }
    #[doc = "Place the SSDAn pin in the high-impedance state."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(IICSDAS_A::_11)
    }
}
#[doc = "Field `IICSCLS` reader - SCL Output Select"]
pub type IICSCLS_R = crate::FieldReader<IICSCLS_A>;
#[doc = "SCL Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IICSCLS_A {
    #[doc = "0: Serial clock output"]
    _00 = 0,
    #[doc = "1: Generate a start, restart, or stop condition."]
    _01 = 1,
    #[doc = "2: Output the low level on the SSCLn pin."]
    _10 = 2,
    #[doc = "3: Place the SSCLn pin in the high-impedance state."]
    _11 = 3,
}
impl From<IICSCLS_A> for u8 {
    #[inline(always)]
    fn from(variant: IICSCLS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IICSCLS_A {
    type Ux = u8;
}
impl IICSCLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IICSCLS_A {
        match self.bits {
            0 => IICSCLS_A::_00,
            1 => IICSCLS_A::_01,
            2 => IICSCLS_A::_10,
            3 => IICSCLS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Serial clock output"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == IICSCLS_A::_00
    }
    #[doc = "Generate a start, restart, or stop condition."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == IICSCLS_A::_01
    }
    #[doc = "Output the low level on the SSCLn pin."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == IICSCLS_A::_10
    }
    #[doc = "Place the SSCLn pin in the high-impedance state."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == IICSCLS_A::_11
    }
}
#[doc = "Field `IICSCLS` writer - SCL Output Select"]
pub type IICSCLS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, IICSCLS_A>;
impl<'a, REG, const O: u8> IICSCLS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Serial clock output"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(IICSCLS_A::_00)
    }
    #[doc = "Generate a start, restart, or stop condition."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(IICSCLS_A::_01)
    }
    #[doc = "Output the low level on the SSCLn pin."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(IICSCLS_A::_10)
    }
    #[doc = "Place the SSCLn pin in the high-impedance state."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(IICSCLS_A::_11)
    }
}
impl R {
    #[doc = "Bit 0 - Start Condition Generation"]
    #[inline(always)]
    pub fn iicstareq(&self) -> IICSTAREQ_R {
        IICSTAREQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Restart Condition Generation"]
    #[inline(always)]
    pub fn iicrstareq(&self) -> IICRSTAREQ_R {
        IICRSTAREQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop Condition Generation"]
    #[inline(always)]
    pub fn iicstpreq(&self) -> IICSTPREQ_R {
        IICSTPREQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Issuing of Start, Restart, or Stop Condition Completed Flag (When 0 is written to IICSTIF, it is cleared to 0.)"]
    #[inline(always)]
    pub fn iicstif(&self) -> IICSTIF_R {
        IICSTIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - SDA Output Select"]
    #[inline(always)]
    pub fn iicsdas(&self) -> IICSDAS_R {
        IICSDAS_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - SCL Output Select"]
    #[inline(always)]
    pub fn iicscls(&self) -> IICSCLS_R {
        IICSCLS_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Start Condition Generation"]
    #[inline(always)]
    #[must_use]
    pub fn iicstareq(&mut self) -> IICSTAREQ_W<SIMR3_SPEC, 0> {
        IICSTAREQ_W::new(self)
    }
    #[doc = "Bit 1 - Restart Condition Generation"]
    #[inline(always)]
    #[must_use]
    pub fn iicrstareq(&mut self) -> IICRSTAREQ_W<SIMR3_SPEC, 1> {
        IICRSTAREQ_W::new(self)
    }
    #[doc = "Bit 2 - Stop Condition Generation"]
    #[inline(always)]
    #[must_use]
    pub fn iicstpreq(&mut self) -> IICSTPREQ_W<SIMR3_SPEC, 2> {
        IICSTPREQ_W::new(self)
    }
    #[doc = "Bit 3 - Issuing of Start, Restart, or Stop Condition Completed Flag (When 0 is written to IICSTIF, it is cleared to 0.)"]
    #[inline(always)]
    #[must_use]
    pub fn iicstif(&mut self) -> IICSTIF_W<SIMR3_SPEC, 3> {
        IICSTIF_W::new(self)
    }
    #[doc = "Bits 4:5 - SDA Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn iicsdas(&mut self) -> IICSDAS_W<SIMR3_SPEC, 4> {
        IICSDAS_W::new(self)
    }
    #[doc = "Bits 6:7 - SCL Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn iicscls(&mut self) -> IICSCLS_W<SIMR3_SPEC, 6> {
        IICSCLS_W::new(self)
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
#[doc = "I2C Mode Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`simr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`simr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIMR3_SPEC;
impl crate::RegisterSpec for SIMR3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`simr3::R`](R) reader structure"]
impl crate::Readable for SIMR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`simr3::W`](W) writer structure"]
impl crate::Writable for SIMR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x08;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIMR3 to value 0"]
impl crate::Resettable for SIMR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
