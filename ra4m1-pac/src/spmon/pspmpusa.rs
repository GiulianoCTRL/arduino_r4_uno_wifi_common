#[doc = "Register `PSPMPUSA` reader"]
pub type R = crate::R<PSPMPUSA_SPEC>;
#[doc = "Register `PSPMPUSA` writer"]
pub type W = crate::W<PSPMPUSA_SPEC>;
#[doc = "Field `PSPMPUSA` reader - Region start address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0."]
pub type PSPMPUSA_R = crate::FieldReader<u32>;
#[doc = "Field `PSPMPUSA` writer - Region start address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0."]
pub type PSPMPUSA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Region start address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    pub fn pspmpusa(&self) -> PSPMPUSA_R {
        PSPMPUSA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region start address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    #[must_use]
    pub fn pspmpusa(&mut self) -> PSPMPUSA_W<PSPMPUSA_SPEC, 0> {
        PSPMPUSA_W::new(self)
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
#[doc = "Process Stack Pointer (PSP) Monitor Start Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pspmpusa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pspmpusa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSPMPUSA_SPEC;
impl crate::RegisterSpec for PSPMPUSA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pspmpusa::R`](R) reader structure"]
impl crate::Readable for PSPMPUSA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pspmpusa::W`](W) writer structure"]
impl crate::Writable for PSPMPUSA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSPMPUSA to value 0"]
impl crate::Resettable for PSPMPUSA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
