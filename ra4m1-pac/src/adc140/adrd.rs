#[doc = "Register `ADRD` reader"]
pub type R = crate::R<ADRD_SPEC>;
#[doc = "Field `AD` reader - A/D-converted value (right-justified) The format for data determine ADCER.ADRFMT and ADCER.ADPRC."]
pub type AD_R = crate::FieldReader<u16>;
#[doc = "Field `DIAGST` reader - Self-Diagnosis Status"]
pub type DIAGST_R = crate::FieldReader<DIAGST_A>;
#[doc = "Self-Diagnosis Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIAGST_A {
    #[doc = "0: Self-diagnosis has never been executed since power-on."]
    _00 = 0,
    #[doc = "1: Self-diagnosis using the voltage of 0 V has been executed."]
    _01 = 1,
    #[doc = "2: Self-diagnosis using the voltage of reference power supply(VREFH) x 1/2 has been executed."]
    _10 = 2,
    #[doc = "3: Self-diagnosis using the voltage of reference power supply(VREFH) has been executed."]
    _11 = 3,
}
impl From<DIAGST_A> for u8 {
    #[inline(always)]
    fn from(variant: DIAGST_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DIAGST_A {
    type Ux = u8;
}
impl DIAGST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIAGST_A {
        match self.bits {
            0 => DIAGST_A::_00,
            1 => DIAGST_A::_01,
            2 => DIAGST_A::_10,
            3 => DIAGST_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Self-diagnosis has never been executed since power-on."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DIAGST_A::_00
    }
    #[doc = "Self-diagnosis using the voltage of 0 V has been executed."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DIAGST_A::_01
    }
    #[doc = "Self-diagnosis using the voltage of reference power supply(VREFH) x 1/2 has been executed."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DIAGST_A::_10
    }
    #[doc = "Self-diagnosis using the voltage of reference power supply(VREFH) has been executed."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DIAGST_A::_11
    }
}
impl R {
    #[doc = "Bits 0:13 - A/D-converted value (right-justified) The format for data determine ADCER.ADRFMT and ADCER.ADPRC."]
    #[inline(always)]
    pub fn ad(&self) -> AD_R {
        AD_R::new(self.bits & 0x3fff)
    }
    #[doc = "Bits 14:15 - Self-Diagnosis Status"]
    #[inline(always)]
    pub fn diagst(&self) -> DIAGST_R {
        DIAGST_R::new(((self.bits >> 14) & 3) as u8)
    }
}
#[doc = "A/D Self-Diagnosis Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adrd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADRD_SPEC;
impl crate::RegisterSpec for ADRD_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adrd::R`](R) reader structure"]
impl crate::Readable for ADRD_SPEC {}
#[doc = "`reset()` method sets ADRD to value 0"]
impl crate::Resettable for ADRD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
