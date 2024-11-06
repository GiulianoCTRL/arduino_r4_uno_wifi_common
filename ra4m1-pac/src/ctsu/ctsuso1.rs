#[doc = "Register `CTSUSO1` reader"]
pub type R = crate::R<CTSUSO1_SPEC>;
#[doc = "Register `CTSUSO1` writer"]
pub type W = crate::W<CTSUSO1_SPEC>;
#[doc = "Field `CTSURICOA` reader - CTSU Reference ICO Current Adjustment Current offset amount is CTSUSO ( 0 to 255 )"]
pub type CTSURICOA_R = crate::FieldReader;
#[doc = "Field `CTSURICOA` writer - CTSU Reference ICO Current Adjustment Current offset amount is CTSUSO ( 0 to 255 )"]
pub type CTSURICOA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `CTSUSDPA` reader - CTSU Base Clock Setting Operating clock divided by ( CTSUSDPA + 1 ) x 2"]
pub type CTSUSDPA_R = crate::FieldReader;
#[doc = "Field `CTSUSDPA` writer - CTSU Base Clock Setting Operating clock divided by ( CTSUSDPA + 1 ) x 2"]
pub type CTSUSDPA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `CTSUICOG` reader - CTSU ICO Gain Adjustment"]
pub type CTSUICOG_R = crate::FieldReader<CTSUICOG_A>;
#[doc = "CTSU ICO Gain Adjustment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUICOG_A {
    #[doc = "0: 100 percent gain"]
    _00 = 0,
    #[doc = "1: 66 percent gain"]
    _01 = 1,
    #[doc = "2: 50 percent gain"]
    _10 = 2,
    #[doc = "3: 40 percent gain"]
    _11 = 3,
}
impl From<CTSUICOG_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUICOG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CTSUICOG_A {
    type Ux = u8;
}
impl CTSUICOG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CTSUICOG_A {
        match self.bits {
            0 => CTSUICOG_A::_00,
            1 => CTSUICOG_A::_01,
            2 => CTSUICOG_A::_10,
            3 => CTSUICOG_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "100 percent gain"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CTSUICOG_A::_00
    }
    #[doc = "66 percent gain"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CTSUICOG_A::_01
    }
    #[doc = "50 percent gain"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CTSUICOG_A::_10
    }
    #[doc = "40 percent gain"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CTSUICOG_A::_11
    }
}
#[doc = "Field `CTSUICOG` writer - CTSU ICO Gain Adjustment"]
pub type CTSUICOG_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, CTSUICOG_A>;
impl<'a, REG, const O: u8> CTSUICOG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "100 percent gain"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUICOG_A::_00)
    }
    #[doc = "66 percent gain"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUICOG_A::_01)
    }
    #[doc = "50 percent gain"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUICOG_A::_10)
    }
    #[doc = "40 percent gain"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUICOG_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:7 - CTSU Reference ICO Current Adjustment Current offset amount is CTSUSO ( 0 to 255 )"]
    #[inline(always)]
    pub fn ctsuricoa(&self) -> CTSURICOA_R {
        CTSURICOA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - CTSU Base Clock Setting Operating clock divided by ( CTSUSDPA + 1 ) x 2"]
    #[inline(always)]
    pub fn ctsusdpa(&self) -> CTSUSDPA_R {
        CTSUSDPA_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - CTSU ICO Gain Adjustment"]
    #[inline(always)]
    pub fn ctsuicog(&self) -> CTSUICOG_R {
        CTSUICOG_R::new(((self.bits >> 13) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CTSU Reference ICO Current Adjustment Current offset amount is CTSUSO ( 0 to 255 )"]
    #[inline(always)]
    #[must_use]
    pub fn ctsuricoa(&mut self) -> CTSURICOA_W<CTSUSO1_SPEC, 0> {
        CTSURICOA_W::new(self)
    }
    #[doc = "Bits 8:12 - CTSU Base Clock Setting Operating clock divided by ( CTSUSDPA + 1 ) x 2"]
    #[inline(always)]
    #[must_use]
    pub fn ctsusdpa(&mut self) -> CTSUSDPA_W<CTSUSO1_SPEC, 8> {
        CTSUSDPA_W::new(self)
    }
    #[doc = "Bits 13:14 - CTSU ICO Gain Adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn ctsuicog(&mut self) -> CTSUICOG_W<CTSUSO1_SPEC, 13> {
        CTSUICOG_W::new(self)
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
#[doc = "CTSU Sensor Offset Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsuso1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsuso1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTSUSO1_SPEC;
impl crate::RegisterSpec for CTSUSO1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctsuso1::R`](R) reader structure"]
impl crate::Readable for CTSUSO1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctsuso1::W`](W) writer structure"]
impl crate::Writable for CTSUSO1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUSO1 to value 0"]
impl crate::Resettable for CTSUSO1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
