#[doc = "Register `RYRAR` reader"]
pub type R = crate::R<RYRAR_SPEC>;
#[doc = "Register `RYRAR` writer"]
pub type W = crate::W<RYRAR_SPEC>;
#[doc = "Field `YR1` reader - 1 Year Value for the ones place of years"]
pub type YR1_R = crate::FieldReader;
#[doc = "Field `YR1` writer - 1 Year Value for the ones place of years"]
pub type YR1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `YR10` reader - 10 Years Value for the tens place of years"]
pub type YR10_R = crate::FieldReader;
#[doc = "Field `YR10` writer - 10 Years Value for the tens place of years"]
pub type YR10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - 1 Year Value for the ones place of years"]
    #[inline(always)]
    pub fn yr1(&self) -> YR1_R {
        YR1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 10 Years Value for the tens place of years"]
    #[inline(always)]
    pub fn yr10(&self) -> YR10_R {
        YR10_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1 Year Value for the ones place of years"]
    #[inline(always)]
    #[must_use]
    pub fn yr1(&mut self) -> YR1_W<RYRAR_SPEC, 0> {
        YR1_W::new(self)
    }
    #[doc = "Bits 4:7 - 10 Years Value for the tens place of years"]
    #[inline(always)]
    #[must_use]
    pub fn yr10(&mut self) -> YR10_W<RYRAR_SPEC, 4> {
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
#[doc = "Year Alarm Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ryrar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ryrar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RYRAR_SPEC;
impl crate::RegisterSpec for RYRAR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ryrar::R`](R) reader structure"]
impl crate::Readable for RYRAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ryrar::W`](W) writer structure"]
impl crate::Writable for RYRAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RYRAR to value 0"]
impl crate::Resettable for RYRAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
