#[doc = "Register `BUS%sERRADD` reader"]
pub type R = crate::R<BUSERRADD_SPEC>;
#[doc = "Field `BERAD` reader - Bus Error Address When a bus error occurs, It stores an error address."]
pub type BERAD_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bus Error Address When a bus error occurs, It stores an error address."]
    #[inline(always)]
    pub fn berad(&self) -> BERAD_R {
        BERAD_R::new(self.bits)
    }
}
#[doc = "Bus Error Address Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buserradd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUSERRADD_SPEC;
impl crate::RegisterSpec for BUSERRADD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buserradd::R`](R) reader structure"]
impl crate::Readable for BUSERRADD_SPEC {}
#[doc = "`reset()` method sets BUS%sERRADD to value 0"]
impl crate::Resettable for BUSERRADD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
