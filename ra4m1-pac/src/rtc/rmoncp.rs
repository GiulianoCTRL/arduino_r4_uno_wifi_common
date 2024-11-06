#[doc = "Register `RMONCP%s` reader"]
pub type R = crate::R<RMONCP_SPEC>;
#[doc = "Field `MON1` reader - 1-Month Capture Capture value for the ones place of months"]
pub type MON1_R = crate::FieldReader;
#[doc = "Field `MON10` reader - 10-Month Capture Capture value for the tens place of months"]
pub type MON10_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - 1-Month Capture Capture value for the ones place of months"]
    #[inline(always)]
    pub fn mon1(&self) -> MON1_R {
        MON1_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - 10-Month Capture Capture value for the tens place of months"]
    #[inline(always)]
    pub fn mon10(&self) -> MON10_R {
        MON10_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Month Capture Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rmoncp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RMONCP_SPEC;
impl crate::RegisterSpec for RMONCP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rmoncp::R`](R) reader structure"]
impl crate::Readable for RMONCP_SPEC {}
#[doc = "`reset()` method sets RMONCP%s to value 0"]
impl crate::Resettable for RMONCP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
