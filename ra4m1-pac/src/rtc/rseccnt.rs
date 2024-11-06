#[doc = "Register `RSECCNT` reader"]
pub type R = crate::R<RSECCNT_SPEC>;
#[doc = "Register `RSECCNT` writer"]
pub type W = crate::W<RSECCNT_SPEC>;
#[doc = "Field `SEC1` reader - 1-Second Count Counts from 0 to 9 every second. When a carry is generated, 1 is added to the tens place."]
pub type SEC1_R = crate::FieldReader;
#[doc = "Field `SEC1` writer - 1-Second Count Counts from 0 to 9 every second. When a carry is generated, 1 is added to the tens place."]
pub type SEC1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SEC10` reader - 10-Second Count Counts from 0 to 5 for 60-second counting."]
pub type SEC10_R = crate::FieldReader;
#[doc = "Field `SEC10` writer - 10-Second Count Counts from 0 to 5 for 60-second counting."]
pub type SEC10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:3 - 1-Second Count Counts from 0 to 9 every second. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    pub fn sec1(&self) -> SEC1_R {
        SEC1_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - 10-Second Count Counts from 0 to 5 for 60-second counting."]
    #[inline(always)]
    pub fn sec10(&self) -> SEC10_R {
        SEC10_R::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-Second Count Counts from 0 to 9 every second. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    #[must_use]
    pub fn sec1(&mut self) -> SEC1_W<RSECCNT_SPEC, 0> {
        SEC1_W::new(self)
    }
    #[doc = "Bits 4:6 - 10-Second Count Counts from 0 to 5 for 60-second counting."]
    #[inline(always)]
    #[must_use]
    pub fn sec10(&mut self) -> SEC10_W<RSECCNT_SPEC, 4> {
        SEC10_W::new(self)
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
#[doc = "Second Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rseccnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rseccnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSECCNT_SPEC;
impl crate::RegisterSpec for RSECCNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rseccnt::R`](R) reader structure"]
impl crate::Readable for RSECCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rseccnt::W`](W) writer structure"]
impl crate::Writable for RSECCNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSECCNT to value 0"]
impl crate::Resettable for RSECCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
