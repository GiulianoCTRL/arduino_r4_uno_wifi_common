#[doc = "Register `MMPUSA%s` reader"]
pub type R = crate::R<MMPUSA_SPEC>;
#[doc = "Register `MMPUSA%s` writer"]
pub type W = crate::W<MMPUSA_SPEC>;
#[doc = "Field `MMPUSA` reader - Address where the region starts, for use in region determination. NOTE: The low-order 2 bits are fixed to 0."]
pub type MMPUSA_R = crate::FieldReader<u32>;
#[doc = "Field `MMPUSA` writer - Address where the region starts, for use in region determination. NOTE: The low-order 2 bits are fixed to 0."]
pub type MMPUSA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Address where the region starts, for use in region determination. NOTE: The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    pub fn mmpusa(&self) -> MMPUSA_R {
        MMPUSA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address where the region starts, for use in region determination. NOTE: The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    #[must_use]
    pub fn mmpusa(&mut self) -> MMPUSA_W<MMPUSA_SPEC, 0> {
        MMPUSA_W::new(self)
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
#[doc = "Group A Region %s Start Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmpusa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmpusa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMPUSA_SPEC;
impl crate::RegisterSpec for MMPUSA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmpusa::R`](R) reader structure"]
impl crate::Readable for MMPUSA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmpusa::W`](W) writer structure"]
impl crate::Writable for MMPUSA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMPUSA%s to value 0"]
impl crate::Resettable for MMPUSA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
