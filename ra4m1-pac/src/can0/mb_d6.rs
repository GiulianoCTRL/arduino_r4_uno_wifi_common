#[doc = "Register `MB%s_D6` reader"]
pub type R = crate::R<MB_D6_SPEC>;
#[doc = "Register `MB%s_D6` writer"]
pub type W = crate::W<MB_D6_SPEC>;
#[doc = "Field `DATA6` reader - Data Bytes 6 DATA6 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
pub type DATA6_R = crate::FieldReader;
#[doc = "Field `DATA6` writer - Data Bytes 6 DATA6 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
pub type DATA6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Data Bytes 6 DATA6 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub fn data6(&self) -> DATA6_R {
        DATA6_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Bytes 6 DATA6 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    #[must_use]
    pub fn data6(&mut self) -> DATA6_W<MB_D6_SPEC, 0> {
        DATA6_W::new(self)
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
#[doc = "Mailbox Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mb_d6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mb_d6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MB_D6_SPEC;
impl crate::RegisterSpec for MB_D6_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mb_d6::R`](R) reader structure"]
impl crate::Readable for MB_D6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mb_d6::W`](W) writer structure"]
impl crate::Writable for MB_D6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MB%s_D6 to value 0"]
impl crate::Resettable for MB_D6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
