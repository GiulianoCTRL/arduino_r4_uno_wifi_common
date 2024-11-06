#[doc = "Register `RSECCP%s` reader"]
pub type R = crate::R<RSECCP_SPEC>;
#[doc = "Field `SEC1` reader - 1-Second Capture Capture value for the ones place of seconds"]
pub type SEC1_R = crate::FieldReader;
#[doc = "Field `SEC10` reader - 10-Second Capture Capture value for the tens place of seconds"]
pub type SEC10_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - 1-Second Capture Capture value for the ones place of seconds"]
    #[inline(always)]
    pub fn sec1(&self) -> SEC1_R {
        SEC1_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - 10-Second Capture Capture value for the tens place of seconds"]
    #[inline(always)]
    pub fn sec10(&self) -> SEC10_R {
        SEC10_R::new((self.bits >> 4) & 7)
    }
}
#[doc = "Second Capture Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rseccp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSECCP_SPEC;
impl crate::RegisterSpec for RSECCP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rseccp::R`](R) reader structure"]
impl crate::Readable for RSECCP_SPEC {}
#[doc = "`reset()` method sets RSECCP%s to value 0"]
impl crate::Resettable for RSECCP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
