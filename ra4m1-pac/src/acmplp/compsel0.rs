#[doc = "Register `COMPSEL0` reader"]
pub type R = crate::R<COMPSEL0_SPEC>;
#[doc = "Register `COMPSEL0` writer"]
pub type W = crate::W<COMPSEL0_SPEC>;
#[doc = "Field `CMPSEL20` reader - ACMPLP0 Input(IVCMP0) Selection"]
pub type CMPSEL20_R = crate::FieldReader<CMPSEL20_A>;
#[doc = "ACMPLP0 Input(IVCMP0) Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPSEL20_A {
    #[doc = "0: No input"]
    _000 = 0,
    #[doc = "1: CMPIN0 (P100)"]
    _001 = 1,
    #[doc = "4: CMPIN0 (P503)"]
    _100 = 4,
}
impl From<CMPSEL20_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPSEL20_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMPSEL20_A {
    type Ux = u8;
}
impl CMPSEL20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CMPSEL20_A> {
        match self.bits {
            0 => Some(CMPSEL20_A::_000),
            1 => Some(CMPSEL20_A::_001),
            4 => Some(CMPSEL20_A::_100),
            _ => None,
        }
    }
    #[doc = "No input"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CMPSEL20_A::_000
    }
    #[doc = "CMPIN0 (P100)"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CMPSEL20_A::_001
    }
    #[doc = "CMPIN0 (P503)"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CMPSEL20_A::_100
    }
}
#[doc = "Field `CMPSEL20` writer - ACMPLP0 Input(IVCMP0) Selection"]
pub type CMPSEL20_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, CMPSEL20_A>;
impl<'a, REG, const O: u8> CMPSEL20_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No input"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSEL20_A::_000)
    }
    #[doc = "CMPIN0 (P100)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSEL20_A::_001)
    }
    #[doc = "CMPIN0 (P503)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSEL20_A::_100)
    }
}
#[doc = "Field `CMPSEL64` reader - ACMPLP1 Input (IVCMP1) Selection"]
pub type CMPSEL64_R = crate::FieldReader<CMPSEL64_A>;
#[doc = "ACMPLP1 Input (IVCMP1) Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPSEL64_A {
    #[doc = "0: No input"]
    _000 = 0,
    #[doc = "1: CMPIN1 (P102)"]
    _001 = 1,
    #[doc = "4: CMPIN1 (P501)"]
    _100 = 4,
}
impl From<CMPSEL64_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPSEL64_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMPSEL64_A {
    type Ux = u8;
}
impl CMPSEL64_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CMPSEL64_A> {
        match self.bits {
            0 => Some(CMPSEL64_A::_000),
            1 => Some(CMPSEL64_A::_001),
            4 => Some(CMPSEL64_A::_100),
            _ => None,
        }
    }
    #[doc = "No input"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CMPSEL64_A::_000
    }
    #[doc = "CMPIN1 (P102)"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CMPSEL64_A::_001
    }
    #[doc = "CMPIN1 (P501)"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CMPSEL64_A::_100
    }
}
#[doc = "Field `CMPSEL64` writer - ACMPLP1 Input (IVCMP1) Selection"]
pub type CMPSEL64_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, CMPSEL64_A>;
impl<'a, REG, const O: u8> CMPSEL64_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No input"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSEL64_A::_000)
    }
    #[doc = "CMPIN1 (P102)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSEL64_A::_001)
    }
    #[doc = "CMPIN1 (P501)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSEL64_A::_100)
    }
}
impl R {
    #[doc = "Bits 0:2 - ACMPLP0 Input(IVCMP0) Selection"]
    #[inline(always)]
    pub fn cmpsel20(&self) -> CMPSEL20_R {
        CMPSEL20_R::new(self.bits & 7)
    }
    #[doc = "Bits 4:6 - ACMPLP1 Input (IVCMP1) Selection"]
    #[inline(always)]
    pub fn cmpsel64(&self) -> CMPSEL64_R {
        CMPSEL64_R::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - ACMPLP0 Input(IVCMP0) Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmpsel20(&mut self) -> CMPSEL20_W<COMPSEL0_SPEC, 0> {
        CMPSEL20_W::new(self)
    }
    #[doc = "Bits 4:6 - ACMPLP1 Input (IVCMP1) Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmpsel64(&mut self) -> CMPSEL64_W<COMPSEL0_SPEC, 4> {
        CMPSEL64_W::new(self)
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
#[doc = "Comparator Input Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`compsel0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`compsel0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMPSEL0_SPEC;
impl crate::RegisterSpec for COMPSEL0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`compsel0::R`](R) reader structure"]
impl crate::Readable for COMPSEL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`compsel0::W`](W) writer structure"]
impl crate::Writable for COMPSEL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMPSEL0 to value 0x11"]
impl crate::Resettable for COMPSEL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x11;
}
