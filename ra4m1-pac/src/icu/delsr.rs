#[doc = "Register `DELSR%s` reader"]
pub type R = crate::R<DELSR_SPEC>;
#[doc = "Register `DELSR%s` writer"]
pub type W = crate::W<DELSR_SPEC>;
#[doc = "Field `DELS` reader - Event selection to DMAC Start request"]
pub type DELS_R = crate::FieldReader<DELS_A>;
#[doc = "Event selection to DMAC Start request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DELS_A {
    #[doc = "0: Nothing is selected."]
    _0X000 = 0,
}
impl From<DELS_A> for u8 {
    #[inline(always)]
    fn from(variant: DELS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DELS_A {
    type Ux = u8;
}
impl DELS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DELS_A> {
        match self.bits {
            0 => Some(DELS_A::_0X000),
            _ => None,
        }
    }
    #[doc = "Nothing is selected."]
    #[inline(always)]
    pub fn is_0x000(&self) -> bool {
        *self == DELS_A::_0X000
    }
}
#[doc = "Field `DELS` writer - Event selection to DMAC Start request"]
pub type DELS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, DELS_A>;
impl<'a, REG, const O: u8> DELS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Nothing is selected."]
    #[inline(always)]
    pub fn _0x000(self) -> &'a mut crate::W<REG> {
        self.variant(DELS_A::_0X000)
    }
}
impl R {
    #[doc = "Bits 0:7 - Event selection to DMAC Start request"]
    #[inline(always)]
    pub fn dels(&self) -> DELS_R {
        DELS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Event selection to DMAC Start request"]
    #[inline(always)]
    #[must_use]
    pub fn dels(&mut self) -> DELS_W<DELSR_SPEC, 0> {
        DELS_W::new(self)
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
#[doc = "DMAC Event Link Setting Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`delsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`delsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DELSR_SPEC;
impl crate::RegisterSpec for DELSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`delsr::R`](R) reader structure"]
impl crate::Readable for DELSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`delsr::W`](W) writer structure"]
impl crate::Writable for DELSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DELSR%s to value 0"]
impl crate::Resettable for DELSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
