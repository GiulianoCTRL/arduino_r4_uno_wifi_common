#[doc = "Register `MKR[%s]` reader"]
pub type R = crate::R<MKR_SPEC>;
#[doc = "Register `MKR[%s]` writer"]
pub type W = crate::W<MKR_SPEC>;
#[doc = "Field `EID` reader - Extended ID"]
pub type EID_R = crate::FieldReader<u32>;
#[doc = "Field `EID` writer - Extended ID"]
pub type EID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 18, O, u32>;
#[doc = "Field `SID` reader - Standard ID"]
pub type SID_R = crate::FieldReader<u16>;
#[doc = "Field `SID` writer - Standard ID"]
pub type SID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
impl R {
    #[doc = "Bits 0:17 - Extended ID"]
    #[inline(always)]
    pub fn eid(&self) -> EID_R {
        EID_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:28 - Standard ID"]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:17 - Extended ID"]
    #[inline(always)]
    #[must_use]
    pub fn eid(&mut self) -> EID_W<MKR_SPEC, 0> {
        EID_W::new(self)
    }
    #[doc = "Bits 18:28 - Standard ID"]
    #[inline(always)]
    #[must_use]
    pub fn sid(&mut self) -> SID_W<MKR_SPEC, 18> {
        SID_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mkr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mkr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MKR_SPEC;
impl crate::RegisterSpec for MKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mkr::R`](R) reader structure"]
impl crate::Readable for MKR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mkr::W`](W) writer structure"]
impl crate::Writable for MKR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MKR[%s]
to value 0"]
impl crate::Resettable for MKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
