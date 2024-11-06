#[doc = "Register `RMINCNT` reader"]
pub type R = crate::R<RMINCNT_SPEC>;
#[doc = "Register `RMINCNT` writer"]
pub type W = crate::W<RMINCNT_SPEC>;
#[doc = "Field `MIN1` reader - 1-Minute Count Counts from 0 to 9 every minute. When a carry is generated, 1 is added to the tens place."]
pub type MIN1_R = crate::FieldReader;
#[doc = "Field `MIN1` writer - 1-Minute Count Counts from 0 to 9 every minute. When a carry is generated, 1 is added to the tens place."]
pub type MIN1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MIN10` reader - 10-Minute Count Counts from 0 to 5 for 60-minute counting."]
pub type MIN10_R = crate::FieldReader;
#[doc = "Field `MIN10` writer - 10-Minute Count Counts from 0 to 5 for 60-minute counting."]
pub type MIN10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:3 - 1-Minute Count Counts from 0 to 9 every minute. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    pub fn min1(&self) -> MIN1_R {
        MIN1_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - 10-Minute Count Counts from 0 to 5 for 60-minute counting."]
    #[inline(always)]
    pub fn min10(&self) -> MIN10_R {
        MIN10_R::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-Minute Count Counts from 0 to 9 every minute. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    #[must_use]
    pub fn min1(&mut self) -> MIN1_W<RMINCNT_SPEC, 0> {
        MIN1_W::new(self)
    }
    #[doc = "Bits 4:6 - 10-Minute Count Counts from 0 to 5 for 60-minute counting."]
    #[inline(always)]
    #[must_use]
    pub fn min10(&mut self) -> MIN10_W<RMINCNT_SPEC, 4> {
        MIN10_W::new(self)
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
#[doc = "Minute Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rmincnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rmincnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RMINCNT_SPEC;
impl crate::RegisterSpec for RMINCNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rmincnt::R`](R) reader structure"]
impl crate::Readable for RMINCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rmincnt::W`](W) writer structure"]
impl crate::Writable for RMINCNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RMINCNT to value 0"]
impl crate::Resettable for RMINCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
