#[doc = "Register `D0FIFO` reader"]
pub type R = crate::R<D0FIFO_SPEC>;
#[doc = "Register `D0FIFO` writer"]
pub type W = crate::W<D0FIFO_SPEC>;
#[doc = "Field `FIFOPORT` reader - FIFO Port Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
pub type FIFOPORT_R = crate::FieldReader<u16>;
#[doc = "Field `FIFOPORT` writer - FIFO Port Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
pub type FIFOPORT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - FIFO Port Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
    #[inline(always)]
    pub fn fifoport(&self) -> FIFOPORT_R {
        FIFOPORT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - FIFO Port Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
    #[inline(always)]
    #[must_use]
    pub fn fifoport(&mut self) -> FIFOPORT_W<D0FIFO_SPEC, 0> {
        FIFOPORT_W::new(self)
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
#[doc = "D0FIFO Port Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d0fifo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d0fifo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D0FIFO_SPEC;
impl crate::RegisterSpec for D0FIFO_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`d0fifo::R`](R) reader structure"]
impl crate::Readable for D0FIFO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`d0fifo::W`](W) writer structure"]
impl crate::Writable for D0FIFO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D0FIFO to value 0"]
impl crate::Resettable for D0FIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
