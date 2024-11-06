#[doc = "Register `RYRCNT` reader"]
pub type R = crate::R<RYRCNT_SPEC>;
#[doc = "Register `RYRCNT` writer"]
pub type W = crate::W<RYRCNT_SPEC>;
#[doc = "Field `YR1` reader - 1-Year Count Counts from 0 to 9 once per year. When a carry is generated, 1 is added to the tens place."]
pub type YR1_R = crate::FieldReader;
#[doc = "Field `YR1` writer - 1-Year Count Counts from 0 to 9 once per year. When a carry is generated, 1 is added to the tens place."]
pub type YR1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `YR10` reader - 10-Year Count Counts from 0 to 9 once per carry from ones place. When a carry is generated in the tens place, 1 is added to the hundreds place."]
pub type YR10_R = crate::FieldReader;
#[doc = "Field `YR10` writer - 10-Year Count Counts from 0 to 9 once per carry from ones place. When a carry is generated in the tens place, 1 is added to the hundreds place."]
pub type YR10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - 1-Year Count Counts from 0 to 9 once per year. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    pub fn yr1(&self) -> YR1_R {
        YR1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 10-Year Count Counts from 0 to 9 once per carry from ones place. When a carry is generated in the tens place, 1 is added to the hundreds place."]
    #[inline(always)]
    pub fn yr10(&self) -> YR10_R {
        YR10_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-Year Count Counts from 0 to 9 once per year. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    #[must_use]
    pub fn yr1(&mut self) -> YR1_W<RYRCNT_SPEC, 0> {
        YR1_W::new(self)
    }
    #[doc = "Bits 4:7 - 10-Year Count Counts from 0 to 9 once per carry from ones place. When a carry is generated in the tens place, 1 is added to the hundreds place."]
    #[inline(always)]
    #[must_use]
    pub fn yr10(&mut self) -> YR10_W<RYRCNT_SPEC, 4> {
        YR10_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Year Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ryrcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ryrcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RYRCNT_SPEC;
impl crate::RegisterSpec for RYRCNT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ryrcnt::R`](R) reader structure"]
impl crate::Readable for RYRCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ryrcnt::W`](W) writer structure"]
impl crate::Writable for RYRCNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RYRCNT to value 0"]
impl crate::Resettable for RYRCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
