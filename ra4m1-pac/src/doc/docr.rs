#[doc = "Register `DOCR` reader"]
pub type R = crate::R<DOCR_SPEC>;
#[doc = "Register `DOCR` writer"]
pub type W = crate::W<DOCR_SPEC>;
#[doc = "Field `OMS` reader - Operating Mode Select"]
pub type OMS_R = crate::FieldReader<OMS_A>;
#[doc = "Operating Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OMS_A {
    #[doc = "0: Data comparison mode"]
    _00 = 0,
    #[doc = "1: Data addition mode"]
    _01 = 1,
    #[doc = "2: Data subtraction mode"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<OMS_A> for u8 {
    #[inline(always)]
    fn from(variant: OMS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OMS_A {
    type Ux = u8;
}
impl OMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OMS_A {
        match self.bits {
            0 => OMS_A::_00,
            1 => OMS_A::_01,
            2 => OMS_A::_10,
            3 => OMS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Data comparison mode"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OMS_A::_00
    }
    #[doc = "Data addition mode"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OMS_A::_01
    }
    #[doc = "Data subtraction mode"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OMS_A::_10
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == OMS_A::_11
    }
}
#[doc = "Field `OMS` writer - Operating Mode Select"]
pub type OMS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, OMS_A>;
impl<'a, REG, const O: u8> OMS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data comparison mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(OMS_A::_00)
    }
    #[doc = "Data addition mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(OMS_A::_01)
    }
    #[doc = "Data subtraction mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(OMS_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(OMS_A::_11)
    }
}
#[doc = "Field `DCSEL` reader - Detection Condition Select"]
pub type DCSEL_R = crate::BitReader<DCSEL_A>;
#[doc = "Detection Condition Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCSEL_A {
    #[doc = "0: DOPCF is set when data mismatch is detected."]
    _0 = 0,
    #[doc = "1: DOPCF is set when data match is detected."]
    _1 = 1,
}
impl From<DCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DCSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl DCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCSEL_A {
        match self.bits {
            false => DCSEL_A::_0,
            true => DCSEL_A::_1,
        }
    }
    #[doc = "DOPCF is set when data mismatch is detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCSEL_A::_0
    }
    #[doc = "DOPCF is set when data match is detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCSEL_A::_1
    }
}
#[doc = "Field `DCSEL` writer - Detection Condition Select"]
pub type DCSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DCSEL_A>;
impl<'a, REG, const O: u8> DCSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DOPCF is set when data mismatch is detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DCSEL_A::_0)
    }
    #[doc = "DOPCF is set when data match is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DCSEL_A::_1)
    }
}
#[doc = "Field `DOPCF` reader - Data Operation Circuit Flag Indicates the result of an operation."]
pub type DOPCF_R = crate::BitReader;
#[doc = "Field `DOPCFCL` reader - DOPCF Clear"]
pub type DOPCFCL_R = crate::BitReader<DOPCFCL_A>;
#[doc = "DOPCF Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOPCFCL_A {
    #[doc = "0: Maintains the DOPCF flag state."]
    _0 = 0,
    #[doc = "1: Clears the DOPCF flag."]
    _1 = 1,
}
impl From<DOPCFCL_A> for bool {
    #[inline(always)]
    fn from(variant: DOPCFCL_A) -> Self {
        variant as u8 != 0
    }
}
impl DOPCFCL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DOPCFCL_A {
        match self.bits {
            false => DOPCFCL_A::_0,
            true => DOPCFCL_A::_1,
        }
    }
    #[doc = "Maintains the DOPCF flag state."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOPCFCL_A::_0
    }
    #[doc = "Clears the DOPCF flag."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOPCFCL_A::_1
    }
}
#[doc = "Field `DOPCFCL` writer - DOPCF Clear"]
pub type DOPCFCL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DOPCFCL_A>;
impl<'a, REG, const O: u8> DOPCFCL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Maintains the DOPCF flag state."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DOPCFCL_A::_0)
    }
    #[doc = "Clears the DOPCF flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DOPCFCL_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Operating Mode Select"]
    #[inline(always)]
    pub fn oms(&self) -> OMS_R {
        OMS_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Detection Condition Select"]
    #[inline(always)]
    pub fn dcsel(&self) -> DCSEL_R {
        DCSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Operation Circuit Flag Indicates the result of an operation."]
    #[inline(always)]
    pub fn dopcf(&self) -> DOPCF_R {
        DOPCF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DOPCF Clear"]
    #[inline(always)]
    pub fn dopcfcl(&self) -> DOPCFCL_R {
        DOPCFCL_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Operating Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn oms(&mut self) -> OMS_W<DOCR_SPEC, 0> {
        OMS_W::new(self)
    }
    #[doc = "Bit 2 - Detection Condition Select"]
    #[inline(always)]
    #[must_use]
    pub fn dcsel(&mut self) -> DCSEL_W<DOCR_SPEC, 2> {
        DCSEL_W::new(self)
    }
    #[doc = "Bit 6 - DOPCF Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dopcfcl(&mut self) -> DOPCFCL_W<DOCR_SPEC, 6> {
        DOPCFCL_W::new(self)
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
#[doc = "DOC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`docr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`docr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOCR_SPEC;
impl crate::RegisterSpec for DOCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`docr::R`](R) reader structure"]
impl crate::Readable for DOCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`docr::W`](W) writer structure"]
impl crate::Writable for DOCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOCR to value 0"]
impl crate::Resettable for DOCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
