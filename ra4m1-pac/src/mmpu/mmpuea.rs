#[doc = "Register `MMPUEA%s` reader"]
pub type R = crate::R<MMPUEA_SPEC>;
#[doc = "Register `MMPUEA%s` writer"]
pub type W = crate::W<MMPUEA_SPEC>;
#[doc = "Field `MMPUEA` reader - Region end address register Address where the region end, for use in region determination. NOTE: The low-order 2 bits are fixed to 1."]
pub type MMPUEA_R = crate::FieldReader<u32>;
#[doc = "Field `MMPUEA` writer - Region end address register Address where the region end, for use in region determination. NOTE: The low-order 2 bits are fixed to 1."]
pub type MMPUEA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Region end address register Address where the region end, for use in region determination. NOTE: The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    pub fn mmpuea(&self) -> MMPUEA_R {
        MMPUEA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region end address register Address where the region end, for use in region determination. NOTE: The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    #[must_use]
    pub fn mmpuea(&mut self) -> MMPUEA_W<MMPUEA_SPEC, 0> {
        MMPUEA_W::new(self)
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
#[doc = "Group A Region %s End Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmpuea::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmpuea::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMPUEA_SPEC;
impl crate::RegisterSpec for MMPUEA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmpuea::R`](R) reader structure"]
impl crate::Readable for MMPUEA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmpuea::W`](W) writer structure"]
impl crate::Writable for MMPUEA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMPUEA%s to value 0x03"]
impl crate::Resettable for MMPUEA_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
