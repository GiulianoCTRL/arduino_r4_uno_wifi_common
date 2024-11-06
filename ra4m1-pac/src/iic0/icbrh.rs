#[doc = "Register `ICBRH` reader"]
pub type R = crate::R<ICBRH_SPEC>;
#[doc = "Register `ICBRH` writer"]
pub type W = crate::W<ICBRH_SPEC>;
#[doc = "Field `BRH` reader - Bit Rate High-Level Period (High-level period of SCL clock)"]
pub type BRH_R = crate::FieldReader;
#[doc = "Field `BRH` writer - Bit Rate High-Level Period (High-level period of SCL clock)"]
pub type BRH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Bit Rate High-Level Period (High-level period of SCL clock)"]
    #[inline(always)]
    pub fn brh(&self) -> BRH_R {
        BRH_R::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Bit Rate High-Level Period (High-level period of SCL clock)"]
    #[inline(always)]
    #[must_use]
    pub fn brh(&mut self) -> BRH_W<ICBRH_SPEC, 0> {
        BRH_W::new(self)
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
#[doc = "I2C Bus Bit Rate High-Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icbrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icbrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICBRH_SPEC;
impl crate::RegisterSpec for ICBRH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`icbrh::R`](R) reader structure"]
impl crate::Readable for ICBRH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icbrh::W`](W) writer structure"]
impl crate::Writable for ICBRH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICBRH to value 0xff"]
impl crate::Resettable for ICBRH_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
