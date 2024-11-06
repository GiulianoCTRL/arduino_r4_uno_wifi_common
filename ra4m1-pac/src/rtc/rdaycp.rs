#[doc = "Register `RDAYCP%s` reader"]
pub type R = crate::R<RDAYCP_SPEC>;
#[doc = "Field `DATE1` reader - 1-Day Capture Capture value for the ones place of minutes"]
pub type DATE1_R = crate::FieldReader;
#[doc = "Field `DATE10` reader - 10-Day Capture Capture value for the tens place of minutes"]
pub type DATE10_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - 1-Day Capture Capture value for the ones place of minutes"]
    #[inline(always)]
    pub fn date1(&self) -> DATE1_R {
        DATE1_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - 10-Day Capture Capture value for the tens place of minutes"]
    #[inline(always)]
    pub fn date10(&self) -> DATE10_R {
        DATE10_R::new((self.bits >> 4) & 3)
    }
}
#[doc = "Date Capture Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdaycp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDAYCP_SPEC;
impl crate::RegisterSpec for RDAYCP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rdaycp::R`](R) reader structure"]
impl crate::Readable for RDAYCP_SPEC {}
#[doc = "`reset()` method sets RDAYCP%s to value 0"]
impl crate::Resettable for RDAYCP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
