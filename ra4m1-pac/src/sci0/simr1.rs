#[doc = "Register `SIMR1` reader"]
pub type R = crate::R<SIMR1_SPEC>;
#[doc = "Register `SIMR1` writer"]
pub type W = crate::W<SIMR1_SPEC>;
#[doc = "Field `IICM` reader - Simple I2C Mode Select"]
pub type IICM_R = crate::BitReader<IICM_A>;
#[doc = "Simple I2C Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICM_A {
    #[doc = "0: Asynchronous mode, Multi-processor mode, Clock synchronous mode(SCMR.SMIF=0) /Smart card interface mode(SCMR.SMIF=1)"]
    _0 = 0,
    #[doc = "1: Simple I2C mode(SCMR.SMIF=0) / Setting prohibited.(SCMR.SMIF=1)"]
    _1 = 1,
}
impl From<IICM_A> for bool {
    #[inline(always)]
    fn from(variant: IICM_A) -> Self {
        variant as u8 != 0
    }
}
impl IICM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IICM_A {
        match self.bits {
            false => IICM_A::_0,
            true => IICM_A::_1,
        }
    }
    #[doc = "Asynchronous mode, Multi-processor mode, Clock synchronous mode(SCMR.SMIF=0) /Smart card interface mode(SCMR.SMIF=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICM_A::_0
    }
    #[doc = "Simple I2C mode(SCMR.SMIF=0) / Setting prohibited.(SCMR.SMIF=1)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICM_A::_1
    }
}
#[doc = "Field `IICM` writer - Simple I2C Mode Select"]
pub type IICM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IICM_A>;
impl<'a, REG, const O: u8> IICM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Asynchronous mode, Multi-processor mode, Clock synchronous mode(SCMR.SMIF=0) /Smart card interface mode(SCMR.SMIF=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IICM_A::_0)
    }
    #[doc = "Simple I2C mode(SCMR.SMIF=0) / Setting prohibited.(SCMR.SMIF=1)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IICM_A::_1)
    }
}
#[doc = "Field `IICDL` reader - SDA Delay Output Select Cycles below are of the clock signal from the on-chip baud rate generator."]
pub type IICDL_R = crate::FieldReader<IICDL_A>;
#[doc = "SDA Delay Output Select Cycles below are of the clock signal from the on-chip baud rate generator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IICDL_A {
    #[doc = "0: No output delay"]
    _00000 = 0,
}
impl From<IICDL_A> for u8 {
    #[inline(always)]
    fn from(variant: IICDL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IICDL_A {
    type Ux = u8;
}
impl IICDL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IICDL_A> {
        match self.bits {
            0 => Some(IICDL_A::_00000),
            _ => None,
        }
    }
    #[doc = "No output delay"]
    #[inline(always)]
    pub fn is_00000(&self) -> bool {
        *self == IICDL_A::_00000
    }
}
#[doc = "Field `IICDL` writer - SDA Delay Output Select Cycles below are of the clock signal from the on-chip baud rate generator."]
pub type IICDL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O, IICDL_A>;
impl<'a, REG, const O: u8> IICDL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No output delay"]
    #[inline(always)]
    pub fn _00000(self) -> &'a mut crate::W<REG> {
        self.variant(IICDL_A::_00000)
    }
}
impl R {
    #[doc = "Bit 0 - Simple I2C Mode Select"]
    #[inline(always)]
    pub fn iicm(&self) -> IICM_R {
        IICM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:7 - SDA Delay Output Select Cycles below are of the clock signal from the on-chip baud rate generator."]
    #[inline(always)]
    pub fn iicdl(&self) -> IICDL_R {
        IICDL_R::new((self.bits >> 3) & 0x1f)
    }
}
impl W {
    #[doc = "Bit 0 - Simple I2C Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn iicm(&mut self) -> IICM_W<SIMR1_SPEC, 0> {
        IICM_W::new(self)
    }
    #[doc = "Bits 3:7 - SDA Delay Output Select Cycles below are of the clock signal from the on-chip baud rate generator."]
    #[inline(always)]
    #[must_use]
    pub fn iicdl(&mut self) -> IICDL_W<SIMR1_SPEC, 3> {
        IICDL_W::new(self)
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
#[doc = "I2C Mode Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`simr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`simr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIMR1_SPEC;
impl crate::RegisterSpec for SIMR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`simr1::R`](R) reader structure"]
impl crate::Readable for SIMR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`simr1::W`](W) writer structure"]
impl crate::Writable for SIMR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIMR1 to value 0"]
impl crate::Resettable for SIMR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
