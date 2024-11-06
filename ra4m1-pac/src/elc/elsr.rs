#[doc = "Register `ELSR%s` reader"]
pub type R = crate::R<ELSR_SPEC>;
#[doc = "Register `ELSR%s` writer"]
pub type W = crate::W<ELSR_SPEC>;
#[doc = "Field `ELS` reader - Event Link Select"]
pub type ELS_R = crate::FieldReader<ELS_A>;
#[doc = "Event Link Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ELS_A {
    #[doc = "0: Event output to the corresponding peripheral module is disabled."]
    _0X00 = 0,
}
impl From<ELS_A> for u8 {
    #[inline(always)]
    fn from(variant: ELS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ELS_A {
    type Ux = u8;
}
impl ELS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ELS_A> {
        match self.bits {
            0 => Some(ELS_A::_0X00),
            _ => None,
        }
    }
    #[doc = "Event output to the corresponding peripheral module is disabled."]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == ELS_A::_0X00
    }
}
#[doc = "Field `ELS` writer - Event Link Select"]
pub type ELS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, ELS_A>;
impl<'a, REG, const O: u8> ELS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Event output to the corresponding peripheral module is disabled."]
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut crate::W<REG> {
        self.variant(ELS_A::_0X00)
    }
}
impl R {
    #[doc = "Bits 0:7 - Event Link Select"]
    #[inline(always)]
    pub fn els(&self) -> ELS_R {
        ELS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Event Link Select"]
    #[inline(always)]
    #[must_use]
    pub fn els(&mut self) -> ELS_W<ELSR_SPEC, 0> {
        ELS_W::new(self)
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
#[doc = "Event Link Setting Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`elsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`elsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ELSR_SPEC;
impl crate::RegisterSpec for ELSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`elsr::R`](R) reader structure"]
impl crate::Readable for ELSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`elsr::W`](W) writer structure"]
impl crate::Writable for ELSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ELSR%s to value 0"]
impl crate::Resettable for ELSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
