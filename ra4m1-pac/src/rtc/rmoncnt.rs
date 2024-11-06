#[doc = "Register `RMONCNT` reader"]
pub type R = crate::R<RMONCNT_SPEC>;
#[doc = "Register `RMONCNT` writer"]
pub type W = crate::W<RMONCNT_SPEC>;
#[doc = "Field `MON1` reader - 1-Month Count Counts from 0 to 9 once per month. When a carry is generated, 1 is added to the tens place."]
pub type MON1_R = crate::FieldReader;
#[doc = "Field `MON1` writer - 1-Month Count Counts from 0 to 9 once per month. When a carry is generated, 1 is added to the tens place."]
pub type MON1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MON10` reader - 10-Month Count Counts from 0 to 1 once per carry from the ones place."]
pub type MON10_R = crate::BitReader;
#[doc = "Field `MON10` writer - 10-Month Count Counts from 0 to 1 once per carry from the ones place."]
pub type MON10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - 1-Month Count Counts from 0 to 9 once per month. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    pub fn mon1(&self) -> MON1_R {
        MON1_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - 10-Month Count Counts from 0 to 1 once per carry from the ones place."]
    #[inline(always)]
    pub fn mon10(&self) -> MON10_R {
        MON10_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-Month Count Counts from 0 to 9 once per month. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    #[must_use]
    pub fn mon1(&mut self) -> MON1_W<RMONCNT_SPEC, 0> {
        MON1_W::new(self)
    }
    #[doc = "Bit 4 - 10-Month Count Counts from 0 to 1 once per carry from the ones place."]
    #[inline(always)]
    #[must_use]
    pub fn mon10(&mut self) -> MON10_W<RMONCNT_SPEC, 4> {
        MON10_W::new(self)
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
#[doc = "Month Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rmoncnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rmoncnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RMONCNT_SPEC;
impl crate::RegisterSpec for RMONCNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rmoncnt::R`](R) reader structure"]
impl crate::Readable for RMONCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rmoncnt::W`](W) writer structure"]
impl crate::Writable for RMONCNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RMONCNT to value 0"]
impl crate::Resettable for RMONCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
