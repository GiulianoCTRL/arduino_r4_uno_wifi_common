#[doc = "Register `MSPMPUEA` reader"]
pub type R = crate::R<MSPMPUEA_SPEC>;
#[doc = "Register `MSPMPUEA` writer"]
pub type W = crate::W<MSPMPUEA_SPEC>;
#[doc = "Field `MSPMPUEA` reader - Region end address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1."]
pub type MSPMPUEA_R = crate::FieldReader<u32>;
#[doc = "Field `MSPMPUEA` writer - Region end address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1."]
pub type MSPMPUEA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Region end address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    pub fn mspmpuea(&self) -> MSPMPUEA_R {
        MSPMPUEA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region end address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    #[must_use]
    pub fn mspmpuea(&mut self) -> MSPMPUEA_W<MSPMPUEA_SPEC, 0> {
        MSPMPUEA_W::new(self)
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
#[doc = "Main Stack Pointer (MSP) Monitor End Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mspmpuea::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mspmpuea::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSPMPUEA_SPEC;
impl crate::RegisterSpec for MSPMPUEA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mspmpuea::R`](R) reader structure"]
impl crate::Readable for MSPMPUEA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mspmpuea::W`](W) writer structure"]
impl crate::Writable for MSPMPUEA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSPMPUEA to value 0x03"]
impl crate::Resettable for MSPMPUEA_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
