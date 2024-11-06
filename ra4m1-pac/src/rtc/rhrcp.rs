#[doc = "Register `RHRCP%s` reader"]
pub type R = crate::R<RHRCP_SPEC>;
#[doc = "Field `HR1` reader - 1-Minute Capture Capture value for the ones place of minutes"]
pub type HR1_R = crate::FieldReader;
#[doc = "Field `HR10` reader - 10-Minute Capture Capture value for the tens place of minutes"]
pub type HR10_R = crate::FieldReader;
#[doc = "Field `PM` reader - A.m./p.m. select for time counter setting."]
pub type PM_R = crate::BitReader<PM_A>;
#[doc = "A.m./p.m. select for time counter setting.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PM_A {
    #[doc = "0: a.m."]
    _0 = 0,
    #[doc = "1: p.m."]
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
    #[doc = "a.m."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PM_A::_0
    }
    #[doc = "p.m."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PM_A::_1
    }
}
impl R {
    #[doc = "Bits 0:3 - 1-Minute Capture Capture value for the ones place of minutes"]
    #[inline(always)]
    pub fn hr1(&self) -> HR1_R {
        HR1_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - 10-Minute Capture Capture value for the tens place of minutes"]
    #[inline(always)]
    pub fn hr10(&self) -> HR10_R {
        HR10_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - A.m./p.m. select for time counter setting."]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Hour Capture Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rhrcp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RHRCP_SPEC;
impl crate::RegisterSpec for RHRCP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rhrcp::R`](R) reader structure"]
impl crate::Readable for RHRCP_SPEC {}
#[doc = "`reset()` method sets RHRCP%s to value 0"]
impl crate::Resettable for RHRCP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
