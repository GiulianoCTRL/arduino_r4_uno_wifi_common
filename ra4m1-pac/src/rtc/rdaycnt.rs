#[doc = "Register `RDAYCNT` reader"]
pub type R = crate::R<RDAYCNT_SPEC>;
#[doc = "Register `RDAYCNT` writer"]
pub type W = crate::W<RDAYCNT_SPEC>;
#[doc = "Field `DATE1` reader - 1-Day Count Counts from 0 to 9 once per day. When a carry is generated, 1 is added to the tens place."]
pub type DATE1_R = crate::FieldReader;
#[doc = "Field `DATE1` writer - 1-Day Count Counts from 0 to 9 once per day. When a carry is generated, 1 is added to the tens place."]
pub type DATE1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DATE10` reader - 10-Day Count Counts from 0 to 3 once per carry from the ones place."]
pub type DATE10_R = crate::FieldReader;
#[doc = "Field `DATE10` writer - 10-Day Count Counts from 0 to 3 once per carry from the ones place."]
pub type DATE10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:3 - 1-Day Count Counts from 0 to 9 once per day. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    pub fn date1(&self) -> DATE1_R {
        DATE1_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - 10-Day Count Counts from 0 to 3 once per carry from the ones place."]
    #[inline(always)]
    pub fn date10(&self) -> DATE10_R {
        DATE10_R::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-Day Count Counts from 0 to 9 once per day. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    #[must_use]
    pub fn date1(&mut self) -> DATE1_W<RDAYCNT_SPEC, 0> {
        DATE1_W::new(self)
    }
    #[doc = "Bits 4:5 - 10-Day Count Counts from 0 to 3 once per carry from the ones place."]
    #[inline(always)]
    #[must_use]
    pub fn date10(&mut self) -> DATE10_W<RDAYCNT_SPEC, 4> {
        DATE10_W::new(self)
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
#[doc = "Day Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdaycnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdaycnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDAYCNT_SPEC;
impl crate::RegisterSpec for RDAYCNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rdaycnt::R`](R) reader structure"]
impl crate::Readable for RDAYCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rdaycnt::W`](W) writer structure"]
impl crate::Writable for RDAYCNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RDAYCNT to value 0"]
impl crate::Resettable for RDAYCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
