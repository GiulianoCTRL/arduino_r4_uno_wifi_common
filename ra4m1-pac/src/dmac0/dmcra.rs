#[doc = "Register `DMCRA` reader"]
pub type R = crate::R<DMCRA_SPEC>;
#[doc = "Register `DMCRA` writer"]
pub type W = crate::W<DMCRA_SPEC>;
#[doc = "Field `DMCRAL` reader - Lower bits of transfer count"]
pub type DMCRAL_R = crate::FieldReader<u16>;
#[doc = "Field `DMCRAL` writer - Lower bits of transfer count"]
pub type DMCRAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `DMCRAH` reader - Upper bits of transfer count"]
pub type DMCRAH_R = crate::FieldReader<u16>;
#[doc = "Field `DMCRAH` writer - Upper bits of transfer count"]
pub type DMCRAH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Lower bits of transfer count"]
    #[inline(always)]
    pub fn dmcral(&self) -> DMCRAL_R {
        DMCRAL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:25 - Upper bits of transfer count"]
    #[inline(always)]
    pub fn dmcrah(&self) -> DMCRAH_R {
        DMCRAH_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Lower bits of transfer count"]
    #[inline(always)]
    #[must_use]
    pub fn dmcral(&mut self) -> DMCRAL_W<DMCRA_SPEC, 0> {
        DMCRAL_W::new(self)
    }
    #[doc = "Bits 16:25 - Upper bits of transfer count"]
    #[inline(always)]
    #[must_use]
    pub fn dmcrah(&mut self) -> DMCRAH_W<DMCRA_SPEC, 16> {
        DMCRAH_W::new(self)
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
#[doc = "DMA Transfer Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmcra::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmcra::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMCRA_SPEC;
impl crate::RegisterSpec for DMCRA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmcra::R`](R) reader structure"]
impl crate::Readable for DMCRA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmcra::W`](W) writer structure"]
impl crate::Writable for DMCRA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMCRA to value 0"]
impl crate::Resettable for DMCRA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
