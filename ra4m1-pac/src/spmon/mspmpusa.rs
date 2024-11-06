#[doc = "Register `MSPMPUSA` reader"]
pub type R = crate::R<MSPMPUSA_SPEC>;
#[doc = "Register `MSPMPUSA` writer"]
pub type W = crate::W<MSPMPUSA_SPEC>;
#[doc = "Field `MSPMPUSA` reader - Region start address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0."]
pub type MSPMPUSA_R = crate::FieldReader<u32>;
#[doc = "Field `MSPMPUSA` writer - Region start address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0."]
pub type MSPMPUSA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Region start address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    pub fn mspmpusa(&self) -> MSPMPUSA_R {
        MSPMPUSA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region start address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    #[must_use]
    pub fn mspmpusa(&mut self) -> MSPMPUSA_W<MSPMPUSA_SPEC, 0> {
        MSPMPUSA_W::new(self)
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
#[doc = "Main Stack Pointer (MSP) Monitor Start Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mspmpusa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mspmpusa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSPMPUSA_SPEC;
impl crate::RegisterSpec for MSPMPUSA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mspmpusa::R`](R) reader structure"]
impl crate::Readable for MSPMPUSA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mspmpusa::W`](W) writer structure"]
impl crate::Writable for MSPMPUSA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSPMPUSA to value 0"]
impl crate::Resettable for MSPMPUSA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
