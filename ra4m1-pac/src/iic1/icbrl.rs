#[doc = "Register `ICBRL` reader"]
pub type R = crate::R<ICBRL_SPEC>;
#[doc = "Register `ICBRL` writer"]
pub type W = crate::W<ICBRL_SPEC>;
#[doc = "Field `BRL` reader - Bit Rate Low-Level Period (Low-level period of SCL clock)"]
pub type BRL_R = crate::FieldReader;
#[doc = "Field `BRL` writer - Bit Rate Low-Level Period (Low-level period of SCL clock)"]
pub type BRL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Bit Rate Low-Level Period (Low-level period of SCL clock)"]
    #[inline(always)]
    pub fn brl(&self) -> BRL_R {
        BRL_R::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Bit Rate Low-Level Period (Low-level period of SCL clock)"]
    #[inline(always)]
    #[must_use]
    pub fn brl(&mut self) -> BRL_W<ICBRL_SPEC, 0> {
        BRL_W::new(self)
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
#[doc = "I2C Bus Bit Rate Low-Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icbrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icbrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICBRL_SPEC;
impl crate::RegisterSpec for ICBRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`icbrl::R`](R) reader structure"]
impl crate::Readable for ICBRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icbrl::W`](W) writer structure"]
impl crate::Writable for ICBRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICBRL to value 0xff"]
impl crate::Resettable for ICBRL_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
