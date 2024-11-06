#[doc = "Register `BCNT3AER` reader"]
pub type R = crate::R<BCNT3AER_SPEC>;
#[doc = "Register `BCNT3AER` writer"]
pub type W = crate::W<BCNT3AER_SPEC>;
#[doc = "Field `ENB` reader - The BCNT3AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b31 to b24."]
pub type ENB_R = crate::FieldReader;
#[doc = "Field `ENB` writer - The BCNT3AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b31 to b24."]
pub type ENB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - The BCNT3AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b31 to b24."]
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - The BCNT3AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b31 to b24."]
    #[inline(always)]
    #[must_use]
    pub fn enb(&mut self) -> ENB_W<BCNT3AER_SPEC, 0> {
        ENB_W::new(self)
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
#[doc = "Binary Counter 3 Alarm Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcnt3aer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcnt3aer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BCNT3AER_SPEC;
impl crate::RegisterSpec for BCNT3AER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bcnt3aer::R`](R) reader structure"]
impl crate::Readable for BCNT3AER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bcnt3aer::W`](W) writer structure"]
impl crate::Writable for BCNT3AER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCNT3AER to value 0"]
impl crate::Resettable for BCNT3AER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
